# Backend API (`notes-server-axum`)

## Purpose

An Axum HTTP server that exposes note CRUD operations over a REST API. It wraps a `StorageBackend`, delegates authentication to the configured `AuthStore`, and enforces permissions in the API layer.

## Route Model

- Vault route identifiers are vault slugs in the MVP
- Note route identifiers remain stable note IDs in the MVP
- Stable vault IDs may still appear in metadata and response bodies, but they are not the canonical route keys
- Team routes are removed from the active MVP contract

## Auth Routes

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/auth/mode` | Returns the active auth mode and optional server metadata |
| `POST` | `/api/auth/login` | `secret_key` mode only — accepts credentials and returns a signed JWT |

## Server Routes

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/server` | Returns active server metadata and the caller's server role |
| `GET` | `/api/server/vaults` | List shared server vaults only |
| `POST` | `/api/server/vaults` | Create a shared server vault (`admin` only) |
| `DELETE` | `/api/server/vaults/:vault_id` | Delete a shared server vault (`admin` only; must be empty) |

## User Routes

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/users` | List managed users (`admin`, `secret_key` mode only) |
| `POST` | `/api/users` | Create a managed user (`admin`, `secret_key` mode only) |
| `PATCH` | `/api/users/:user_id` | Update a managed user (`admin`, `secret_key` mode only) |
| `DELETE` | `/api/users/:user_id` | Delete a managed user (`admin`, `secret_key` mode only) |

## Vault Routes

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/vaults` | List all vaults accessible to the caller (personal home + shared server vaults) |
| `GET` | `/api/vaults/personal` | List only personal vaults in the caller's home |
| `POST` | `/api/vaults` | Create a personal vault in the caller's home |
| `PATCH` | `/api/vaults/:vault_id` | Update vault name, description, and/or icon |
| `DELETE` | `/api/vaults/:vault_id` | Delete a personal vault owned by the caller (must be empty) |

## Note Routes

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/vaults/:vault_id/notes` | List notes in a vault |
| `GET` | `/api/vaults/:vault_id/notes/:id` | Fetch a single note |
| `POST` | `/api/vaults/:vault_id/notes` | Create a note using a stable note ID |
| `PUT` | `/api/vaults/:vault_id/notes/:id` | Update a note |
| `DELETE` | `/api/vaults/:vault_id/notes/:id` | Delete a note |
| `PATCH` | `/api/vaults/:vault_id/notes/:id/permission` | Change note-level permission |

## Comment Routes

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/vaults/:vault_id/notes/:id/comments` | List visible comment threads for a note |
| `POST` | `/api/vaults/:vault_id/notes/:id/comments` | Create a line-based comment thread |
| `PATCH` | `/api/vaults/:vault_id/notes/:id/comments/:comment_id` | Resolve or reopen a thread |
| `DELETE` | `/api/vaults/:vault_id/notes/:id/comments/:comment_id` | Delete a thread |
| `POST` | `/api/vaults/:vault_id/notes/:id/comments/:comment_id/replies` | Add a reply |
| `DELETE` | `/api/vaults/:vault_id/notes/:id/comments/:comment_id/replies/:reply_id` | Delete a reply |

## Permission Rules

### Personal vaults

- Caller must own the home namespace to update or delete the vault
- Notes remain owner-scoped unless note-level permission allows broader access

### Shared server vaults

- `admin` can create/delete shared server vaults and update their metadata
- `user` can use existing shared server vaults but cannot create or delete them
- Shared server vaults default to `permission: edit` for newly created notes in the MVP

### Managed users

- User management is available only in `secret_key` mode for the MVP
- Only `admin` callers may list, create, update, or delete managed users
- Every managed user requires a unique `username` and a unique `email`
- Passwords must be at least 12 characters and include at least 3 of these 4 categories: lowercase letters, uppercase letters, digits, and symbols
- The currently signed-in administrator may edit their own account details, but may not delete themselves or remove their own `admin` role
- The last remaining administrator may not be deleted or demoted

### Notes and comments

- Restricted notes are visible only to the author
- `comment` notes allow reads and comments, but not edits
- `edit` notes allow reads, comments, and edits
- Note author or owning home/admin may delete a note

## `GET /api/server`

Response:

```json
{
  "id": "server-main-server",
  "slug": "main-server",
  "name": "Main Server",
  "role": "admin",
  "root_path": "/home/arnedecant/.local/share/nyx-notes"
}
```

The frontend should use `name` as the active built-in server label instead of showing a generic `Local` label.

## `POST /api/vaults`

Request:

```json
{
  "slug": "journal",
  "name": "Journal",
  "description": "Private daily writing.",
  "icon": "book"
}
```

Response: `201 Created` with the created `Vault`.

## `POST /api/server/vaults`

Request:

```json
{
  "slug": "handbook",
  "name": "Handbook",
  "description": "Shared server documentation.",
  "icon": "briefcase"
}
```

Response: `201 Created` with the created `Vault`.

## `POST /api/vaults/:vault_id/notes`

Request:

```json
{
  "title": "Project Plan",
  "content": "# Hello\n\nMarkdown body.",
  "tags": ["rust"],
  "category": "work",
  "permission": "restricted"
}
```

Behavior:

- The note ID is generated as a stable opaque identifier at creation time
- Note titles do not rename note files or route IDs
- `vault_id` in returned note metadata is derived from the containing vault directory on the server side
- `permission` defaults to the vault's default note permission when omitted

Response: `201 Created` with the created `NoteMeta`.

## Removed MVP Routes

- `GET /api/teams`
- `POST /api/teams`
- `GET /api/teams/:team_id`
- `DELETE /api/teams/:team_id`
- `POST /api/teams/:team_id/members`
- `PATCH /api/teams/:team_id/members/:user_id`
- `DELETE /api/teams/:team_id/members/:user_id`
- `POST /api/teams/:team_id/vaults`
- `DELETE /api/teams/:team_id/vaults/:vault_id`
- `PATCH /api/teams/:team_id/vaults/:vault_id/permission`
