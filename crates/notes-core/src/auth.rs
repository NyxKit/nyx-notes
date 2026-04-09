use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::ServerRole;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub role: ServerRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedUserSummary {
    pub id: String,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub role: ServerRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub can_edit: bool,
    pub can_delete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub display_name: String,
    #[serde(default)]
    pub role: Option<ServerRole>,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserInput {
    pub email: String,
    pub display_name: String,
    pub role: ServerRole,
    pub password: Option<String>,
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
    #[error("forbidden")]
    Forbidden,
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("service error: {0}")]
    ServiceError(String),
}

pub trait AuthStore: Send + Sync {
    fn find_user(&self, user_id: &str) -> Result<Option<User>, AuthError>;
    fn find_user_by_username(&self, username: &str) -> Result<Option<User>, AuthError>;
    fn verify_token(&self, token: &str) -> Result<User, AuthError>;

    /// Authenticate with username and password, returning a signed token.
    /// Default implementation returns an error; only `secret_key` mode overrides this.
    fn login(&self, _username: &str, _password: &str) -> Result<LoginToken, AuthError> {
        Err(AuthError::ServiceError(
            "login not supported in this auth mode".into(),
        ))
    }

    fn list_users(&self, _actor: &User) -> Result<Vec<ManagedUserSummary>, AuthError> {
        Err(AuthError::ServiceError(
            "user management not supported in this auth mode".into(),
        ))
    }

    fn create_user(
        &self,
        _actor: &User,
        _input: CreateUserInput,
    ) -> Result<ManagedUserSummary, AuthError> {
        Err(AuthError::ServiceError(
            "user management not supported in this auth mode".into(),
        ))
    }

    fn update_user(
        &self,
        _actor: &User,
        _user_id: &str,
        _input: UpdateUserInput,
    ) -> Result<ManagedUserSummary, AuthError> {
        Err(AuthError::ServiceError(
            "user management not supported in this auth mode".into(),
        ))
    }

    fn delete_user(&self, _actor: &User, _user_id: &str) -> Result<(), AuthError> {
        Err(AuthError::ServiceError(
            "user management not supported in this auth mode".into(),
        ))
    }

    /// Check if any users exist in the system.
    fn is_initialized(&self) -> Result<bool, AuthError> {
        Err(AuthError::ServiceError(
            "user management not supported in this auth mode".into(),
        ))
    }

    /// Create the first admin user. Only succeeds if `is_initialized()` returns false.
    fn setup_initial_user(&self, _input: CreateUserInput) -> Result<LoginToken, AuthError> {
        Err(AuthError::ServiceError(
            "user management not supported in this auth mode".into(),
        ))
    }
}
