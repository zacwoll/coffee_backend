use axum::serve::Serve;
use axum::Router;
use coffee_backend::{self, configuration, AppState};

use sqlx::PgPool;

use testcontainers_modules::postgres;
use testcontainers_modules::testcontainers::runners::AsyncRunner;

// This function creates a test database to run in your local docker
// using testcontainers & testcontainers_modules::postgres
pub async fn get_test_db() -> Result<PgPool, anyhow::Error> {

	// Config will contain defaults for this connection
	let config = configuration::get_configuration().unwrap();

	// Creates a postgres instance of the defined type (default)
    let postgres_instance = postgres::Postgres::default().start().await.unwrap();

	// Get the new SocketAddr from docker
	let pg_host = postgres_instance.get_host().await.unwrap();
    let pg_port = postgres_instance.get_host_port_ipv4(config.database.port).await.unwrap();

	// Apply Test Postgres values to configured postgres {host}:{port}
    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database.username,
        config.database.password,
        pg_host,
        pg_port,
        config.database.database_name,
    );

	// Get a PgPool of connections
    let db: PgPool = PgPool::connect(&connection_string).await.unwrap();

	// Return pool
	Ok(db)
}

// This is a test version of run that swaps out the DB for a testcontainers Postgres instance
// Returns a handle to the server
pub async fn test_run() -> Result<Serve<tokio::net::TcpListener, Router, Router>, anyhow::Error> {

	// Get a pool of connections to a test database
	let pool = get_test_db().await.unwrap();

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

// This function creates a test version of the server to be used during your test. 
// Added to our test functions, when they leave scope, this server is cleaned up.
fn test_app() {
	// Run server
	let server = test_run();

	// Spawn server as a background task
	let _ = tokio::spawn(server);
}

// This is a test of the test database function that starts alongside the test app
#[tokio::test]
async fn test_db_connect() {
	let pool = get_test_db().await.expect("Connection to Database failed");

	// If connected, size returns 1
    // Size is 1 connection to PgPool
    assert_eq!(pool.size(), 1);
}
