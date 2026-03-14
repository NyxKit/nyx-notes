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
    pub author_id: String,
    pub tags: Vec<String>,
    pub category: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_encrypted: bool,   // reserved for future E2EE
    pub permission: NotePermission,
}

pub struct Note {
    pub meta: NoteMeta,
    /// Raw body. In non-E2EE mode: plaintext Markdown.
    /// In E2EE mode: opaque ciphertext — the server never parses this.
    pub content: String,
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

> "Any authenticated user" means any user who can successfully present a valid Firebase ID token. Specific per-user sharing (an allowlist) is a future feature.

## Storage Trait

Implementations live in separate crates (e.g. `notes-storage-fs`). The backend must not assume `content` is plaintext. Notes are vault-scoped; the `vault_id` in `NoteMeta` determines the storage path.

```rust
pub trait StorageBackend: Send + Sync {
    // Vault management
    fn list_vaults(&self, owner: &VaultOwner) -> Result<Vec<Vault>, StorageError>;
    fn create_vault(&self, vault: &Vault) -> Result<(), StorageError>;
    fn delete_vault(&self, vault_id: &str) -> Result<(), StorageError>;
    fn update_vault_permission(&self, vault_id: &str, permission: NotePermission) -> Result<(), StorageError>;

    // Team management
    fn load_team(&self, team_id: &str) -> Result<Team, StorageError>;
    fn save_team(&self, team: &Team) -> Result<(), StorageError>;
    fn delete_team(&self, team_id: &str) -> Result<(), StorageError>;

    // Notes (vault-scoped; vault_id is always required)
    fn list_notes(&self, vault_id: &str) -> Result<Vec<NoteMeta>, StorageError>;
    fn load_note(&self, vault_id: &str, id: &str) -> Result<Note, StorageError>;
    fn save_note(&self, note: &Note) -> Result<(), StorageError>;   // vault_id taken from note.meta
    fn delete_note(&self, vault_id: &str, id: &str) -> Result<(), StorageError>;
}
```

See [vaults-and-teams.md](./vaults-and-teams.md) for the full vault and team model.

## Auth Trait

```rust
pub struct User {
    pub id: String,
    pub email: String,
    pub display_name: String,
}

pub trait AuthStore: Send + Sync {
    fn find_user(&self, user_id: &str) -> Result<Option<User>, AuthError>;
    fn verify_token(&self, token: &str) -> Result<User, AuthError>;
}
```

> For Firebase auth, `verify_token` validates a Firebase ID token (JWT). Passwords are never stored by this app.

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
    ServiceError(String),
}
```

## Non-Goals

- No IO of any kind
- No HTTP or network code
- No filesystem or database access
- No framework dependencies (Axum, Tokio are not imported here)
