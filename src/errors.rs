//! # Error Handling for Smart Notes API
//!
//! Comprehensive error types with GraphQL integration

use async_graphql::{ErrorExtensions, FieldError};

pub type AppResult<T> = Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Database error: {message}")]
    DatabaseError { message: String },

    #[error("Invalid UUID: {uuid}")]
    InvalidUuid { uuid: String },

    #[error("Invalid title: {message}")]
    InvalidTitle { message: String },

    #[error("Invalid content: {message}")]
    InvalidContent { message: String },

    #[error("Note not found")]
    NoteNotFound,

    // ✅ Authentication error variants
    #[error("Authentication error: {message}")]
    AuthError { message: String },

    #[error("User not found")]
    UserNotFound,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Validation error: {message}")]
    ValidationError { message: String },
}

impl ErrorExtensions for AppError {
    fn extend(&self) -> FieldError {
        self.extend_with(|_err, e| match self {
            AppError::DatabaseError { .. } => {
                e.set("code", "DATABASE_ERROR");
            }
            AppError::InvalidUuid { .. } => {
                e.set("code", "INVALID_UUID");
            }
            AppError::InvalidTitle { .. } => {
                e.set("code", "INVALID_TITLE");
            }
            AppError::InvalidContent { .. } => {
                e.set("code", "INVALID_CONTENT");
            }
            AppError::NoteNotFound => {
                e.set("code", "NOTE_NOT_FOUND");
            }
            AppError::AuthError { .. } => {
                e.set("code", "AUTH_ERROR");
            }
            AppError::UserNotFound => {
                e.set("code", "USER_NOT_FOUND");
            }
            AppError::EmailAlreadyExists => {
                e.set("code", "EMAIL_ALREADY_EXISTS");
            }
            AppError::InvalidCredentials => {
                e.set("code", "INVALID_CREDENTIALS");
            }
            AppError::Unauthorized => {
                e.set("code", "UNAUTHORIZED");
            }
            AppError::ValidationError { .. } => {
                e.set("code", "VALIDATION_ERROR");
            }
        })
    }
}

// ✅ Removed conflicting From trait implementation
// async-graphql already provides a generic From implementation
