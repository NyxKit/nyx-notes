use notes_core::{slugify, Note, StorageBackend, StorageError, Vault, VaultOwner};
use notes_storage_fs::FsStorage;

/// Resolve a vault specifier to a `Vault`.
///
/// Specifier formats:
/// - `"home"` — personal vault with slug "home"
/// - `"work"` — personal vault with slug "work"
/// - `"server:handbook"` — shared server vault
pub fn resolve_vault(storage: &FsStorage, user_id: &str, specifier: &str) -> anyhow::Result<Vault> {
    let server_slug =
        slugify(&std::env::var("SERVER_NAME").unwrap_or_else(|_| "Main Server".into()));

    if let Some(slug) = specifier.strip_prefix("server:") {
        let vaults = storage.list_vaults(&VaultOwner::Server { server_slug })?;
        vaults
            .into_iter()
            .find(|v| v.slug == slug)
            .ok_or_else(|| anyhow::anyhow!("shared server vault '{slug}' not found"))
    } else {
        let vaults = storage.list_vaults(&VaultOwner::Home {
            server_slug,
            home_slug: user_id.to_string(),
        })?;
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
    let server_slug =
        slugify(&std::env::var("SERVER_NAME").unwrap_or_else(|_| "Main Server".into()));

    // Personal vaults
    for vault in storage.list_vaults(&VaultOwner::Home {
        server_slug: server_slug.clone(),
        home_slug: user_id.to_string(),
    })? {
        match storage.load_note(&vault.slug, note_id) {
            Ok(note) => return Ok((vault, note)),
            Err(StorageError::NotFound) => continue,
            Err(e) => return Err(anyhow::Error::from(e)),
        }
    }

    for vault in storage.list_vaults(&VaultOwner::Server { server_slug })? {
        match storage.load_note(&vault.slug, note_id) {
            Ok(note) => return Ok((vault, note)),
            Err(StorageError::NotFound) => continue,
            Err(e) => return Err(anyhow::Error::from(e)),
        }
    }

    anyhow::bail!("note '{note_id}' not found in any accessible vault")
}
