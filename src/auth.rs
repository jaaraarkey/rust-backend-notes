//! # Authentication and Authorization
//!
//! JWT-based authentication system with bcrypt password hashing

use async_graphql::{InputObject, SimpleObject};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::errors::{AppError, AppResult};

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub exp: i64, // Expiration timestamp
    pub iat: i64, // Issued at timestamp
}

impl Claims {
    /// Create new claims for a user
    pub fn new(user_id: Uuid, email: String) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // Token valid for 24 hours

        Self {
            sub: user_id.to_string(),
            email,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
}

/// User registration input
#[derive(InputObject, Validate)]
pub struct RegisterInput {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,

    #[validate(length(
        min = 2,
        max = 100,
        message = "Full name must be between 2 and 100 characters"
    ))]
    pub full_name: Option<String>,
}

/// User login input
#[derive(InputObject, Validate)]
pub struct LoginInput {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub password: String,
}

/// Authentication response
#[derive(SimpleObject)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

/// User type for GraphQL
#[derive(SimpleObject, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub full_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub is_active: bool,
}

/// Database user row helper
#[derive(sqlx::FromRow, Clone)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id.to_string(),
            email: row.email,
            full_name: row.full_name,
            created_at: row.created_at.to_rfc3339(),
            updated_at: row.updated_at.to_rfc3339(),
            is_active: row.is_active,
        }
    }
}

/// Authentication service
pub struct AuthService {
    jwt_secret: String,
}

impl AuthService {
    /// Create new auth service
    pub fn new() -> Self {
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string());

        Self { jwt_secret }
    }

    /// Hash password using bcrypt
    pub fn hash_password(&self, password: &str) -> AppResult<String> {
        hash(password, DEFAULT_COST).map_err(|e| AppError::AuthError {
            message: format!("Failed to hash password: {}", e),
        })
    }

    /// Verify password against hash
    pub fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        verify(password, hash).map_err(|e| AppError::AuthError {
            message: format!("Failed to verify password: {}", e),
        })
    }

    /// Generate JWT token
    pub fn generate_token(&self, user_id: Uuid, email: String) -> AppResult<String> {
        let claims = Claims::new(user_id, email);
        let header = Header::default();
        let encoding_key = EncodingKey::from_secret(self.jwt_secret.as_ref());

        encode(&header, &claims, &encoding_key).map_err(|e| AppError::AuthError {
            message: format!("Failed to generate token: {}", e),
        })
    }

    /// Verify and decode JWT token
    pub fn verify_token(&self, token: &str) -> AppResult<Claims> {
        let decoding_key = DecodingKey::from_secret(self.jwt_secret.as_ref());
        let validation = Validation::default();

        decode::<Claims>(token, &decoding_key, &validation)
            .map(|token_data| token_data.claims)
            .map_err(|e| AppError::AuthError {
                message: format!("Invalid token: {}", e),
            })
    }

    /// Extract user ID from authorization header token
    pub fn extract_user_id_from_token(&self, token: &str) -> AppResult<Uuid> {
        let claims = self.verify_token(token)?;

        Uuid::parse_str(&claims.sub).map_err(|_| AppError::AuthError {
            message: "Invalid user ID in token".to_string(),
        })
    }
}
