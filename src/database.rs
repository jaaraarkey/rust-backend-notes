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
use crate::types::{CreateFolderInput, Folder, Note, UpdateFolderInput}; // ✅ Add missing imports

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
            // ✅ Add missing fields with default values for compatibility
            is_pinned: false,
            pinned_at: None,
            view_count: 0,
            word_count: 0,
            folder: None,
        }
    }
}

/// Internal folder row structure
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct FolderRow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub icon: String,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub position: i32,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Enhanced note row with folder information
#[derive(sqlx::FromRow, Debug)]
pub struct EnhancedNoteRow {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub is_pinned: bool,
    pub pinned_at: Option<DateTime<Utc>>,
    pub view_count: i32,
    pub word_count: i32,
    // Folder info (from JOIN)
    pub folder_name: Option<String>,
    pub folder_color: Option<String>,
    pub folder_icon: Option<String>,
}

impl From<EnhancedNoteRow> for Note {
    fn from(row: EnhancedNoteRow) -> Self {
        let folder = if let (Some(folder_id), Some(folder_name)) =
            (row.folder_id, row.folder_name.as_ref())
        {
            Some(Folder {
                id: folder_id.to_string(),
                name: folder_name.clone(),
                description: None, // We'll load full folder details separately if needed
                color: row.folder_color.unwrap_or_else(|| "#3B82F6".to_string()),
                icon: row.folder_icon.unwrap_or_else(|| "folder".to_string()),
                position: 0,
                notes_count: 0,
                is_default: false,          // Add this missing field
                created_at: "".to_string(), // Placeholder for list view
                updated_at: "".to_string(),
                parent_folder: None,
                subfolders: vec![],
            })
        } else {
            None
        };

        Note {
            id: row.id.to_string(),
            title: row.title,
            content: row.content,
            created_at: row.created_at.to_rfc3339(),
            updated_at: row.updated_at.to_rfc3339(),
            is_pinned: row.is_pinned,
            pinned_at: row.pinned_at.map(|dt| dt.to_rfc3339()),
            view_count: row.view_count,
            word_count: row.word_count,
            folder,
        }
    }
}

impl From<FolderRow> for Folder {
    fn from(row: FolderRow) -> Self {
        Folder {
            id: row.id.to_string(),
            name: row.name,
            description: row.description,
            color: row.color,
            icon: row.icon,
            position: row.position,
            notes_count: 0,             // Will be loaded separately
            is_default: row.is_default, // Add this line
            created_at: row.created_at.to_rfc3339(),
            updated_at: row.updated_at.to_rfc3339(),
            parent_folder: None, // Will be loaded separately if needed
            subfolders: vec![],  // Will be loaded separately
        }
    }
}

/// Database operations struct
#[derive(Clone)] // ✅ Add Clone trait here
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

    /// 📁 Create a new folder
    pub async fn create_folder(
        &self,
        user_id: Uuid,
        input: &CreateFolderInput,
    ) -> AppResult<Folder> {
        let folder_id = Uuid::new_v4();
        let now = Utc::now();
        let color = input.color.as_deref().unwrap_or("#3B82F6");
        let icon = input.icon.as_deref().unwrap_or("folder");

        let row = sqlx::query(
            r#"
            INSERT INTO folders (id, name, description, color, icon, user_id, parent_id, position, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, name, description, color, icon, user_id, parent_id, position, is_default, created_at, updated_at
            "#,
        )
        .bind(folder_id)
        .bind(&input.name)
        .bind(&input.description)
        .bind(color)
        .bind(icon)
        .bind(user_id)
        .bind(None::<Uuid>) // parent_id for now
        .bind(input.position.unwrap_or(0))
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to create folder: {}", e),
        })?;

        Ok(Folder {
            id: row.get::<Uuid, _>("id").to_string(),
            name: row.get("name"),
            description: row.get("description"),
            color: row.get("color"),
            icon: row.get("icon"),
            position: row.get("position"),
            notes_count: 0,
            is_default: row.get("is_default"), // Add this line
            created_at: row.get::<DateTime<Utc>, _>("created_at").to_rfc3339(),
            updated_at: row.get::<DateTime<Utc>, _>("updated_at").to_rfc3339(),
            parent_folder: None,
            subfolders: vec![],
        })
    }

    /// 📁 Get user's folders with hierarchy
    pub async fn get_user_folders(&self, user_id: Uuid) -> AppResult<Vec<Folder>> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, description, color, icon, user_id, parent_id, position, is_default, created_at, updated_at
            FROM folders
            WHERE user_id = $1
            ORDER BY parent_id NULLS FIRST, position ASC, name ASC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to fetch user folders: {}", e),
        })?;

        let folders: Vec<Folder> = rows
            .into_iter()
            .map(|row| Folder {
                id: row.get::<Uuid, _>("id").to_string(),
                name: row.get("name"),
                description: row.get("description"),
                color: row.get("color"),
                icon: row.get("icon"),
                position: row.get("position"),
                notes_count: 0,                    // We'll load this separately
                is_default: row.get("is_default"), // Add this line
                created_at: row.get::<DateTime<Utc>, _>("created_at").to_rfc3339(),
                updated_at: row.get::<DateTime<Utc>, _>("updated_at").to_rfc3339(),
                parent_folder: None,
                subfolders: vec![],
            })
            .collect();

        Ok(folders)
    }

    /// 📁 Get folder by ID with full details
    pub async fn get_folder_by_id(
        &self,
        folder_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Option<Folder>> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, color, icon, user_id, parent_id, position, is_default, created_at, updated_at
            FROM folders
            WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(folder_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to fetch folder: {}", e),
        })?;

        match row {
            Some(row) => Ok(Some(Folder {
                id: row.get::<Uuid, _>("id").to_string(),
                name: row.get("name"),
                description: row.get("description"),
                color: row.get("color"),
                icon: row.get("icon"),
                position: row.get("position"),
                notes_count: 0,                    // Load separately if needed
                is_default: row.get("is_default"), // Add this line
                created_at: row.get::<DateTime<Utc>, _>("created_at").to_rfc3339(),
                updated_at: row.get::<DateTime<Utc>, _>("updated_at").to_rfc3339(),
                parent_folder: None,
                subfolders: vec![],
            })),
            None => Ok(None),
        }
    }

    /// 📁 Update folder (simplified)
    pub async fn update_folder(
        &self,
        folder_id: Uuid,
        user_id: Uuid,
        input: &UpdateFolderInput,
    ) -> AppResult<Option<Folder>> {
        // Simple update - just name for now
        if let Some(name) = &input.name {
            sqlx::query(
                "UPDATE folders SET name = $1, updated_at = NOW() WHERE id = $2 AND user_id = $3",
            )
            .bind(name)
            .bind(folder_id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: format!("Failed to update folder: {}", e),
            })?;
        }

        // Return updated folder
        self.get_folder_by_id(folder_id, user_id).await
    }

    /// 📁 Delete folder (simplified)
    pub async fn delete_folder(
        &self,
        folder_id: Uuid,
        user_id: Uuid,
        _move_notes_to: Option<Uuid>,
    ) -> AppResult<bool> {
        let result = sqlx::query("DELETE FROM folders WHERE id = $1 AND user_id = $2")
            .bind(folder_id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: format!("Failed to delete folder: {}", e),
            })?;

        Ok(result.rows_affected() > 0)
    }

    /// 📝 Enhanced note creation with folder support (simplified)
    pub async fn create_note_with_folder(
        &self,
        user_id: Uuid,
        title: &str,
        content: &str,
        folder_id: Option<Uuid>,
        is_pinned: bool,
    ) -> AppResult<Note> {
        let note_id = Uuid::new_v4();
        let now = Utc::now();
        let pinned_at = if is_pinned { Some(now) } else { None };

        let row = sqlx::query(
            r#"
            INSERT INTO notes (id, user_id, title, content, folder_id, is_pinned, pinned_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, title, content, created_at, updated_at, user_id, folder_id, is_pinned, pinned_at, view_count, word_count
            "#,
        )
        .bind(note_id)
        .bind(user_id)
        .bind(title)
        .bind(content)
        .bind(folder_id)
        .bind(is_pinned)
        .bind(pinned_at)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to create note: {}", e),
        })?;

        Ok(Note {
            id: row.get::<Uuid, _>("id").to_string(),
            title: row.get("title"),
            content: row.get("content"),
            created_at: row.get::<DateTime<Utc>, _>("created_at").to_rfc3339(),
            updated_at: row.get::<DateTime<Utc>, _>("updated_at").to_rfc3339(),
            is_pinned: row.get("is_pinned"),
            pinned_at: row
                .get::<Option<DateTime<Utc>>, _>("pinned_at")
                .map(|dt| dt.to_rfc3339()),
            view_count: row.get("view_count"),
            word_count: row.get("word_count"),
            folder: None, // Load separately if needed
        })
    }

    /// 📚 Get notes in a specific folder (simplified)
    pub async fn get_notes_in_folder(
        &self,
        user_id: Uuid,
        folder_id: Option<Uuid>,
    ) -> AppResult<Vec<Note>> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, content, created_at, updated_at, user_id, folder_id, 
                   is_pinned, pinned_at, view_count, word_count
            FROM notes
            WHERE user_id = $1 AND ($2::UUID IS NULL AND folder_id IS NULL OR folder_id = $2)
            ORDER BY is_pinned DESC, updated_at DESC, created_at DESC
            "#,
        )
        .bind(user_id)
        .bind(folder_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to fetch notes in folder: {}", e),
        })?;

        let notes: Vec<Note> = rows
            .into_iter()
            .map(|row| Note {
                id: row.get::<Uuid, _>("id").to_string(),
                title: row.get("title"),
                content: row.get("content"),
                created_at: row.get::<DateTime<Utc>, _>("created_at").to_rfc3339(),
                updated_at: row.get::<DateTime<Utc>, _>("updated_at").to_rfc3339(),
                is_pinned: row.get("is_pinned"),
                pinned_at: row
                    .get::<Option<DateTime<Utc>>, _>("pinned_at")
                    .map(|dt| dt.to_rfc3339()),
                view_count: row.get("view_count"),
                word_count: row.get("word_count"),
                folder: None, // Simplify for now
            })
            .collect();

        Ok(notes)
    }

    /// ⭐ Pin/unpin a note (simplified)
    pub async fn toggle_note_pin(
        &self,
        note_id: Uuid,
        user_id: Uuid,
        pin: bool,
    ) -> AppResult<Option<Note>> {
        let pinned_at = if pin { Some(Utc::now()) } else { None };

        let rows_affected = sqlx::query(
            "UPDATE notes SET is_pinned = $1, pinned_at = $2, updated_at = NOW() WHERE id = $3 AND user_id = $4"
        )
        .bind(pin)
        .bind(pinned_at)
        .bind(note_id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to toggle note pin: {}", e),
        })?
        .rows_affected();

        if rows_affected > 0 {
            self.get_note_by_id(&note_id.to_string()).await
        } else {
            Ok(None)
        }
    }

    /// ⭐ Get pinned notes for user (simplified)
    pub async fn get_pinned_notes(&self, user_id: Uuid) -> AppResult<Vec<Note>> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, content, created_at, updated_at, user_id, folder_id,
                   is_pinned, pinned_at, view_count, word_count
            FROM notes
            WHERE user_id = $1 AND is_pinned = TRUE
            ORDER BY pinned_at DESC, updated_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: format!("Failed to fetch pinned notes: {}", e),
        })?;

        let notes: Vec<Note> = rows
            .into_iter()
            .map(|row| Note {
                id: row.get::<Uuid, _>("id").to_string(),
                title: row.get("title"),
                content: row.get("content"),
                created_at: row.get::<DateTime<Utc>, _>("created_at").to_rfc3339(),
                updated_at: row.get::<DateTime<Utc>, _>("updated_at").to_rfc3339(),
                is_pinned: row.get("is_pinned"),
                pinned_at: row
                    .get::<Option<DateTime<Utc>>, _>("pinned_at")
                    .map(|dt| dt.to_rfc3339()),
                view_count: row.get("view_count"),
                word_count: row.get("word_count"),
                folder: None,
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
