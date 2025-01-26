use coffee_backend::{self, AppState};
use axum::{
	body::Body,
	http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt; // for 'call', 'oneshot', and 'ready'
use pretty_assertions::assert_eq;

#[path = "common.rs"]
mod common;

#[tokio::test]
async fn health_check() {

	let test_pool = common::get_test_db().await.unwrap();

	let state = AppState { pool: test_pool };

	let app = coffee_backend::app().with_state(state);

	// Using the Request builder, we call the router with the built request
	let response = app
		.oneshot(Request::builder().uri("/health_check").body(Body::empty()).unwrap())
		.await
		.unwrap();

	// Assert Status code
	assert_eq!(response.status(), StatusCode::OK);

	let body = response.into_body().collect().await.unwrap().to_bytes();

	// Convert the body to a UTF-8 string
    let body_str = std::str::from_utf8(&body).expect("Response body is not valid UTF-8");

    assert_eq!(body_str, "Healthy.");
}
