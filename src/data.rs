//! Sample data and data access functions.

use crate::types::Note;

/// Returns sample notes for testing purposes.
///
/// In a real application, this would be replaced with database queries.
/// For now, it provides consistent test data with proper UUIDs.
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
