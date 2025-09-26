//! # Database Layer
//!
//! This module handles all database operations for the notes application.
//! Uses SQLite with connection pooling and transaction support.

use chrono::Utc;
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::types::Note;

/// Database connection pool type alias
pub type DbPool = Pool<Sqlite>;

/// Database operations for notes
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
        // Read migration file
        let migration_sql = include_str!("../migrations/001_initial_schema.sql");

        // Execute migration
        sqlx::query(migration_sql)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: format!("Migration failed: {}", e),
            })?;

        Ok(())
    }

    /// Get all notes ordered by creation date (newest first)
    pub async fn get_all_notes(&self) -> AppResult<Vec<Note>> {
        let rows = sqlx::query(
            "SELECT id, title, content, created_at, updated_at 
             FROM notes 
             ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to fetch notes: {}", e),
        })?;

        let notes = rows
            .into_iter()
            .map(|row| Note {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(notes)
    }

    /// Get a single note by ID
    pub async fn get_note_by_id(&self, id: &str) -> AppResult<Option<Note>> {
        let row = sqlx::query(
            "SELECT id, title, content, created_at, updated_at 
             FROM notes 
             WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to fetch note by ID: {}", e),
        })?;

        if let Some(row) = row {
            Ok(Some(Note {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Create a new note
    pub async fn create_note(&self, title: &str, content: &str) -> AppResult<Note> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO notes (id, title, content, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(title)
        .bind(content)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to create note: {}", e),
        })?;

        Ok(Note {
            id,
            title: title.to_string(),
            content: content.to_string(),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    /// Update an existing note
    pub async fn update_note(
        &self,
        id: &str,
        title: Option<&str>,
        content: Option<&str>,
    ) -> AppResult<Option<Note>> {
        // First check if note exists
        let existing = self.get_note_by_id(id).await?;

        if let Some(mut note) = existing {
            // Update fields if provided
            if let Some(new_title) = title {
                note.title = new_title.to_string();
            }
            if let Some(new_content) = content {
                note.content = new_content.to_string();
            }

            // Always update the timestamp
            note.updated_at = Utc::now().to_rfc3339();

            // Save to database
            sqlx::query(
                "UPDATE notes 
                 SET title = ?, content = ?, updated_at = ? 
                 WHERE id = ?",
            )
            .bind(&note.title)
            .bind(&note.content)
            .bind(&note.updated_at)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: format!("Failed to update note: {}", e),
            })?;

            Ok(Some(note))
        } else {
            Ok(None)
        }
    }

    /// Delete a note by ID
    pub async fn delete_note(&self, id: &str) -> AppResult<bool> {
        let result = sqlx::query("DELETE FROM notes WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: format!("Failed to delete note: {}", e),
            })?;

        Ok(result.rows_affected() > 0)
    }
}

/// Initialize database connection pool with better path handling
pub async fn create_database_pool() -> AppResult<DbPool> {
    // Use current directory with explicit path
    let db_path = std::env::current_dir()
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to get current directory: {}", e),
        })?
        .join("notes.db");

    println!("📁 Database path: {}", db_path.display());

    // Ensure the parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| AppError::DatabaseError {
            message: format!("Failed to create database directory: {}", e),
        })?;
    }

    let database_url = format!("sqlite:{}?mode=rwc", db_path.display());
    println!("🔗 Connecting to: {}", database_url);

    let pool = SqlitePool::connect(&database_url)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to connect to database: {}", e),
        })?;

    Ok(pool)
}
