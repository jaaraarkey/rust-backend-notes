//! # Notes App Backend - Day 4 Enhanced
//!
//! A GraphQL API server for managing notes, built with Rust, Axum, and async-graphql.
//! Now with clean, modular architecture!
//!
//! ## Architecture
//!
//! - `types.rs` - GraphQL types and input definitions
//! - `resolvers.rs` - Query and Mutation implementations  
//! - `data.rs` - Data access and sample data
//! - `web.rs` - HTTP handlers and server setup
//! - `main.rs` - Application entry point
//!
//! ## Features
//!
//! - GraphQL API with introspection
//! - GraphiQL playground for development
//! - UUID-based unique identifiers
//! - Clean, modular code organization
//! - Type-safe schema definition
//!
//! ## Usage
//!
//! Start the server:
//! ```bash
//! cargo run
//! ```
//!
//! Then visit http://127.0.0.1:8000 for the GraphiQL playground.

mod data;
mod resolvers;
mod types;
mod web;

use async_graphql::{EmptySubscription, Schema};
use axum::{
    routing::{get, post},
    Router, Server,
};
use std::net::SocketAddr;

use resolvers::{Mutation, Query};
use web::{graphiql, graphql_handler, AppSchema};

/// Application entry point.
///
/// This function:
/// 1. Creates the GraphQL schema
/// 2. Sets up Axum routes
/// 3. Starts the HTTP server
/// 4. Listens for incoming connections
#[tokio::main]
async fn main() {
    // Initialize the GraphQL schema with our modular resolvers
    let schema: AppSchema = Schema::build(Query, Mutation, EmptySubscription).finish();

    // Build the Axum application with clean route structure
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/", get(graphiql))
        .layer(axum::extract::Extension(schema));

    // Server configuration
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    println!("🚀 GraphQL server running on http://127.0.0.1:8000");
    println!("📊 GraphiQL playground available at http://127.0.0.1:8000");
    println!("📝 Send GraphQL requests to http://127.0.0.1:8000/graphql");
    println!("🏗️  Day 4 Enhanced: Clean, modular architecture!");

    // Start serving requests
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
