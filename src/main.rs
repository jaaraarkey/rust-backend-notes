//! # Notes App Backend - Pure Axum Implementation
//!
//! Bypassing async-graphql-axum to avoid version conflicts

mod database;
mod errors;
mod resolvers;
mod types;
mod web;

use async_graphql::{EmptySubscription, Schema};
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use database::{create_database_pool, Database};
use resolvers::{MutationRoot, QueryRoot};
use web::{graphiql, graphql_handler, landing_page, AppSchema};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    println!("🗃️  Initializing database connection...");

    // Create database pool
    let pool = create_database_pool().await?;
    let db = Database::new(pool);

    println!("⚡ Running database migrations...");
    db.migrate().await?;

    println!("✅ Database ready!");

    // Create GraphQL schema with EmptySubscription
    let schema: AppSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish();

    // Build application routes
    let app = Router::new()
        .route("/", get(landing_page)) // Beautiful landing page
        .route("/graphiql", get(graphiql)) // Interactive GraphiQL
        .route("/graphql", post(graphql_handler)) // GraphQL endpoint
        .layer(CorsLayer::permissive())
        .with_state(schema);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .unwrap_or(8000);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!(
        "🚀 GraphQL server with Docker PostgreSQL ready at http://127.0.0.1:{}",
        port
    );
    println!("🌟 Beautiful landing page at http://127.0.0.1:{}", port);
    println!(
        "🎮 Interactive GraphiQL at http://127.0.0.1:{}/graphiql",
        port
    );
    println!("📡 GraphQL endpoint at http://127.0.0.1:{}/graphql", port);
    println!("🐳 Database: Docker PostgreSQL (smart_notes) - Port 5433");
    println!("🎯 Smart auto-title generation: ENABLED");
    println!("🔍 Full-text search: ENABLED");
    println!("✨ Your BRILLIANT NoteRow pattern matching is operational!");

    // Modern Axum server startup
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
