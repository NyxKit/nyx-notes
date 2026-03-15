use notes_core::{Note, StorageBackend, StorageError, Vault, VaultOwner};
use notes_storage_fs::FsStorage;

/// Resolve a vault specifier to a `Vault`.
///
/// Specifier formats:
/// - `"home"` — personal vault with slug "home"
/// - `"work"` — personal vault with slug "work"
/// - `"team:<team-id>/home"` — team vault
pub fn resolve_vault(
    storage: &FsStorage,
    user_id: &str,
    specifier: &str,
) -> anyhow::Result<Vault> {
    if let Some(team_spec) = specifier.strip_prefix("team:") {
        let (team_id, slug) = team_spec
            .split_once('/')
            .ok_or_else(|| anyhow::anyhow!("team vault format must be 'team:<id>/<slug>'"))?;
        let vaults = storage.list_vaults(&VaultOwner::Team(team_id.to_string()))?;
        vaults
            .into_iter()
            .find(|v| v.slug == slug)
            .ok_or_else(|| anyhow::anyhow!("vault '{slug}' not found in team '{team_id}'"))
    } else {
        let vaults = storage.list_vaults(&VaultOwner::User(user_id.to_string()))?;
        vaults
            .into_iter()
            .find(|v| v.slug == specifier)
            .ok_or_else(|| anyhow::anyhow!("vault '{specifier}' not found"))
    }
}

/// Find a note by ID across all vaults accessible to the user.
/// Used by commands that don't take a `--vault` flag (`show`, `delete`).
pub fn find_note(
    storage: &FsStorage,
    user_id: &str,
    note_id: &str,
) -> anyhow::Result<(Vault, Note)> {
    // Personal vaults
    for vault in storage.list_vaults(&VaultOwner::User(user_id.to_string()))? {
        match storage.load_note(&vault.id, note_id) {
            Ok(note) => return Ok((vault, note)),
            Err(StorageError::NotFound) => continue,
            Err(e) => return Err(anyhow::Error::from(e)),
        }
    }
    // Team vaults
    for team in storage.list_teams_for_user(user_id)? {
        for vault in storage.list_vaults(&VaultOwner::Team(team.id))? {
            match storage.load_note(&vault.id, note_id) {
                Ok(note) => return Ok((vault, note)),
                Err(StorageError::NotFound) => continue,
                Err(e) => return Err(anyhow::Error::from(e)),
            }
        }
    }
    anyhow::bail!("note '{note_id}' not found in any accessible vault")
}
