use thiserror::Error;

use crate::domain::{Comment, Note, NoteMeta, NotePermission, Team, Vault, VaultOwner};

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("not found")]
    NotFound,
    #[error("permission denied")]
    PermissionDenied,
    #[error("vault is not empty")]
    VaultNotEmpty,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("parse error: {0}")]
    ParseError(String),
}

pub trait StorageBackend: Send + Sync {
    // Vault management
    fn list_vaults(&self, owner: &VaultOwner) -> Result<Vec<Vault>, StorageError>;
    fn load_vault(&self, vault_id: &str) -> Result<Vault, StorageError>;
    fn create_vault(&self, vault: &Vault) -> Result<(), StorageError>;
    fn delete_vault(&self, vault_id: &str) -> Result<(), StorageError>;
    fn update_vault_permission(
        &self,
        vault_id: &str,
        permission: NotePermission,
    ) -> Result<(), StorageError>;

    // Team management
    fn load_team(&self, team_id: &str) -> Result<Team, StorageError>;
    fn save_team(&self, team: &Team) -> Result<(), StorageError>;
    fn delete_team(&self, team_id: &str) -> Result<(), StorageError>;
    fn list_teams_for_user(&self, user_id: &str) -> Result<Vec<Team>, StorageError>;

    // Notes (vault-scoped; vault_id is always required)
    fn list_notes(&self, vault_id: &str) -> Result<Vec<NoteMeta>, StorageError>;
    fn load_note(&self, vault_id: &str, id: &str) -> Result<Note, StorageError>;
    /// vault_id is taken from note.meta.vault_id
    fn save_note(&self, note: &Note) -> Result<(), StorageError>;
    fn delete_note(&self, vault_id: &str, id: &str) -> Result<(), StorageError>;

    // Comments (note-scoped sidecar; stored as `<note_id>.comments.json` in the vault dir)
    fn load_comments(&self, vault_id: &str, note_id: &str) -> Result<Vec<Comment>, StorageError>;
    fn save_comments(
        &self,
        vault_id: &str,
        note_id: &str,
        comments: &[Comment],
    ) -> Result<(), StorageError>;
}
