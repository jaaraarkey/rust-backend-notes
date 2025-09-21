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

use async_graphql::{EmptySubscription, InputObject, Object, Schema, SimpleObject}; // Add EmptySubscription
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::Html,
    routing::{get, post},
    Router, Server,
};
use std::net::SocketAddr;
use uuid::Uuid;

/// Input type for creating a new note.
///
/// This demonstrates GraphQL Input types, which are used for complex arguments
/// in mutations and queries. Input types are different from regular types -
/// they can only be used as arguments, not return values.
#[derive(InputObject)]
pub struct CreateNoteInput {
    /// The title of the new note (required)
    pub title: String,
    /// The content/body of the new note (required)  
    pub content: String,
}

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

/// The root Mutation type for our GraphQL schema.
///
/// This contains all the "write" operations that clients can perform.
/// Each method in this impl block becomes a field in the GraphQL Mutation type.
pub struct Mutation;

#[Object]
impl Mutation {
    /// Creates a new note with auto-generated UUID.
    ///
    /// This demonstrates:
    /// - GraphQL mutations (write operations)
    /// - Input types for complex arguments
    /// - UUID generation for unique identifiers
    /// - Returning the created object
    ///
    /// Arguments:
    /// - input: CreateNoteInput containing title and content
    ///
    /// Returns:
    /// - The newly created Note with generated UUID
    ///
    /// GraphQL Schema:
    /// ```graphql
    /// createNote(input: CreateNoteInput!): Note!
    /// ```
    async fn create_note(&self, input: CreateNoteInput) -> Note {
        // Generate a new UUID for this note
        let new_id = Uuid::new_v4().to_string();

        // Create and return the new note
        Note {
            id: new_id,
            title: input.title,
            content: input.content,
        }
    }
}

/// Our complete GraphQL schema type.
///
/// This combines Query (read operations), Mutation (write operations),
/// and Subscription (real-time operations) into a single schema.
/// Now we have both Query and Mutation implemented!
type MySchema = Schema<Query, Mutation, EmptySubscription>;

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
    <title>GraphQL Test Interface - Day 4 (Mutations)</title>
    <style>
        body { font-family: monospace; padding: 20px; background: #f8f9fa; }
        .container { max-width: 900px; margin: 0 auto; }
        textarea { width: 100%; height: 350px; margin: 10px 0; font-family: monospace; font-size: 14px; }
        button { padding: 10px 20px; background: #0066cc; color: white; border: none; cursor: pointer; margin: 5px; }
        button:hover { background: #0052a3; }
        .result { background: #fff; padding: 15px; margin-top: 20px; white-space: pre-wrap; border: 1px solid #ddd; border-radius: 4px; max-height: 400px; overflow-y: auto; }
        .examples { background: #fff; padding: 15px; margin: 20px 0; border-radius: 4px; border: 1px solid #ddd; }
        .examples h4 { margin-top: 0; color: #333; }
        code { background: #f1f3f4; padding: 2px 4px; border-radius: 3px; font-size: 12px; }
        .query-btn { background: #28a745; font-size: 12px; padding: 5px 10px; }
        .mutation-btn { background: #fd7e14; }
        .uuid-btn { background: #17a2b8; }
    </style>
</head>
<body>
    <div class="container">
        <h1>GraphQL Test Interface - Day 4 (Mutations)</h1>
        <p>Your GraphQL server now supports creating notes with mutations! ✨</p>
        
        <h3>Test Query/Mutation:</h3>
        <textarea id="query" placeholder="Enter your GraphQL query or mutation...">mutation {
  createNote(input: {
    title: "My New Note"
    content: "This note was created via GraphQL mutation!"
  }) {
    id
    title
    content
  }
}</textarea>
        
        <button onclick="executeQuery()">Execute Query/Mutation</button>
        
        <h4>Quick Test Buttons:</h4>
        <button class="query-btn" onclick="loadQuery('hello')">Hello</button>
        <button class="query-btn" onclick="loadQuery('allNotes')">All Notes</button>
        <button class="uuid-btn" onclick="loadQuery('noteUuid1')">Note UUID #1</button>
        <button class="mutation-btn" onclick="loadQuery('createNote')">Create Note</button>
        <button class="mutation-btn" onclick="loadQuery('createLongNote')">Create Long Note</button>
        <button class="query-btn" onclick="loadQuery('introspection')">Schema Info</button>
        
        <div class="result" id="result">Results will appear here...</div>
        
        <div class="examples">
            <h4>📋 Day 4 Example Operations:</h4>
            
            <h5>🔍 Queries (Read):</h5>
            <ul>
                <li><code>{ notes { id title } }</code> - List all notes</li>
                <li><code>{ note(id: "uuid-here") { title content } }</code> - Get single note</li>
            </ul>
            
            <h5>✨ Mutations (Write):</h5>
            <ul>
                <li><code>mutation { createNote(input: {title: "Test", content: "Content"}) { id } }</code></li>
                <li><code>mutation { createNote(input: {title: "Long Title Here", content: "Much longer content here..."}) { id title } }</code></li>
            </ul>
            
            <h4>🎯 Day 4 GraphQL Concepts:</h4>
            <ul>
                <li><strong>Mutations:</strong> Write operations that modify data</li>
                <li><strong>Input Types:</strong> Structured arguments for complex data</li>
                <li><strong>UUID Generation:</strong> Automatic unique ID creation</li>
                <li><strong>Input Validation:</strong> Type-safe argument processing</li>
            </ul>
            
            <h4>📊 Updated Schema (Day 4):</h4>
            <pre>type Query {
  hello: String!
  notes: [Note!]!
  note(id: String!): Note
}

type Mutation {
  createNote(input: CreateNoteInput!): Note!
}

input CreateNoteInput {
  title: String!
  content: String!
}

type Note {
  id: String!
  title: String!
  content: String!
}</pre>

            <h4>💡 Mutation vs Query:</h4>
            <ul>
                <li><strong>Query:</strong> <code>query { ... }</code> - Read data, no side effects</li>
                <li><strong>Mutation:</strong> <code>mutation { ... }</code> - Write data, has side effects</li>
                <li><strong>Convention:</strong> Use mutations for create/update/delete operations</li>
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
            createNote: `mutation {
  createNote(input: {
    title: "My New Note"
    content: "This note was created via GraphQL mutation!"
  }) {
    id
    title
    content
  }
}`,
            createLongNote: `mutation {
  createNote(input: {
    title: "Learning GraphQL Mutations"
    content: "This is a longer note about GraphQL mutations. They allow us to modify data on the server. Each mutation returns the created/modified object so we can see the results immediately. UUIDs are generated automatically!"
  }) {
    id
    title
    content
  }
}`,
            introspection: `query {
  __schema {
    mutationType {
      name
      fields {
        name
        description
      }
    }
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
    // Initialize the GraphQL schema with our Query AND Mutation types
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish(); // Changed EmptyMutation to Mutation

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
    println!("✨ Day 4: Mutations now available!");

    // Start serving requests (Axum 0.6 syntax)
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
