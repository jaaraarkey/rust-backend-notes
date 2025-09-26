//! # GraphQL Resolvers with Database Integration
//!
//! This module implements resolvers that use persistent SQLite storage
//! while maintaining smart auto-title generation and comprehensive error handling.

use async_graphql::{Context, Object};

use crate::database::Database;
use crate::errors::{AppError, AppResult};
use crate::types::{CreateNoteInput, Note, UpdateNoteInput};
use crate::validation::{validate_and_process_create_input, validate_update_input, validate_uuid};

/// The root Query type for our GraphQL schema with database integration.
pub struct Query;

#[Object]
impl Query {
    /// A simple hello world query for testing the GraphQL setup.
    async fn hello(&self) -> &str {
        "Hello from GraphQL with persistent database storage and smart auto-titles!"
    }

    /// Returns all notes from the database, ordered by creation date (newest first).
    async fn notes(&self, ctx: &Context<'_>) -> AppResult<Vec<Note>> {
        let database = ctx
            .data::<Database>()
            .map_err(|_| AppError::InternalError)?;
        database.get_all_notes().await
    }

    /// Returns a single note by UUID from the database, with validation.
    async fn note(&self, ctx: &Context<'_>, id: String) -> AppResult<Note> {
        // Validate UUID format first
        validate_uuid(&id)?;

        let database = ctx
            .data::<Database>()
            .map_err(|_| AppError::InternalError)?;

        match database.get_note_by_id(&id).await? {
            Some(note) => Ok(note),
            None => Err(AppError::NoteNotFound { id }),
        }
    }

    /// 🔍 Search notes using PostgreSQL full-text search
    async fn search_notes(&self, ctx: &Context<'_>, query: String) -> AppResult<Vec<Note>> {
        if query.trim().is_empty() {
            return Err(AppError::InvalidContent {
                message: "Search query cannot be empty".to_string(),
            });
        }

        let database = ctx
            .data::<Database>()
            .map_err(|_| AppError::InternalError)?;
        database.search_notes(&query).await
    }
}

/// The root Mutation type for our GraphQL schema with database integration.
pub struct Mutation;

#[Object]
impl Mutation {
    /// Creates a new note with smart auto-title generation and saves to database.
    async fn create_note(&self, ctx: &Context<'_>, input: CreateNoteInput) -> AppResult<Note> {
        // Process input with smart title extraction (content preserved)
        let (final_title, final_content) = validate_and_process_create_input(
            input.title.as_deref(), // Convert Option<String> to Option<&str>
            &input.content,
        )?;

        let database = ctx
            .data::<Database>()
            .map_err(|_| AppError::InternalError)?;
        database.create_note(&final_title, &final_content).await
    }

    /// Updates an existing note in the database with validation and automatic timestamp management.
    async fn update_note(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateNoteInput,
    ) -> AppResult<Note> {
        // Validate UUID format first
        validate_uuid(&id)?;

        // Validate input fields
        validate_update_input(
            input.title.as_deref(), // Convert Option<String> to Option<&str>
            input.content.as_deref(),
        )?;

        let database = ctx
            .data::<Database>()
            .map_err(|_| AppError::InternalError)?;

        match database
            .update_note(&id, input.title.as_deref(), input.content.as_deref())
            .await?
        {
            Some(note) => Ok(note),
            None => Err(AppError::NoteNotFound { id }),
        }
    }

    /// Deletes a note by ID from the database with validation.
    async fn delete_note(&self, ctx: &Context<'_>, id: String) -> AppResult<bool> {
        // Validate UUID format first
        validate_uuid(&id)?;

        let database = ctx
            .data::<Database>()
            .map_err(|_| AppError::InternalError)?;

        let deleted = database.delete_note(&id).await?;
        if deleted {
            Ok(true)
        } else {
            Err(AppError::NoteNotFound { id })
        }
    }
}
