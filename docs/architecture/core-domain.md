# Core Domain (`notes-core`)

## Purpose

The `notes-core` crate is the shared foundation for all other crates. It contains **only pure domain logic** — no IO, no HTTP, no filesystem, no database. It must compile without any runtime dependencies.

## Responsibilities

- Define canonical domain types (`Note`, `NoteMeta`, `User`, `Vault`, `Team`)
- Define abstract traits (`StorageBackend`, `AuthStore`)
- Frontmatter parsing helpers
- Search/filter helpers (later)

## Domain Types

```rust
/// Controls who (beyond the owner) can interact with a note.
/// The owner always retains full access regardless of this value.
pub enum NotePermission {
    /// Private. Only the owner can view, edit, or comment.
    Restricted,
    /// Any authenticated user can view and add comments, but cannot edit.
    Comment,
    /// Any authenticated user can view, comment, and edit.
    Edit,
}

pub struct NoteMeta {
    pub id: String,           // globally unique slug or UUID; used as filename stem
    pub vault_id: String,     // which vault this note belongs to
    pub title: String,
    pub description: Option<String>,  // distilled from the first actual Markdown paragraph on save
    pub author_id: String,
    pub tags: Vec<String>,
    pub category: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_encrypted: bool,   // reserved for future E2EE
    pub permission: NotePermission,
}

pub struct Vault {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub owner: VaultOwner,
    pub permission: NotePermission,
    pub icon: Option<String>,   // optional curated icon slug (e.g. "briefcase"); None = no icon
}

/// Used by `StorageBackend::update_vault` for partial vault updates.
pub struct VaultUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<VaultIconUpdate>,
}

pub enum VaultIconUpdate {
    Set(String),   // set to the given slug
    Clear,         // remove the icon field
}

pub struct Note {
    pub meta: NoteMeta,
    /// Raw body. In non-E2EE mode: plaintext Markdown.
    /// In E2EE mode: opaque ciphertext — the server never parses this.
    pub content: String,
}

pub struct CommentAnchor {
    pub text: String,
    pub prefix: String,
    pub suffix: String,
    pub range_from: u32,
    pub range_to: u32,
    pub attachment: CommentAttachment,
    pub line_preview: String,
    pub last_matched_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub enum CommentAttachment {
    Attached,
    Detached,
}

pub enum CommentVisibility {
    Visible,
    HiddenLegacy,
}

pub struct CommentReply {
    pub id: String,
    pub author_id: String,
    pub author_name: String,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct Comment {
    pub id: String,
    pub note_id: String,
    pub author_id: String,
    pub author_name: String,
    pub body: String,
    pub anchor: CommentAnchor,
    pub resolved: bool,
    pub visibility: CommentVisibility,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub replies: Vec<CommentReply>,
}
```

### Permission Matrix

| Action | Owner | `edit` | `comment` | `restricted` |
|---|---|---|---|---|
| View note | ✓ | ✓ | ✓ | ✗ |
| Edit note | ✓ | ✓ | ✗ | ✗ |
| Add comment | ✓ | ✓ | ✓ | ✗ |
| Delete note | ✓ | ✗ | ✗ | ✗ |
| Change permission | ✓ | ✗ | ✗ | ✗ |

> "Any authenticated user" means any user who can successfully present a valid token for the configured auth mode. Specific per-user sharing (an allowlist) is a future feature.

## Storage Trait

Implementations live in separate crates (e.g. `notes-storage-fs`). The backend must not assume `content` is plaintext. Notes are vault-scoped; the `vault_id` in `NoteMeta` determines the storage path.

```rust
pub trait StorageBackend: Send + Sync {
    // Vault management
    fn list_vaults(&self, owner: &VaultOwner) -> Result<Vec<Vault>, StorageError>;
    fn load_vault(&self, vault_id: &str) -> Result<Vault, StorageError>;
    fn create_vault(&self, vault: &Vault) -> Result<(), StorageError>;
    fn delete_vault(&self, vault_id: &str) -> Result<(), StorageError>;
    fn update_vault_permission(&self, vault_id: &str, permission: NotePermission) -> Result<(), StorageError>;
    fn update_vault(&self, vault_id: &str, update: &VaultUpdate) -> Result<(), StorageError>;

    // Team management
    fn load_team(&self, team_id: &str) -> Result<Team, StorageError>;
    fn save_team(&self, team: &Team) -> Result<(), StorageError>;
    fn delete_team(&self, team_id: &str) -> Result<(), StorageError>;
    fn list_teams_for_user(&self, user_id: &str) -> Result<Vec<Team>, StorageError>;

    // Notes (vault-scoped; vault_id is always required)
    fn list_notes(&self, vault_id: &str) -> Result<Vec<NoteMeta>, StorageError>;
    fn load_note(&self, vault_id: &str, id: &str) -> Result<Note, StorageError>;
    fn save_note(&self, note: &Note) -> Result<(), StorageError>;   // vault_id taken from note.meta
    fn delete_note(&self, vault_id: &str, id: &str) -> Result<(), StorageError>;

    // Comment sidecars (vault-scoped; visible + hidden legacy records)
    fn load_comments(&self, vault_id: &str, note_id: &str) -> Result<Vec<Comment>, StorageError>;
    fn save_comments(&self, vault_id: &str, note_id: &str, comments: &[Comment]) -> Result<(), StorageError>;
}
```

Comment visibility is a domain concern, not a frontend-only flag:

- `Visible` comments appear in the default line-discussion UI and map to editor annotations
- `HiddenLegacy` comments preserve older note-level history that lacks a reliable structured anchor

See [vaults-and-teams.md](./vaults-and-teams.md) for the full vault and team model.

## Auth Trait

```rust
pub struct User {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub role: ServerRole,
}

pub struct ManagedUserSummary {
    pub id: String,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub role: ServerRole,
    pub can_edit: bool,
    pub can_delete: bool,
}

pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub role: ServerRole,
    pub password: String,
}

pub struct UpdateUserInput {
    pub email: String,
    pub display_name: String,
    pub role: ServerRole,
    pub password: Option<String>,
}

/// Returned by `AuthStore::login` on success.
pub struct LoginToken {
    pub token: String,
    pub expires_in: u64,  // seconds
}

pub trait AuthStore: Send + Sync {
    fn find_user(&self, user_id: &str) -> Result<Option<User>, AuthError>;
    fn find_user_by_username(&self, username: &str) -> Result<Option<User>, AuthError>;
    fn verify_token(&self, token: &str) -> Result<User, AuthError>;

    /// Authenticate with username + password. Only `secret_key` mode implements this.
    /// Default returns `AuthError::ServiceError("login not supported")`.
    fn login(&self, username: &str, password: &str) -> Result<LoginToken, AuthError> { ... }

    /// List all managed users visible to an administrator. MVP support is `secret_key` only.
    fn list_users(&self, actor: &User) -> Result<Vec<ManagedUserSummary>, AuthError> { ... }

    /// Create a managed user. MVP support is `secret_key` only.
    fn create_user(&self, actor: &User, input: CreateUserInput) -> Result<ManagedUserSummary, AuthError> { ... }

    /// Update a managed user. MVP support is `secret_key` only.
    fn update_user(&self, actor: &User, user_id: &str, input: UpdateUserInput) -> Result<ManagedUserSummary, AuthError> { ... }

    /// Delete a managed user. MVP support is `secret_key` only.
    fn delete_user(&self, actor: &User, user_id: &str) -> Result<(), AuthError> { ... }
}
```

> For `oidc` mode, `verify_token` validates a JWT issued by the configured OIDC provider. For `secret_key` mode, it validates an HMAC-signed JWT issued by the server itself. Passwords are never stored by this app in `oidc` mode.

## Error Types

```rust
pub enum StorageError {
    NotFound,
    PermissionDenied,
    VaultNotEmpty,
    IoError(std::io::Error),
    ParseError(String),
}

pub enum AuthError {
    InvalidToken,
    UserNotFound,
    InvalidCredentials,
    ServiceError(String),
}
```

## Non-Goals

- No IO of any kind
- No HTTP or network code
- No filesystem or database access
- No framework dependencies (Axum, Tokio are not imported here)
