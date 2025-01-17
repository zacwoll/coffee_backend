use axum::{
    routing::{post, delete},
    Router,
};
use axum::extract::Json;
use axum::http::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use anyhow::Context;

#[derive(Serialize, Deserialize)]
pub struct User {
	pub id: String,
    pub username: String,
    pub email: String,
	// password: String,
}

#[derive(Serialize, Deserialize)]

pub struct RegisterUser {
	pub username: String,
	pub email: String,
	password: String,
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

/// Returns a router with all the user functions
pub fn user_router() -> Router {
    let user_router = Router::new()
		.route("/register", post(register_user))
		.route("/login", post(login_user))
		.route("/delete", delete(delete_user))
		;
	user_router
}

/// Register a new user in the database
pub async fn register_user(Json(payload): Json<RegisterUser>) -> impl axum::response::IntoResponse {
	let username = payload.username;
	// TODO: salt the password
	// let password: String = payload.password;

	let uuid = Uuid::new_v4();
	let id = uuid.to_string();

	let new_user = User {
		id: id.clone(),
		username: username.clone(),
		email: payload.email,
	};
	println!("User {} created with id: {}", username, id);
	(StatusCode::CREATED, Json(new_user))
}

/// Checks a set of credentials against the DB and if found, logs in the user (creates session token)
pub async fn login_user(Json(payload): Json<UserCredentials>) -> impl axum::response::IntoResponse {
	// So first I have payload which is user credentials
	// let's check those credentials
    let session_token = "I am a token".to_string();

	match validate_credentials(payload).await {
		Ok(_user_id) => {
			// Success, create session token, store in DB, store in server-side cookies for user
			// let session_token = "I am a token";
			(StatusCode::OK, Json(session_token));
		},
		Err(e) => {
            // Error, inform the user trying to log in that it failed
            let error_message = match e {
                AuthError::ValidationError(msg) => (StatusCode::UNAUTHORIZED, Json(msg)),
                AuthError::UnexpectedError(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal server error".to_string())),
            };
            error_message;
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
        verify_password_hash(
            expected_password_hash, 
            &credentials.password
        )
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
fn verify_password_hash(expected_password_hash: &str, password_candidate: &str) -> Result<(), AuthError> {
	// Parse the password using PHC format using Argon2 struct PasswordHash
	let expected_password_hash = PasswordHash::new(&expected_password_hash)
        .context("Failed to parse hash in PHC string format.")
        .map_err(AuthError::UnexpectedError)?;

		// Perform an equality check of the hashed passwords
    Argon2::default()
        .verify_password(
            password_candidate.as_bytes(), 
            &expected_password_hash
        )
        .context("Invalid password.")
        .map_err(|e| AuthError::ValidationError(e.to_string()))?;

    Ok(())
}



/// Deletes a user from the database
pub async fn delete_user(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}