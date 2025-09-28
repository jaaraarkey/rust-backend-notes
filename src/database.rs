//! # Database Module with PostgreSQL Integration
//!
//! Comprehensive database operations using SQLx with PostgreSQL

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::env;
use uuid::Uuid;
use validator::Validate;

use crate::auth::{AuthService, RegisterInput, UserRow};
use crate::errors::{AppError, AppResult};
use crate::types::Note;

/// Internal row structure that matches the PostgreSQL schema
#[derive(sqlx::FromRow)]
struct NoteRow {
    id: Uuid,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    user_id: Option<Uuid>, // Optional for backward compatibility
}

impl From<NoteRow> for Note {
    fn from(row: NoteRow) -> Self {
        Note {
            id: row.id.to_string(),
            title: row.title,
            content: row.content,
            created_at: row.created_at.to_rfc3339(),
            updated_at: row.updated_at.to_rfc3339(),
        }
    }
}

/// Database operations struct
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create new database instance with connection pool
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Run database migrations
    pub async fn migrate(&self) -> AppResult<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: format!("Migration failed: {}", e),
            })?;
        Ok(())
    }

    /// Create a new note in PostgreSQL
    pub async fn create_note(&self, title: &str, content: &str) -> AppResult<Note> {
        let uuid = Uuid::new_v4();
        let now = Utc::now();

        let row = sqlx::query(
            r#"
            INSERT INTO notes (id, title, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, content, created_at, updated_at, user_id
            "#,
        )
        .bind(uuid)
        .bind(title)
        .bind(content)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to create note: {}", e),
        })?;

        let note_row = NoteRow {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            user_id: row.get("user_id"),
        };

        Ok(note_row.into())
    }

    /// Get all notes from PostgreSQL
    pub async fn get_all_notes(&self) -> AppResult<Vec<Note>> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, content, created_at, updated_at, user_id 
            FROM notes 
            ORDER BY updated_at DESC, created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to fetch notes: {}", e),
        })?;

        let notes: Vec<Note> = rows
            .into_iter()
            .map(|row| {
                let note_row = NoteRow {
                    id: row.get("id"),
                    title: row.get("title"),
                    content: row.get("content"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    user_id: row.get("user_id"),
                };
                note_row.into()
            })
            .collect();

        Ok(notes)
    }

    /// Get a single note by ID from PostgreSQL
    pub async fn get_note_by_id(&self, id: &str) -> AppResult<Option<Note>> {
        let uuid = Uuid::parse_str(id).map_err(|_| AppError::InvalidUuid {
            uuid: id.to_string(),
        })?;

        let row = sqlx::query(
            r#"
            SELECT id, title, content, created_at, updated_at, user_id 
            FROM notes 
            WHERE id = $1
            "#,
        )
        .bind(uuid)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to fetch note: {}", e),
        })?;

        match row {
            Some(row) => {
                let note_row = NoteRow {
                    id: row.get("id"),
                    title: row.get("title"),
                    content: row.get("content"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    user_id: row.get("user_id"),
                };
                Ok(Some(note_row.into()))
            }
            None => Ok(None),
        }
    }

    /// Update a note in PostgreSQL
    pub async fn update_note(
        &self,
        id: &str,
        title: Option<&str>,
        content: Option<&str>,
    ) -> AppResult<Option<Note>> {
        let uuid = Uuid::parse_str(id).map_err(|_| AppError::InvalidUuid {
            uuid: id.to_string(),
        })?;

        let row = match (title, content) {
            (Some(title), Some(content)) => {
                sqlx::query(
                    r#"
                    UPDATE notes 
                    SET title = $2, content = $3
                    WHERE id = $1
                    RETURNING id, title, content, created_at, updated_at, user_id
                    "#,
                )
                .bind(uuid)
                .bind(title)
                .bind(content)
                .fetch_optional(&self.pool)
                .await
            }
            (Some(title), None) => {
                sqlx::query(
                    r#"
                    UPDATE notes 
                    SET title = $2
                    WHERE id = $1
                    RETURNING id, title, content, created_at, updated_at, user_id
                    "#,
                )
                .bind(uuid)
                .bind(title)
                .fetch_optional(&self.pool)
                .await
            }
            (None, Some(content)) => {
                sqlx::query(
                    r#"
                    UPDATE notes 
                    SET content = $2
                    WHERE id = $1
                    RETURNING id, title, content, created_at, updated_at, user_id
                    "#,
                )
                .bind(uuid)
                .bind(content)
                .fetch_optional(&self.pool)
                .await
            }
            (None, None) => {
                sqlx::query(
                    r#"
                    UPDATE notes 
                    SET updated_at = NOW()
                    WHERE id = $1
                    RETURNING id, title, content, created_at, updated_at, user_id
                    "#,
                )
                .bind(uuid)
                .fetch_optional(&self.pool)
                .await
            }
        }
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to update note: {}", e),
        })?;

        match row {
            Some(row) => {
                let note_row = NoteRow {
                    id: row.get("id"),
                    title: row.get("title"),
                    content: row.get("content"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    user_id: row.get("user_id"),
                };
                Ok(Some(note_row.into()))
            }
            None => Ok(None),
        }
    }

    /// Delete a note from PostgreSQL
    pub async fn delete_note(&self, id: &str) -> AppResult<bool> {
        let uuid = Uuid::parse_str(id).map_err(|_| AppError::InvalidUuid {
            uuid: id.to_string(),
        })?;

        let result = sqlx::query("DELETE FROM notes WHERE id = $1")
            .bind(uuid)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: format!("Failed to delete note: {}", e),
            })?;

        Ok(result.rows_affected() > 0)
    }

    /// Search notes with full-text search
    pub async fn search_notes(&self, query: &str) -> AppResult<Vec<Note>> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, content, created_at, updated_at, user_id
            FROM notes 
            WHERE to_tsvector('english', title || ' ' || content) @@ plainto_tsquery('english', $1)
            ORDER BY ts_rank(to_tsvector('english', title || ' ' || content), plainto_tsquery('english', $1)) DESC, 
                     updated_at DESC
            LIMIT 100
            "#,
        )
        .bind(query)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to search notes: {}", e),
        })?;

        let notes: Vec<Note> = rows
            .into_iter()
            .map(|row| {
                let note_row = NoteRow {
                    id: row.get("id"),
                    title: row.get("title"),
                    content: row.get("content"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    user_id: row.get("user_id"),
                };
                note_row.into()
            })
            .collect();

        Ok(notes)
    }

    /// Create a new user
    pub async fn create_user(
        &self,
        input: &RegisterInput,
        auth: &AuthService,
    ) -> AppResult<UserRow> {
        // Validate input
        input.validate().map_err(|e| AppError::ValidationError {
            message: format!("Validation failed: {}", e),
        })?;

        // Check if email already exists
        let existing = self.get_user_by_email(&input.email).await?;
        if existing.is_some() {
            return Err(AppError::EmailAlreadyExists);
        }

        // Hash password
        let password_hash = auth.hash_password(&input.password)?;

        let uuid = Uuid::new_v4();
        let now = Utc::now();

        let row = sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, full_name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, password_hash, full_name, created_at, updated_at, is_active
            "#,
        )
        .bind(uuid)
        .bind(input.email.to_lowercase().trim())
        .bind(password_hash)
        .bind(&input.full_name)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to create user: {}", e),
        })?;

        let user = UserRow {
            id: row.get("id"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            full_name: row.get("full_name"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            is_active: row.get("is_active"),
        };

        Ok(user)
    }

    /// Get user by email
    pub async fn get_user_by_email(&self, email: &str) -> AppResult<Option<UserRow>> {
        let row = sqlx::query(
            r#"
            SELECT id, email, password_hash, full_name, created_at, updated_at, is_active
            FROM users
            WHERE email = $1 AND is_active = true
            "#,
        )
        .bind(email.to_lowercase().trim())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to fetch user by email: {}", e),
        })?;

        match row {
            Some(row) => {
                let user = UserRow {
                    id: row.get("id"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    full_name: row.get("full_name"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    is_active: row.get("is_active"),
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, user_id: Uuid) -> AppResult<Option<UserRow>> {
        let row = sqlx::query(
            r#"
            SELECT id, email, password_hash, full_name, created_at, updated_at, is_active
            FROM users
            WHERE id = $1 AND is_active = true
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to fetch user by ID: {}", e),
        })?;

        match row {
            Some(row) => {
                let user = UserRow {
                    id: row.get("id"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    full_name: row.get("full_name"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    is_active: row.get("is_active"),
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    /// Create note for authenticated user
    pub async fn create_note_for_user(
        &self,
        user_id: Uuid,
        title: &str,
        content: &str,
    ) -> AppResult<Note> {
        let note_id = Uuid::new_v4();
        let now = Utc::now();

        let row = sqlx::query(
            r#"
            INSERT INTO notes (id, user_id, title, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, title, content, created_at, updated_at, user_id
            "#,
        )
        .bind(note_id)
        .bind(user_id)
        .bind(title)
        .bind(content)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to create note: {}", e),
        })?;

        let note_row = NoteRow {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            user_id: row.get("user_id"),
        };

        Ok(note_row.into())
    }

    /// Get user's notes only
    pub async fn get_user_notes(&self, user_id: Uuid) -> AppResult<Vec<Note>> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, content, created_at, updated_at, user_id
            FROM notes
            WHERE user_id = $1
            ORDER BY updated_at DESC, created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to fetch user notes: {}", e),
        })?;

        let notes: Vec<Note> = rows
            .into_iter()
            .map(|row| {
                let note_row = NoteRow {
                    id: row.get("id"),
                    title: row.get("title"),
                    content: row.get("content"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    user_id: row.get("user_id"),
                };
                note_row.into()
            })
            .collect();

        Ok(notes)
    }
}

/// Create database connection pool
pub async fn create_database_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:smartnotes2024@localhost:5433/smart_notes".to_string()
    });

    println!(
        "🐘 Connecting to PostgreSQL: {}",
        database_url.replace("smartnotes2024", "***")
    );

    PgPool::connect(&database_url).await
}
