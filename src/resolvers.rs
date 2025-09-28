//! # GraphQL Resolvers with Database Integration
//!
//! This module implements resolvers that use persistent PostgreSQL storage
//! while maintaining smart auto-title generation and comprehensive error handling.

use async_graphql::{Context, EmptySubscription, Object, Result};
use validator::Validate;

use crate::auth::{AuthResponse, AuthService, LoginInput, RegisterInput, User};
use crate::database::Database;
use crate::errors::AppError;
use crate::types::{Note, NoteInput, UpdateNoteInput};

pub struct QueryRoot;
pub struct MutationRoot;

// ✅ Use EmptySubscription as a type, not a value
pub type SubscriptionRoot = EmptySubscription;

#[Object]
impl QueryRoot {
    /// 👋 Hello world query with database info
    async fn hello(&self) -> &'static str {
        "Hello from GraphQL with PostgreSQL database, smart auto-titles, and authentication!"
    }

    /// 📚 Get all notes (public - for now, will be user-specific later)
    async fn notes(&self, ctx: &Context<'_>) -> Result<Vec<Note>> {
        let db = ctx.data::<Database>()?;
        let notes = db.get_all_notes().await?;
        Ok(notes)
    }

    /// 🔍 Get note by ID
    async fn note(&self, ctx: &Context<'_>, id: String) -> Result<Option<Note>> {
        let db = ctx.data::<Database>()?;
        let note = db.get_note_by_id(&id).await?;
        Ok(note)
    }

    /// 🔎 Search notes with full-text search
    async fn search_notes(&self, ctx: &Context<'_>, query: String) -> Result<Vec<Note>> {
        let db = ctx.data::<Database>()?;
        let notes = db.search_notes(&query).await?;
        Ok(notes)
    }
}

#[Object]
impl MutationRoot {
    /// 📝 Create note with smart auto-title generation (SINGLE IMPLEMENTATION)
    async fn create_note(&self, ctx: &Context<'_>, input: NoteInput) -> Result<Note> {
        if input.content.is_empty() {
            return Err(AppError::InvalidContent {
                message: "Content cannot be empty".to_string(),
            }
            .into());
        }

        if let Some(title) = &input.title {
            if title.len() > 200 {
                return Err(AppError::InvalidTitle {
                    message: "Title too long (max 200 characters)".to_string(),
                }
                .into());
            }
        }

        // Smart auto-title generation
        let title = match input.title {
            Some(title) if !title.trim().is_empty() => title,
            _ => generate_smart_title(&input.content),
        };

        let db = ctx
            .data::<Database>()
            .map_err(|_| AppError::DatabaseError {
                message: "Database connection not available".to_string(),
            })?;

        // For now, create note without user authentication
        // Later we'll switch to create_note_for_user when auth is fully implemented
        db.create_note(&title, &input.content)
            .await
            .map_err(Into::into)
    }

    /// 📝 Update note
    async fn update_note(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateNoteInput,
    ) -> Result<Option<Note>> {
        let db = ctx.data::<Database>()?;
        let note = db
            .update_note(&id, input.title.as_deref(), input.content.as_deref())
            .await?;
        Ok(note)
    }

    /// 🗑️ Delete note
    async fn delete_note(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let db = ctx.data::<Database>()?;
        let deleted = db.delete_note(&id).await?;
        Ok(deleted)
    }

    /// 🔐 Register a new user
    async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<AuthResponse> {
        let db = ctx.data::<Database>()?;
        let auth = ctx.data::<AuthService>()?;

        // Create user
        let user_row = db.create_user(&input, auth).await?;
        let user = User::from(user_row.clone());

        // Generate JWT token
        let token = auth.generate_token(user_row.id, user_row.email)?;

        Ok(AuthResponse { token, user })
    }

    /// 🔑 Login user
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<AuthResponse> {
        input.validate().map_err(|e| AppError::ValidationError {
            message: format!("Validation failed: {}", e),
        })?;

        let db = ctx.data::<Database>()?;
        let auth = ctx.data::<AuthService>()?;

        // Get user by email
        let user_row = db
            .get_user_by_email(&input.email)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        // Verify password
        let is_valid = auth.verify_password(&input.password, &user_row.password_hash)?;
        if !is_valid {
            return Err(AppError::InvalidCredentials.into());
        }

        let user = User::from(user_row.clone());
        let token = auth.generate_token(user_row.id, user_row.email)?;

        Ok(AuthResponse { token, user })
    }
}

/// 🎯 Enhanced Smart Auto-Title Generation
fn generate_smart_title(content: &str) -> String {
    const MAX_LENGTH: usize = 50;

    // Clean the content first - take first line and normalize whitespace
    let cleaned = content
        .lines()
        .next()
        .unwrap_or(content)
        .trim()
        .replace('\n', " ") // ✅ Fixed: Use single quotes for char literals
        .replace('\r', " ") // ✅ Fixed: Use single quotes for char literals
        .replace('\t', " ") // ✅ Fixed: Use single quotes for char literals
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    if cleaned.is_empty() {
        return "Untitled Note".to_string();
    }

    if cleaned.len() <= MAX_LENGTH {
        return cleaned;
    }

    if let Some(sentence_end) = find_sentence_boundary(&cleaned, MAX_LENGTH) {
        return cleaned[..sentence_end].trim().to_string();
    }

    if let Some(word_end) = find_word_boundary(&cleaned, MAX_LENGTH) {
        return format!("{}...", cleaned[..word_end].trim());
    }

    let mut end = MAX_LENGTH - 3;
    while end > 0 && !cleaned.is_char_boundary(end) {
        end -= 1;
    }

    format!("{}...", cleaned[..end].trim())
}

fn find_sentence_boundary(text: &str, max_len: usize) -> Option<usize> {
    let search_area = &text[..max_len.min(text.len())];
    let mut best_pos = None;
    let mut in_quotes = false;
    let mut current_quote = None;

    for (byte_pos, ch) in search_area.char_indices() {
        match ch {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                current_quote = Some(ch);
            }
            '"' | '\'' if in_quotes && Some(ch) == current_quote => {
                in_quotes = false;
                current_quote = None;
            }
            _ => {}
        }

        if !in_quotes && matches!(ch, '.' | '!' | '?') {
            let end_pos = byte_pos + ch.len_utf8();

            let is_valid = if end_pos >= search_area.len() {
                true
            } else {
                search_area[end_pos..]
                    .chars()
                    .next()
                    .map(|next| next.is_whitespace() || next == '"' || next == '\'')
                    .unwrap_or(false)
            };

            if is_valid {
                best_pos = Some(end_pos);
            }
        }
    }

    best_pos
}

fn find_word_boundary(text: &str, max_len: usize) -> Option<usize> {
    let min_length = max_len / 3;

    text[..max_len.min(text.len())]
        .rfind(' ')
        .filter(|&pos| pos >= min_length)
}
