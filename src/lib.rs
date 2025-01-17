mod user;

use axum::{routing::get, Router};
use std::env;

/// Health Check handler for app
pub async fn health_check() -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::OK,
        format!("Healthy."),
    )
}

/// Utility function for app
/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

pub async fn run() -> std::io::Result<()> {
	// build our application with a single route
    let app = Router::new()
        .fallback(fallback) // Falls back to 404
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .nest("/user", user::user_router());

    // Get the host and the port from the env
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    println!("Server is listening at {}:{}", host, port);
    axum::serve(listener, app).await.unwrap();

	Ok(())
}