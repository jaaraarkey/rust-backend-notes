//! # GraphQL Types for Smart Notes API
//!
//! Enhanced types with folder system and advanced features

use async_graphql::{InputObject, SimpleObject};

/// Note type for GraphQL responses
#[derive(SimpleObject, Clone)]
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
    /// Is the note pinned?
    pub is_pinned: bool,
    /// When was the note pinned? (RFC3339 format, optional)
    pub pinned_at: Option<String>,
    /// Number of times the note has been viewed
    pub view_count: i32,
    /// Number of words in the note
    pub word_count: i32,
    /// The folder containing the note, if any
    pub folder: Option<Folder>,
}

/// 📁 Folder type for organization
#[derive(SimpleObject, Clone)]
pub struct Folder {
    /// Unique identifier (UUID as string)
    pub id: String,
    /// Folder name
    pub name: String,
    /// Folder description, if any
    pub description: Option<String>,
    /// Folder color (hex code)
    pub color: String,
    /// Folder icon (font icon name)
    pub icon: String,
    /// Position of the folder in the list
    pub position: i32,
    /// Number of notes in the folder
    pub notes_count: i32,
    /// Is this the default folder for the user?
    #[graphql(name = "isDefault")]
    pub is_default: bool,
    /// Creation timestamp (RFC3339 format)
    #[graphql(name = "createdAt")]
    pub created_at: String,
    /// Last update timestamp (RFC3339 format)
    #[graphql(name = "updatedAt")]
    pub updated_at: String,
    /// Parent folder, if this is a subfolder
    pub parent_folder: Option<Box<Folder>>,
    /// Subfolders contained in this folder
    pub subfolders: Vec<Folder>,
}

/// 📊 Folder statistics
#[derive(SimpleObject)]
pub struct FolderStats {
    /// Total number of notes in the folder
    pub total_notes: i32,
    /// Number of pinned notes in the folder
    pub pinned_notes: i32,
    /// Number of recent notes (from last 7 days)
    pub recent_notes: i32,
    /// Total number of words in the folder
    pub total_words: i32,
    /// Last activity timestamp (RFC3339 format, optional)
    pub last_activity: Option<String>,
}

/// Input for creating notes with folder support
#[derive(InputObject)]
pub struct NoteInput {
    /// Optional title (if not provided, will be auto-generated)
    pub title: Option<String>,
    /// Note content (required)
    pub content: String,
    /// Optional folder ID to place the note in
    pub folder_id: Option<String>,
    /// Optional flag to pin the note
    pub is_pinned: Option<bool>,
}

/// Input for updating notes
#[derive(InputObject)]
pub struct UpdateNoteInput {
    /// Optional new title
    pub title: Option<String>,
    /// Optional new content
    pub content: Option<String>,
    /// Optional new folder ID
    pub folder_id: Option<String>,
    /// Optional flag to pin or unpin the note
    pub is_pinned: Option<bool>,
}

/// 📁 Input for creating folders
#[derive(InputObject)]
pub struct CreateFolderInput {
    /// Folder name (required)
    pub name: String,
    /// Optional folder description
    pub description: Option<String>,
    /// Optional folder color
    pub color: Option<String>,
    /// Optional folder icon
    pub icon: Option<String>,
    /// Optional parent folder ID (for subfolders)
    pub parent_id: Option<String>,
    /// Optional position of the folder in the list
    pub position: Option<i32>,
}

/// 📁 Input for updating folders
#[derive(InputObject)]
pub struct UpdateFolderInput {
    /// Optional new name
    pub name: Option<String>,
    /// Optional new description
    pub description: Option<String>,
    /// Optional new color
    pub color: Option<String>,
    /// Optional new icon
    pub icon: Option<String>,
    /// Optional new parent folder ID
    pub parent_id: Option<String>,
    /// Optional new position
    pub position: Option<i32>,
}

/// 🔄 Input for moving folders/notes
#[derive(InputObject)]
pub struct MoveToFolderInput {
    /// Optional target folder ID (None means root level)
    pub target_folder_id: Option<String>,
    /// Optional new position
    pub position: Option<i32>,
}

/// User type for GraphQL responses
#[derive(SimpleObject)]
pub struct User {
    /// Unique identifier (UUID as string)
    pub id: String,
    /// User email address
    pub email: String,
    /// Full name of the user, if provided
    pub full_name: Option<String>,
    /// Account creation timestamp (RFC3339 format)
    #[graphql(name = "createdAt")]
    pub created_at: String,
    /// Last update timestamp (RFC3339 format)
    #[graphql(name = "updatedAt")]
    pub updated_at: String,
    /// Is the user active?
    pub is_active: bool,
}
