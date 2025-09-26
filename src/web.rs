//! # Web Layer - HTTP Handlers & GraphiQL Interface
//!
//! This module provides the HTTP layer for our GraphQL API, including:
//! - GraphQL request handling
//! - GraphiQL playground interface
//! - CORS and middleware support

use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
};

use crate::resolvers::{Mutation, Query};

/// GraphQL schema type alias for convenience
pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// GraphQL endpoint handler
///
/// Processes GraphQL queries and mutations sent via HTTP POST
pub async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// GraphiQL playground interface
///
/// Provides an interactive web UI for testing GraphQL queries
pub async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .title("Smart Notes GraphQL API - Database Edition")
            .subscription_endpoint("/ws") // Optional: for subscriptions later
            .finish(),
    )
}
