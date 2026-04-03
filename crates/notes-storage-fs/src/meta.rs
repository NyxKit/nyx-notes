/// Serde types for namespace metadata on disk.
/// These are separate from the domain types in `notes-core` so that
/// the on-disk format can evolve without changing the public API.
use notes_core::{NotePermission, VaultOwner};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerJson {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeJson {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub owner_user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalJson {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultJson {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub kind: String,
    pub owner: VaultOwner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<NotePermission>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}
