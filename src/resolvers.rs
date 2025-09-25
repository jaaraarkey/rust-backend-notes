//! # GraphQL Resolvers with Smart Auto-Title Generation
//!
//! This module implements the core business logic for all GraphQL operations.
//! Resolvers are functions that fetch/modify data in response to GraphQL queries and mutations.
//! Now includes automatic timestamp management and smart title extraction.

use async_graphql::Object;
use chrono::Utc;
use uuid::Uuid;

use crate::data::get_sample_notes;
use crate::errors::{AppError, AppResult};
use crate::types::{CreateNoteInput, Note, UpdateNoteInput};
use crate::validation::{validate_and_process_create_input, validate_update_input, validate_uuid};

/// The root Query type for our GraphQL schema.
pub struct Query;

#[Object]
impl Query {
    /// A simple hello world query for testing the GraphQL setup.
    async fn hello(&self) -> &str {
        "Hello from GraphQL with smart auto-title generation!"
    }

    /// Returns a list of sample notes with timestamps for testing.
    async fn notes(&self) -> Vec<Note> {
        get_sample_notes()
    }

    /// Returns a single note by UUID with validation, or error if not found.
    async fn note(&self, id: String) -> AppResult<Note> {
        // Validate UUID format first
        validate_uuid(&id)?;

        let notes = get_sample_notes();
        notes
            .into_iter()
            .find(|note| note.id == id)
            .ok_or_else(|| AppError::NoteNotFound { id: id.clone() })
    }
}

/// The root Mutation type for our GraphQL schema.
pub struct Mutation;

#[Object]
impl Mutation {
    /// Creates a new note with smart auto-title generation and validation.
    async fn create_note(&self, input: CreateNoteInput) -> AppResult<Note> {
        // Process input with smart title extraction (content preserved)
        let (final_title, final_content) = validate_and_process_create_input(
            input.title.as_deref(), // ← FIX: Convert Option<String> to Option<&str>
            &input.content,
        )?;

        let now = Utc::now().to_rfc3339();

        Ok(Note {
            id: Uuid::new_v4().to_string(),
            title: final_title,
            content: final_content, // Always complete, never modified
            created_at: now.clone(),
            updated_at: now,
        })
    }

    /// Updates an existing note by ID with automatic timestamp management.
    async fn update_note(&self, id: String, input: UpdateNoteInput) -> AppResult<Note> {
        // Validate UUID format first
        validate_uuid(&id)?;

        // Validate input fields
        validate_update_input(
            input.title.as_deref(), // Convert Option<String> to Option<&str>
            input.content.as_deref(),
        )?;

        let notes = get_sample_notes();

        if let Some(mut existing_note) = notes.into_iter().find(|note| note.id == id) {
            // Update only the fields that were provided (after validation)
            if let Some(new_title) = input.title {
                existing_note.title = new_title.trim().to_string();
            }

            if let Some(new_content) = input.content {
                existing_note.content = new_content.trim().to_string();
            }

            // Always update the updatedAt timestamp
            existing_note.updated_at = Utc::now().to_rfc3339();
            // createdAt remains unchanged

            Ok(existing_note)
        } else {
            Err(AppError::NoteNotFound { id })
        }
    }

    /// Deletes a note by ID with validation.
    async fn delete_note(&self, id: String) -> AppResult<bool> {
        // Validate UUID format first
        validate_uuid(&id)?;

        let notes = get_sample_notes();
        let found = notes.into_iter().any(|note| note.id == id);

        if found {
            Ok(true)
        } else {
            Err(AppError::NoteNotFound { id })
        }
    }
}
