//! # GraphQL Type Definitions
//!
//! This module defines all the GraphQL types used in the API,
//! including input types for mutations and the main Note type.

use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

/// A note with metadata, compatible with GraphQL and database storage
#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    /// Unique identifier (UUID as string)
    pub id: String,
    /// Note title (auto-generated or user-provided)
    pub title: String,
    /// Note content
    pub content: String,
    /// Creation timestamp (RFC3339 format)
    #[graphql(name = "createdAt")]
    pub created_at: String,
    /// Last update timestamp (RFC3339 format)
    #[graphql(name = "updatedAt")]
    pub updated_at: String,
}

/// Input type for creating new notes
#[derive(InputObject, Debug)]
pub struct NoteInput {
    /// Optional title (if not provided, will be auto-generated)
    pub title: Option<String>,
    /// Note content (required)
    pub content: String,
}

/// Input type for updating existing notes
#[derive(InputObject, Debug)]
pub struct UpdateNoteInput {
    /// Optional new title
    pub title: Option<String>,
    /// Optional new content
    pub content: Option<String>,
}
