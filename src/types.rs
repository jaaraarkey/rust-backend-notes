//! GraphQL types and input definitions.

use async_graphql::{InputObject, SimpleObject};

/// Represents a note in our application with UUID-based unique identification.
///
/// Notes have a UUID, title, and content. UUIDs are globally unique identifiers
/// that are much more robust than simple integer IDs.
#[derive(SimpleObject, Clone, Debug)]
pub struct Note {
    /// Unique identifier for the note (UUID format)
    pub id: String,
    /// The note's title
    pub title: String,
    /// The note's content/body
    pub content: String,
}

/// Input type for creating a new note.
///
/// This demonstrates GraphQL Input types, which are used for complex arguments
/// in mutations and queries. Input types are different from regular types -
/// they can only be used as arguments, not return values.
#[derive(InputObject)]
pub struct CreateNoteInput {
    /// The title of the new note (required)
    pub title: String,
    /// The content/body of the new note (required)  
    pub content: String,
}
