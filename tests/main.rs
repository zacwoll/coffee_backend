use coffee_backend::{self, AppState};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt; // for `collect`
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`
use std::error::Error;

#[path = "common.rs"]
mod common;

#[tokio::test]
async fn hello_world() -> Result<(), Box<dyn Error>> {
    let test_pool = common::get_test_db().await.unwrap();

	let state = AppState { pool: test_pool };

	let app = coffee_backend::app().with_state(state);


    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await?.to_bytes();
    assert_eq!(&body[..], b"Hello, World!");

    Ok(())
}

