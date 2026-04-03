# Vault Ownership And Shared Access

## Status

This file retains its historical path for cross-doc compatibility, but the active MVP no longer uses teams as the primary sharing primitive.

The active model is:

- personal vaults inside one user home per server
- shared server vaults governed by server roles
- no team storage or team routes in the MVP

## Concepts

### Vault

A **vault** is a named collection of notes.

- A vault maps directly to a directory on the filesystem
- Personal vaults live inside a home namespace
- Shared vaults live directly under the server namespace

### Home

A **home** is the personal namespace for one user inside the active server namespace.

- The MVP maps `NOTES_USER_ID` to the caller's home slug
- Personal vaults are owner-scoped

### Server Vault

A **server vault** is a shared vault owned by the active server namespace.

- `admin` users can create, update, and delete server vaults
- `user` users can view and edit notes inside existing server vaults

## Domain Types

```rust
pub enum VaultOwner {
    Home { server_slug: String, home_slug: String },
    Server { server_slug: String },
    Local,
}

pub enum ServerRole {
    Admin,
    User,
}

pub struct Vault {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub owner: VaultOwner,
    pub permission: NotePermission,
    pub icon: Option<String>,
}
```

## Access Rules

### Personal vaults

- Owner can view, edit, comment, and delete notes
- Other users have no access in the MVP

### Shared server vaults

| Action | `admin` | `user` |
|---|---|---|
| View notes | ✓ | ✓ |
| Edit notes | ✓ | ✓ |
| Add comments | ✓ | ✓ |
| Delete notes | ✓ | ✓ |
| Create vault | ✓ | ✗ |
| Delete vault | ✓ | ✗ |
| Update vault metadata | ✓ | ✗ |

## Filesystem Mapping

```text
$NOTES_ROOT/
  <server-slug>/
    homes/
      <home-slug>/
        <vault-slug>/
    vaults/
      <vault-slug>/
```

Metadata details live in [file-system.md](./file-system.md) and [filesystem-storage.md](./filesystem-storage.md).

## Notes

- Stable internal IDs are still stored in metadata
- Slugs are the canonical MVP API and filesystem identifiers
- `local/` stays reserved for future work but is not runtime-active in the MVP
