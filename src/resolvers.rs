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
///
/// This contains all the "read" operations that clients can perform.
/// Each method in this impl block becomes a field in the GraphQL Query type.
pub struct Query;

#[Object]
impl Query {
    /// A simple hello world query for testing the GraphQL setup.
    ///
    /// This is useful for:
    /// - Verifying the server is running
    /// - Testing GraphQL client connections
    /// - Basic health checks
    ///
    /// ## GraphQL Schema
    /// ```graphql
    /// hello: String!
    /// ```
    async fn hello(&self) -> &str {
        "Hello from GraphQL with UUID and timestamp support!"
    }

    /// Returns a list of sample notes with timestamps for testing.
    ///
    /// This demonstrates:
    /// - GraphQL list types with non-null constraints
    /// - Complex return types with multiple fields including timestamps
    /// - Field selection capabilities
    /// - Static data serving (will be dynamic in later days)
    ///
    /// ## GraphQL Schema
    /// ```graphql
    /// notes: [Note!]!
    /// ```
    ///
    /// ## Example Usage
    /// ```graphql
    /// query {
    ///   notes {
    ///     id
    ///     title
    ///     content
    ///     createdAt
    ///     updatedAt
    ///   }
    /// }
    /// ```
    async fn notes(&self) -> Vec<Note> {
        get_sample_notes()
    }

    /// Returns a single note by UUID with timestamps, or None if not found.
    ///
    /// This demonstrates:
    /// - GraphQL arguments with required types
    /// - UUID-based identification
    /// - Optional return types for missing data
    /// - Error handling for missing data
    ///
    /// ## GraphQL Schema
    /// ```graphql
    /// note(id: String!): Note
    /// ```
    ///
    /// ## Example Usage
    /// ```graphql
    /// query {
    ///   note(id: "550e8400-e29b-41d4-a716-446655440001") {
    ///     title
    ///     content
    ///     createdAt
    ///     updatedAt
    ///   }
    /// }
    /// ```
    async fn note(&self, id: String) -> Option<Note> {
        let notes = get_sample_notes();
        notes.into_iter().find(|note| note.id == id)
    }
}

/// The root Mutation type for our GraphQL schema.
///
/// This contains all the "write" operations that clients can perform.
/// Each method in this impl block becomes a field in the GraphQL Mutation type.
pub struct Mutation;

#[Object]
impl Mutation {
    /// Creates a new note with auto-generated UUID and timestamps.
    ///
    /// Automatically sets:
    /// - `id`: Generated UUID v4
    /// - `createdAt`: Current UTC timestamp  
    /// - `updatedAt`: Same as createdAt (initially)
    ///
    /// ## GraphQL Schema
    /// ```graphql
    /// createNote(input: CreateNoteInput!): Note!
    /// ```
    ///
    /// ## Usage Examples
    /// ```graphql
    /// mutation {
    ///   createNote(input: {
    ///     title: "My Note"
    ///     content: "Note content here"
    ///   }) {
    ///     id
    ///     title
    ///     content
    ///     createdAt
    ///     updatedAt
    ///   }
    /// }
    /// ```
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
    ///
    /// Automatically updates:
    /// - `updatedAt`: Set to current UTC timestamp
    /// - `createdAt`: Preserved unchanged
    ///
    /// ## GraphQL Schema
    /// ```graphql
    /// updateNote(id: String!, input: UpdateNoteInput!): Note
    /// ```
    ///
    /// ## Usage Examples
    /// ```graphql
    /// mutation {
    ///   updateNote(id: "550e8400-e29b-41d4-a716-446655440001", input: {
    ///     title: "Updated Title"
    ///   }) {
    ///     id
    ///     title
    ///     content
    ///     createdAt    # Unchanged
    ///     updatedAt    # Automatically updated to now
    ///   }
    /// }
    /// ```
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
    ///
    /// ## GraphQL Schema
    /// ```graphql
    /// deleteNote(id: String!): Boolean!
    /// ```
    ///
    /// ## Usage Examples
    /// ```graphql
    /// mutation {
    ///   deleteNote(id: "550e8400-e29b-41d4-a716-446655440001")
    /// }
    /// ```
    async fn delete_note(&self, id: String) -> bool {
        let notes = get_sample_notes();
        notes.into_iter().any(|note| note.id == id)
    }
}
