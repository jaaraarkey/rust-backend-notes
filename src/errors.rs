//! # Error Handling & Validation
//!
//! This module provides comprehensive error handling for the GraphQL API,
//! including validation errors, not-found errors, and client-friendly messages.

use async_graphql::ErrorExtensions;
use thiserror::Error;

/// Application-specific error types with detailed context
#[derive(Debug, Error, Clone)] // ← ADD Clone HERE
pub enum AppError {
    #[error("Note not found with ID: {id}")]
    NoteNotFound { id: String },

    #[error("Invalid UUID format: '{uuid}' - must be valid UUID v4")]
    InvalidUuid { uuid: String },

    #[error("Title validation failed: {message}")]
    InvalidTitle { message: String },

    #[error("Content validation failed: {message}")]
    InvalidContent { message: String },

    #[error("Multiple validation errors occurred")]
    ValidationErrors { errors: Vec<String> },

    #[error("Database operation failed: {message}")]
    DatabaseError { message: String },

    #[error("Internal server error occurred")]
    InternalError,
}

impl ErrorExtensions for AppError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            AppError::NoteNotFound { id } => {
                e.set("code", "NOTE_NOT_FOUND");
                e.set("type", "CLIENT_ERROR");
                e.set("noteId", id.as_str());
            }
            AppError::InvalidUuid { uuid } => {
                e.set("code", "INVALID_UUID");
                e.set("type", "CLIENT_ERROR");
                e.set("invalidUuid", uuid.as_str());
            }
            AppError::InvalidTitle { message } => {
                e.set("code", "INVALID_TITLE");
                e.set("type", "CLIENT_ERROR");
                e.set("field", "title");
                e.set("details", message.as_str());
            }
            AppError::InvalidContent { message } => {
                e.set("code", "INVALID_CONTENT");
                e.set("type", "CLIENT_ERROR");
                e.set("field", "content");
                e.set("details", message.as_str());
            }
            AppError::ValidationErrors { errors } => {
                e.set("code", "VALIDATION_ERRORS");
                e.set("type", "CLIENT_ERROR");
                e.set("errorCount", errors.len());
                let error_strs: Vec<&str> = errors.iter().map(|s| s.as_str()).collect();
                e.set("errors", error_strs);
            }
            AppError::DatabaseError { message } => {
                e.set("code", "DATABASE_ERROR");
                e.set("type", "SERVER_ERROR");
                e.set("details", message.as_str());
            }
            AppError::InternalError => {
                e.set("code", "INTERNAL_ERROR");
                e.set("type", "SERVER_ERROR");
            }
        })
    }
}

/// Convenient type alias for our application results
pub type AppResult<T> = Result<T, AppError>;
