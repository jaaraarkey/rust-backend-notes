//! # Smart Notes GraphQL API with JWT Authentication
//!
//! ## Overview
//!
//! A production-ready GraphQL API built with Rust that provides:
//! - 🔐 JWT-based authentication with bcrypt password hashing
//! - 🗄️ PostgreSQL database integration with SQLx
//! - 🎯 Intelligent auto-title generation for notes
//! - 🔍 Full-text search capabilities
//! - 🌐 Modern GraphQL API with interactive playground
//! - 🛡️ Comprehensive security and error handling
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
//! │   GraphQL API   │────│ JWT Middleware  │────│  PostgreSQL DB  │
//! │  (async-graphql)│    │   (Auth Layer)  │    │   (SQLx Pool)   │
//! └─────────────────┘    └─────────────────┘    └─────────────────┘
//! ```
//!
//! ## Features
//!
//! - **Authentication**: JWT tokens with 24h validity, bcrypt password hashing
//! - **Database**: PostgreSQL with connection pooling and migrations
//! - **API**: GraphQL with type-safe operations and interactive playground
//! - **Security**: Route protection, input validation, CORS support
//! - **Intelligence**: Auto-title generation, full-text search
//!
//! ## Quick Start
//!
//! 1. Set up PostgreSQL database
//! 2. Set environment variables (DATABASE_URL, JWT_SECRET)
//! 3. Run `cargo run` to start the server
//! 4. Visit http://127.0.0.1:8000 for the landing page
//! 5. Use http://127.0.0.1:8000/graphiql for API testing
//!
//! ## Environment Variables
//!
//! - `DATABASE_URL`: PostgreSQL connection string (required)
//! - `JWT_SECRET`: Secret key for JWT signing (optional, auto-generated)
//! - `PORT`: Server port (optional, defaults to 8000)

mod auth;
mod database;
mod errors;
mod resolvers;
mod types;
mod web;

use async_graphql::{EmptySubscription, Schema};
use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use auth::AuthService;
use database::{create_database_pool, Database};
use resolvers::{MutationRoot, QueryRoot};
use web::{graphiql, graphql_handler, landing_page, AppSchema};

/// 🔐 JWT Authentication Middleware
///
/// This middleware automatically:
/// - Extracts JWT tokens from Authorization headers
/// - Validates token signatures and expiration
/// - Fetches user data from the database
/// - Creates authentication context for GraphQL resolvers
/// - Handles both authenticated and unauthenticated requests gracefully
///
/// ## Flow
/// 1. Extract `Authorization: Bearer <token>` header
/// 2. Verify JWT signature and claims
/// 3. Load user data from database
/// 4. Create `AuthContext` with user information
/// 5. Add context to request extensions
/// 6. Continue to next middleware/handler
///
/// ## Error Handling
/// - Invalid tokens result in unauthenticated context (not errors)
/// - Database failures result in unauthenticated context
/// - Missing headers are handled gracefully
async fn jwt_middleware(
    State((auth_service, db)): State<(AuthService, Database)>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    // Extract Authorization header with Bearer token
    let authorization = headers.get("authorization").and_then(|h| h.to_str().ok());

    // Create authentication context (never fails, returns unauth if invalid)
    let auth_context = auth_service.create_auth_context(authorization, &db).await;

    // Add auth context to request extensions for GraphQL resolvers
    request.extensions_mut().insert(auth_context);

    // Continue to next handler
    next.run(request).await
}

/// 🚀 Application Entry Point
///
/// Initializes and starts the Smart Notes GraphQL API server with:
/// - PostgreSQL database connection and migrations
/// - JWT authentication middleware
/// - GraphQL schema with resolvers
/// - Web routes (landing page, GraphiQL, API endpoint)
/// - CORS support for cross-origin requests
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    println!("🗃️  Initializing database connection...");

    // Create PostgreSQL connection pool with automatic retries
    let pool = create_database_pool().await?;
    let db = Database::new(pool);

    println!("⚡ Running database migrations...");
    // Apply any pending database migrations
    db.migrate().await?;

    println!("✅ Database ready!");

    // Initialize JWT authentication service
    let auth_service = AuthService::new();

    // Build GraphQL schema with query/mutation resolvers and shared state
    let schema: AppSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db.clone()) // Database access for resolvers
        .data(auth_service.clone()) // Auth service for login/register
        .finish();

    // Build application routes with JWT middleware
    let app = Router::new()
        .route("/", get(landing_page)) // Beautiful landing page
        .route("/graphiql", get(graphiql)) // Interactive GraphQL playground
        .route("/graphql", post(graphql_handler)) // GraphQL API endpoint
        .layer(middleware::from_fn_with_state(
            // JWT authentication middleware
            (auth_service, db),
            jwt_middleware,
        ))
        .layer(CorsLayer::permissive()) // CORS support
        .with_state(schema); // GraphQL schema state

    // Configure server port from environment or default to 8000
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .unwrap_or(8000);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // Print server information and feature summary
    println!(
        "🚀 GraphQL server with JWT Authentication ready at http://127.0.0.1:{}",
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
    println!("🔐 Authentication: JWT-based with bcrypt password hashing");
    println!("✨ JWT Middleware: OPERATIONAL!");

    println!("🔧 Starting server on port {}", port);

    // Start the HTTP server with graceful shutdown support
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
