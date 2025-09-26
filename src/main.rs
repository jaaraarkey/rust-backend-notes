//! # Notes App Backend - Day 7: Database Integration
//!
//! A production-ready GraphQL API server with persistent SQLite storage,
//! smart auto-title generation, and comprehensive error handling.

mod database;
mod errors;
mod resolvers;
mod types;
mod validation;
mod web;

use async_graphql::{EmptySubscription, Schema};
use axum::{
    extract::Extension,
    http::Method,
    routing::{get, post},
    Router,
    Server, // ← Import Server for axum 0.6
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use crate::database::{create_database_pool, Database};
use crate::resolvers::{Mutation, Query};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database connection pool
    println!("🗃️  Initializing database connection...");
    let pool = create_database_pool().await?;

    // Create database instance and run migrations
    let database = Database::new(pool);
    println!("⚡ Running database migrations...");
    database.migrate().await?;
    println!("✅ Database ready!");

    // Build GraphQL schema with database context
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(database)
        .finish();

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    // Build application with routes and middleware
    let app = Router::new()
        .route("/", get(web::graphiql))
        .route("/graphql", post(web::graphql_handler))
        .layer(Extension(schema)) // ← BACK TO Extension pattern
        .layer(cors);

    // Server startup messages
    println!("🚀 GraphQL server with database ready at http://127.0.0.1:8000");
    println!("📊 GraphiQL interface at http://127.0.0.1:8000");
    println!("🗃️  Database: SQLite (notes.db)");
    println!("🎯 Smart auto-title generation: ENABLED");
    println!("✨ Stable Axum 0.6 API with Extension pattern");

    // Create server and bind to address (Axum 0.6 API)
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
