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

#[derive(Debug, Clone, Default)]
pub struct SyncResult {
    pub homes_scanned: usize,
    pub vaults_fixed: usize,
    pub notes_fixed: usize,
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

    // Sync: scan all homes on disk, resolve each home slug to the canonical
    // user ID, and rewrite vault/note author_ids to that ID.
    fn sync_all_homes_author_id(
        &self,
        resolve_user_id: &dyn Fn(&str) -> Option<String>,
    ) -> Result<SyncResult, StorageError>;

    // Destructive cleanup operations (admin-only, no undo)
    fn remove_all_notes(&self) -> Result<usize, StorageError>;
    fn remove_all_vaults(&self) -> Result<usize, StorageError>;
    fn remove_all_homes(&self) -> Result<usize, StorageError>;
    fn remove_all_server_vaults(&self) -> Result<usize, StorageError>;
}
