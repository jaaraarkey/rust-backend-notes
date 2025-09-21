//! GraphQL resolvers for queries and mutations.

use async_graphql::Object;
use uuid::Uuid;

use crate::data::get_sample_notes;
use crate::types::{CreateNoteInput, Note};

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
    async fn hello(&self) -> &str {
        "Hello from GraphQL with UUID support!"
    }

    /// Returns a list of sample notes for testing.
    ///
    /// This demonstrates:
    /// - GraphQL list types: [Note!]!
    /// - Complex return types with multiple fields
    /// - Field selection capabilities
    /// - Static data serving (will be dynamic in later days)
    ///
    /// The return type [Note!]! means:
    /// - Outer []: This is a list/array
    /// - Note: Each item in the list is a Note type
    /// - Inner !: Each Note in the list is non-null
    /// - Outer !: The list itself is non-null (but can be empty)
    async fn notes(&self) -> Vec<Note> {
        get_sample_notes()
    }

    /// Returns a single note by UUID, or None if not found.
    ///
    /// This demonstrates:
    /// - GraphQL arguments: note(id: String!)
    /// - UUID-based identification
    /// - Optional return types: Note vs Note!
    /// - Error handling for missing data
    ///
    /// Arguments:
    /// - id: The UUID of the note to retrieve
    ///
    /// Returns:
    /// - Some(Note) if found
    /// - None if no note exists with the given UUID
    ///
    /// GraphQL Schema:
    /// ```graphql
    /// note(id: String!): Note
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
    /// Creates a new note with auto-generated UUID.
    ///
    /// This demonstrates:
    /// - GraphQL mutations (write operations)
    /// - Input types for complex arguments
    /// - UUID generation for unique identifiers
    /// - Returning the created object
    ///
    /// Arguments:
    /// - input: CreateNoteInput containing title and content
    ///
    /// Returns:
    /// - The newly created Note with generated UUID
    ///
    /// GraphQL Schema:
    /// ```graphql
    /// createNote(input: CreateNoteInput!): Note!
    /// ```
    async fn create_note(&self, input: CreateNoteInput) -> Note {
        // Generate a new UUID for this note
        let new_id = Uuid::new_v4().to_string();

        // Create and return the new note
        Note {
            id: new_id,
            title: input.title,
            content: input.content,
        }
    }
}
