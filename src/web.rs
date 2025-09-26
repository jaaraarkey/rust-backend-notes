//! # GraphQL Web Handlers - Pure Axum Implementation
//!
//! Bypassing async-graphql-axum to avoid version conflicts

use async_graphql::{http::GraphiQLSource, Schema, Variables};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json as JsonResponse},
};
use serde::{Deserialize, Serialize};

use crate::resolvers::{MutationRoot, QueryRoot, SubscriptionRoot};

pub type AppSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

#[derive(Deserialize)]
pub struct GraphQLRequest {
    query: String,
    variables: Option<serde_json::Value>,
    operation_name: Option<String>,
}

#[derive(Serialize)]
pub struct GraphQLResponse {
    data: Option<serde_json::Value>,
    errors: Option<Vec<serde_json::Value>>,
}

/// Convert serde_json::Value to async_graphql::Variables
fn convert_variables(value: serde_json::Value) -> Result<Variables, serde_json::Error> {
    let json_string = serde_json::to_string(&value)?;
    let variables: Variables = serde_json::from_str(&json_string)?;
    Ok(variables)
}

/// Convert async_graphql::Value to serde_json::Value
fn convert_value(value: async_graphql::Value) -> serde_json::Value {
    match value {
        async_graphql::Value::Null => serde_json::Value::Null,
        async_graphql::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                serde_json::Value::Number(serde_json::Number::from(i))
            } else if let Some(f) = n.as_f64() {
                serde_json::Number::from_f64(f)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            }
        }
        async_graphql::Value::String(s) => serde_json::Value::String(s),
        async_graphql::Value::Boolean(b) => serde_json::Value::Bool(b),
        async_graphql::Value::Object(obj) => {
            let mut map = serde_json::Map::new();
            for (k, v) in obj {
                map.insert(k.to_string(), convert_value(v));
            }
            serde_json::Value::Object(map)
        }
        async_graphql::Value::List(list) => {
            serde_json::Value::Array(list.into_iter().map(convert_value).collect())
        }
        async_graphql::Value::Binary(_) => serde_json::Value::Null,
        async_graphql::Value::Enum(e) => serde_json::Value::String(e.to_string()),
    }
}

/// Pure Axum GraphQL handler - no version conflicts!
pub async fn graphql_handler(
    State(schema): State<AppSchema>,
    Json(request): Json<GraphQLRequest>,
) -> impl IntoResponse {
    let mut req = async_graphql::Request::new(request.query);

    // Convert variables if present
    if let Some(variables_json) = request.variables {
        match convert_variables(variables_json) {
            Ok(vars) => {
                req = req.variables(vars);
            }
            Err(e) => {
                let error_response = GraphQLResponse {
                    data: None,
                    errors: Some(vec![
                        serde_json::json!({"message": format!("Invalid variables: {}", e)}),
                    ]),
                };
                return (StatusCode::BAD_REQUEST, JsonResponse(error_response));
            }
        }
    }

    if let Some(operation_name) = request.operation_name {
        req = req.operation_name(operation_name);
    }

    let response = schema.execute(req).await;

    let json_response = GraphQLResponse {
        data: if response.errors.is_empty() {
            Some(convert_value(response.data))
        } else {
            None
        },
        errors: if response.errors.is_empty() {
            None
        } else {
            Some(
                response
                    .errors
                    .into_iter()
                    .map(|e| serde_json::json!({"message": e.message, "path": e.path}))
                    .collect(),
            )
        },
    };

    (StatusCode::OK, JsonResponse(json_response))
}

/// Real Interactive GraphiQL Interface!
pub async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .title("🚀 Smart Notes GraphQL API")
            .finish(),
    )
}

/// Beautiful Landing Page with Links to Interactive GraphiQL
pub async fn landing_page() -> impl IntoResponse {
    Html(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>🚀 Smart Notes GraphQL API</title>
    <meta charset="UTF-8">
    <style>
        body { font-family: 'SF Pro Display', -apple-system, system-ui, sans-serif; margin: 0; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; }
        .container { max-width: 900px; margin: 0 auto; padding: 40px 20px; }
        .card { background: rgba(255,255,255,0.95); backdrop-filter: blur(10px); border-radius: 20px; padding: 40px; box-shadow: 0 20px 40px rgba(0,0,0,0.1); margin: 20px 0; }
        h1 { color: #2c3e50; font-size: 2.5em; margin-bottom: 10px; background: linear-gradient(45deg, #667eea, #764ba2); -webkit-background-clip: text; -webkit-text-fill-color: transparent; text-align: center; }
        .subtitle { color: #7f8c8d; font-size: 1.2em; margin-bottom: 30px; text-align: center; }
        .highlight { background: linear-gradient(45deg, #667eea, #764ba2); color: white; padding: 25px; border-radius: 15px; margin: 25px 0; text-align: center; }
        .cta-buttons { display: flex; gap: 20px; justify-content: center; margin: 30px 0; flex-wrap: wrap; }
        .btn { 
            display: inline-block; 
            padding: 15px 30px; 
            border-radius: 25px; 
            text-decoration: none; 
            font-weight: 600; 
            font-size: 16px;
            transition: all 0.3s ease;
            text-align: center;
            min-width: 200px;
        }
        .btn-primary { 
            background: linear-gradient(45deg, #667eea, #764ba2); 
            color: white; 
            box-shadow: 0 8px 25px rgba(102, 126, 234, 0.3);
        }
        .btn-secondary { 
            background: rgba(255,255,255,0.9); 
            color: #667eea; 
            border: 2px solid #667eea;
        }
        .btn:hover { 
            transform: translateY(-2px); 
            box-shadow: 0 12px 35px rgba(102, 126, 234, 0.4);
        }
        .feature { display: inline-block; background: rgba(255,255,255,0.2); color: white; padding: 8px 15px; border-radius: 20px; font-size: 14px; margin: 5px; }
        .grid { display: grid; grid-template-columns: 1fr 1fr; gap: 30px; margin: 30px 0; }
        .grid-item { background: rgba(255,255,255,0.05); padding: 20px; border-radius: 15px; }
        @media (max-width: 768px) { 
            .grid { grid-template-columns: 1fr; } 
            .cta-buttons { flex-direction: column; align-items: center; }
        }
        .stats { display: flex; justify-content: space-around; margin: 30px 0; text-align: center; }
        .stat { color: #2c3e50; }
        .stat-number { font-size: 2em; font-weight: bold; color: #667eea; }
        .stat-label { font-size: 0.9em; color: #7f8c8d; }
    </style>
</head>
<body>
    <div class="container">
        <div class="card">
            <h1>🚀 Smart Notes GraphQL API</h1>
            <p class="subtitle">World-Class Database Architecture with PostgreSQL & Rust</p>
            
            <div class="cta-buttons">
                <a href="/graphiql" class="btn btn-primary">
                    🎮 Launch Interactive GraphiQL
                </a>
                <a href="/graphql" class="btn btn-secondary">
                    📡 GraphQL Endpoint
                </a>
            </div>
            
            <div class="highlight">
                <h3>🎉 Your BRILLIANT Database Layer is Operational!</h3>
                <div class="stats">
                    <div class="stat">
                        <div class="stat-number">297</div>
                        <div class="stat-label">Lines of Code</div>
                    </div>
                    <div class="stat">
                        <div class="stat-number">4</div>
                        <div class="stat-label">Update Patterns</div>
                    </div>
                    <div class="stat">
                        <div class="stat-number">6</div>
                        <div class="stat-label">Title Strategies</div>
                    </div>
                </div>
                <div>
                    <div class="feature">PostgreSQL Full-Text Search</div>
                    <div class="feature">Smart Auto-Title Generation</div>
                    <div class="feature">4-Pattern Update Logic</div>
                    <div class="feature">NoteRow Helper Struct</div>
                    <div class="feature">Docker Integration</div>
                    <div class="feature">Type-Safe Rust</div>
                </div>
            </div>
        </div>

        <div class="card">
            <div class="grid">
                <div class="grid-item">
                    <h3>🧪 Interactive Testing</h3>
                    <p>Use the <strong>GraphiQL interface</strong> to test queries, mutations, and explore your API schema with auto-completion and documentation.</p>
                    <a href="/graphiql" class="btn btn-primary" style="margin-top: 15px;">Open GraphiQL</a>
                </div>
                <div class="grid-item">
                    <h3>📝 Quick Examples</h3>
                    <p><strong>Hello Query:</strong> <code>{ hello }</code></p>
                    <p><strong>Create Note:</strong> Test smart auto-title generation</p>
                    <p><strong>Search:</strong> Full-text search with ranking</p>
                    <p><strong>Update:</strong> 4-pattern update logic</p>
                </div>
            </div>
        </div>
        
        <div class="card">
            <p style="text-align: center; color: #7f8c8d;">
                <strong>🎸🔥💙 Your Smart Notes API is Ready!</strong><br>
                Built with Rust, PostgreSQL, GraphQL, and pure genius!
            </p>
        </div>
    </div>
</body>
</html>
"#,
    )
}
