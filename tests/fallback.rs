use coffee_backend::{self, AppState};
use axum::{
	body::Body,
	http::{Request, StatusCode},
};
use tower::ServiceExt; // for 'oneshot' and 'ready'
use pretty_assertions::assert_eq;

#[path = "common.rs"]
mod common;

#[tokio::test]
async fn test_fallback() {
	let test_pool = common::get_test_db().await.unwrap();

	let state = AppState { pool: test_pool };

	let app = coffee_backend::app().with_state(state);

	let response = app
		.oneshot(Request::builder().uri("/not_found").body(Body::empty()).unwrap())
		.await
		.unwrap();

	assert_eq!(response.status(), StatusCode::NOT_FOUND);
}