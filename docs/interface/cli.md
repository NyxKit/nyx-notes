# CLI (`notes-cli`)

## Purpose

A terminal-first interface that works directly against the same filesystem layout used by the backend server.

## Vault Context

The active vault is resolved in order:

1. `--vault <SPEC>`
2. `NOTES_VAULT`
3. `vault` in `~/.config/nyx-notes/config.toml`
4. fallback: `home`

Vault specifiers:

- `home` or `journal` -> personal vault in the caller's home
- `server:handbook` -> shared server vault

The active home is resolved from `NOTES_USER_ID`.

## Commands

```text
notes <COMMAND> [OPTIONS]

Commands:
  new
  edit
  list
  show
  delete
  search
  tags
  vault
```

## Behavior

### `notes new`

- Generates a stable note ID at creation time
- Leaves the note ID stable when the title changes later
- Uses the vault's default note permission

### `notes vault list`

- Lists personal home vaults first
- Also lists shared server vaults accessible from the active server namespace

### `notes vault new --slug <SLUG> --name <NAME>`

- Creates a personal vault in the caller's home namespace

## Configuration

| Setting | Env var | Default |
|---|---|---|
| Notes root | `NOTES_ROOT` | `~/notes` |
| User ID | `NOTES_USER_ID` | `default` |
| Server name | `SERVER_NAME` | `Main Server` |
| Active vault | `NOTES_VAULT` | `home` |
| Editor | `EDITOR` | `vi` |

Example config:

```toml
notes_root = "/mnt/data/notes"
user_id = "alice"
server_name = "Main Server"
vault = "home"
```

## Non-Goals

- No HTTP calls to the backend server
- No team-management command surface in the MVP
- No TUI
