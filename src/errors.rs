//! # Error Handling for Smart Notes API
//!
//! Comprehensive error types with GraphQL integration

use async_graphql::{ErrorExtensions, Result as GraphQLResult};
use thiserror::Error;

/// Application result type
pub type AppResult<T> = Result<T, AppError>;

/// Main error types for the application
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {message}")]
    DatabaseError { message: String },

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid UUID: {uuid}")]
    InvalidUuid { uuid: String },

    #[error("Validation error: {message}")]
    ValidationError { message: String },

    #[error("Invalid content: {message}")]
    InvalidContent { message: String },

    #[error("Invalid title: {message}")]
    InvalidTitle { message: String },

    #[error("JWT error: {message}")]
    JwtError { message: String },

    #[error("Auth error: {message}")]
    AuthError { message: String },

    #[error("Configuration error: {message}")]
    ConfigError { message: String },

    #[error("Internal server error")]
    InternalServerError,
}

impl ErrorExtensions for AppError {
    fn extend(&self) -> async_graphql::Error {
        let extensions = match self {
            AppError::DatabaseError { .. } => async_graphql::Error::new(format!("{}", self))
                .extend_with(|_, e| e.set("code", "DATABASE_ERROR")),
            AppError::AuthenticationFailed => async_graphql::Error::new(format!("{}", self))
                .extend_with(|_, e| e.set("code", "AUTHENTICATION_FAILED")),
            AppError::Unauthorized => async_graphql::Error::new(format!("{}", self))
                .extend_with(|_, e| e.set("code", "UNAUTHORIZED")),
            AppError::InvalidCredentials => async_graphql::Error::new(format!("{}", self))
                .extend_with(|_, e| e.set("code", "INVALID_CREDENTIALS")),
            AppError::EmailAlreadyExists => async_graphql::Error::new(format!("{}", self))
                .extend_with(|_, e| e.set("code", "EMAIL_ALREADY_EXISTS")),
            AppError::UserNotFound => async_graphql::Error::new(format!("{}", self))
                .extend_with(|_, e| e.set("code", "USER_NOT_FOUND")),
            AppError::InvalidUuid { .. } => async_graphql::Error::new(format!("{}", self))
                .extend_with(|_, e| e.set("code", "INVALID_UUID")),
            AppError::ValidationError { .. } => async_graphql::Error::new(format!("{}", self))
                .extend_with(|_, e| e.set("code", "VALIDATION_ERROR")),
            AppError::InvalidContent { .. } => async_graphql::Error::new(format!("{}", self))
                .extend_with(|_, e| e.set("code", "INVALID_CONTENT")),
            AppError::InvalidTitle { .. } => async_graphql::Error::new(format!("{}", self))
                .extend_with(|_, e| e.set("code", "INVALID_TITLE")),
            AppError::JwtError { .. } => async_graphql::Error::new(format!("{}", self))
                .extend_with(|_, e| e.set("code", "JWT_ERROR")),
            AppError::AuthError { .. } => async_graphql::Error::new(format!("{}", self))
                .extend_with(|_, e| e.set("code", "AUTH_ERROR")),
            AppError::ConfigError { .. } => async_graphql::Error::new(format!("{}", self))
                .extend_with(|_, e| e.set("code", "CONFIG_ERROR")),
            AppError::InternalServerError => async_graphql::Error::new(format!("{}", self))
                .extend_with(|_, e| e.set("code", "INTERNAL_SERVER_ERROR")),
        };

        extensions
    }
}

/// Convert AppError to GraphQL Result
impl<T> From<AppError> for GraphQLResult<T> {
    fn from(err: AppError) -> Self {
        Err(err.extend())
    }
}
