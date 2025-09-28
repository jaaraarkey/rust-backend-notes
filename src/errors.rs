//! # Error Handling for Smart Notes API
//!
//! Comprehensive error types with GraphQL integration

use async_graphql::{ErrorExtensions, FieldError};

pub type AppResult<T> = Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    // 🗄️ Database Errors
    #[error("Database error: {message}")]
    DatabaseError { message: String },

    #[error("Invalid UUID: {uuid}")]
    InvalidUuid { uuid: String },

    // 📝 Content Validation Errors
    #[error("Invalid title: {message}")]
    InvalidTitle { message: String },

    #[error("Invalid content: {message}")]
    InvalidContent { message: String },

    #[error("Note not found")]
    NoteNotFound,

    // 🔐 Authentication Errors
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

impl AppError {
    /// Get error code for GraphQL response
    fn error_code(&self) -> &'static str {
        match self {
            // 🗄️ Database error codes
            Self::DatabaseError { .. } => "DATABASE_ERROR",
            Self::InvalidUuid { .. } => "INVALID_UUID",

            // 📝 Content error codes
            Self::InvalidTitle { .. } => "INVALID_TITLE",
            Self::InvalidContent { .. } => "INVALID_CONTENT",
            Self::NoteNotFound => "NOTE_NOT_FOUND",

            // 🔐 Auth error codes
            Self::AuthError { .. } => "AUTH_ERROR",
            Self::UserNotFound => "USER_NOT_FOUND",
            Self::EmailAlreadyExists => "EMAIL_ALREADY_EXISTS",
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::ValidationError { .. } => "VALIDATION_ERROR",
        }
    }

    /// Get error category for logging/monitoring
    pub fn category(&self) -> ErrorCategory {
        match self {
            Self::DatabaseError { .. } | Self::InvalidUuid { .. } => ErrorCategory::Database,

            Self::InvalidTitle { .. } | Self::InvalidContent { .. } | Self::NoteNotFound => {
                ErrorCategory::Content
            }

            Self::AuthError { .. }
            | Self::UserNotFound
            | Self::EmailAlreadyExists
            | Self::InvalidCredentials
            | Self::Unauthorized => ErrorCategory::Auth,

            Self::ValidationError { .. } => ErrorCategory::Validation,
        }
    }

    /// Check if error should be logged (security-sensitive errors)
    pub fn should_log(&self) -> bool {
        matches!(
            self,
            Self::DatabaseError { .. } | Self::AuthError { .. } | Self::Unauthorized
        )
    }
}

/// Error categories for better organization
#[derive(Debug, Clone, Copy)]
pub enum ErrorCategory {
    Database,
    Content,
    Auth,
    Validation,
}

impl ErrorCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Database => "database",
            Self::Content => "content",
            Self::Auth => "auth",
            Self::Validation => "validation",
        }
    }
}

impl ErrorExtensions for AppError {
    fn extend(&self) -> FieldError {
        self.extend_with(|_err, e| {
            e.set("code", self.error_code());
            e.set("category", self.category().as_str());

            // 🔒 Don't expose sensitive database errors in production
            if cfg!(debug_assertions) {
                e.set("debug", true);
            }
        })
    }
}

// ✅ Helper macros for common error creation patterns
#[macro_export]
macro_rules! db_error {
    ($msg:expr) => {
        AppError::DatabaseError {
            message: $msg.to_string(),
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        AppError::DatabaseError {
            message: format!($fmt, $($arg)*),
        }
    };
}

#[macro_export]
macro_rules! auth_error {
    ($msg:expr) => {
        AppError::AuthError {
            message: $msg.to_string(),
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        AppError::AuthError {
            message: format!($fmt, $($arg)*),
        }
    };
}

#[macro_export]
macro_rules! validation_error {
    ($msg:expr) => {
        AppError::ValidationError {
            message: $msg.to_string(),
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        AppError::ValidationError {
            message: format!($fmt, $($arg)*),
        }
    };
}
