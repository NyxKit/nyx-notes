pub mod notes;
pub mod team;
pub mod vault;

use notes_storage_fs::FsStorage;

use crate::{
    config::CliConfig,
    context::{find_note, resolve_vault},
    Cli, Command, TeamCommand, VaultCommand,
};

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
            let v = resolve_vault(storage, &config.user_id, &vault.unwrap_or_else(default_vault))?;
            notes::list(storage, &v.id, tag.as_deref(), category.as_deref(), &sort, json)
        }

        Command::New {
            vault,
            title,
            tags,
            category,
            edit,
        } => {
            let v = resolve_vault(storage, &config.user_id, &vault.unwrap_or_else(default_vault))?;
            notes::new(
                storage,
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
            let v = resolve_vault(storage, &config.user_id, &vault.unwrap_or_else(default_vault))?;
            notes::edit(storage, &v.id, &id, &config.editor)
        }

        Command::Show { id, raw } => {
            let (_, note) = find_note(storage, &config.user_id, &id)?;
            notes::show(&note, raw)
        }

        Command::Delete { id, force } => {
            let (vault, _) = find_note(storage, &config.user_id, &id)?;
            notes::delete(storage, &vault.id, &id, force)
        }

        Command::Search {
            query,
            vault,
            tag,
            body,
        } => {
            let v = resolve_vault(storage, &config.user_id, &vault.unwrap_or_else(default_vault))?;
            notes::search(storage, &v.id, &query, tag.as_deref(), body)
        }

        Command::Tags { vault } => {
            let v = resolve_vault(storage, &config.user_id, &vault.unwrap_or_else(default_vault))?;
            notes::tags(storage, &v.id)
        }

        Command::Vault { cmd } => match cmd {
            VaultCommand::List => vault::list(storage, &config.user_id),
            VaultCommand::New { slug, name } => vault::new(storage, &config.user_id, slug, name),
            VaultCommand::Delete { slug } => vault::delete(storage, &config.user_id, slug),
        },

        Command::Team { cmd } => match cmd {
            TeamCommand::List => team::list(storage, &config.user_id),
            TeamCommand::Members { team_id } => team::members(storage, &team_id),
        },
    }
}
