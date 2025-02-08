// Import the other routers
pub mod user;
mod health_check;
mod fallback;

// Needed for Axum
use axum::{routing::get, Router};

// Needed for Tracing
use axum::{
    body::{Bytes, Body},
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::{Html, Response},
};
use uuid::Uuid;
use std::time::Duration;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


use super::AppState;


pub fn app() -> Router<AppState> {
    let app = Router::new()
        .fallback(fallback::fallback) // Falls back to 404
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check::health_check))
        // User Router
        .nest("/user", user::user_router())
        // Tracing Middleware Layer
        .layer(TraceLayer::new_for_http()
            .make_span_with(|request: &Request<Body>| {
                tracing::debug_span!(
                    "http-request",
                    request_id = tracing::field::Empty
                )
            })
            .on_request(|request: &Request<Body>, span: &Span| {
                let request_id = Uuid::new_v4();
                span.record("request_id", request_id.to_string());
                tracing::debug!("started {} {}", request.method(), request.uri().path())
            })
            .on_response(|response: &Response<Body>, latency: Duration, _span: &Span| {
                tracing::debug!("response generated in {:?}", latency)
            })
            .on_body_chunk(|chunk: &Bytes, latency: Duration, _span: &Span| {
                tracing::debug!("sending {} bytes", chunk.len())
            })
            .on_eos(|trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
                tracing::debug!("stream closed after {:?}", stream_duration)
            })
            .on_failure(|error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                tracing::debug!("something went wrong")
            })
        );

    app
}
