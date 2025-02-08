pub mod configuration;
pub mod routes;
pub mod startup;

pub use routes::app;

use sqlx::PgPool;


#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
}
