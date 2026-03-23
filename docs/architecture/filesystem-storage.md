# Filesystem Storage (`notes-storage-fs`)

## Purpose

Implements `StorageBackend` from `notes-core` using the local filesystem. The filesystem is the **source of truth** for all note content. Notes survive independently of any database, server, or cloud service.

## Principles

- Each note is a single `.md` file — human-readable, git-friendly, portable
- No proprietary binary format, no database required for content
- The directory layout is the organizational structure (folders = vaults)
- Notes can be edited directly with any text editor outside the app

## Directory Layout

Notes are organized by vault, and vaults are owned by either a user or a team. See [vaults-and-teams.md](./vaults-and-teams.md) for the full model.

```
$NOTES_ROOT/
  users/
    <uid>/                    # one directory per user
      home/                   # default personal vault (always exists)
        .vault.json           # vault metadata: id, name, slug[, icon] (no permission field)
        <slug>.md
      <vault-slug>/           # additional personal vaults
        .vault.json
        <slug>.md
  teams/
    <team-id>/
      .team.json              # team metadata: name, members, roles
      home/                   # default team vault (always exists)
        .vault.json           # vault metadata: id, name, slug, permission[, icon]
        <slug>.md
      <vault-slug>/
        .vault.json
        <slug>.md
```

## Note File Format

Each `.md` file uses YAML frontmatter:

```markdown
---
id: "my-note-slug"
vault_id: "vault-xyz456"
title: "My Note"
author_id: "user-123"
tags: ["rust", "backend"]
category: "work"
created_at: "2026-03-14T10:00:00Z"
updated_at: "2026-03-14T12:00:00Z"
is_encrypted: false
permission: "restricted"
---

Note body in **Markdown**.
```

### Rules

- `id` must match the filename stem (e.g. `my-note-slug.md`) and be globally unique
- `vault_id` identifies the containing vault; it must match the actual directory the file resides in
- `created_at` is set on first write and never updated
- `updated_at` is updated on every save
- If `is_encrypted: true`, the body below `---` is opaque ciphertext
- `permission` must be one of `"restricted"`, `"comment"`, `"edit"`; defaults to the vault's permission if absent

## Implementation

```rust
pub struct FsStorage {
    root: std::path::PathBuf,
}

impl FsStorage {
    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn vault_path(&self, vault: &Vault) -> PathBuf {
        match &vault.owner {
            VaultOwner::User(uid) => self.root.join("users").join(uid).join(&vault.slug),
            VaultOwner::Team(team_id) => self.root.join("teams").join(team_id).join(&vault.slug),
        }
    }

    fn note_path(&self, vault: &Vault, id: &str) -> PathBuf {
        self.vault_path(vault).join(format!("{id}.md"))
    }
}
```

### Vault ID Resolution

Methods that accept a bare `vault_id: &str` (such as `list_notes`, `load_note`, `delete_vault`) need to resolve that ID to a filesystem path. `FsStorage` does this by scanning all vault directories and reading their `.vault.json` files until a match is found.

This is intentionally simple: the expected number of vaults per installation is small (tens, not thousands), so a linear scan on each operation is acceptable. If this becomes a bottleneck, a startup index built into `FsStorage::new` can be added without changing the `StorageBackend` trait.

All vault directories — personal and team alike — carry a `.vault.json` file. This uniformity is what makes the scan possible. See the `.vault.json` format in [vaults-and-teams.md](./vaults-and-teams.md). Both personal and team `.vault.json` files may carry an optional `icon` field (a curated slug string). Missing `icon` is treated as `None`.

### `list_notes(vault_id)`

- Resolve the vault path from `vault_id`
- Walk the vault directory for `*.md` files (non-recursive — notes live flat in the vault dir)
- Parse only the frontmatter of each file (skip body for performance)
- Return `Vec<NoteMeta>` sorted by `updated_at` descending

### `load_note(vault_id, id)`

- Read `<vault_path>/<id>.md`
- Split on the second `---` delimiter
- Parse frontmatter as YAML into `NoteMeta`
- Return `Note { meta, content: body_string }`

### `save_note(note)`

- Derive vault path from `note.meta.vault_id`
- Serialize `NoteMeta` as YAML frontmatter
- Write `---\n{frontmatter}---\n{content}` to `<vault_path>/<id>.md`
- Create vault directory if it doesn't exist
- On create: use the provided `created_at`; on update: only update `updated_at`

### `delete_note(vault_id, id)`

- Remove `<vault_path>/<id>.md`
- Return `StorageError::NotFound` if the file doesn't exist

### `create_vault(vault)`

- Create the directory at `vault_path(vault)`
- For team vaults: write `.vault.json` with name and permission

### `delete_vault(vault_id)`

- Return `StorageError::VaultNotEmpty` if any `.md` files exist in the directory
- Remove the vault directory and its `.vault.json` if present

### `load_team(team_id)` / `save_team(team)`

- Read/write `teams/<team_id>/.team.json`
- `save_team` creates the team directory if it doesn't exist and writes the default `home/` vault

## Configuration

| Env var | Default | Description |
|---|---|---|
| `NOTES_ROOT` | `./notes` | Root directory for all notes |

## Dependencies

- `std::fs` for synchronous IO (no async runtime dependency; the server layer handles blocking via `tokio::task::spawn_blocking`)
- `serde` + `serde_yaml` for frontmatter (YAML)
- `serde_json` for `.vault.json` and `.team.json` (JSON)
- `walkdir` for recursive directory traversal

## Non-Goals

- No HTTP or network code
- No auth logic — caller is responsible for passing a valid `vault_id`
- No search indexing (that lives in the server layer)
