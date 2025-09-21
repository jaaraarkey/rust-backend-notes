//! # Data Access Layer
//!
//! This module handles all data access operations for the notes application.
//! Currently provides sample data for development and testing.
//!
//! ## 🗄️ Current Implementation
//!
//! - **Static Data**: Returns hardcoded sample notes for testing
//! - **UUID-based IDs**: All sample notes use proper UUID format
//! - **Consistent Structure**: Matches production [`Note`] type exactly
//!
//! ## 🔄 Future Evolution
//!
//! This module is designed to evolve through the learning roadmap:
//!
//! - **Day 4**: ✅ Static sample data (current)
//! - **Day 8**: 🔄 Database integration with SQLite
//! - **Day 12**: 🔄 Advanced querying and pagination
//! - **Day 14**: 🔄 Production database with PostgreSQL
//!
//! ## 📊 Sample Data
//!
//! The [`get_sample_notes`] function provides 4 test notes covering:
//! - GraphQL introduction and concepts
//! - Rust language features
//! - async-graphql library capabilities  
//! - UUID benefits and implementation
//!
//! ## Example Usage
//!
//! ```rust
//! use crate::data::get_sample_notes;
//!
//! // Get all sample notes
//! let notes = get_sample_notes();
//! println!("Found {} sample notes", notes.len()); // 4
//!
//! // Find specific note by ID
//! let target_id = "550e8400-e29b-41d4-a716-446655440001";
//! let note = notes.into_iter().find(|n| n.id == target_id);
//! ```

use crate::types::Note;

/// Returns sample notes for testing purposes.
///
/// In a real application, this would be replaced with database queries.
/// For now, it provides consistent test data with proper UUIDs.
///
/// # Returns
/// A vector of 4 sample [`Note`] objects with:
/// - Valid UUID v4 identifiers
/// - Educational content about GraphQL and Rust
/// - Consistent structure matching the production schema
///
/// # Example
/// ```rust
/// use crate::data::get_sample_notes;
///
/// let notes = get_sample_notes();
/// assert_eq!(notes.len(), 4);
///
/// // All notes have valid UUID format
/// for note in &notes {
///     assert!(note.id.len() > 30); // UUIDs are 36 characters
///     assert!(!note.title.is_empty());
///     assert!(!note.content.is_empty());
/// }
/// ```
pub fn get_sample_notes() -> Vec<Note> {
    vec![
        Note {
            id: "550e8400-e29b-41d4-a716-446655440001".to_string(),
            title: "Welcome to GraphQL".to_string(),
            content: "This is your first note! GraphQL allows you to query exactly the fields you need. Now with UUID support!".to_string(),
        },
        Note {
            id: "550e8400-e29b-41d4-a716-446655440002".to_string(),
            title: "Learning Rust".to_string(),
            content: "Rust's type system helps catch errors at compile time, making GraphQL APIs more reliable. UUIDs provide better data integrity.".to_string(),
        },
        Note {
            id: "550e8400-e29b-41d4-a716-446655440003".to_string(),
            title: "async-graphql Features".to_string(), 
            content: "The async-graphql crate provides powerful features like field selection, introspection, and automatic schema generation with UUID support.".to_string(),
        },
        Note {
            id: "550e8400-e29b-41d4-a716-446655440004".to_string(),
            title: "UUID Benefits".to_string(),
            content: "UUIDs are globally unique, don't reveal sequence information, and work great in distributed systems!".to_string(),
        },
    ]
}
