# Contract: Filesystem Metadata

## Layout

```text
$NOTES_ROOT/
  <server-slug>/
    .server.json
    homes/
      <home-slug>/
        .home.json
        <vault-slug>/
          .vault.json
          <note-slug>.md
          <note-slug>.comments.json
    vaults/
      <vault-slug>/
        .vault.json
        <note-slug>.md
        <note-slug>.comments.json
  local/
    .local.json
    <vault-slug>/
      .vault.json
      <note-slug>.md
      <note-slug>.comments.json
```

## Slug Contract

- Regex: `^[a-z0-9_-]{1,64}$`
- Immutable in MVP
- Lowercase only
- No path separators
- Not `.` or `..`

## `.server.json`

Path: `<server-slug>/.server.json`

```json
{
  "id": "server-main",
  "slug": "server-main",
  "name": "Main Server",
  "roles": ["admin", "user"]
}
```

Rules:

- `slug` must equal the parent directory name
- `roles` lists supported server roles for this namespace

## `.home.json`

Path: `<server-slug>/homes/<home-slug>/.home.json`

```json
{
  "id": "home-arne",
  "slug": "arnedecant",
  "name": "Arne's Home",
  "owner_user_id": "arnedecant"
}
```

Rules:

- `slug` must equal the parent directory name
- `owner_user_id` is the MVP lookup key used by CLI/server auth flows

## `.vault.json`

Path: every vault directory

Personal vault example:

```json
{
  "id": "vault-journal",
  "slug": "journal",
  "name": "Journal",
  "kind": "personal",
  "owner": {
    "type": "home",
    "server_slug": "server-main",
    "home_slug": "arnedecant"
  }
}
```

Server vault example:

```json
{
  "id": "vault-handbook",
  "slug": "handbook",
  "name": "Handbook",
  "kind": "server",
  "owner": {
    "type": "server",
    "server_slug": "server-main"
  },
  "permissions": {
    "admin": ["view", "edit", "create_note", "delete_note", "create_vault", "delete_vault"],
    "user": ["view", "edit", "create_note", "delete_note"]
  }
}
```

Rules:

- `slug` must equal the parent directory name
- `kind` is one of `personal`, `server`, `local`, `local_sync`
- `permissions` is required for `server` vaults in MVP
- stable `id` remains the canonical API/storage identifier

## `.local.json`

Path: `local/.local.json`

```json
{
  "id": "local",
  "name": "Local Storage"
}
```

## Notes And Comment Sidecars

- `<note-slug>.md` keeps the current YAML frontmatter and Markdown body format
- `<note-slug>.comments.json` keeps the current comment sidecar format
- note file stem must match note `id`
