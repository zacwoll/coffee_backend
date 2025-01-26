use std::{net::SocketAddr};

use axum::{routing::connect, Router};
use coffee_backend::{self, configuration::get_configuration, AppState};
use sqlx::{PgPool, Connection};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't get configuration
    let configuration = get_configuration().expect("Failed to get configuration");
    let connection_string = configuration.database.connection_string();
    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to postgres");

    let state = AppState { pool };

    let app= coffee_backend::app().with_state(state);

    let addr = format!("127.0.0.1:{}", configuration.application_port);
    let socket_addr: SocketAddr = addr.parse().expect("Unable to parse socket address");

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .unwrap();
    println!("Server listening on {}:{}", "127.0.0.1", configuration.application_port);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}