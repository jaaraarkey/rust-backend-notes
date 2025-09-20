//! # Notes App Backend
//!
//! A GraphQL API server for managing notes, built with Rust, Axum, and async-graphql.
//!
//! ## Features
//!
//! - GraphQL API with introspection
//! - GraphiQL playground for development
//! - Async/await throughout for high performance
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

use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::Html,
    routing::{get, post},
    Router,
    Server, // Add Server import
};
use std::net::SocketAddr; // Add this import

/// Represents a note in our application.
///
/// Notes have a unique ID, title, and content. This struct is automatically
/// converted to a GraphQL type thanks to the `SimpleObject` derive macro.
#[derive(SimpleObject, Clone, Debug)]
pub struct Note {
    /// Unique identifier for the note
    pub id: i32,
    /// The note's title
    pub title: String,
    /// The note's content/body
    pub content: String,
}

/// The root Query type for our GraphQL schema.
///
/// This contains all the "read" operations that clients can perform.
/// Each method in this impl block becomes a field in the GraphQL Query type.
pub struct Query;

#[Object]
impl Query {
    /// A simple hello world query for testing the GraphQL setup.
    ///
    /// This is useful for:
    /// - Verifying the server is running
    /// - Testing GraphQL client connections
    /// - Basic health checks
    async fn hello(&self) -> &str {
        "Hello from GraphQL!"
    }
}

/// Our complete GraphQL schema type.
///
/// This combines Query (read operations), Mutation (write operations),
/// and Subscription (real-time operations) into a single schema.
/// Currently we only have Query implemented.
type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// Handles incoming GraphQL requests.
///
/// This function:
/// 1. Extracts the GraphQL schema from Axum's Extension layer
/// 2. Parses the incoming GraphQL request
/// 3. Executes the query against our schema
/// 4. Returns the result as a GraphQL response
async fn graphql_handler(
    Extension(schema): Extension<MySchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// Serves the GraphiQL playground interface.
///
/// GraphiQL is a browser-based IDE for exploring GraphQL schemas.
/// It provides:
/// - Query editing with syntax highlighting
/// - Auto-completion based on schema
/// - Documentation browser
/// - Query validation
async fn graphiql() -> Html<&'static str> {
    Html(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>GraphQL Test Interface</title>
    <style>
        body { font-family: monospace; padding: 20px; }
        .container { max-width: 800px; margin: 0 auto; }
        textarea { width: 100%; height: 200px; margin: 10px 0; }
        button { padding: 10px 20px; background: #0066cc; color: white; border: none; cursor: pointer; }
        .result { background: #f5f5f5; padding: 15px; margin-top: 20px; white-space: pre-wrap; }
    </style>
</head>
<body>
    <div class="container">
        <h1>GraphQL Test Interface</h1>
        <p>Your GraphQL server is running! 🚀</p>
        
        <h3>Test Query:</h3>
        <textarea id="query" placeholder="Enter your GraphQL query...">query {
  hello
}</textarea>
        
        <button onclick="executeQuery()">Execute Query</button>
        
        <div class="result" id="result">Results will appear here...</div>
        
        <h3>Example Queries:</h3>
        <ul>
            <li><code>{ hello }</code> - Basic hello query</li>
            <li><code>{ __schema { queryType { name } } }</code> - Schema introspection</li>
        </ul>
    </div>

    <script>
        async function executeQuery() {
            const query = document.getElementById('query').value;
            const resultDiv = document.getElementById('result');
            
            try {
                const response = await fetch('/graphql', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ query })
                });
                
                const result = await response.json();
                resultDiv.textContent = JSON.stringify(result, null, 2);
            } catch (error) {
                resultDiv.textContent = 'Error: ' + error.message;
            }
        }
    </script>
</body>
</html>
    "#,
    )
}

/// Application entry point.
///
/// This function:
/// 1. Creates the GraphQL schema
/// 2. Sets up Axum routes
/// 3. Starts the HTTP server
/// 4. Listens for incoming connections
#[tokio::main]
async fn main() {
    // Initialize the GraphQL schema with our Query type
    // EmptyMutation and EmptySubscription are placeholders for now
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    // Build the Axum application with our routes
    let app = Router::new()
        // POST /graphql - handles GraphQL queries and mutations
        .route("/graphql", post(graphql_handler))
        // GET / - serves the GraphiQL playground
        .route("/", get(graphiql))
        // Make the schema available to all handlers via Extension
        .layer(Extension(schema));

    // Bind to localhost port 8000 (Axum 0.6 syntax)
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    println!("🚀 GraphQL server running on http://127.0.0.1:8000");
    println!("📊 GraphiQL playground available at http://127.0.0.1:8000");
    println!("📝 Send GraphQL requests to http://127.0.0.1:8000/graphql");

    // Start serving requests (Axum 0.6 syntax)
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
