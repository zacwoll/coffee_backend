use coffee_backend::{self, configuration::get_configuration, startup::{get_db, get_listener, run}, AppState};


#[tokio::main]
async fn main() -> std::io::Result<()> {
        // Panic if we can't get configuration
    let configuration = get_configuration().expect("Failed to get configuration");
    
    // Get Pool of Postgres Connections
    let db = get_db().await;
    
    // Create AppState from pool
    let state = AppState { pool: db };
    
    // Create Application Router with State (AppState)
    let app= coffee_backend::app().with_state(state);
    
    // Get a listener from the OS
    let listener = get_listener().await;
    
    // Serve Application
    println!("Server listening on {}:{}", configuration.database.host, configuration.application_port);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}