use notes_core::{NotePermission, TeamRole};
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
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum AuthModeResponse {
    Local,
    SecretKey,
    Oidc { issuer: String, client_id: String },
}

impl From<&AuthConfig> for AuthModeResponse {
    fn from(cfg: &AuthConfig) -> Self {
        match cfg {
            AuthConfig::Local => AuthModeResponse::Local,
            AuthConfig::SecretKey => AuthModeResponse::SecretKey,
            AuthConfig::Oidc { issuer, client_id } => AuthModeResponse::Oidc {
                issuer: issuer.clone(),
                client_id: client_id.clone(),
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
pub struct CreateCommentRequest {
    pub quoted_text: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct CreateReplyRequest {
    pub body: String,
}

#[derive(Deserialize)]
pub struct PatchCommentRequest {
    pub resolved: bool,
}

// --- Team request types ---

#[derive(Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct AddMemberRequest {
    pub user_id: String,
    pub role: TeamRole,
}

#[derive(Deserialize)]
pub struct PatchMemberRequest {
    pub role: TeamRole,
}
