pub mod configuration;
pub mod routes;
pub mod startup;

pub use routes::app;

use sqlx::PgPool;


#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}
