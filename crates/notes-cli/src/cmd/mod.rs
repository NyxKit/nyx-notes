pub mod notes;
pub mod vault;

use notes_core::VaultOwner;
use notes_storage_fs::FsStorage;

use crate::{
    config::CliConfig,
    context::{find_note, resolve_vault},
    Cli, Command, VaultCommand,
};

fn vault_owner_from_vault(vault: &notes_core::Vault) -> VaultOwner {
    vault.owner.clone()
}

pub fn run(cli: Cli, config: &CliConfig, storage: &FsStorage) -> anyhow::Result<()> {
    let default_vault = || config.vault.clone();

    match cli.command {
        Command::List {
            vault,
            tag,
            category,
            sort,
            json,
        } => {
            let v = resolve_vault(
                storage,
                &config.user_id,
                &vault.unwrap_or_else(default_vault),
            )?;
            let owner = vault_owner_from_vault(&v);
            notes::list(
                storage,
                &owner,
                &v.slug,
                tag.as_deref(),
                category.as_deref(),
                &sort,
                json,
            )
        }

        Command::New {
            vault,
            title,
            tags,
            category,
            edit,
        } => {
            let v = resolve_vault(
                storage,
                &config.user_id,
                &vault.unwrap_or_else(default_vault),
            )?;
            let owner = vault_owner_from_vault(&v);
            notes::new(
                storage,
                &owner,
                &v,
                &config.user_id,
                title,
                tags,
                category,
                edit,
                &config.editor,
            )
        }

        Command::Edit { id, vault } => {
            let v = resolve_vault(
                storage,
                &config.user_id,
                &vault.unwrap_or_else(default_vault),
            )?;
            let owner = vault_owner_from_vault(&v);
            notes::edit(storage, &owner, &v.slug, &id, &config.editor)
        }

        Command::Show { id, raw } => {
            let (_, note) = find_note(storage, &config.user_id, &id)?;
            notes::show(&note, raw)
        }

        Command::Delete { id, force } => {
            let (vault, _) = find_note(storage, &config.user_id, &id)?;
            let owner = vault_owner_from_vault(&vault);
            notes::delete(storage, &owner, &vault.slug, &id, force)
        }

        Command::Search {
            query,
            vault,
            tag,
            body,
        } => {
            let v = resolve_vault(
                storage,
                &config.user_id,
                &vault.unwrap_or_else(default_vault),
            )?;
            let owner = vault_owner_from_vault(&v);
            notes::search(storage, &owner, &v.slug, &query, tag.as_deref(), body)
        }

        Command::Tags { vault } => {
            let v = resolve_vault(
                storage,
                &config.user_id,
                &vault.unwrap_or_else(default_vault),
            )?;
            let owner = vault_owner_from_vault(&v);
            notes::tags(storage, &owner, &v.slug)
        }

        Command::Vault { cmd } => match cmd {
            VaultCommand::List => vault::list(storage, &config.user_id),
            VaultCommand::New { slug, name } => vault::new(storage, &config.user_id, slug, name),
            VaultCommand::Delete { slug } => vault::delete(storage, &config.user_id, slug),
        },
    }
}
