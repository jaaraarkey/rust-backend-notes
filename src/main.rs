//! # Notes App Backend - Day 4 Enhanced: Clean Architecture
//!
//! A production-ready GraphQL API server for managing notes, built with Rust, Axum, and async-graphql.
//! Features clean, modular architecture following Rust best practices.
//!
//! ## 🏗️ Architecture Overview
//!
//! This application uses a layered, modular architecture:
//!
//! ```text
//! ┌─────────────────┐
//! │   HTTP Layer    │  web.rs - GraphQL handlers, GraphiQL UI
//! ├─────────────────┤
//! │ Resolver Layer  │  resolvers.rs - Business logic, Query/Mutation
//! ├─────────────────┤
//! │   Type Layer    │  types.rs - GraphQL schema definitions
//! ├─────────────────┤
//! │   Data Layer    │  data.rs - Data access, sample data
//! └─────────────────┘
//! ```
//!
//! ## 🚀 Features
//!
//! ### Day 1-2: Foundation
//! - ✅ GraphQL server with Axum integration
//! - ✅ Interactive GraphiQL playground
//! - ✅ Type-safe schema definition
//! - ✅ Query operations (hello, notes list)
//!
//! ### Day 3: Advanced Queries
//! - ✅ UUID-based unique identifiers
//! - ✅ Single note queries with error handling
//! - ✅ Optional vs required GraphQL types
//! - ✅ Field selection and query composition
//!
//! ### Day 4: Mutations & Architecture
//! - ✅ GraphQL mutations for data modification
//! - ✅ Input types for structured arguments
//! - ✅ Automatic UUID generation
//! - ✅ Clean, modular code organization
//!
//! ## 📊 GraphQL Schema
//!
//! ```graphql
//! type Query {
//!   hello: String!
//!   notes: [Note!]!
//!   note(id: String!): Note
//! }
//!
//! type Mutation {
//!   createNote(input: CreateNoteInput!): Note!
//! }
//!
//! input CreateNoteInput {
//!   title: String!
//!   content: String!
//! }
//!
//! type Note {
//!   id: String!      # UUID format
//!   title: String!
//!   content: String!
//! }
//! ```
//!
//! ## 🛠️ Usage
//!
//! ### Development Server
//! ```bash
//! # Start the development server
//! cargo run
//!
//! # Server runs on http://127.0.0.1:8000
//! # GraphiQL playground: http://127.0.0.1:8000
//! # GraphQL endpoint: http://127.0.0.1:8000/graphql
//! ```
//!
//! ### Example Queries
//! ```bash
//! # List all notes
//! curl -X POST http://127.0.0.1:8000/graphql \
//!   -H "Content-Type: application/json" \
//!   -d '{"query": "{ notes { id title } }"}'
//!
//! # Get single note
//! curl -X POST http://127.0.0.1:8000/graphql \
//!   -H "Content-Type: application/json" \
//!   -d '{"query": "{ note(id: \"uuid-here\") { title content } }"}'
//!
//! # Create new note
//! curl -X POST http://127.0.0.1:8000/graphql \
//!   -H "Content-Type: application/json" \
//!   -d '{"query": "mutation { createNote(input: {title: \"Test\", content: \"Content\"}) { id } }"}'
//! ```
//!
//! ## 🗂️ Module Documentation
//!
//! - [`types`] - GraphQL type definitions and input structures
//! - [`resolvers`] - Query and Mutation business logic implementations  
//! - [`data`] - Data access functions and sample data management
//! - [`web`] - HTTP handlers, GraphiQL UI, and server configuration
//!
//! ## 🎯 Learning Roadmap
//!
//! This codebase demonstrates progressive GraphQL concepts:
//!
//! - **Days 1-4**: ✅ Core GraphQL (queries, mutations, types)
//! - **Days 5-6**: 🔄 Complete CRUD operations
//! - **Days 7-11**: 🔄 Flutter integration
//! - **Days 12-14**: 🔄 Database, real-time, deployment
//!
//! ## 📚 Dependencies
//!
//! - [`async-graphql`] - GraphQL server implementation
//! - [`axum`] - Modern web framework
//! - [`tokio`] - Async runtime
//! - [`uuid`] - UUID generation

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
