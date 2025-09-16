use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info, Level};
use tracing_subscriber;
use uuid::Uuid;

// Application state
#[derive(Clone)]
struct AppState {
    // Add your application state here
    // For example: database connection, configuration, etc.
}

// Example data models
#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: Uuid,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

#[derive(Deserialize)]
struct UserQuery {
    limit: Option<u32>,
    offset: Option<u32>,
}

// Custom error type
#[derive(Debug)]
enum AppError {
    NotFound,
    ValidationError(String),
    InternalError(String),
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.as_str()),
        };

        (status, Json(serde_json::json!({
            "error": error_message
        }))).into_response()
    }
}

// Handler functions
async fn health_check() -> &'static str {
    "OK"
}

async fn get_users(
    Query(params): Query<UserQuery>,
) -> Result<Json<Vec<User>>, AppError> {
    // Example implementation - replace with actual database logic
    let users = vec![
        User {
            id: Uuid::new_v4(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        },
        User {
            id: Uuid::new_v4(),
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
        },
    ];

    let limit = params.limit.unwrap_or(10);
    let offset = params.offset.unwrap_or(0);
    
    let paginated_users: Vec<User> = users
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();

    Ok(Json(paginated_users))
}

async fn get_user(Path(id): Path<Uuid>) -> Result<Json<User>, AppError> {
    // Example implementation - replace with actual database logic
    if id == Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap() {
        return Err(AppError::NotFound);
    }

    let user = User {
        id,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };

    Ok(Json(user))
}

async fn create_user(Json(payload): Json<CreateUser>) -> Result<Json<User>, AppError> {
    // Validate input
    if payload.name.is_empty() {
        return Err(AppError::ValidationError("Name cannot be empty".to_string()));
    }
    
    if !payload.email.contains('@') {
        return Err(AppError::ValidationError("Invalid email format".to_string()));
    }

    let user = User {
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email,
    };

    // Here you would typically save to database
    info!("Created user: {}", user.id);

    Ok(Json(user))
}

async fn update_user(
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    // Example implementation - replace with actual database logic
    if id == Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap() {
        return Err(AppError::NotFound);
    }

    let mut user = User {
        id,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };

    if let Some(name) = payload.name {
        user.name = name;
    }
    
    if let Some(email) = payload.email {
        if !email.contains('@') {
            return Err(AppError::ValidationError("Invalid email format".to_string()));
        }
        user.email = email;
    }

    info!("Updated user: {}", user.id);

    Ok(Json(user))
}

async fn delete_user(Path(id): Path<Uuid>) -> Result<StatusCode, AppError> {
    // Example implementation - replace with actual database logic
    if id == Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap() {
        return Err(AppError::NotFound);
    }

    info!("Deleted user: {}", id);
    Ok(StatusCode::NO_CONTENT)
}

// API routes
fn create_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/users", get(get_users).post(create_user))
        .route("/api/users/:id", get(get_user).put(update_user).delete(delete_user))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Create application state
    let state = AppState {};

    // Build our application with middleware
    let app = create_router()
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                ),
        );

    // Run the server
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");
    
    info!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
