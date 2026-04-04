use std::path::{Path, PathBuf};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use notes_core::{
    AuthError, CreateUserInput, ManagedUserSummary, ServerRole, UpdateUserInput, User,
};
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

const DEFAULT_DB_FILE: &str = ".auth.db";

#[derive(Debug, Clone)]
struct StoredUser {
    id: String,
    username: String,
    email: String,
    display_name: String,
    role: ServerRole,
    password_hash: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct SqliteUserStore {
    db_path: PathBuf,
}

impl SqliteUserStore {
    pub fn new(notes_root: &Path) -> Result<Self, AuthError> {
        let db_path = std::env::var("NYX_DB_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| notes_root.join(DEFAULT_DB_FILE));

        let store = Self { db_path };
        store.init()?;
        Ok(store)
    }

    pub fn bootstrap_admin(&self, password: Option<&str>) -> Result<(), AuthError> {
        if self.user_count()? > 0 {
            return Ok(());
        }

        match password {
            Some(password) => {
                self.create_initial_admin(password)?;
                eprintln!("Created initial admin user.");
            }
            None => {
                eprintln!(
                    "Warning: no users in auth database and NXY_DB_PASSWORD is not set. Login will fail until a user is created."
                );
            }
        }

        Ok(())
    }

    pub fn find_user(&self, user_id: &str) -> Result<Option<User>, AuthError> {
        Ok(self.load_user_by_id(user_id)?.map(|user| User {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            role: user.role,
        }))
    }

    pub fn login_user(&self, username: &str, password: &str) -> Result<User, AuthError> {
        let normalized_username = normalize_username(username)?;
        let connection = self.connection()?;
        let user = connection
            .query_row(
                "SELECT id, username, email, display_name, role, password_hash, created_at, updated_at FROM users WHERE username = ?1",
                params![normalized_username],
                map_stored_user,
            )
            .optional()
            .map_err(to_service_error)?
            .ok_or(AuthError::InvalidCredentials)?;

        verify_password_hash(&user.password_hash, password)?;

        Ok(User {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            role: user.role,
        })
    }

    pub fn list_users(&self, actor: &User) -> Result<Vec<ManagedUserSummary>, AuthError> {
        ensure_admin(actor)?;
        let connection = self.connection()?;
        let mut stmt = connection
            .prepare(
                "SELECT id, username, email, display_name, role, password_hash, created_at, updated_at
                 FROM users ORDER BY username ASC",
            )
            .map_err(to_service_error)?;

        let rows = stmt
            .query_map([], map_stored_user)
            .map_err(to_service_error)?;

        let users = rows
            .collect::<Result<Vec<_>, _>>()
            .map_err(to_service_error)?;
        let admin_count = users
            .iter()
            .filter(|user| matches!(user.role, ServerRole::Admin))
            .count();

        Ok(users
            .into_iter()
            .map(|user| to_summary(actor, user, admin_count))
            .collect())
    }

    pub fn create_user(
        &self,
        actor: &User,
        input: CreateUserInput,
    ) -> Result<ManagedUserSummary, AuthError> {
        ensure_admin(actor)?;

        let username = normalize_username(&input.username)?;
        let email = normalize_email(&input.email)?;
        let display_name = require_non_empty("display name", &input.display_name)?;
        validate_password_policy(&input.password)?;
        self.ensure_unique_username_email(Some(&username), &email, None)?;

        let now = Utc::now();
        let password_hash = hash_password(&input.password)?;
        let connection = self.connection()?;
        connection
            .execute(
                "INSERT INTO users (id, username, email, display_name, role, password_hash, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    Uuid::new_v4().to_string(),
                    username,
                    email,
                    display_name,
                    role_to_db(&input.role),
                    password_hash,
                    now.to_rfc3339(),
                    now.to_rfc3339(),
                ],
            )
            .map_err(to_constraint_error)?;

        let created = self
            .load_user_by_email(&normalize_email(&input.email)?)?
            .ok_or_else(|| AuthError::ServiceError("created user not found".into()))?;
        let admin_count = self.admin_count()?;
        Ok(to_summary(actor, created, admin_count))
    }

    pub fn update_user(
        &self,
        actor: &User,
        user_id: &str,
        input: UpdateUserInput,
    ) -> Result<ManagedUserSummary, AuthError> {
        ensure_admin(actor)?;

        let existing = self
            .load_user_by_id(user_id)?
            .ok_or(AuthError::UserNotFound)?;
        let email = normalize_email(&input.email)?;
        let display_name = require_non_empty("display name", &input.display_name)?;

        if actor.id == user_id && !matches!(input.role, ServerRole::Admin) {
            return Err(AuthError::Conflict(
                "administrators cannot remove their own admin role".into(),
            ));
        }

        if matches!(existing.role, ServerRole::Admin)
            && !matches!(input.role, ServerRole::Admin)
            && self.admin_count()? <= 1
        {
            return Err(AuthError::Conflict(
                "the last administrator cannot be demoted".into(),
            ));
        }

        self.ensure_unique_username_email(None, &email, Some(user_id))?;

        let password_hash = match input.password.as_deref() {
            Some(password) => {
                validate_password_policy(password)?;
                hash_password(password)?
            }
            None => existing.password_hash.clone(),
        };

        let now = Utc::now().to_rfc3339();
        let connection = self.connection()?;
        connection
            .execute(
                "UPDATE users
                 SET email = ?1, display_name = ?2, role = ?3, password_hash = ?4, updated_at = ?5
                 WHERE id = ?6",
                params![
                    email,
                    display_name,
                    role_to_db(&input.role),
                    password_hash,
                    now,
                    user_id,
                ],
            )
            .map_err(to_constraint_error)?;

        let updated = self
            .load_user_by_id(user_id)?
            .ok_or(AuthError::UserNotFound)?;
        let admin_count = self.admin_count()?;
        Ok(to_summary(actor, updated, admin_count))
    }

    pub fn delete_user(&self, actor: &User, user_id: &str) -> Result<(), AuthError> {
        ensure_admin(actor)?;

        if actor.id == user_id {
            return Err(AuthError::Conflict(
                "administrators cannot delete themselves".into(),
            ));
        }

        let existing = self
            .load_user_by_id(user_id)?
            .ok_or(AuthError::UserNotFound)?;
        if matches!(existing.role, ServerRole::Admin) && self.admin_count()? <= 1 {
            return Err(AuthError::Conflict(
                "the last administrator cannot be deleted".into(),
            ));
        }

        let connection = self.connection()?;
        connection
            .execute("DELETE FROM users WHERE id = ?1", params![user_id])
            .map_err(to_service_error)?;
        Ok(())
    }

    fn init(&self) -> Result<(), AuthError> {
        let connection = self.connection()?;
        connection
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS users (
                    id TEXT PRIMARY KEY,
                    username TEXT NOT NULL UNIQUE,
                    email TEXT NOT NULL UNIQUE,
                    display_name TEXT NOT NULL,
                    role TEXT NOT NULL,
                    password_hash TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );",
            )
            .map_err(to_service_error)?;
        Ok(())
    }

    fn connection(&self) -> Result<Connection, AuthError> {
        if let Some(parent) = self.db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| AuthError::ServiceError(e.to_string()))?;
        }
        Connection::open(&self.db_path).map_err(to_service_error)
    }

    fn create_initial_admin(&self, password: &str) -> Result<(), AuthError> {
        validate_password_policy(password)?;
        let now = Utc::now();
        let connection = self.connection()?;
        connection
            .execute(
                "INSERT INTO users (id, username, email, display_name, role, password_hash, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    Uuid::new_v4().to_string(),
                    "admin",
                    "admin@localhost",
                    "Admin",
                    role_to_db(&ServerRole::Admin),
                    hash_password(password)?,
                    now.to_rfc3339(),
                    now.to_rfc3339(),
                ],
            )
            .map_err(to_constraint_error)?;
        Ok(())
    }

    fn load_user_by_id(&self, user_id: &str) -> Result<Option<StoredUser>, AuthError> {
        let connection = self.connection()?;
        connection
            .query_row(
                "SELECT id, username, email, display_name, role, password_hash, created_at, updated_at FROM users WHERE id = ?1",
                params![user_id],
                map_stored_user,
            )
            .optional()
            .map_err(to_service_error)
    }

    fn load_user_by_email(&self, email: &str) -> Result<Option<StoredUser>, AuthError> {
        let connection = self.connection()?;
        connection
            .query_row(
                "SELECT id, username, email, display_name, role, password_hash, created_at, updated_at FROM users WHERE email = ?1",
                params![email],
                map_stored_user,
            )
            .optional()
            .map_err(to_service_error)
    }

    fn user_count(&self) -> Result<usize, AuthError> {
        let connection = self.connection()?;
        connection
            .query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))
            .map_err(to_service_error)
    }

    fn admin_count(&self) -> Result<usize, AuthError> {
        let connection = self.connection()?;
        connection
            .query_row(
                "SELECT COUNT(*) FROM users WHERE role = 'admin'",
                [],
                |row| row.get(0),
            )
            .map_err(to_service_error)
    }

    fn ensure_unique_username_email(
        &self,
        username: Option<&str>,
        email: &str,
        exclude_user_id: Option<&str>,
    ) -> Result<(), AuthError> {
        let connection = self.connection()?;

        if let Some(username) = username {
            let existing: Option<String> = connection
                .query_row(
                    "SELECT id FROM users WHERE username = ?1",
                    params![username],
                    |row| row.get(0),
                )
                .optional()
                .map_err(to_service_error)?;

            if existing.as_deref() != exclude_user_id && existing.is_some() {
                return Err(AuthError::Conflict("username already exists".into()));
            }
        }

        let existing: Option<String> = connection
            .query_row(
                "SELECT id FROM users WHERE email = ?1",
                params![email],
                |row| row.get(0),
            )
            .optional()
            .map_err(to_service_error)?;

        if existing.as_deref() != exclude_user_id && existing.is_some() {
            return Err(AuthError::Conflict("email already exists".into()));
        }

        Ok(())
    }
}

fn map_stored_user(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredUser> {
    let role: String = row.get(4)?;
    let created_at: String = row.get(6)?;
    let updated_at: String = row.get(7)?;

    Ok(StoredUser {
        id: row.get(0)?,
        username: row.get(1)?,
        email: row.get(2)?,
        display_name: row.get(3)?,
        role: role_from_db(&role).map_err(|err| {
            rusqlite::Error::FromSqlConversionFailure(
                4,
                rusqlite::types::Type::Text,
                Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, err)),
            )
        })?,
        password_hash: row.get(5)?,
        created_at: DateTime::parse_from_rfc3339(&created_at)
            .map(|value| value.with_timezone(&Utc))
            .map_err(|err| {
                rusqlite::Error::FromSqlConversionFailure(
                    6,
                    rusqlite::types::Type::Text,
                    Box::new(err),
                )
            })?,
        updated_at: DateTime::parse_from_rfc3339(&updated_at)
            .map(|value| value.with_timezone(&Utc))
            .map_err(|err| {
                rusqlite::Error::FromSqlConversionFailure(
                    7,
                    rusqlite::types::Type::Text,
                    Box::new(err),
                )
            })?,
    })
}

fn to_summary(actor: &User, user: StoredUser, admin_count: usize) -> ManagedUserSummary {
    let can_delete =
        actor.id != user.id && !(matches!(user.role, ServerRole::Admin) && admin_count <= 1);

    ManagedUserSummary {
        id: user.id,
        username: user.username,
        email: user.email,
        display_name: user.display_name,
        role: user.role,
        created_at: user.created_at,
        updated_at: user.updated_at,
        can_edit: true,
        can_delete,
    }
}

fn normalize_username(value: &str) -> Result<String, AuthError> {
    let normalized = require_non_empty("username", value)?.to_ascii_lowercase();
    Ok(normalized)
}

fn normalize_email(value: &str) -> Result<String, AuthError> {
    let normalized = require_non_empty("email", value)?.to_ascii_lowercase();
    if !normalized.contains('@') {
        return Err(AuthError::Validation("email must be valid".into()));
    }
    Ok(normalized)
}

fn require_non_empty(field: &str, value: &str) -> Result<String, AuthError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AuthError::Validation(format!("{field} is required")));
    }
    Ok(trimmed.to_string())
}

fn validate_password_policy(password: &str) -> Result<(), AuthError> {
    if password.chars().count() < 12 {
        return Err(AuthError::Validation(
            "password must be at least 12 characters".into(),
        ));
    }

    let mut categories = 0;
    if password.chars().any(|ch| ch.is_ascii_lowercase()) {
        categories += 1;
    }
    if password.chars().any(|ch| ch.is_ascii_uppercase()) {
        categories += 1;
    }
    if password.chars().any(|ch| ch.is_ascii_digit()) {
        categories += 1;
    }
    if password.chars().any(|ch| !ch.is_ascii_alphanumeric()) {
        categories += 1;
    }

    if categories < 3 {
        return Err(AuthError::Validation(
            "password must include at least 3 of: lowercase, uppercase, digit, symbol".into(),
        ));
    }

    Ok(())
}

fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| AuthError::ServiceError(err.to_string()))
        .map(|hash| hash.to_string())
}

fn verify_password_hash(password_hash: &str, password: &str) -> Result<(), AuthError> {
    let parsed =
        PasswordHash::new(password_hash).map_err(|err| AuthError::ServiceError(err.to_string()))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|_| AuthError::InvalidCredentials)
}

fn ensure_admin(actor: &User) -> Result<(), AuthError> {
    if matches!(actor.role, ServerRole::Admin) {
        Ok(())
    } else {
        Err(AuthError::Forbidden)
    }
}

fn role_to_db(role: &ServerRole) -> &'static str {
    match role {
        ServerRole::Admin => "admin",
        ServerRole::User => "user",
    }
}

fn role_from_db(value: &str) -> Result<ServerRole, String> {
    match value {
        "admin" => Ok(ServerRole::Admin),
        "user" => Ok(ServerRole::User),
        other => Err(format!("unknown role: {other}")),
    }
}

fn to_service_error(error: rusqlite::Error) -> AuthError {
    AuthError::ServiceError(error.to_string())
}

fn to_constraint_error(error: rusqlite::Error) -> AuthError {
    match error {
        rusqlite::Error::SqliteFailure(inner, _)
            if inner.extended_code == 2067 || inner.extended_code == 1555 =>
        {
            AuthError::Conflict("managed user already exists".into())
        }
        other => AuthError::ServiceError(other.to_string()),
    }
}
