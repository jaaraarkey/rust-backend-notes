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
    <title>GraphQL Test Interface - Day 5 (Complete CRUD)</title>
    <style>
        body { font-family: monospace; padding: 20px; background: #f8f9fa; }
        .container { max-width: 900px; margin: 0 auto; }
        textarea { width: 100%; height: 350px; margin: 10px 0; font-family: monospace; font-size: 14px; }
        button { padding: 10px 20px; background: #0066cc; color: white; border: none; cursor: pointer; margin: 5px; border-radius: 4px; }
        button:hover { background: #0052a3; }
        .result { background: #fff; padding: 15px; margin-top: 20px; white-space: pre-wrap; border: 1px solid #ddd; border-radius: 4px; max-height: 400px; overflow-y: auto; font-family: monospace; }
        .examples { background: #fff; padding: 15px; margin: 20px 0; border-radius: 4px; border: 1px solid #ddd; }
        .examples h4 { margin-top: 0; color: #333; }
        code { background: #f1f3f4; padding: 2px 4px; border-radius: 3px; font-size: 12px; }
        .query-btn { background: #28a745; font-size: 12px; padding: 8px 12px; }
        .mutation-btn { background: #fd7e14; font-size: 12px; padding: 8px 12px; }
        .update-btn { background: #17a2b8; font-size: 12px; padding: 8px 12px; }
        .delete-btn { background: #dc3545; font-size: 12px; padding: 8px 12px; }
        .crud-note { background: #e8f5e8; padding: 10px; border-left: 4px solid #28a745; margin: 10px 0; }
        .error { background: #f8d7da; color: #721c24; padding: 10px; border-radius: 4px; margin-top: 10px; }
        .success { background: #d4edda; color: #155724; padding: 10px; border-radius: 4px; margin-top: 10px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>GraphQL Test Interface - Day 5 (Complete CRUD)</h1>
        <p>Your GraphQL server now supports full CRUD operations! 🎯✨</p>
        
        <div class="crud-note">
            <strong>🎯 Day 5 Complete CRUD:</strong> Create, Read, Update, Delete operations all working!
            <br><strong>Fixed:</strong> JavaScript interaction now working properly!
        </div>
        
        <h3>Test Query/Mutation:</h3>
        <textarea id="query" placeholder="Enter your GraphQL query or mutation...">mutation {
  createNote(input: {
    title: "Test Day 5 CRUD"
    content: "Testing our complete CRUD operations!"
  }) {
    id
    title
    content
  }
}</textarea>
        
        <button onclick="executeQuery()" style="background: #007acc; padding: 12px 24px; font-size: 14px;">Execute Query/Mutation</button>
        <button onclick="clearResult()" style="background: #6c757d; padding: 8px 16px;">Clear Result</button>
        
        <h4>🧪 CRUD Test Buttons:</h4>
        <button class="query-btn" onclick="loadQuery('hello')">👋 Hello</button>
        <button class="query-btn" onclick="loadQuery('allNotes')">📖 Read All</button>
        <button class="query-btn" onclick="loadQuery('singleNote')">📖 Read One</button>
        <button class="mutation-btn" onclick="loadQuery('createNote')">➕ Create</button>
        <button class="update-btn" onclick="loadQuery('updateTitle')">✏️ Update Title</button>
        <button class="update-btn" onclick="loadQuery('updateBoth')">✏️ Update Both</button>
        <button class="delete-btn" onclick="loadQuery('deleteNote')">❌ Delete</button>
        <button class="query-btn" onclick="loadQuery('introspection')">🔍 Schema</button>
        
        <div class="result" id="result">🚀 Ready to test! Click a button or execute your query above...</div>
        
        <div class="examples">
            <h4>📋 Day 5: Complete CRUD Operations</h4>
            
            <h5>📖 READ Operations:</h5>
            <ul>
                <li><code>{ notes { id title } }</code> - List all notes</li>
                <li><code>{ note(id: "uuid") { title content } }</code> - Get single note</li>
            </ul>
            
            <h5>➕ CREATE Operations:</h5>
            <ul>
                <li><code>mutation { createNote(input: {title: "New", content: "Content"}) { id } }</code></li>
            </ul>
            
            <h5>✏️ UPDATE Operations:</h5>
            <ul>
                <li><code>mutation { updateNote(id: "uuid", input: {title: "New Title"}) { id title } }</code></li>
                <li><code>mutation { updateNote(id: "uuid", input: {content: "New Content"}) { content } }</code></li>
                <li><code>mutation { updateNote(id: "uuid", input: {title: "Both", content: "Updated"}) { id } }</code></li>
            </ul>
            
            <h5>❌ DELETE Operations:</h5>
            <ul>
                <li><code>mutation { deleteNote(id: "uuid") }</code> - Returns true/false</li>
            </ul>
            
            <h4>📊 Complete Schema (Day 5):</h4>
            <pre>type Query {
  hello: String!
  notes: [Note!]!
  note(id: String!): Note
}

type Mutation {
  createNote(input: CreateNoteInput!): Note!
  updateNote(id: String!, input: UpdateNoteInput!): Note
  deleteNote(id: String!): Boolean!
}

input CreateNoteInput {
  title: String!
  content: String!
}

input UpdateNoteInput {
  title: String      # Optional for partial updates
  content: String    # Optional for partial updates
}

type Note {
  id: String!
  title: String!
  content: String!
}</pre>

            <h4>🔄 Complete CRUD Workflow:</h4>
            <ol>
                <li><strong>Create</strong> a new note → Get ID back</li>
                <li><strong>Read</strong> the note → Verify it exists</li>
                <li><strong>Update</strong> the note → Modify title/content</li>
                <li><strong>Delete</strong> the note → Remove from system</li>
            </ol>
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
            singleNote: `query {
  note(id: "550e8400-e29b-41d4-a716-446655440001") {
    id
    title
    content
  }
}`,
            createNote: `mutation {
  createNote(input: {
    title: "Day 5 CRUD Note"
    content: "This note demonstrates our complete CRUD operations!"
  }) {
    id
    title
    content
  }
}`,
            updateTitle: `mutation {
  updateNote(id: "550e8400-e29b-41d4-a716-446655440001", input: {
    title: "Updated Title Only"
  }) {
    id
    title
    content
  }
}`,
            updateBoth: `mutation {
  updateNote(id: "550e8400-e29b-41d4-a716-446655440002", input: {
    title: "Both Fields Updated"
    content: "Both the title and content have been updated in this mutation!"
  }) {
    id
    title
    content
  }
}`,
            deleteNote: `mutation {
  deleteNote(id: "550e8400-e29b-41d4-a716-446655440003")
}`,
            crudWorkflow: `# Complete CRUD workflow demonstration
# 1. CREATE a new note
mutation CreateStep {
  createNote(input: {
    title: "CRUD Workflow Demo"
    content: "This note will go through the full CRUD cycle!"
  }) {
    id
    title
    content
  }
}

# 2. READ the note (use ID from step 1)
# query ReadStep {
#   note(id: "generated-uuid-here") {
#     id
#     title
#     content
#   }
# }

# 3. UPDATE the note (use ID from step 1)
# mutation UpdateStep {
#   updateNote(id: "generated-uuid-here", input: {
#     title: "Updated CRUD Demo"
#     content: "This content was updated!"
#   }) {
#     id
#     title
#     content
#   }
# }

# 4. DELETE the note (use ID from step 1)
# mutation DeleteStep {
#   deleteNote(id: "generated-uuid-here")
# }`
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
                        'Content-Type': 'application/json'  //! Properly quoted string
                    },
                    body: JSON.stringify({ query })
                });
                
                const result = await response.json();
                resultDiv.textContent = JSON.stringify(result, null, 2);
            } catch (error) {
                resultDiv.textContent = 'Error: ' + error.message;
            }
        }

        function clearResult() {
            document.getElementById('result').textContent = '🚀 Ready to test! Click a button or execute your query above...';
        }
    </script>
</body>
</html>
    "#,
    )
}
