use std::path::Path;

use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use notes_core::{
    AuthError, AuthStore, CreateUserInput, LoginToken, ManagedUserSummary, UpdateUserInput, User,
};
use serde::{Deserialize, Serialize};

use crate::sqlite_user_store::SqliteUserStore;

const TOKEN_TTL_SECS: u64 = 86_400; // 24 hours

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// Subject: user id.
    sub: String,
    /// Expiry (Unix timestamp, seconds).
    exp: usize,
}

/// `AuthStore` implementation for `AUTH_MODE=secret_key`.
///
/// Signs HS256 JWTs with a server-managed key. Users are stored in
/// a local SQLite database with Argon2-hashed passwords.
pub struct SecretKeyAuthStore {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    users: SqliteUserStore,
}

impl SecretKeyAuthStore {
    /// Construct from a raw 32-byte (256-bit) key and load users from `notes_root`.
    /// If the auth database is empty and `admin_password` is provided, an initial admin user is created automatically.
    pub fn new(
        notes_root: &Path,
        key_bytes: &[u8],
        admin_password: Option<&str>,
    ) -> Result<Self, AuthError> {
        let users = SqliteUserStore::new(notes_root)?;
        users.bootstrap_admin(admin_password)?;

        Ok(Self {
            encoding_key: EncodingKey::from_secret(key_bytes),
            decoding_key: DecodingKey::from_secret(key_bytes),
            users,
        })
    }
}

impl AuthStore for SecretKeyAuthStore {
    fn find_user(&self, user_id: &str) -> Result<Option<User>, AuthError> {
        self.users.find_user(user_id)
    }

    fn verify_token(&self, token: &str) -> Result<User, AuthError> {
        let data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        let user_id = data.claims.sub;

        self.find_user(&user_id)?.ok_or(AuthError::UserNotFound)
    }

    fn login(&self, username: &str, password: &str) -> Result<LoginToken, AuthError> {
        let user = self.users.login_user(username, password)?;

        let exp = (Utc::now().timestamp() as u64 + TOKEN_TTL_SECS) as usize;
        let claims = Claims { sub: user.id, exp };
        let token = encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AuthError::ServiceError(e.to_string()))?;

        Ok(LoginToken {
            token,
            expires_in: TOKEN_TTL_SECS,
        })
    }

    fn list_users(&self, actor: &User) -> Result<Vec<ManagedUserSummary>, AuthError> {
        self.users.list_users(actor)
    }

    fn create_user(
        &self,
        actor: &User,
        input: CreateUserInput,
    ) -> Result<ManagedUserSummary, AuthError> {
        self.users.create_user(actor, input)
    }

    fn update_user(
        &self,
        actor: &User,
        user_id: &str,
        input: UpdateUserInput,
    ) -> Result<ManagedUserSummary, AuthError> {
        self.users.update_user(actor, user_id, input)
    }

    fn delete_user(&self, actor: &User, user_id: &str) -> Result<(), AuthError> {
        self.users.delete_user(actor, user_id)
    }
}

// --- Key loading helpers (used by main.rs) ---

/// Load or generate the HMAC signing key.
///
/// Resolution order:
/// 1. `NOTES_SECRET_KEY` env var — 64 hex characters (32 bytes).
/// 2. File at `NOTES_SECRET_KEY_PATH` (default: `~/.config/nyx-notes/secret.key`).
/// 3. Generate a fresh random key and persist it to the path above.
pub fn load_or_generate_key() -> Vec<u8> {
    if let Ok(hex) = std::env::var("NOTES_SECRET_KEY") {
        return hex_decode(&hex)
            .expect("NOTES_SECRET_KEY must be exactly 64 hex characters (32 bytes)");
    }

    let key_path = std::env::var("NOTES_SECRET_KEY_PATH").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        format!("{home}/.config/nyx-notes/secret.key")
    });
    let key_path = std::path::Path::new(&key_path);

    if key_path.exists() {
        return std::fs::read(key_path).expect("failed to read secret key file");
    }

    let mut key = vec![0u8; 32];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut key);

    if let Some(parent) = key_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(key_path, &key).expect("failed to write secret key");
    eprintln!("Generated new signing key at {}", key_path.display());
    key
}

fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}
