use coffee_backend;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`
use axum::{
	body::Body,
	http::{Request, StatusCode},
};
use pretty_assertions::assert_eq;
use http_body_util::BodyExt; // for `collect`


#[tokio::test]
async fn health_check() {
	let app = coffee_backend::app();

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

    assert_eq!("Healthy.", body_str);
}