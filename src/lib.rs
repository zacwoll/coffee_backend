mod user;

use axum::{routing::get, Router};

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

pub fn app() -> Router {
    let app = Router::new()
        .fallback(fallback) // Falls back to 404
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .nest("/user", user::user_router());

    app
}

use std::env;
use axum::serve::Serve;

pub async fn run(listener: tokio::net::TcpListener) -> Result<Serve<tokio::net::TcpListener, Router, Router>, std::io::Error>  {
    let app = app();
    let serve = axum::serve(listener, app);

    Ok(serve)
}

pub async fn get_listener() -> Result<tokio::net::TcpListener, std::io::Error> {
            // Get the host and the port from the env
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let addr = format!("{}:{}", host, port);
    let socket_addr: std::net::SocketAddr = addr.parse().expect("Unable to parse socket address");

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .unwrap();

    Ok(listener)
}