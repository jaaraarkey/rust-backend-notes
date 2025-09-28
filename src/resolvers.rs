//! # GraphQL Resolvers with Database Integration
//!
//! This module implements resolvers that use persistent PostgreSQL storage
//! while maintaining smart auto-title generation and comprehensive error handling.

use async_graphql::{Context, EmptySubscription, Object, Result};

use crate::database::Database;
use crate::errors::AppError; // ✨ Add this import!
use crate::types::{Note, NoteInput, UpdateNoteInput};

pub struct QueryRoot;
pub struct MutationRoot;

// ✅ Use EmptySubscription as a type, not a value
pub type SubscriptionRoot = EmptySubscription;

#[Object]
impl QueryRoot {
    /// A simple hello world query for testing the GraphQL setup.
    async fn hello(&self) -> &str {
        "Hello from GraphQL with PostgreSQL database and smart auto-titles!"
    }

    /// Returns all notes from the database, ordered by creation date (newest first).
    async fn notes(&self, ctx: &Context<'_>) -> Result<Vec<Note>> {
        let db = ctx.data::<Database>()?;
        let notes = db.get_all_notes().await?;
        Ok(notes)
    }

    /// Returns a single note by UUID from the database, with validation.
    async fn note(&self, ctx: &Context<'_>, id: String) -> Result<Option<Note>> {
        let db = ctx.data::<Database>()?;
        let note = db.get_note_by_id(&id).await?;
        Ok(note)
    }

    /// 🔍 Search notes using PostgreSQL full-text search
    async fn search_notes(&self, ctx: &Context<'_>, query: String) -> Result<Vec<Note>> {
        let db = ctx.data::<Database>()?;

        if query.trim().is_empty() {
            return Err("Search query cannot be empty".into());
        }

        let notes = db.search_notes(&query).await?;
        Ok(notes)
    }
}

#[Object]
impl MutationRoot {
    /// Create a new note with smart auto-title generation
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

        // 🌟 ENHANCED: Smart auto-title generation
        let title = match input.title {
            Some(title) if !title.trim().is_empty() => title,
            _ => generate_smart_title(&input.content),
        };

        let db = ctx
            .data::<Database>()
            .map_err(|_| AppError::DatabaseError {
                message: "Database connection not available".to_string(),
            })?;

        db.create_note(&title, &input.content)
            .await
            .map_err(Into::into)
    }

    /// Update an existing note using your BRILLIANT 4-pattern logic!
    async fn update_note(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateNoteInput,
    ) -> Result<Option<Note>> {
        let db = ctx.data::<Database>()?;

        // Use your genius 4-pattern update logic!
        let note = db
            .update_note(&id, input.title.as_deref(), input.content.as_deref())
            .await?;

        Ok(note)
    }

    /// Delete a note by ID
    async fn delete_note(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let db = ctx.data::<Database>()?;
        let deleted = db.delete_note(&id).await?;
        Ok(deleted)
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
        // Normalize multiple spaces to single space
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    if cleaned.is_empty() {
        return "Untitled Note".to_string();
    }

    // If content is already short enough, use as-is
    if cleaned.len() <= MAX_LENGTH {
        return cleaned;
    }

    // Strategy 1: Find sentence boundary within limit
    if let Some(sentence_end) = find_sentence_boundary(&cleaned, MAX_LENGTH) {
        return cleaned[..sentence_end].trim().to_string();
    }

    // Strategy 2: Find word boundary within limit
    if let Some(word_end) = find_word_boundary(&cleaned, MAX_LENGTH) {
        return format!("{}...", cleaned[..word_end].trim());
    }

    // Strategy 3: Fallback - cut at character boundary with ellipsis
    let mut end = MAX_LENGTH - 3; // Leave room for "..."
    while end > 0 && !cleaned.is_char_boundary(end) {
        end -= 1;
    }

    format!("{}...", cleaned[..end].trim())
}

/// Find the best sentence boundary within the character limit
fn find_sentence_boundary(text: &str, max_len: usize) -> Option<usize> {
    let search_area = &text[..max_len.min(text.len())];
    let mut best_pos = None;
    let mut in_quotes = false;
    let mut current_quote = None;

    for (byte_pos, ch) in search_area.char_indices() {
        // Handle quotes
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

        // Look for sentence endings only when not in quotes
        if !in_quotes && matches!(ch, '.' | '!' | '?') {
            let end_pos = byte_pos + ch.len_utf8();

            // Check if this is a valid sentence ending
            let is_valid = if end_pos >= search_area.len() {
                true // End of text
            } else {
                // Check what follows
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

/// Find the best word boundary within the character limit
fn find_word_boundary(text: &str, max_len: usize) -> Option<usize> {
    let min_length = max_len / 3; // Don't make title too short (at least 1/3 of max)

    // Find the last space before the limit
    text[..max_len.min(text.len())]
        .rfind(' ') // ✅ Fixed: Use ' ' (char) for rfind, not " " (str)
        .filter(|&pos| pos >= min_length)
}
