use notes_core::{NotePermission, StorageBackend, Vault, VaultOwner};
use notes_storage_fs::FsStorage;
use uuid::Uuid;

use crate::context::resolve_vault;

pub fn list(storage: &FsStorage, user_id: &str) -> anyhow::Result<()> {
    let mut vaults = storage.list_vaults(&VaultOwner::User(user_id.to_string()))?;

    let teams = storage.list_teams_for_user(user_id)?;
    for team in &teams {
        let team_vaults = storage.list_vaults(&VaultOwner::Team(team.id.clone()))?;
        vaults.extend(team_vaults);
    }

    if vaults.is_empty() {
        println!("(no vaults)");
        return Ok(());
    }

    for v in &vaults {
        let owner_label = match &v.owner {
            VaultOwner::User(_) => "personal".into(),
            VaultOwner::Team(id) => {
                let name = teams
                    .iter()
                    .find(|t| &t.id == id)
                    .map(|t| t.name.as_str())
                    .unwrap_or(id.as_str());
                format!("team: {name}")
            }
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
        owner: VaultOwner::User(user_id.to_string()),
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

    storage.delete_vault(&vault.id)?;
    println!("Deleted vault: {slug}");
    Ok(())
}
