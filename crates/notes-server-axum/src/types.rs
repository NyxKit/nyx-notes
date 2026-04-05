use notes_core::{NotePermission, ServerRole};
use serde::{Deserialize, Serialize};

// --- Auth request types ---

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// --- Auth config ---

/// The auth mode the server is running in.
/// Stored in `AppState` and returned by `GET /api/auth/mode`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthConfig {
    Local,
    SecretKey,
    Oidc { issuer: String, client_id: String },
}

/// JSON response shape for `GET /api/auth/mode`.
#[derive(Serialize)]
pub struct AuthModeResponse {
    pub mode: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}

impl From<&AuthConfig> for AuthModeResponse {
    fn from(cfg: &AuthConfig) -> Self {
        let server_name =
            std::env::var("SERVER_NAME").unwrap_or_else(|_| "Main Server".to_string());
        let server_id = format!("server-{}", notes_core::slugify(&server_name));
        match cfg {
            AuthConfig::Local => AuthModeResponse {
                mode: "local",
                issuer: None,
                client_id: None,
                server_id: Some(server_id.clone()),
                server_name: Some(server_name.clone()),
                api_version: Some(env!("CARGO_PKG_VERSION").to_string()),
            },
            AuthConfig::SecretKey => AuthModeResponse {
                mode: "secret_key",
                issuer: None,
                client_id: None,
                server_id: Some(server_id.clone()),
                server_name: Some(server_name.clone()),
                api_version: Some(env!("CARGO_PKG_VERSION").to_string()),
            },
            AuthConfig::Oidc { issuer, client_id } => AuthModeResponse {
                mode: "oidc",
                issuer: Some(issuer.clone()),
                client_id: Some(client_id.clone()),
                server_id: Some(server_id),
                server_name: Some(server_name),
                api_version: Some(env!("CARGO_PKG_VERSION").to_string()),
            },
        }
    }
}

// --- Note request types ---

#[derive(Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub category: Option<String>,
    /// Defaults to the vault's permission if omitted; not yet enforced server-side.
    pub permission: Option<NotePermission>,
}

#[derive(Deserialize)]
pub struct UpdateNoteRequest {
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub category: Option<String>,
}

#[derive(Deserialize)]
pub struct PatchPermissionRequest {
    pub permission: NotePermission,
}

// --- Vault request types ---

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub role: Option<ServerRole>,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub email: String,
    pub display_name: String,
    pub role: ServerRole,
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateVaultRequest {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Deserialize)]
pub struct PatchVaultRequest {
    pub name: Option<String>,
    /// None = field absent (no change); Some(None) = clear description; Some(Some(text)) = set description.
    #[serde(default)]
    pub description: Option<Option<String>>,
    /// None = field absent (no change); Some(None) = clear icon; Some(Some(slug)) = set icon.
    #[serde(default)]
    pub icon: Option<Option<String>>,
}

// --- Comment request types ---

#[derive(Deserialize)]
pub struct CreateCommentAnchorRequest {
    pub text: String,
    pub prefix: String,
    pub suffix: String,
    pub range_from: u32,
    pub range_to: u32,
    pub line_preview: String,
}

#[derive(Deserialize)]
pub struct CreateCommentRequest {
    pub body: String,
    pub anchor: CreateCommentAnchorRequest,
}

#[derive(Deserialize)]
pub struct CreateReplyRequest {
    pub body: String,
}

#[derive(Deserialize)]
pub struct PatchCommentRequest {
    pub resolved: bool,
}

#[derive(Serialize)]
pub struct ServerMetadataResponse {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub role: &'static str,
    pub root_path: Option<String>,
    pub current_user_id: String,
    pub current_user_username: String,
}
