//! # GraphQL Web Handlers with JWT Support
//!
//! Pure Axum implementation with JWT authentication

use async_graphql::{http::GraphiQLSource, Schema, Variables};
use axum::{
    extract::{FromRequest, Json, Request, State}, // ✅ Add FromRequest import
    http::StatusCode,
    response::{Html, IntoResponse, Json as JsonResponse},
};
use serde::{Deserialize, Serialize};

use crate::auth::AuthContext;
use crate::resolvers::{MutationRoot, QueryRoot, SubscriptionRoot};

pub type AppSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

#[derive(Deserialize)]
pub struct GraphQLRequest {
    pub query: String,
    pub variables: Option<serde_json::Value>,
    pub operation_name: Option<String>,
}

#[derive(Serialize)]
pub struct GraphQLResponse {
    pub data: Option<serde_json::Value>,
    pub errors: Option<Vec<serde_json::Value>>,
}

/// Convert serde_json::Value to async_graphql::Variables
fn convert_variables(value: serde_json::Value) -> Result<Variables, serde_json::Error> {
    Ok(Variables::from_json(value))
}

/// Convert async_graphql::Value to serde_json::Value
fn convert_value(value: async_graphql::Value) -> serde_json::Value {
    serde_json::to_value(value).unwrap_or_default()
}

/// 🔐 GraphQL handler with JWT authentication
pub async fn graphql_handler(
    State(schema): State<AppSchema>,
    request: Request,
) -> impl IntoResponse {
    // Extract auth context from middleware
    let auth_context = request
        .extensions()
        .get::<AuthContext>()
        .cloned()
        .unwrap_or_else(AuthContext::unauthenticated);

    // ✅ Fix: Extract JSON body properly with Axum 0.7
    let (parts, body) = request.into_parts();
    let request_with_body = Request::from_parts(parts, body);

    let Json(graphql_request): Json<GraphQLRequest> =
        match Json::from_request(request_with_body, &()).await {
            Ok(json) => json,
            Err(e) => {
                let error_response = GraphQLResponse {
                    data: None,
                    errors: Some(vec![
                        serde_json::json!({"message": format!("Invalid JSON: {}", e)}),
                    ]),
                };
                return (StatusCode::BAD_REQUEST, JsonResponse(error_response));
            }
        };

    let mut req = async_graphql::Request::new(graphql_request.query);

    // Convert variables if present
    if let Some(variables_json) = graphql_request.variables {
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

    if let Some(operation_name) = graphql_request.operation_name {
        req = req.operation_name(operation_name);
    }

    // 🔐 Add auth context to GraphQL request
    req = req.data(auth_context);

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
            .title("🚀 Smart Notes GraphQL API with JWT Auth")
            .finish(),
    )
}

/// Beautiful Landing Page with JWT Authentication Info
pub async fn landing_page() -> impl IntoResponse {
    Html(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>🚀 Smart Notes GraphQL API with JWT</title>
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
        .stats { display: flex; justify-content: space-around; margin: 30px 0; text-align: center; }
        .stat { color: #2c3e50; }
        .stat-number { font-size: 2em; font-weight: bold; color: #667eea; }
        .stat-label { font-size: 0.9em; color: #7f8c8d; }
        .auth-example { background: #2c3e50; color: #ecf0f1; padding: 15px; border-radius: 10px; font-family: 'Monaco', monospace; font-size: 0.9em; margin: 15px 0; }
        @media (max-width: 768px) { 
            .grid { grid-template-columns: 1fr; } 
            .cta-buttons { flex-direction: column; align-items: center; }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="card">
            <h1>🚀 Smart Notes GraphQL API</h1>
            <p class="subtitle">Production-Ready JWT Authentication + PostgreSQL + AI Features</p>
            
            <div class="cta-buttons">
                <a href="/graphiql" class="btn btn-primary">
                    🎮 Launch Interactive GraphiQL
                </a>
                <a href="/graphql" class="btn btn-secondary">
                    📡 GraphQL Endpoint
                </a>
            </div>
            
            <div class="highlight">
                <h3>🔐 JWT Authentication is LIVE!</h3>
                <div class="stats">
                    <div class="stat">
                        <div class="stat-number">24h</div>
                        <div class="stat-label">Token Validity</div>
                    </div>
                    <div class="stat">
                        <div class="stat-number">bcrypt</div>
                        <div class="stat-label">Password Hashing</div>
                    </div>
                    <div class="stat">
                        <div class="stat-number">JWT</div>
                        <div class="stat-label">Secure Tokens</div>
                    </div>
                </div>
                <div>
                    <div class="feature">User-Specific Notes</div>
                    <div class="feature">Protected Routes</div>
                    <div class="feature">JWT Middleware</div>
                    <div class="feature">Auth Context</div>
                    <div class="feature">Password Security</div>
                </div>
            </div>
        </div>

        <div class="card">
            <div class="grid">
                <div class="grid-item">
                    <h3>🔐 Authentication Flow</h3>
                    <p><strong>1. Register:</strong> Create account with email/password</p>
                    <p><strong>2. Login:</strong> Get JWT token (24h validity)</p>
                    <p><strong>3. Authenticated Requests:</strong> Include Bearer token</p>
                    <p><strong>4. Protected Operations:</strong> User-specific data access</p>
                </div>
                <div class="grid-item">
                    <h3>🎯 Example: Register User</h3>
                    <div class="auth-example">mutation {
  register(input: {
    email: "genius@notes.com"
    password: "supersecure123"
    fullName: "Smart Developer"
  }) {
    token
    user { id email }
  }
}</div>
                </div>
            </div>
        </div>

        <div class="card">
            <div class="grid">
                <div class="grid-item">
                    <h3>🔑 Login & Get Token</h3>
                    <div class="auth-example">mutation {
  login(input: {
    email: "genius@notes.com"
    password: "supersecure123"
  }) {
    token
    user { id email }
  }
}</div>
                </div>
                <div class="grid-item">
                    <h3>📝 Create Authenticated Note</h3>
                    <div class="auth-example">// Headers:
Authorization: Bearer YOUR_JWT_TOKEN

mutation {
  createNote(input: {
    content: "My secure note!"
  }) {
    id title content
  }
}</div>
                </div>
            </div>
        </div>
        
        <div class="card">
            <p style="text-align: center; color: #7f8c8d;">
                <strong>🎸🔥💙 Your JWT Authentication System is LIVE!</strong><br>
                Built with Rust, PostgreSQL, GraphQL, JWT middleware, and pure genius!
            </p>
        </div>
    </div>
</body>
</html>
"#,
    )
}
