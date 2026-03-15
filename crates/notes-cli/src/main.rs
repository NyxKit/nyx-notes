mod cmd;
mod config;
mod context;
mod output;

use clap::{Parser, Subcommand};
use config::CliConfig;
use notes_storage_fs::FsStorage;

#[derive(Parser)]
#[command(name = "notes", about = "Nyx Notes — terminal interface")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List notes in the active vault
    List {
        #[arg(long)]
        vault: Option<String>,
        #[arg(long, help = "Filter by tag")]
        tag: Option<String>,
        #[arg(long, help = "Filter by category")]
        category: Option<String>,
        #[arg(long, default_value = "updated", help = "Sort by: updated, created, title")]
        sort: String,
        #[arg(long, help = "Output as JSON")]
        json: bool,
    },
    /// Create a new note
    New {
        #[arg(long)]
        vault: Option<String>,
        #[arg(long)]
        title: Option<String>,
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,
        #[arg(long)]
        category: Option<String>,
        #[arg(long, help = "Open in $EDITOR after creation")]
        edit: bool,
    },
    /// Open a note in $EDITOR
    Edit {
        id: String,
        #[arg(long)]
        vault: Option<String>,
    },
    /// Print a note to stdout
    Show {
        id: String,
        #[arg(long, help = "Include YAML frontmatter")]
        raw: bool,
    },
    /// Delete a note
    Delete {
        id: String,
        #[arg(long, help = "Skip confirmation prompt")]
        force: bool,
    },
    /// Search notes by title (or body with --body)
    Search {
        query: String,
        #[arg(long)]
        vault: Option<String>,
        #[arg(long, help = "Filter results by tag")]
        tag: Option<String>,
        #[arg(long, help = "Also search note body content")]
        body: bool,
    },
    /// List all tags in the active vault
    Tags {
        #[arg(long)]
        vault: Option<String>,
    },
    /// Manage vaults
    Vault {
        #[command(subcommand)]
        cmd: VaultCommand,
    },
    /// Manage teams
    Team {
        #[command(subcommand)]
        cmd: TeamCommand,
    },
}

#[derive(Subcommand)]
enum VaultCommand {
    /// List all accessible vaults
    List,
    /// Create a new personal vault
    New {
        #[arg(long)]
        slug: String,
        #[arg(long)]
        name: String,
    },
    /// Delete a personal vault (must be empty)
    Delete { slug: String },
}

#[derive(Subcommand)]
enum TeamCommand {
    /// List all teams you belong to
    List,
    /// List members of a team
    Members { team_id: String },
}

fn main() {
    let cli = Cli::parse();
    let config = CliConfig::load();

    std::fs::create_dir_all(config.notes_root_path())
        .expect("failed to create NOTES_ROOT directory");

    let storage = FsStorage::new(config.notes_root_path());

    if let Err(e) = cmd::run(cli, &config, &storage) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
