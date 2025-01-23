use std::{env, net::SocketAddr};

use coffee_backend;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = coffee_backend::app();

        // Get the host and the port from the env
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let addr = format!("{}:{}", host, port);
    let socket_addr: SocketAddr = addr.parse().expect("Unable to parse socket address");

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .unwrap();
    println!("Server listening on {}:{}", host, port);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
