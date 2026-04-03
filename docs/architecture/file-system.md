# File System Draft

## Status

This document is a forward-looking filesystem draft for a future storage layout.

- MVP target: single server setup
- Future target: multiple server roots plus optional local synced vault copies
- Metadata files remain part of the design; this is not a metadata-free layout

Until the rest of `docs/architecture/` is updated to match, this document should be read as a draft proposal rather than the active canonical storage contract.

## Goals

- Keep server, home, and vault paths human-readable while using stable IDs for note storage
- Support a simple MVP with one server and two roles: `admin` and `user`
- Leave room for future multi-server expansion without redesigning the top-level tree
- Leave room for future per-vault local sync from a remote server
- Preserve metadata files for stable IDs, display names, permissions, and future extensibility

## Slug Rules

Users provide slugs for filesystem-facing entities.

- Server slugs become directory names under the root
- User home slugs become directory names under `homes/`
- Vault slugs become directory names under `vaults/` and under synced local copies
- Slugs must be filesystem-safe and URL-safe
- Slugs are immutable in the MVP; slug renames are not supported
- Server, home, and vault paths remain slug-based, while note paths use stable IDs to avoid title-change and collision edge cases

Recommended constraints:

- lowercase ASCII letters and numbers only
- `-` and `_` are the only allowed special characters
- no spaces
- no path separators
- no `.` or `..`
- maximum length: `64`
- must match: `^[a-z0-9_-]{1,64}$`

Uniqueness is scoped by parent:

- server slugs are unique at `$NYX_NOTES_ROOT/`
- home slugs are unique within a server
- personal vault slugs are unique within a home
- server vault slugs are unique within a server
- note IDs are unique within a vault

Reserved structural names such as `local`, `homes`, and `vaults` cannot be used where they would collide with layout directories.

The 64-character cap keeps paths readable and avoids unnecessary filesystem path growth.

## Top-Level Layout

```text
$NYX_NOTES_ROOT/
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
    <vault-slug>/
      .vault.json
      <note-id>.md
      <note-id>.comments.json
```

## Layout Meaning

### `<server-slug>/`

Represents one logical server namespace.

For the MVP, there is only one active server namespace in practice, but the layout reserves this level now so future multi-server support does not require a top-level rewrite.

### `homes/<home-slug>/`

Represents a personal area for one user on that server.

- Each user has exactly one home slug on a given server
- A home contains only that user's personal vaults
- Home slugs are user-provided and path-stable
- Username changes are not supported in the MVP
- Display name changes do not affect path layout
- Access is owner-scoped in the MVP

### `vaults/<vault-slug>/`

Represents a server-owned shared vault.

- These are not personal vaults
- In the MVP they are governed by server roles, not by teams
- `admin` users can create, update, and delete server vaults
- `user` users can view and edit notes in server vaults, but cannot create or delete vaults

### `local/`

Represents device-local storage.

For the MVP, this can be used for fully local vaults.

In a later iteration, it may also contain a locally synced copy of a vault selected from a remote server. That sync behavior is out of scope for the MVP, but the filesystem shape should not block it.

## Metadata Files

This draft keeps metadata files as first-class storage artifacts.

### `.server.json`

Stored at `<server-slug>/.server.json`.

Purpose:

- Human-facing server name
- The active user-facing label for the current server namespace
- Stable internal server ID if needed
- Server mode and future feature flags
- Future permission model versioning

Example:

```json
{
  "id": "server-main",
  "slug": "server-main",
  "name": "Main Server",
  "roles": ["admin", "user"]
}
```

### `.home.json`

Stored at `<server-slug>/homes/<home-slug>/.home.json`.

Purpose:

- Stable internal home ID if needed
- Display name for the home area
- Owning user reference
- Future user metadata without changing path layout

Example:

```json
{
  "id": "home-arne",
  "slug": "arnedecant",
  "name": "Arne's Home",
  "owner_user_id": "user-arne"
}
```

### `.vault.json`

Stored in every vault directory, including personal, server, and local vaults.

Purpose:

- Stable internal vault ID
- Display name
- Slug verification
- Vault kind
- Ownership and permission metadata
- Future sync metadata

Example personal vault:

```json
{
  "id": "vault-journal",
  "slug": "journal",
  "name": "Journal",
  "kind": "personal",
  "owner": {
    "type": "home",
    "slug": "arnedecant"
  }
}
```

Example server vault:

```json
{
  "id": "vault-handbook",
  "slug": "handbook",
  "name": "Handbook",
  "kind": "server",
  "owner": {
    "type": "server",
    "slug": "server-main"
  },
  "permissions": {
    "admin": ["view", "edit", "create_note", "delete_note", "create_vault", "delete_vault"],
    "user": ["view", "edit", "create_note", "delete_note"]
  }
}
```

Example future synced local vault:

```json
{
  "id": "vault-handbook-local",
  "slug": "handbook",
  "name": "Handbook",
  "kind": "local_sync",
  "sync": {
    "source_server": "server-main",
    "source_vault": "handbook"
  }
}
```

### `.local.json`

Stored at `local/.local.json`.

Purpose:

- Mark the `local/` namespace explicitly
- Store future client-local settings for sync bookkeeping
- Avoid baking client assumptions into directory names alone

Example:

```json
{
  "id": "local",
  "name": "Local Storage"
}
```

## Note Storage

Each note is stored as one Markdown file plus an optional comment sidecar.

```text
<vault-dir>/
  .vault.json
  project-plan.md
  project-plan.comments.json
```

- File names use stable note IDs
- Note IDs must be unique within their containing vault
- Notes remain flat within a vault directory in the MVP
- The `.md` file keeps frontmatter plus Markdown body
- The `.comments.json` sidecar remains alongside the note

Vault directory names remain slug-based and stable in the MVP. Note titles do not rename note files. A note's stable ID stays fixed after creation.

This draft does not change the note file format itself; it changes the surrounding directory model.

## MVP Semantics

The MVP intentionally keeps the permission model small.

### Roles

- `admin`
- `user`

### Personal home vaults

- Owned by the user
- Not shared
- Full control for the owner

### Server vaults

- Shared at the server level
- `admin` can create and delete vaults
- `user` cannot create or delete vaults
- `user` can view and edit notes in existing server vaults

This replaces team-based shared vaults for the MVP only. Team concepts can be introduced later without changing the top-level server-first layout.

## Future Extensions

### Multiple servers

Additional server namespaces can be added as siblings:

```text
$NYX_NOTES_ROOT/
  personal-server/
  work-server/
  local/
```

### Team-based permissions

Server vault permissions may later be backed by team membership instead of, or in addition to, the MVP's `admin` / `user` roles.

This can be added by expanding `.server.json` and `.vault.json` without changing directory names.

### Local sync of remote vaults

A later flow may allow a user to select:

`sync this vault from this server, locally`

The expected shape is a local vault entry that keeps sync metadata pointing back to the source server and source vault. Conflict handling, sync direction, and offline behavior are intentionally unspecified here.

Slug collisions between local-only vaults and future synced vault copies are out of scope for the MVP and will be defined with the sync model.

## Open Questions

- Whether vault and note slugs should stay as editable metadata only, or be removed entirely in a later pass
- Whether `home-slug` should always equal a user's login slug, or merely reference it
- Whether local-only vaults should live directly under `local/` or under `local/homes/<home-slug>/`
- Whether server vault permissions should be expressed as roles, capabilities, or a simpler enum in MVP metadata
