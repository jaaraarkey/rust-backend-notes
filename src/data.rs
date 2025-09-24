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
use chrono::{DateTime, Utc}; // Add this import

/// Returns sample notes with realistic timestamps for testing purposes.
///
/// In a real application, this would be replaced with database queries.
/// For now, it provides consistent test data with proper UUIDs and timestamps.
///
/// # Returns
/// A vector of 4 sample [`Note`] objects with:
/// - Valid UUID v4 identifiers
/// - Educational content about GraphQL and Rust
/// - Realistic `createdAt` and `updatedAt` timestamps
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
    // Create some realistic timestamps (past dates)
    let base_time = DateTime::parse_from_rfc3339("2024-01-10T10:00:00Z")
        .unwrap()
        .with_timezone(&Utc);

    vec![
        Note {
            id: "550e8400-e29b-41d4-a716-446655440001".to_string(),
            title: "Welcome to GraphQL".to_string(),
            content: "This is your first note! GraphQL allows you to query exactly the fields you need. Now with UUID support and timestamps!".to_string(),
            created_at: base_time.to_rfc3339(),
            updated_at: (base_time + chrono::Duration::hours(1)).to_rfc3339(), // Updated 1 hour later
        },
        Note {
            id: "550e8400-e29b-41d4-a716-446655440002".to_string(),
            title: "Learning Rust".to_string(),
            content: "Rust's type system helps catch errors at compile time, making GraphQL APIs more reliable. UUIDs and timestamps provide better data integrity.".to_string(),
            created_at: (base_time + chrono::Duration::hours(2)).to_rfc3339(),
            updated_at: (base_time + chrono::Duration::hours(5)).to_rfc3339(), // Updated 3 hours after creation
        },
        Note {
            id: "550e8400-e29b-41d4-a716-446655440003".to_string(),
            title: "async-graphql Features".to_string(), 
            content: "The async-graphql crate provides powerful features like field selection, introspection, automatic schema generation, and now timestamp support!".to_string(),
            created_at: (base_time + chrono::Duration::hours(6)).to_rfc3339(),
            updated_at: (base_time + chrono::Duration::hours(6)).to_rfc3339(), // Never updated (same as created)
        },
        Note {
            id: "550e8400-e29b-41d4-a716-446655440004".to_string(),
            title: "Timestamps in GraphQL".to_string(),
            content: "Adding createdAt and updatedAt fields provides valuable metadata about when content was created and last modified. Essential for production apps!".to_string(),
            created_at: (base_time + chrono::Duration::hours(8)).to_rfc3339(),
            updated_at: (base_time + chrono::Duration::days(1)).to_rfc3339(), // Updated next day
        },
    ]
}
