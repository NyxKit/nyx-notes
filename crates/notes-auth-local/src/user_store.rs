use std::path::Path;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use notes_core::{AuthError, ServerRole};
use serde::{Deserialize, Serialize};

/// A user record stored in `$NOTES_ROOT/.users.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub display_name: String,
    #[serde(default = "default_role")]
    pub role: ServerRole,
    pub(crate) password_hash: String,
}

fn default_role() -> ServerRole {
    ServerRole::User
}

impl LocalUser {
    pub fn new(
        id: String,
        username: String,
        email: String,
        display_name: String,
        plaintext_password: &str,
    ) -> Result<Self, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(plaintext_password.as_bytes(), &salt)
            .map_err(|e| AuthError::ServiceError(e.to_string()))?
            .to_string();
        Ok(Self {
            id,
            username,
            email,
            display_name,
            role: ServerRole::User,
            password_hash: hash,
        })
    }

    pub fn verify_password(&self, password: &str) -> Result<(), AuthError> {
        let parsed = PasswordHash::new(&self.password_hash)
            .map_err(|e| AuthError::ServiceError(e.to_string()))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .map_err(|_| AuthError::InvalidCredentials)
    }
}

/// Load users from `<root>/.users.json`. Returns an empty list if the file does not exist.
pub fn load_users(root: &Path) -> Result<Vec<LocalUser>, AuthError> {
    let path = root.join(".users.json");
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content =
        std::fs::read_to_string(&path).map_err(|e| AuthError::ServiceError(e.to_string()))?;
    serde_json::from_str(&content).map_err(|e| AuthError::ServiceError(e.to_string()))
}

/// Persist users to `<root>/.users.json`.
pub fn save_users(root: &Path, users: &[LocalUser]) -> Result<(), AuthError> {
    let path = root.join(".users.json");
    let content =
        serde_json::to_string_pretty(users).map_err(|e| AuthError::ServiceError(e.to_string()))?;
    std::fs::write(&path, content).map_err(|e| AuthError::ServiceError(e.to_string()))
}
