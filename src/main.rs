use coffee_backend::{self, configuration::get_configuration, startup::{get_db, get_listener}, AppState};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    // TODO: Move the env filter declaration up here.
    // let env_filter = EnvFilter::try_from_default_env()
    //     .unwrap_or_else(|_| EnvFilter::new("info"));

    // Create Bunyan Formatting Layer
    let formatting_layer = BunyanFormattingLayer::new(
        format!(
            "{}",
            env!("CARGO_PKG_NAME")
        )
        .into(),
        // Output the formatted spans to stdout. 
        std::io::stdout
    );

    // Adding Tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=trace,tower_http=trace,axum::rejection=trace",
                    env!("CARGO_PKG_NAME")
                )
                .into()
            })
            .add_directive("axum=trace".parse().unwrap())
            .add_directive("tower_http=trace".parse().unwrap())
        )
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .init();

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

    let port = listener.local_addr().unwrap().port();
    
    // Serve Application
    tracing::info!("Server listening on {}:{}", configuration.database.host, port);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}