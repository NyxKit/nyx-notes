/// Serde types for `.vault.json` and `.team.json` on disk.
/// These are separate from the domain types in `notes-core` so that
/// the on-disk format can evolve without changing the public API.
use notes_core::{NotePermission, TeamMember};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultJson {
    pub id: String,
    pub name: String,
    pub slug: String,
    /// Present only in team vaults. Absent for personal vaults (always restricted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<NotePermission>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamJson {
    pub id: String,
    pub name: String,
    pub members: Vec<TeamMember>,
}
