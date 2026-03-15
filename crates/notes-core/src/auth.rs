use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub display_name: String,
}

/// Returned by `AuthStore::login` on success.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginToken {
    pub token: String,
    /// Seconds until the token expires.
    pub expires_in: u64,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid token")]
    InvalidToken,
    #[error("user not found")]
    UserNotFound,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("service error: {0}")]
    ServiceError(String),
}

pub trait AuthStore: Send + Sync {
    fn find_user(&self, user_id: &str) -> Result<Option<User>, AuthError>;
    fn verify_token(&self, token: &str) -> Result<User, AuthError>;

    /// Authenticate with username and password, returning a signed token.
    /// Default implementation returns an error; only `secret_key` mode overrides this.
    fn login(&self, _username: &str, _password: &str) -> Result<LoginToken, AuthError> {
        Err(AuthError::ServiceError(
            "login not supported in this auth mode".into(),
        ))
    }
}
