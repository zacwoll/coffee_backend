use coffee_backend::{self, AppState};
use axum::{
	body::Body,
	http::{Request, StatusCode},
};
use tower::ServiceExt; // for 'call', 'oneshot', and 'ready'
use pretty_assertions::assert_eq;

struct MockPgPool;

impl MockPgPool {
	async fn new() -> Self {
		MockPgPool
	}
}
// This is a test version of run that swaps out the DB for a testcontainers Postgres instance
async fn test_run_with_mock_db() -> Result<Serve<tokio::net::TcpListener, Router, Router>, anyhow::Error> {

	// Create a mock PgPool type

	// Use the mock PgPool instead of a real database connection pool
	let pool = MockPgPool::new().await;

	// Declare application state
	let state = AppState { pool };
	
	// Create the application with the state hook
    let app = coffee_backend::app().with_state(state);
	
	// Get a listener to serve the application on
	let listener = coffee_backend::startup::get_listener().await;

	// Get a handle to the server listening
	let serve = axum::serve(listener, app);

	// Return Server handle
	Ok(serve)
}

#[tokio::test]
async fn test_fallback() {
	let app = coffee_backend::app();

	let response = app
		.oneshot(Request::builder().uri("/not_found").body(Body::empty()).unwrap())
		.await
		.unwrap();

	assert_eq!(response.status(), StatusCode::NOT_FOUND);
}