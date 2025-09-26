//! # GraphQL Resolvers with Database Integration
//!
//! This module implements resolvers that use persistent PostgreSQL storage
//! while maintaining smart auto-title generation and comprehensive error handling.

use async_graphql::{Context, EmptySubscription, Object, Result};

use crate::database::Database;
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
        let db = ctx.data::<Database>()?;

        // Smart auto-title generation if no title provided
        let title = match input.title {
            Some(title) if !title.trim().is_empty() => title,
            _ => {
                // Generate title from content (first 50 chars, clean up)
                let auto_title = input
                    .content
                    .lines()
                    .next()
                    .unwrap_or(&input.content)
                    .chars()
                    .take(50)
                    .collect::<String>()
                    .trim()
                    .trim_end_matches(|c: char| c.is_ascii_punctuation())
                    .to_string();

                if auto_title.is_empty() {
                    "Untitled Note".to_string()
                } else {
                    auto_title
                }
            }
        };

        let note = db.create_note(&title, &input.content).await?;
        Ok(note)
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
