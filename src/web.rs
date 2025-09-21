//! Web server and HTTP handling.

use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::Extension, response::Html};

use crate::resolvers::{Mutation, Query};

/// Our complete GraphQL schema type.
pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Handles incoming GraphQL requests.
///
/// This function:
/// 1. Extracts the GraphQL schema from Axum's Extension layer
/// 2. Parses the incoming GraphQL request
/// 3. Executes the query against our schema
/// 4. Returns the result as a GraphQL response
pub async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
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
pub async fn graphiql() -> Html<&'static str> {
    Html(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>GraphQL Test Interface - Day 4 (Clean Architecture)</title>
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
        .arch-note { background: #e7f3ff; padding: 10px; border-left: 4px solid #007acc; margin: 10px 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>GraphQL Test Interface - Day 4 (Clean Architecture)</h1>
        <p>Your GraphQL server now has clean, modular architecture! 🏗️✨</p>
        
        <div class="arch-note">
            <strong>🏗️ Architecture Improvement:</strong> Code is now organized into separate modules:
            <code>types.rs</code>, <code>resolvers.rs</code>, <code>data.rs</code>, <code>web.rs</code>
        </div>
        
        <h3>Test Query/Mutation:</h3>
        <textarea id="query" placeholder="Enter your GraphQL query or mutation...">mutation {
  createNote(input: {
    title: "Clean Architecture Note"
    content: "This note was created with our newly refactored, modular codebase!"
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
        <button class="mutation-btn" onclick="loadQuery('createCleanNote')">Create Clean Arch Note</button>
        <button class="query-btn" onclick="loadQuery('introspection')">Schema Info</button>
        
        <div class="result" id="result">Results will appear here...</div>
        
        <div class="examples">
            <h4>📋 Day 4 Enhanced: Clean Architecture</h4>
            
            <h5>🏗️ Module Structure:</h5>
            <ul>
                <li><strong>types.rs:</strong> GraphQL types and input definitions</li>
                <li><strong>resolvers.rs:</strong> Query and Mutation implementations</li>
                <li><strong>data.rs:</strong> Data access and sample data</li>
                <li><strong>web.rs:</strong> HTTP handlers and server setup</li>
                <li><strong>main.rs:</strong> Application entry point (now clean!)</li>
            </ul>
            
            <h5>✨ Mutations (Write):</h5>
            <ul>
                <li><code>mutation { createNote(input: {title: "Test", content: "Content"}) { id } }</code></li>
                <li><code>mutation { createNote(input: {title: "Clean Code", content: "Modular architecture rocks!"}) { id title } }</code></li>
            </ul>
            
            <h4>🎯 Benefits of Clean Architecture:</h4>
            <ul>
                <li><strong>Maintainable:</strong> Each module has a single responsibility</li>
                <li><strong>Testable:</strong> Easy to test individual components</li>
                <li><strong>Scalable:</strong> Easy to add new features without mess</li>
                <li><strong>Readable:</strong> Clear separation of concerns</li>
            </ul>
            
            <h4>📊 Same Great Schema:</h4>
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
            createCleanNote: `mutation {
  createNote(input: {
    title: "Clean Architecture Success"
    content: "Our refactored codebase is much more maintainable! Each module has a clear purpose: types for definitions, resolvers for business logic, data for persistence, and web for HTTP handling."
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
