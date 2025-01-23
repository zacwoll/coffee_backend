use axum::{routing::get, Router};

pub mod user;
mod health_check;
mod fallback;

pub fn app() -> Router {
    let app = Router::new()
        .fallback(fallback::fallback) // Falls back to 404
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check::health_check))
        .nest("/user", user::user_router());

    app
}
