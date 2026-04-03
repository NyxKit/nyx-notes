# Filesystem Storage (`notes-storage-fs`)

## Purpose

Implements `StorageBackend` from `notes-core` using the local filesystem. The filesystem remains the source of truth for note content, vault metadata, and comment sidecars.

The authoritative namespace layout is defined in [file-system.md](./file-system.md). This document describes how `FsStorage` realizes that layout in the MVP.

## MVP Layout

```text
$NOTES_ROOT/
  <server-slug>/
    .server.json
    homes/
      <home-slug>/
        .home.json
        <vault-slug>/
          .vault.json
          <note-id>.md
          <note-id>.comments.json
    vaults/
      <vault-slug>/
        .vault.json
        <note-id>.md
        <note-id>.comments.json
  local/
    .local.json
```

- Personal vaults live under `<server-slug>/homes/<home-slug>/`
- Shared server vaults live under `<server-slug>/vaults/`
- `local/` is reserved in the on-disk contract but is not runtime-active in the MVP

## Runtime Resolution

`FsStorage` derives the active server namespace from `SERVER_NAME` by slugifying it at startup.

- `SERVER_NAME="Main Server"` -> `main-server`
- `NOTES_USER_ID` is used as the active home slug in the MVP

This means the CLI and server both resolve a caller's personal vaults under:

```text
$NOTES_ROOT/<server-slug>/homes/<NOTES_USER_ID>/
```

## Metadata Files

### `.server.json`

Stored at `<server-slug>/.server.json`.

Used for:

- stable internal server ID
- slug verification
- display name
- supported server roles (`admin`, `user`)

### `.home.json`

Stored at `<server-slug>/homes/<home-slug>/.home.json`.

Used for:

- stable internal home ID
- slug verification
- display name
- `owner_user_id` lookup

### `.vault.json`

Stored in every vault directory.

Used for:

- stable internal vault ID
- vault slug and name
- owner metadata (`home`, `server`, or `local`)
- vault kind (`personal`, `server`, `local`)
- default note permission for notes created inside the vault
- optional description and icon

## Note File Format

Each note is a Markdown file with YAML frontmatter:

```markdown
---
id: "project-plan"
title: "Project Plan"
description: "The first actual paragraph of the note body."
author_id: "local"
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

- `id` must match the filename stem
- Note IDs are stable identifiers and the canonical route/file identifiers
- `vault_id` is derived by the server from the containing vault directory and returned in API responses
- `created_at` is set on first write and never updated
- `updated_at` is updated on every save
- `description` is derived from the first actual Markdown paragraph
- If `is_encrypted: true`, the body below `---` is opaque ciphertext
- `permission` must be one of `restricted`, `comment`, or `edit`

## `FsStorage` Behavior

### Vault resolution

Methods that accept `vault_id: &str` resolve the vault by vault slug in the active MVP contract. Storage may still accept stable vault IDs internally for compatibility.

### `list_vaults(owner)`

- For `VaultOwner::Home`, list vaults inside the caller's home directory
- For `VaultOwner::Server`, list shared server vaults
- For `VaultOwner::Local`, list local vaults when that namespace becomes runtime-active

### `load_vault(vault_id)`

- Resolve the vault directory by vault slug (with optional metadata-ID fallback)
- Read `.vault.json`
- Derive `VaultOwner` from the resolved path and metadata

### `create_vault(vault)`

- Ensure `.server.json` exists for the active server namespace
- Ensure `.home.json` exists for personal vault owners
- Create the vault directory
- Write `.vault.json`

### `delete_vault(vault_id)`

- Resolve the vault directory
- Return `StorageError::VaultNotEmpty` if any `.md` files exist
- Remove the vault directory and its metadata file

### `list_notes(vault_id)`

- Resolve the vault directory
- Walk the vault directory for `*.md` files (non-recursive)
- Parse frontmatter only
- Inject `vault_id` from the containing vault directory rather than trusting note frontmatter
- Return `Vec<NoteMeta>` sorted by `updated_at` descending

### `load_note(vault_id, id)`

- Resolve the vault directory
- Read `<vault-dir>/<id>.md`
- Parse frontmatter plus Markdown body
- Inject `vault_id` from the containing vault directory rather than trusting note frontmatter

### `save_note(note)`

- Resolve the vault directory from `note.meta.vault_id`
- Serialize YAML frontmatter and body without persisting `vault_id` to disk
- Write `<vault-dir>/<id>.md`

### `delete_note(vault_id, id)`

- Remove `<vault-dir>/<id>.md`
- Remove `<vault-dir>/<id>.comments.json` if it exists

## Comment Sidecars

Each note may have an optional sidecar named `<id>.comments.json`.

- Structured anchors are preserved
- Legacy quote-only sidecars remain readable
- Hidden legacy comments stay stored but are omitted from the default line-discussion UI

## Configuration

| Env var | Default | Description |
|---|---|---|
| `NOTES_ROOT` | `./notes` | Root directory for notes and metadata |
| `SERVER_NAME` | `Main Server` | Human-facing name used to derive `<server-slug>` |
| `NOTES_USER_ID` | `local` | Active user ID and home slug in the MVP |

## Non-Goals

- No HTTP or network code
- No permission enforcement in storage
- No team metadata in the MVP storage contract
- No active runtime support for `local/` yet
