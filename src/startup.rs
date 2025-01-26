use std::net::SocketAddr;

use crate::{app, configuration::get_configuration, AppState};

use axum::{serve::Serve, Router};
use sqlx::PgPool;


/// Run the function given a tokio TCP listener
pub async fn run(listener: tokio::net::TcpListener) -> Result<Serve<tokio::net::TcpListener, Router, Router>, std::io::Error>  {

	let pool = get_db()
		.await;
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

pub async fn get_listener() -> tokio::net::TcpListener {
	// get the configuration
	let config = get_configuration().expect("Failed to get config");

	let addr = format!("127.0.0.1:{}", config.application_port);
    let socket_addr: SocketAddr = addr.parse().expect("Unable to parse socket address");

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .unwrap();

	listener
}
