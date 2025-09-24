//! # GraphQL Type Definitions
//!
//! This module contains all GraphQL type definitions used throughout the application.
//! It includes both output types (returned by queries/mutations) and input types
//! (used as arguments in mutations and complex queries).
//!
//! ## 📝 Type Categories
//!
//! ### Output Types
//! - [`Note`] - Core note entity with UUID, title, and content
//!
//! ### Input Types  
//! - [`CreateNoteInput`] - Input structure for creating new notes
//! - [`UpdateNoteInput`] - Input structure for updating existing notes
//!
//! ## 🎯 Design Principles
//!
//! - **UUID-based IDs**: All entities use UUIDs for global uniqueness
//! - **Optional Updates**: Update inputs allow partial field updates
//! - **Type Safety**: Compile-time validation for all operations
//! - **Future-ready**: Structure supports easy addition of new fields
//!
//! ## Example Usage
//!
//! ```rust
//! use crate::types::{Note, CreateNoteInput};
//!
//! // Creating a note from input
//! let input = CreateNoteInput {
//!     title: "My Note".to_string(),
//!     content: "Note content here".to_string(),
//! };
//!
//! let note = Note {
//!     id: uuid::Uuid::new_v4().to_string(),
//!     title: input.title,
//!     content: input.content,
//! };
//! ```

use async_graphql::{InputObject, SimpleObject};

/// Represents a note in our application with UUID-based unique identification.
///
/// Notes have a UUID, title, and content. UUIDs are globally unique identifiers
/// that are much more robust than simple integer IDs.
///
/// # GraphQL Schema
/// ```graphql
/// type Note {
///   id: String!      # UUID format
///   title: String!
///   content: String!
/// }
/// ```
///
/// # Example
/// ```rust
/// use crate::types::Note;
///
/// let note = Note {
///     id: "550e8400-e29b-41d4-a716-446655440001".to_string(),
///     title: "My Note".to_string(),
///     content: "Note content here".to_string(),
/// };
/// ```
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
///
/// # GraphQL Schema
/// ```graphql
/// input CreateNoteInput {
///   title: String!
///   content: String!
/// }
/// ```
///
/// # Example
/// ```rust
/// use crate::types::CreateNoteInput;
///
/// let input = CreateNoteInput {
///     title: "My New Note".to_string(),
///     content: "This will become a Note with auto-generated UUID".to_string(),
/// };
/// ```
#[derive(InputObject)]
pub struct CreateNoteInput {
    /// The title of the new note (required)
    pub title: String,
    /// The content/body of the new note (required)  
    pub content: String,
}

/// Input type for updating an existing note.
///
/// This demonstrates partial updates in GraphQL - all fields are optional
/// so clients can update only the fields they want to change.
///
/// # GraphQL Schema
/// ```graphql
/// input UpdateNoteInput {
///   title: String      # Optional - only update if provided
///   content: String    # Optional - only update if provided
/// }
/// ```
///
/// # Example Usage
/// ```graphql
/// # Update only the title
/// mutation {
///   updateNote(id: "uuid-here", input: {
///     title: "New Title"
///   }) {
///     id title content
///   }
/// }
///
/// # Update both fields
/// mutation {
///   updateNote(id: "uuid-here", input: {
///     title: "New Title"
///     content: "New content here"
///   }) {
///     id title content
///   }
/// }
/// ```
#[derive(InputObject)]
pub struct UpdateNoteInput {
    /// New title for the note (optional - only updates if provided)
    pub title: Option<String>,
    /// New content for the note (optional - only updates if provided)
    pub content: Option<String>,
}
