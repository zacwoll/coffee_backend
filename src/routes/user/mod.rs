use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Form;
use axum::{
    routing::{delete, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use secrecy::{ExposeSecret, SecretString};

#[derive(Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: SecretString,
}

#[derive(Deserialize, Debug)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: SecretString,
}

#[derive(Serialize, Deserialize)]
pub struct UserCredentials {
    pub email: String,
    password: String,
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(thiserror::Error)]
pub enum AuthError {
    #[error("{0}")]
    ValidationError(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// Utility Function for Sourcing Errors
// Error {from} \nCaused By:{source}
pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

use super::AppState;

/// Returns a router with all the user functions
/// coffee_backend:port/user/
pub fn user_router() -> Router<AppState> {
    return Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/delete", delete(delete_user));
}

/// Found this on the Axum Tracing Example, it's a way to encode a 500 response to user
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[tracing::instrument(
    name = "Saving new user details in the database",
    skip(pool)
)]
pub async fn insert_new_user(
    form: RegisterUser,
    pool: PgPool
) -> Result<(), sqlx::Error> {
    // Craft new User sql query

    // Deserialize payload
    let username = form.username;
    let email = form.email;
    // TODO: Create PHC + set up password hashing
    let password = form.password;

    // Initialize Uuid
    let uuid = Uuid::new_v4();

    // Begin Querying the Database from the pool
    // Note: sqlx::query! throws an error when docker is not running locally
    // sqlx tries to create a connection to your database to read the schema.
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, username, password_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        &uuid,
        &email,
        &username,
        &password.expose_secret(),
        Utc::now(),
        Utc::now(),
    )
    .execute(&pool)
    .await.map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e	
    })?;

    tracing::info!("User {} created with id: {}", username, uuid);

    Ok(())
}

/// Register a new user in the database
// Instrument adds the parameters of the function to the traces coming out of this function.
#[tracing::instrument(
    name = "Registering new user",
    skip(state, payload)
)]
pub async fn register_user(
    State(state): State<AppState>,
    Form(payload): Form<RegisterUser>,
) -> impl axum::response::IntoResponse {

    // Start Registering the user from the payload
    tracing::info!("Registering '{}' '{}' as a new subscriber", payload.email, payload.username);

    match insert_new_user(payload, state.pool)
    .await
    {
        Ok(_result) => {
            (StatusCode::CREATED, Json(format!("User created."))).into_response()
        },
        Err(_e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Internal Server Error"))).into_response()
        }
    }
}

/// Checks a set of credentials against the DB and if found, logs in the user (creates session token)
pub async fn login_user (
    Form(payload): Form<UserCredentials>,
) -> impl axum::response::IntoResponse {
    // So first I have payload which is user credentials
    // let's check those credentials
    let session_token = "I am a session token".to_string();

    match validate_credentials(payload).await {
        Ok(_user_id) => {
            // Success, create session token, store in DB, store in server-side cookies for user
            (StatusCode::OK, Json(session_token))
        }
        Err(e) => {
            // Error, inform the user trying to log in that it failed
            let error_message = match e {
                AuthError::ValidationError(msg) => (StatusCode::UNAUTHORIZED, Json(msg)),
                AuthError::UnexpectedError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Internal server error".to_string()),
                ),
            };
            error_message
        }
    }
}

/// Function the validate the user credentials against the DB
async fn validate_credentials(credentials: UserCredentials) -> Result<uuid::Uuid, AuthError> {
    // Query Database using Email
    let _email = credentials.email;
    // Using email, query the database for the password
    // Fake DB Query here
    // DB Query returns the expected password and user_id
    let expected_password_hash = "fake_password";

    // Spawn blocking task in it's own thread to handle password verification
    tokio::task::spawn_blocking(move || {
        verify_password_hash(expected_password_hash, &credentials.password)
    })
    .await
    .context("Failed to spawn blocking task.")
    .map_err(AuthError::UnexpectedError)??;

    // Return the validated user id
    // Return a dummy UUID for now
    let user_id = Uuid::new_v4();
    Ok(user_id)
}

/// Function to verify the password hash
fn verify_password_hash(
    expected_password_hash: &str,
    password_candidate: &str,
) -> Result<(), AuthError> {
    // Parse the password using PHC format using Argon2 struct PasswordHash
    let expected_password_hash = PasswordHash::new(&expected_password_hash)
        .context("Failed to parse hash in PHC string format.")
        .map_err(AuthError::UnexpectedError)?;

    // Perform an equality check of the hashed passwords
    Argon2::default()
        .verify_password(password_candidate.as_bytes(), &expected_password_hash)
        .context("Invalid password.")
        .map_err(|e| AuthError::ValidationError(e.to_string()))?;

    Ok(())
}

/// Deletes a user from the database
pub async fn delete_user(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}
