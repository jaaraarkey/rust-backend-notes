//! # GraphQL Resolvers with JWT Authentication
//!
//! This module implements resolvers with JWT-based authentication

use async_graphql::{Context, EmptySubscription, Object, Result};
use validator::Validate;

use crate::auth::{
    get_auth_context, require_auth, AuthResponse, AuthService, LoginInput, RegisterInput, User,
};
use crate::database::Database;
use crate::errors::{AppError, AppResult};
use crate::types::{Note, NoteInput, UpdateNoteInput};

pub struct QueryRoot;
pub struct MutationRoot;

pub type SubscriptionRoot = EmptySubscription;

#[Object]
impl QueryRoot {
    /// 👋 Hello world query with authentication info
    async fn hello(&self, ctx: &Context<'_>) -> Result<String> {
        let auth_ctx = get_auth_context(ctx)?;

        if auth_ctx.is_authenticated {
            let user = auth_ctx.require_user()?;
            Ok(format!(
                "Hello {}! You're authenticated with Smart Notes API featuring PostgreSQL, JWT auth, and AI-powered features!",
                user.email
            ))
        } else {
            Ok("Hello! Welcome to Smart Notes API - please authenticate to access personalized features.".to_string())
        }
    }

    /// 📚 Get user's notes (authenticated)
    async fn notes(&self, ctx: &Context<'_>) -> Result<Vec<Note>> {
        let (user_id, _user) = require_auth(ctx)?;
        let db = ctx.data::<Database>()?;
        let notes = db.get_user_notes(user_id).await?;
        Ok(notes)
    }

    /// 📚 Get all notes (admin/public access - remove in production)
    async fn all_notes(&self, ctx: &Context<'_>) -> Result<Vec<Note>> {
        let db = ctx.data::<Database>()?;
        let notes = db.get_all_notes().await?;
        Ok(notes)
    }

    /// 🔍 Get note by ID (user-specific)
    async fn note(&self, ctx: &Context<'_>, id: String) -> Result<Option<Note>> {
        let (user_id, _user) = require_auth(ctx)?;
        let db = ctx.data::<Database>()?;

        // First check if note exists and belongs to user
        if let Some(note) = db.get_note_by_id(&id).await? {
            // Verify note belongs to authenticated user (when we add user_id to notes)
            // For now, just return the note
            Ok(Some(note))
        } else {
            Ok(None)
        }
    }

    /// 🔎 Search user's notes with full-text search (authenticated)
    async fn search_notes(&self, ctx: &Context<'_>, query: String) -> Result<Vec<Note>> {
        let (user_id, _user) = require_auth(ctx)?;
        let db = ctx.data::<Database>()?;

        // Search only user's notes (when implemented)
        let notes = db.search_notes(&query).await?;
        Ok(notes)
    }

    /// 👤 Get current user profile
    async fn me(&self, ctx: &Context<'_>) -> Result<User> {
        let (_user_id, user) = require_auth(ctx)?;
        Ok(User::from(user.clone()))
    }
}

#[Object]
impl MutationRoot {
    /// 📝 Create note for authenticated user
    async fn create_note(&self, ctx: &Context<'_>, input: NoteInput) -> Result<Note> {
        let (user_id, _user) = require_auth(ctx)?;

        // Validate input
        validate_note_input(&input)?;

        // Smart auto-title generation
        let title = match input.title {
            Some(title) if !title.trim().is_empty() => title,
            _ => generate_smart_title(&input.content),
        };

        let db = ctx.data::<Database>()?;

        // Create note for authenticated user
        let note = db
            .create_note_for_user(user_id, &title, &input.content)
            .await?;
        Ok(note)
    }

    /// 📝 Create public note (legacy - for testing)
    async fn create_public_note(&self, ctx: &Context<'_>, input: NoteInput) -> Result<Note> {
        // Validate input
        validate_note_input(&input)?;

        // Smart auto-title generation
        let title = match input.title {
            Some(title) if !title.trim().is_empty() => title,
            _ => generate_smart_title(&input.content),
        };

        let db = ctx.data::<Database>()?;
        let note = db.create_note(&title, &input.content).await?;
        Ok(note)
    }

    /// 📝 Update user's note
    async fn update_note(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateNoteInput,
    ) -> Result<Option<Note>> {
        let (user_id, _user) = require_auth(ctx)?;
        let db = ctx.data::<Database>()?;

        // TODO: Verify note belongs to user before updating
        let note = db
            .update_note(&id, input.title.as_deref(), input.content.as_deref())
            .await?;
        Ok(note)
    }

    /// 🗑️ Delete user's note
    async fn delete_note(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let (user_id, _user) = require_auth(ctx)?;
        let db = ctx.data::<Database>()?;

        // TODO: Verify note belongs to user before deleting
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
        .replace('\n', " ")
        .replace('\r', " ")
        .replace('\t', " ")
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

// Helper validation functions
fn validate_note_input(input: &NoteInput) -> AppResult<()> {
    if input.content.is_empty() {
        return Err(AppError::InvalidContent {
            message: "Content cannot be empty".to_string(),
        });
    }

    if let Some(title) = &input.title {
        if title.len() > 200 {
            return Err(AppError::InvalidTitle {
                message: "Title too long (max 200 characters)".to_string(),
            });
        }
    }

    Ok(())
}
