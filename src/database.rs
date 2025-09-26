//! # PostgreSQL Database Layer
//!
//! This module provides database operations using PostgreSQL with advanced features:
//! - Native UUID support
//! - Full-text search capabilities  
//! - Advanced indexing
//! - ACID transactions
//! - Concurrent connection handling

use chrono::{DateTime, Utc};
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::types::Note;

/// Database connection pool type alias
pub type DbPool = Pool<Postgres>;

/// Helper struct for database row mapping
#[derive(FromRow)]
struct NoteRow {
    id: Uuid,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
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
    pool: DbPool,
}

impl Database {
    /// Create a new database instance with connection pool
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Run database migrations
    pub async fn migrate(&self) -> AppResult<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: format!("Failed to run migrations: {}", e),
            })?;
        Ok(())
    }

    /// Create a new note in PostgreSQL
    pub async fn create_note(&self, title: &str, content: &str) -> AppResult<Note> {
        let uuid = Uuid::new_v4();
        let now = Utc::now();

        let row = sqlx::query_as!(
            NoteRow,
            r#"
            INSERT INTO notes (id, title, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, content, created_at, updated_at
            "#,
            uuid,
            title,
            content,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to create note: {}", e),
        })?;

        Ok(row.into())
    }

    /// Get all notes from PostgreSQL (with advanced ordering)
    pub async fn get_all_notes(&self) -> AppResult<Vec<Note>> {
        let rows = sqlx::query_as!(
            NoteRow,
            r#"
            SELECT id, title, content, created_at, updated_at 
            FROM notes 
            ORDER BY updated_at DESC, created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to fetch notes: {}", e),
        })?;

        Ok(rows.into_iter().map(Note::from).collect())
    }

    /// Get a single note by ID from PostgreSQL
    pub async fn get_note_by_id(&self, id: &str) -> AppResult<Option<Note>> {
        let uuid = Uuid::parse_str(id).map_err(|_| AppError::InvalidUuid {
            uuid: id.to_string(),
        })?;

        let row = sqlx::query_as!(
            NoteRow,
            r#"
            SELECT id, title, content, created_at, updated_at 
            FROM notes 
            WHERE id = $1
            "#,
            uuid
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to fetch note: {}", e),
        })?;

        Ok(row.map(Note::from))
    }

    /// Update a note in PostgreSQL (with automatic timestamp update)
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
                // Update both title and content
                sqlx::query_as!(
                    NoteRow,
                    r#"
                    UPDATE notes 
                    SET title = $2, content = $3
                    WHERE id = $1
                    RETURNING id, title, content, created_at, updated_at
                    "#,
                    uuid,
                    title,
                    content
                )
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError {
                    message: format!("Failed to update note: {}", e),
                })?
            }
            (Some(title), None) => {
                // Update only title
                sqlx::query_as!(
                    NoteRow,
                    r#"
                    UPDATE notes 
                    SET title = $2
                    WHERE id = $1
                    RETURNING id, title, content, created_at, updated_at
                    "#,
                    uuid,
                    title
                )
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError {
                    message: format!("Failed to update note: {}", e),
                })?
            }
            (None, Some(content)) => {
                // Update only content
                sqlx::query_as!(
                    NoteRow,
                    r#"
                    UPDATE notes 
                    SET content = $2
                    WHERE id = $1
                    RETURNING id, title, content, created_at, updated_at
                    "#,
                    uuid,
                    content
                )
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError {
                    message: format!("Failed to update note: {}", e),
                })?
            }
            (None, None) => {
                // Just trigger timestamp update
                sqlx::query_as!(
                    NoteRow,
                    r#"
                    UPDATE notes 
                    SET updated_at = NOW()
                    WHERE id = $1
                    RETURNING id, title, content, created_at, updated_at
                    "#,
                    uuid
                )
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError {
                    message: format!("Failed to update note: {}", e),
                })?
            }
        };

        Ok(row.map(Note::from))
    }

    /// Delete a note from PostgreSQL
    pub async fn delete_note(&self, id: &str) -> AppResult<bool> {
        let uuid = Uuid::parse_str(id).map_err(|_| AppError::InvalidUuid {
            uuid: id.to_string(),
        })?;

        let result = sqlx::query!(
            r#"
            DELETE FROM notes 
            WHERE id = $1
            "#,
            uuid
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to delete note: {}", e),
        })?;

        Ok(result.rows_affected() > 0)
    }

    /// 🔍 BONUS: Full-text search with PostgreSQL!
    pub async fn search_notes(&self, query: &str) -> AppResult<Vec<Note>> {
        let rows = sqlx::query_as!(
            NoteRow,
            r#"
            SELECT id, title, content, created_at, updated_at
            FROM notes 
            WHERE to_tsvector('english', title || ' ' || content) @@ plainto_tsquery('english', $1)
            ORDER BY ts_rank(to_tsvector('english', title || ' ' || content), plainto_tsquery('english', $1)) DESC, 
                     updated_at DESC
            LIMIT 100
            "#,
            query
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to search notes: {}", e),
        })?;

        Ok(rows.into_iter().map(Note::from).collect())
    }
}

/// Initialize PostgreSQL connection pool with environment-based configuration
pub async fn create_database_pool() -> AppResult<DbPool> {
    dotenv::dotenv().ok(); // Load .env file if it exists

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        // Default local PostgreSQL connection
        "postgresql://postgres:password@localhost:5432/smart_notes".to_string()
    });

    println!(
        "🐘 Connecting to PostgreSQL: {}",
        database_url.replace("password", "***")
    );

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20) // Production-ready connection pool
        .connect(&database_url)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to connect to PostgreSQL: {}", e),
        })?;

    Ok(pool)
}
