use crate::app;

use axum::{serve::Serve, Router};

/// Run the function given a tokio TCP listener
pub async fn run(listener: tokio::net::TcpListener) -> Result<Serve<tokio::net::TcpListener, Router, Router>, std::io::Error>  {
    let app = app();
    let serve = axum::serve(listener, app);

    Ok(serve)
}