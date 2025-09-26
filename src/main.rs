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
    Router, Server,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use crate::database::{create_database_pool, Database};
use crate::resolvers::{Mutation, Query};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize PostgreSQL connection pool
    println!("🐘 Initializing PostgreSQL connection...");
    let pool = create_database_pool().await?;

    // Create database instance and run migrations
    let database = Database::new(pool);
    println!("⚡ Running PostgreSQL migrations...");
    database.migrate().await?;
    println!("✅ PostgreSQL database ready!");

    // Build GraphQL schema with database context
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(database)
        .finish();

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    // Build application
    let app = Router::new()
        .route("/", get(web::graphiql))
        .route("/graphql", post(web::graphql_handler))
        .layer(Extension(schema))
        .layer(cors);

    // Server startup messages
    println!("🚀 Smart Notes GraphQL API with PostgreSQL ready!");
    println!("📊 GraphiQL interface: http://127.0.0.1:8000");
    println!("🔗 GraphQL endpoint: http://127.0.0.1:8000/graphql");
    println!("🐘 Database: PostgreSQL (enterprise-grade)");
    println!("🎯 Smart auto-title generation: ENABLED");
    println!("🔍 Full-text search: ENABLED");
    println!("⚡ Advanced indexing: ENABLED");

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
