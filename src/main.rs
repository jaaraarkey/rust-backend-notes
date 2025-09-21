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

fn get_sample_notes() -> Vec<Note> {
    vec![
        Note {
            id: 1,
            title: "Welcome to GraphQL".to_string(),
            content: "This is your first note! GraphQL allows you to query exactly the fields you need.".to_string(),
        },
        Note {
            id: 2, 
            title: "Learning Rust".to_string(),
            content: "Rust's type system helps catch errors at compile time, making GraphQL APIs more reliable.".to_string(),
        },
        Note {
            id: 3,
            title: "async-graphql Features".to_string(), 
            content: "The async-graphql crate provides powerful features like field selection, introspection, and automatic schema generation.".to_string(),
        },
        Note {
            id: 4,
            title: "Field Selection".to_string(),
            content: "With GraphQL, clients can request only the fields they need: id, title, content, or any combination!".to_string(),
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
        "Hello from GraphQL!"
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

    /// Returns a single note by ID, or None if not found.
    ///
    /// This demonstrates:
    /// - GraphQL arguments: note(id: Int!)
    /// - Optional return types: Note vs Note!
    /// - Error handling for missing data
    /// - Input validation
    ///
    /// Arguments:
    /// - id: The unique identifier of the note to retrieve
    ///
    /// Returns:
    /// - Some(Note) if found
    /// - None if no note exists with the given ID
    ///
    /// GraphQL Schema:
    /// ```graphql
    /// note(id: Int!): Note
    /// ```
    ///
    /// The return type `Note` (without !) means:
    /// - The field can return null if no note is found
    /// - This is different from `Note!` which would require a note to always exist
    async fn note(&self, id: i32) -> Option<Note> {
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
    <title>GraphQL Test Interface - Day 3</title>
    <style>
        body { font-family: monospace; padding: 20px; background: #f8f9fa; }
        .container { max-width: 900px; margin: 0 auto; }
        textarea { width: 100%; height: 300px; margin: 10px 0; font-family: monospace; font-size: 14px; }
        button { padding: 10px 20px; background: #0066cc; color: white; border: none; cursor: pointer; margin: 5px; }
        button:hover { background: #0052a3; }
        .result { background: #fff; padding: 15px; margin-top: 20px; white-space: pre-wrap; border: 1px solid #ddd; border-radius: 4px; max-height: 400px; overflow-y: auto; }
        .examples { background: #fff; padding: 15px; margin: 20px 0; border-radius: 4px; border: 1px solid #ddd; }
        .examples h4 { margin-top: 0; color: #333; }
        code { background: #f1f3f4; padding: 2px 4px; border-radius: 3px; }
        .query-btn { background: #28a745; font-size: 12px; padding: 5px 10px; }
        .error-btn { background: #dc3545; }
        .combined-btn { background: #6f42c1; }
    </style>
</head>
<body>
    <div class="container">
        <h1>GraphQL Test Interface - Day 3</h1>
        <p>Your GraphQL server now supports single note queries with error handling! 🚀</p>
        
        <h3>Test Query:</h3>
        <textarea id="query" placeholder="Enter your GraphQL query...">query {
  note(id: 1) {
    id
    title
    content
  }
}</textarea>
        
        <button onclick="executeQuery()">Execute Query</button>
        
        <h4>Quick Test Buttons:</h4>
        <button class="query-btn" onclick="loadQuery('hello')">Hello</button>
        <button class="query-btn" onclick="loadQuery('allNotes')">All Notes</button>
        <button class="query-btn" onclick="loadQuery('noteExists')">Note #1</button>
        <button class="query-btn" onclick="loadQuery('noteExists2')">Note #3</button>
        <button class="error-btn" onclick="loadQuery('noteNotFound')">Note #999 (Error)</button>
        <button class="combined-btn" onclick="loadQuery('combined')">Combined Query</button>
        <button class="query-btn" onclick="loadQuery('fieldSelection')">Field Selection</button>
        
        <div class="result" id="result">Results will appear here...</div>
        
        <div class="examples">
            <h4>📋 Day 3 Example Queries:</h4>
            <ul>
                <li><code>{ note(id: 1) { id title } }</code> - Get note #1 with selected fields</li>
                <li><code>{ note(id: 999) { id title } }</code> - Query non-existent note (returns null)</li>
                <li><code>{ note(id: 2) { title content } }</code> - Get note #2 without ID field</li>
                <li><code>{ notes { id title } note(id: 1) { content } }</code> - Combined list + single query</li>
            </ul>
            
            <h4>🎯 Day 3 GraphQL Concepts:</h4>
            <ul>
                <li><strong>Arguments:</strong> <code>note(id: Int!)</code> requires an ID parameter</li>
                <li><strong>Optional Types:</strong> <code>Note</code> can return null, <code>Note!</code> cannot</li>
                <li><strong>Error Handling:</strong> Graceful handling of missing data</li>
                <li><strong>Input Validation:</strong> Type-safe argument handling</li>
            </ul>
            
            <h4>📊 Current Schema (Day 3):</h4>
            <pre>type Query {
  hello: String!
  notes: [Note!]!
  note(id: Int!): Note  # Note: can return null
}

type Note {
  id: Int!
  title: String!
  content: String!
}</pre>
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
            noteExists: `query {
  note(id: 1) {
    id
    title
    content
  }
}`,
            noteExists2: `query {
  note(id: 3) {
    id
    title
  }
}`,
            noteNotFound: `query {
  note(id: 999) {
    id
    title
    content
  }
}`,
            combined: `query {
  hello
  note(id: 1) {
    title
  }
  notes {
    id
    title
  }
}`,
            fieldSelection: `query {
  note(id: 2) {
    title
    content
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
