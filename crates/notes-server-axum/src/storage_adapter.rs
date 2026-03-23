use std::sync::Arc;

use notes_core::{
    Comment, Note, NoteMeta, NotePermission, StorageBackend, StorageError, Team, Vault, VaultOwner,
    VaultUpdate,
};

/// Async wrapper around `Arc<dyn StorageBackend>`.
///
/// `StorageBackend` is a synchronous trait. This adapter bridges the sync trait to
/// async Axum handlers by running each storage call on Tokio's blocking thread pool
/// via `tokio::task::spawn_blocking`. This keeps `notes-core` free of async
/// dependencies and the CLI usable without a runtime.
#[derive(Clone)]
pub struct AsyncStorageAdapter(Arc<dyn StorageBackend>);

impl AsyncStorageAdapter {
    pub fn new(storage: Arc<dyn StorageBackend>) -> Self {
        Self(storage)
    }

    fn wrap_join_err(e: tokio::task::JoinError) -> StorageError {
        StorageError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    // --- Vaults ---

    pub async fn list_vaults(&self, owner: VaultOwner) -> Result<Vec<Vault>, StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.list_vaults(&owner))
            .await
            .map_err(Self::wrap_join_err)?
    }

    pub async fn load_vault(&self, vault_id: String) -> Result<Vault, StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.load_vault(&vault_id))
            .await
            .map_err(Self::wrap_join_err)?
    }

    pub async fn create_vault(&self, vault: Vault) -> Result<(), StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.create_vault(&vault))
            .await
            .map_err(Self::wrap_join_err)?
    }

    pub async fn delete_vault(&self, vault_id: String) -> Result<(), StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.delete_vault(&vault_id))
            .await
            .map_err(Self::wrap_join_err)?
    }

    pub async fn update_vault_permission(
        &self,
        vault_id: String,
        permission: NotePermission,
    ) -> Result<(), StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.update_vault_permission(&vault_id, permission))
            .await
            .map_err(Self::wrap_join_err)?
    }

    pub async fn update_vault(&self, vault_id: String, update: VaultUpdate) -> Result<(), StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.update_vault(&vault_id, &update))
            .await
            .map_err(Self::wrap_join_err)?
    }

    // --- Teams ---

    pub async fn load_team(&self, team_id: String) -> Result<Team, StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.load_team(&team_id))
            .await
            .map_err(Self::wrap_join_err)?
    }

    pub async fn save_team(&self, team: Team) -> Result<(), StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.save_team(&team))
            .await
            .map_err(Self::wrap_join_err)?
    }

    pub async fn list_teams_for_user(&self, user_id: String) -> Result<Vec<Team>, StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.list_teams_for_user(&user_id))
            .await
            .map_err(Self::wrap_join_err)?
    }

    pub async fn delete_team(&self, team_id: String) -> Result<(), StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.delete_team(&team_id))
            .await
            .map_err(Self::wrap_join_err)?
    }

    // --- Notes ---

    pub async fn list_notes(&self, vault_id: String) -> Result<Vec<NoteMeta>, StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.list_notes(&vault_id))
            .await
            .map_err(Self::wrap_join_err)?
    }

    pub async fn load_note(&self, vault_id: String, id: String) -> Result<Note, StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.load_note(&vault_id, &id))
            .await
            .map_err(Self::wrap_join_err)?
    }

    pub async fn save_note(&self, note: Note) -> Result<(), StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.save_note(&note))
            .await
            .map_err(Self::wrap_join_err)?
    }

    pub async fn delete_note(&self, vault_id: String, id: String) -> Result<(), StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.delete_note(&vault_id, &id))
            .await
            .map_err(Self::wrap_join_err)?
    }

    // --- Comments ---

    pub async fn load_comments(
        &self,
        vault_id: String,
        note_id: String,
    ) -> Result<Vec<Comment>, StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.load_comments(&vault_id, &note_id))
            .await
            .map_err(Self::wrap_join_err)?
    }

    pub async fn save_comments(
        &self,
        vault_id: String,
        note_id: String,
        comments: Vec<Comment>,
    ) -> Result<(), StorageError> {
        let s = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || s.save_comments(&vault_id, &note_id, &comments))
            .await
            .map_err(Self::wrap_join_err)?
    }
}
