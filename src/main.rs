use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{database, sqlite::SqlitePool, Row, Sqlite, SqlitePool as Pool};
use std::sync::Arc;
use tokio::net::TcpListener;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Note {
    id: String,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct CreateNoteRequest {
    title: String,
    content: String,
}
#[derive(Serialize)]
struct ApiResponse<T> {
    data: T,
    message: String,
}
#[derive(Clone)]
struct AppState {
    db: Arc<SqlitePool>,
}

async fn hello() -> axum::Json<Message> {
    axum::Json(Message {
        text: "Hello from Json".into(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(hello));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

// Initialize the database and create the notes table if it doesn't exist
async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // Create a new database file if it doesn't exist
    let database_url = "sqlite:notes.db";

    // Create connection pool
    let pool = SqlitePool::connect(database_url).await?;

    // Run schema to create tables
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS notes (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

// Create a new note
async fn create_note(
    pool: &SqlitePool,
    title: String,
    content: String,
) -> Result<Note, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO notes (id, title, content, crreated_at, updated_at) values (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&title)
    .bind(&content)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(Note {
        id,
        title,
        content,
        created_at: now,
        updated_at: now,
    })
}

// Get all notes
async fn get_all_notes(pool: &SqlitePool) -> Result<Vec<Note>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM notes ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;

    let mut notes = Vec::new();
    for row in rows {
        notes.push(Note {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }

    Ok(notes)
}

// Get a single note by ID
async fn get_note_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Note>, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM notes WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    match row {
        Some(row) => Ok(Some(Note {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })),
        None => Ok(None),
    }
}
