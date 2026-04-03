use std::{
    path::Path,
    sync::{Arc, RwLock},
};

use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use notes_core::{AuthError, AuthStore, LoginToken, ServerRole, User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user_store::{load_users, save_users, LocalUser};

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
/// `$NOTES_ROOT/.users.json` with Argon2-hashed passwords.
pub struct SecretKeyAuthStore {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    users: Arc<RwLock<Vec<LocalUser>>>,
}

impl SecretKeyAuthStore {
    /// Construct from a raw 32-byte (256-bit) key and load users from `notes_root`.
    ///
    /// If `notes_root/.users.json` is missing or empty and `admin_password` is provided,
    /// an initial admin user is created automatically.
    pub fn new(
        notes_root: &Path,
        key_bytes: &[u8],
        admin_password: Option<&str>,
    ) -> Result<Self, AuthError> {
        let mut users = load_users(notes_root)?;

        if users.is_empty() {
            match admin_password {
                Some(password) => {
                    let admin = LocalUser::new(
                        Uuid::new_v4().to_string(),
                        "admin".into(),
                        "admin@localhost".into(),
                        "Admin".into(),
                        password,
                    )?;
                    users.push(LocalUser {
                        role: ServerRole::Admin,
                        ..admin
                    });
                    save_users(notes_root, &users)?;
                    eprintln!("Created initial admin user.");
                }
                None => {
                    eprintln!(
                        "Warning: no users in .users.json and NOTES_ADMIN_PASSWORD is not set. \
                         Login will fail until a user is created."
                    );
                }
            }
        }

        Ok(Self {
            encoding_key: EncodingKey::from_secret(key_bytes),
            decoding_key: DecodingKey::from_secret(key_bytes),
            users: Arc::new(RwLock::new(users)),
        })
    }
}

impl AuthStore for SecretKeyAuthStore {
    fn find_user(&self, user_id: &str) -> Result<Option<User>, AuthError> {
        let users = self
            .users
            .read()
            .map_err(|_| AuthError::ServiceError("user store lock poisoned".into()))?;
        Ok(users.iter().find(|u| u.id == user_id).map(to_user))
    }

    fn verify_token(&self, token: &str) -> Result<User, AuthError> {
        let data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        let user_id = data.claims.sub;

        self.find_user(&user_id)?.ok_or(AuthError::UserNotFound)
    }

    fn login(&self, username: &str, password: &str) -> Result<LoginToken, AuthError> {
        let users = self
            .users
            .read()
            .map_err(|_| AuthError::ServiceError("user store lock poisoned".into()))?;

        let local_user = users
            .iter()
            .find(|u| u.username == username)
            .ok_or(AuthError::InvalidCredentials)?;

        local_user.verify_password(password)?;

        let exp = (Utc::now().timestamp() as u64 + TOKEN_TTL_SECS) as usize;
        let claims = Claims {
            sub: local_user.id.clone(),
            exp,
        };
        let token = encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AuthError::ServiceError(e.to_string()))?;

        Ok(LoginToken {
            token,
            expires_in: TOKEN_TTL_SECS,
        })
    }
}

fn to_user(u: &LocalUser) -> User {
    User {
        id: u.id.clone(),
        email: u.email.clone(),
        display_name: u.display_name.clone(),
        role: u.role.clone(),
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
