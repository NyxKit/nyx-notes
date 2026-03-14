# CLI (`notes-cli`)

## Purpose

A command-line interface for managing notes directly from the terminal. The CLI operates on the same filesystem that the backend serves, so it is always in sync — no server needs to be running.

## Design Principles

- **First-class, not an afterthought**: the CLI is a complete interface for power users
- **Direct filesystem access**: reads and writes `.md` files using `FsStorage` directly, no HTTP required
- **Single binary**: ships as a standalone executable

## Vault Context

All note commands operate within a vault. The active vault is resolved in order:

1. `--vault <VAULT_SLUG>` flag on the command
2. `NOTES_VAULT` environment variable
3. `vault` key in `~/.config/nyx-notes/config.toml`
4. Falls back to `home` (the default personal vault)

For team vaults, prefix the slug with the team id: `--vault team:<team-id>/home`.

## Command Structure

```
notes <COMMAND> [OPTIONS]

Commands:
  new       Create a new note
  edit      Open a note in $EDITOR
  list      List notes in the active vault
  show      Print a note to stdout
  delete    Delete a note
  search    Search notes by title or body
  tags      List all tags in the active vault
  vault     Manage vaults
  team      Manage teams
```

### `notes new`

```
notes new [--vault <VAULT>] [--title <TITLE>] [--tags <TAG,...>] [--category <CAT>] [--edit]
```

- Generates a new note with an auto-generated `id` (slug from title, or UUID)
- Sets `created_at` and `updated_at` to now
- Writes the file to the resolved vault directory
- If `--edit` is passed: opens the note in `$EDITOR` after creation
- If `--title` is omitted: prompts interactively

### `notes edit <ID>`

```
notes edit <ID> [--vault <VAULT>]
```

- Opens the note file in `$EDITOR`
- After the editor exits: updates `updated_at` in frontmatter

### `notes vault`

```
notes vault list
notes vault new --slug <SLUG> --name <NAME>
notes vault delete <SLUG>
```

- `list`: show all accessible vaults (personal and team)
- `new`: create a new personal vault
- `delete`: delete a personal vault (must be empty)

### `notes list`

```
notes list [--vault <VAULT>] [--tag <TAG>] [--category <CAT>] [--sort updated|created|title]
```

Output (default: table):
```
ID                   TITLE                    UPDATED
meeting-2026-03-14   Weekly sync              2 hours ago
rust-notes           Learning Rust            yesterday
```

Flags:
- `--json`: output as JSON array of `NoteMeta`
- `--tag <TAG>`: filter by tag
- `--category <CAT>`: filter by category

### `notes show <ID>`

```
notes show <ID> [--raw]
```

- Prints the note body (Markdown) to stdout
- `--raw`: include the YAML frontmatter header

### `notes delete <ID>`

```
notes delete <ID> [--force]
```

- Prompts for confirmation unless `--force` is passed
- Removes `<id>.md` from the filesystem

### `notes search <QUERY>`

```
notes search <QUERY> [--vault <VAULT>] [--tag <TAG>] [--body]
```

- Searches note titles by default
- `--body`: also search note content (full text scan of `.md` files)
- Outputs matching notes in the same format as `notes list`

### `notes tags`

```
notes tags [--vault <VAULT>]
```

Lists all unique tags within the active vault with a count.

### `notes team`

```
notes team list
notes team members <TEAM_ID>
```

- `list`: show all teams the configured user belongs to (reads `.team.json` files)
- `members <TEAM_ID>`: list members and their roles

## Configuration

The CLI reads configuration from (in order of precedence):

1. CLI flags
2. Environment variables
3. `~/.config/nyx-notes/config.toml`

| Setting | Env var | Default |
|---|---|---|
| Notes root | `NOTES_ROOT` | `~/notes` |
| User ID | `NOTES_USER_ID` | `"default"` |
| Active vault | `NOTES_VAULT` | `home` |
| Editor | `EDITOR` | `vi` |

Config file example:
```toml
notes_root = "/mnt/data/notes"
user_id = "default"
```

## Implementation

The CLI uses `FsStorage` directly — it is a thin command-line wrapper around the same storage code the server uses.

```rust
// notes-cli/src/main.rs
use clap::Parser;
use notes_core::StorageBackend;
use notes_storage_fs::FsStorage;

fn main() {
    let cli = Cli::parse();
    let storage = FsStorage::new(config.notes_root);

    match cli.command {
        Command::List { tag, category, sort } => cmd::list(&storage, tag, category, sort),
        Command::New { title, tags, category, edit } => cmd::new(&storage, ...),
        Command::Edit { id } => cmd::edit(&storage, id),
        Command::Show { id, raw } => cmd::show(&storage, id, raw),
        Command::Delete { id, force } => cmd::delete(&storage, id, force),
        Command::Search { query, body } => cmd::search(&storage, query, body),
        Command::Tags => cmd::tags(&storage),
    }
}
```

## Dependencies

- `clap` (derive) — argument parsing
- `notes-core`, `notes-storage-fs` — storage
- `comfy-table` — table output formatting
- `dialoguer` — interactive prompts (confirmation, input)
- `chrono` — relative time formatting

## Non-Goals

- No HTTP calls to the backend server
- No Firebase auth (operates in single-user mode via `NOTES_USER_ID`)
- No TUI (interactive terminal UI) — plain command invocations only
