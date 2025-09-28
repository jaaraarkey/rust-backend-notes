//! # Smart Notes API with JWT Authentication
//!
//! Production-ready GraphQL API with JWT middleware

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

use auth::AuthService; // ✅ Remove AuthContext import (unused)
use database::{create_database_pool, Database};
use resolvers::{MutationRoot, QueryRoot};
use web::{graphiql, graphql_handler, landing_page, AppSchema};

/// 🔐 JWT Authentication Middleware
async fn jwt_middleware(
    State((auth_service, db)): State<(AuthService, Database)>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    // Extract Authorization header
    let authorization = headers.get("authorization").and_then(|h| h.to_str().ok());

    // Create authentication context
    let auth_context = auth_service.create_auth_context(authorization, &db).await;

    // Add auth context to request extensions
    request.extensions_mut().insert(auth_context);

    // Continue to next handler
    next.run(request).await
}

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

    let auth_service = AuthService::new();

    // Create GraphQL schema
    let schema: AppSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db.clone())
        .data(auth_service.clone())
        .finish();

    // Build application routes with JWT middleware
    let app = Router::new()
        .route("/", get(landing_page))
        .route("/graphiql", get(graphiql))
        .route("/graphql", post(graphql_handler))
        .layer(middleware::from_fn_with_state(
            (auth_service, db),
            jwt_middleware,
        ))
        .layer(CorsLayer::permissive())
        .with_state(schema);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .unwrap_or(8000);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

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

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
