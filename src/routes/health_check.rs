/// Health Check handler for app
pub async fn health_check() -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::OK,
        format!("Healthy."),
    )
}