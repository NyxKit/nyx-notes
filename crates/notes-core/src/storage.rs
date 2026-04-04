use thiserror::Error;

use crate::domain::{Comment, Note, NoteMeta, NotePermission, Vault, VaultOwner, VaultUpdate};

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("not found")]
    NotFound,
    #[error("already exists")]
    AlreadyExists,
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
    fn load_vault(&self, owner: &VaultOwner, vault_id: &str) -> Result<Vault, StorageError>;
    fn create_vault(&self, vault: &Vault) -> Result<(), StorageError>;
    fn delete_vault(&self, owner: &VaultOwner, vault_id: &str) -> Result<(), StorageError>;
    fn update_vault_permission(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
        permission: NotePermission,
    ) -> Result<(), StorageError>;
    fn update_vault(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
        update: &VaultUpdate,
    ) -> Result<(), StorageError>;

    // Notes (vault-scoped; vault_id is always required)
    fn list_notes(&self, owner: &VaultOwner, vault_id: &str)
        -> Result<Vec<NoteMeta>, StorageError>;
    fn load_note(&self, owner: &VaultOwner, vault_id: &str, id: &str)
        -> Result<Note, StorageError>;
    /// vault_id is taken from note.meta.vault_id
    fn save_note(&self, owner: &VaultOwner, note: &Note) -> Result<(), StorageError>;
    fn rename_note(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
        old_id: &str,
        new_id: &str,
    ) -> Result<(), StorageError>;
    fn delete_note(&self, owner: &VaultOwner, vault_id: &str, id: &str)
        -> Result<(), StorageError>;

    // Comments (note-scoped sidecar; stored as `<note_id>.comments.json` in the vault dir)
    fn load_comments(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
        note_id: &str,
    ) -> Result<Vec<Comment>, StorageError>;
    fn save_comments(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
        note_id: &str,
        comments: &[Comment],
    ) -> Result<(), StorageError>;
}
