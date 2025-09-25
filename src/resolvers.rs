//! # GraphQL Resolvers
//!
//! This module implements the core business logic for all GraphQL operations.
//! Resolvers are functions that fetch/modify data in response to GraphQL queries and mutations.
//! Now includes automatic timestamp management for all operations.

use async_graphql::Object;
use chrono::Utc;
use uuid::Uuid;

use crate::data::get_sample_notes;
use crate::types::{CreateNoteInput, Note, UpdateNoteInput};

/// The root Query type for our GraphQL schema.
pub struct Query;

#[Object]
impl Query {
    /// A simple hello world query for testing the GraphQL setup.
    async fn hello(&self) -> &str {
        "Hello from GraphQL with UUID and timestamp support!"
    }

    /// Returns a list of sample notes with timestamps for testing.
    async fn notes(&self) -> Vec<Note> {
        get_sample_notes()
    }

    /// Returns a single note by UUID with timestamps, or None if not found.
    async fn note(&self, id: String) -> Option<Note> {
        let notes = get_sample_notes();
        notes.into_iter().find(|note| note.id == id)
    }
}

/// The root Mutation type for our GraphQL schema.
pub struct Mutation;

#[Object]
impl Mutation {
    /// Creates a new note with auto-generated UUID and timestamps.
    async fn create_note(&self, input: CreateNoteInput) -> Note {
        let now = Utc::now().to_rfc3339();

        Note {
            id: Uuid::new_v4().to_string(),
            title: input.title,
            content: input.content,
            created_at: now.clone(),
            updated_at: now, // Initially same as created_at
        }
    }

    /// Updates an existing note by ID with automatic timestamp management.
    async fn update_note(&self, id: String, input: UpdateNoteInput) -> Option<Note> {
        let notes = get_sample_notes();

        if let Some(mut existing_note) = notes.into_iter().find(|note| note.id == id) {
            // Update only the fields that were provided
            if let Some(new_title) = input.title {
                existing_note.title = new_title;
            }

            if let Some(new_content) = input.content {
                existing_note.content = new_content;
            }

            // Always update the updatedAt timestamp
            existing_note.updated_at = Utc::now().to_rfc3339();
            // createdAt remains unchanged

            Some(existing_note)
        } else {
            None
        }
    }

    /// Deletes a note by ID.
    async fn delete_note(&self, id: String) -> bool {
        let notes = get_sample_notes();
        notes.into_iter().any(|note| note.id == id)
    }
}
