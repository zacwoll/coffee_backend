use coffee_backend;
use axum::{
	body::Body,
	http::{Request, StatusCode},
};
use tower::ServiceExt; // for 'call', 'oneshot', and 'ready'
use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_fallback() {
	let app = coffee_backend::app();

	let response = app
		.oneshot(Request::builder().uri("/not_found").body(Body::empty()).unwrap())
		.await
		.unwrap();

	assert_eq!(response.status(), StatusCode::NOT_FOUND);
}