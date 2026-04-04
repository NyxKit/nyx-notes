use notes_core::{slugify, Note, StorageBackend, StorageError, Vault, VaultOwner};
use notes_storage_fs::FsStorage;

fn current_server_slug() -> String {
    slugify(&std::env::var("SERVER_NAME").unwrap_or_else(|_| "Main Server".into()))
}

fn user_home_owner(user_id: &str) -> VaultOwner {
    VaultOwner::Home {
        server_slug: current_server_slug(),
        home_slug: user_id.to_string(),
    }
}

fn server_owner() -> VaultOwner {
    VaultOwner::Server {
        server_slug: current_server_slug(),
    }
}

/// Resolve a vault specifier to a `Vault`.
///
/// Specifier formats:
/// - `"home"` — personal vault with slug "home"
/// - `"work"` — personal vault with slug "work"
/// - `"server:handbook"` — shared server vault
pub fn resolve_vault(storage: &FsStorage, user_id: &str, specifier: &str) -> anyhow::Result<Vault> {
    if let Some(slug) = specifier.strip_prefix("server:") {
        let owner = server_owner();
        let vaults = storage.list_vaults(&owner)?;
        vaults
            .into_iter()
            .find(|v| v.slug == slug)
            .ok_or_else(|| anyhow::anyhow!("shared server vault '{slug}' not found"))
    } else {
        let owner = user_home_owner(user_id);
        let vaults = storage.list_vaults(&owner)?;
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
    let home_owner = user_home_owner(user_id);
    for vault in storage.list_vaults(&home_owner)? {
        match storage.load_note(&home_owner, &vault.slug, note_id) {
            Ok(note) => return Ok((vault, note)),
            Err(StorageError::NotFound) => continue,
            Err(e) => return Err(anyhow::Error::from(e)),
        }
    }

    let server_owner = server_owner();
    for vault in storage.list_vaults(&server_owner)? {
        match storage.load_note(&server_owner, &vault.slug, note_id) {
            Ok(note) => return Ok((vault, note)),
            Err(StorageError::NotFound) => continue,
            Err(e) => return Err(anyhow::Error::from(e)),
        }
    }

    anyhow::bail!("note '{note_id}' not found in any accessible vault")
}
