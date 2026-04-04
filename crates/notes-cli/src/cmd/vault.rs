use notes_core::{slugify, NotePermission, StorageBackend, Vault, VaultOwner};
use notes_storage_fs::FsStorage;
use uuid::Uuid;

use crate::context::resolve_vault;

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

pub fn list(storage: &FsStorage, user_id: &str) -> anyhow::Result<()> {
    let home_owner = user_home_owner(user_id);
    let mut vaults = storage.list_vaults(&home_owner)?;
    vaults.extend(storage.list_vaults(&server_owner())?);

    if vaults.is_empty() {
        println!("(no vaults)");
        return Ok(());
    }

    for v in &vaults {
        let owner_label: &str = match &v.owner {
            VaultOwner::Home { .. } => "personal",
            VaultOwner::Server { .. } => "server",
            VaultOwner::Local => "local",
        };
        println!("{:<20} {:<16} ({})", v.slug, v.name, owner_label);
    }

    Ok(())
}

pub fn new(storage: &FsStorage, user_id: &str, slug: String, name: String) -> anyhow::Result<()> {
    let vault = Vault {
        id: Uuid::new_v4().to_string(),
        slug,
        name,
        description: None,
        owner: user_home_owner(user_id),
        permission: NotePermission::Restricted,
        icon: None,
    };
    storage.create_vault(&vault)?;
    println!("Created vault: {}", vault.slug);
    Ok(())
}

pub fn delete(storage: &FsStorage, user_id: &str, slug: String) -> anyhow::Result<()> {
    let vault = resolve_vault(storage, user_id, &slug)?;

    let confirmed = dialoguer::Confirm::new()
        .with_prompt(format!(
            "Delete vault '{}'? This cannot be undone.",
            vault.slug
        ))
        .default(false)
        .interact()?;

    if !confirmed {
        println!("Aborted.");
        return Ok(());
    }

    storage.delete_vault(&vault.owner, &vault.slug)?;
    println!("Deleted vault: {slug}");
    Ok(())
}
