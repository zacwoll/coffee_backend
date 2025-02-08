use crate::{app, configuration::get_configuration, AppState};

use axum::{serve::Serve, Router};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// Spawns the app and returns the url where it's at.
pub async fn spawn_app() -> TestApp {
	let listener = get_listener().await;

    let port = listener.local_addr().unwrap().port();

	let address = format!("http://127.0.0.1:{}", port);

	let configuration = get_configuration().expect("Failed to read configuration");

	let connection_string = configuration.database.connection_string();

	let connection_pool = PgPool::connect(&connection_string).await.expect("Failed to connect to the database");

	let server = run(listener, connection_pool.clone());

	let _ = tokio::spawn(server);

	TestApp {
		address,
		db_pool: connection_pool,
	}
}

/// Run the function given a tokio TCP listener
pub async fn run(listener: tokio::net::TcpListener, pool: PgPool) -> Result<Serve<tokio::net::TcpListener, Router, Router>, std::io::Error>  {
	let state = AppState { pool };

    let app = app().with_state(state);

    let serve = axum::serve(listener, app);

    Ok(serve)
}

/// Gets a Pool of connections from the configuration.yml database values
pub async fn get_db() -> PgPool {
	let configuration = get_configuration().expect("Failed to get database");
	let connection_string = configuration.database.connection_string();

	let pool = PgPool::connect(&connection_string)
		.await
		.expect("Failed to connect to database");

	pool
}

// Get listener reads the config and returns a TCP listener to the configured port
// The config can fail with config::ConfigError and the Listener can fail with io:Error
pub async fn get_listener() -> tokio::net::TcpListener {
	// get the configuration
	let config = get_configuration().expect("Failed to get config");

	let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind port");

	listener
}
