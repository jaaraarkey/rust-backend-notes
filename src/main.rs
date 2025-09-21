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
    Router, Server,
};
use std::net::SocketAddr;
use uuid::Uuid; // Add UUID import

/// Represents a note in our application with UUID-based unique identification.
///
/// Notes have a UUID, title, and content. UUIDs are globally unique identifiers
/// that are much more robust than simple integer IDs.
#[derive(SimpleObject, Clone, Debug)]
pub struct Note {
    /// Unique identifier for the note (UUID format)
    pub id: String, // Changed from i32 to String to store UUID
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

fn get_sample_notes() -> Vec<Note> {
    vec![
        Note {
            id: "550e8400-e29b-41d4-a716-446655440001".to_string(),  // Example UUID
            title: "Welcome to GraphQL".to_string(),
            content: "This is your first note! GraphQL allows you to query exactly the fields you need. Now with UUID support!".to_string(),
        },
        Note {
            id: "550e8400-e29b-41d4-a716-446655440002".to_string(),
            title: "Learning Rust".to_string(),
            content: "Rust's type system helps catch errors at compile time, making GraphQL APIs more reliable. UUIDs provide better data integrity.".to_string(),
        },
        Note {
            id: "550e8400-e29b-41d4-a716-446655440003".to_string(),
            title: "async-graphql Features".to_string(), 
            content: "The async-graphql crate provides powerful features like field selection, introspection, and automatic schema generation with UUID support.".to_string(),
        },
        Note {
            id: "550e8400-e29b-41d4-a716-446655440004".to_string(),
            title: "UUID Benefits".to_string(),
            content: "UUIDs are globally unique, don't reveal sequence information, and work great in distributed systems!".to_string(),
        },
    ]
}

#[Object]
impl Query {
    /// A simple hello world query for testing the GraphQL setup.
    ///
    /// This is useful for:
    /// - Verifying the server is running
    /// - Testing GraphQL client connections
    /// - Basic health checks
    async fn hello(&self) -> &str {
        "Hello from GraphQL with UUID support!"
    }

    /// Returns a list of sample notes for testing.
    ///
    /// This demonstrates:
    /// - GraphQL list types: [Note!]!
    /// - Complex return types with multiple fields
    /// - Field selection capabilities
    /// - Static data serving (will be dynamic in later days)
    ///
    /// The return type [Note!]! means:
    /// - Outer []: This is a list/array
    /// - Note: Each item in the list is a Note type
    /// - Inner !: Each Note in the list is non-null
    /// - Outer !: The list itself is non-null (but can be empty)
    async fn notes(&self) -> Vec<Note> {
        get_sample_notes()
    }

    /// Returns a single note by UUID, or None if not found.
    ///
    /// This demonstrates:
    /// - GraphQL arguments: note(id: String!)
    /// - UUID-based identification
    /// - Optional return types: Note vs Note!
    /// - Error handling for missing data
    ///
    /// Arguments:
    /// - id: The UUID of the note to retrieve
    ///
    /// Returns:
    /// - Some(Note) if found
    /// - None if no note exists with the given UUID
    ///
    /// GraphQL Schema:
    /// ```graphql
    /// note(id: String!): Note
    /// ```
    async fn note(&self, id: String) -> Option<Note> {
        let notes = get_sample_notes();
        notes.into_iter().find(|note| note.id == id)
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
    <title>GraphQL Test Interface - Day 3 Enhanced (UUID)</title>
    <style>
        body { font-family: monospace; padding: 20px; background: #f8f9fa; }
        .container { max-width: 900px; margin: 0 auto; }
        textarea { width: 100%; height: 300px; margin: 10px 0; font-family: monospace; font-size: 14px; }
        button { padding: 10px 20px; background: #0066cc; color: white; border: none; cursor: pointer; margin: 5px; }
        button:hover { background: #0052a3; }
        .result { background: #fff; padding: 15px; margin-top: 20px; white-space: pre-wrap; border: 1px solid #ddd; border-radius: 4px; max-height: 400px; overflow-y: auto; }
        .examples { background: #fff; padding: 15px; margin: 20px 0; border-radius: 4px; border: 1px solid #ddd; }
        .examples h4 { margin-top: 0; color: #333; }
        code { background: #f1f3f4; padding: 2px 4px; border-radius: 3px; font-size: 12px; }
        .query-btn { background: #28a745; font-size: 12px; padding: 5px 10px; }
        .error-btn { background: #dc3545; }
        .uuid-btn { background: #17a2b8; }
    </style>
</head>
<body>
    <div class="container">
        <h1>GraphQL Test Interface - Day 3 Enhanced (UUID)</h1>
        <p>Your GraphQL server now supports UUID-based unique identifiers! 🚀</p>
        
        <h3>Test Query:</h3>
        <textarea id="query" placeholder="Enter your GraphQL query...">query {
  note(id: "550e8400-e29b-41d4-a716-446655440001") {
    id
    title
    content
  }
}</textarea>
        
        <button onclick="executeQuery()">Execute Query</button>
        
        <h4>Quick Test Buttons:</h4>
        <button class="query-btn" onclick="loadQuery('hello')">Hello</button>
        <button class="query-btn" onclick="loadQuery('allNotes')">All Notes</button>
        <button class="uuid-btn" onclick="loadQuery('noteUuid1')">Note UUID #1</button>
        <button class="uuid-btn" onclick="loadQuery('noteUuid2')">Note UUID #2</button>
        <button class="error-btn" onclick="loadQuery('noteNotFound')">Invalid UUID</button>
        <button class="query-btn" onclick="loadQuery('combined')">Combined Query</button>
        
        <div class="result" id="result">Results will appear here...</div>
        
        <div class="examples">
            <h4>📋 Day 3 Enhanced Example Queries (UUID):</h4>
            <ul>
                <li><code>{ note(id: "550e8400-e29b-41d4-a716-446655440001") { title } }</code> - Get note by UUID</li>
                <li><code>{ note(id: "invalid-uuid") { id title } }</code> - Query with invalid UUID (returns null)</li>
                <li><code>{ notes { id title } }</code> - List all notes with UUID IDs</li>
                <li><code>{ hello notes { id } }</code> - Combined query with UUIDs</li>
            </ul>
            
            <h4>🎯 UUID Benefits:</h4>
            <ul>
                <li><strong>Globally Unique:</strong> No ID collisions across systems</li>
                <li><strong>Non-Sequential:</strong> Doesn't reveal creation order or count</li>
                <li><strong>Distributed Safe:</strong> Works in microservices and replicated systems</li>
                <li><strong>Future-Proof:</strong> Ready for database integration and scaling</li>
            </ul>
            
            <h4>📊 Enhanced Schema (Day 3 + UUID):</h4>
            <pre>type Query {
  hello: String!
  notes: [Note!]!
  note(id: String!): Note  # Now accepts UUID strings
}

type Note {
  id: String!     # UUID format: "550e8400-e29b-41d4-a716-446655440001"
  title: String!
  content: String!
}</pre>

            <h4>🔢 Sample UUIDs for Testing:</h4>
            <ul>
                <li><code>550e8400-e29b-41d4-a716-446655440001</code> - Welcome to GraphQL</li>
                <li><code>550e8400-e29b-41d4-a716-446655440002</code> - Learning Rust</li>
                <li><code>550e8400-e29b-41d4-a716-446655440003</code> - async-graphql Features</li>
                <li><code>550e8400-e29b-41d4-a716-446655440004</code> - UUID Benefits</li>
            </ul>
        </div>
    </div>

    <script>
        const queries = {
            hello: `query {
  hello
}`,
            allNotes: `query {
  notes {
    id
    title
    content
  }
}`,
            noteUuid1: `query {
  note(id: "550e8400-e29b-41d4-a716-446655440001") {
    id
    title
    content
  }
}`,
            noteUuid2: `query {
  note(id: "550e8400-e29b-41d4-a716-446655440002") {
    id
    title
  }
}`,
            noteNotFound: `query {
  note(id: "invalid-uuid-string") {
    id
    title
    content
  }
}`,
            combined: `query {
  hello
  note(id: "550e8400-e29b-41d4-a716-446655440001") {
    title
  }
  notes {
    id
    title
  }
}`
        };

        function loadQuery(type) {
            document.getElementById('query').value = queries[type];
        }

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
