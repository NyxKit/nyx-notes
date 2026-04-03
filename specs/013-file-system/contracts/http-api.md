# Contract: HTTP API

## Scope

This contract describes the MVP HTTP surface after removing team-centric routes and aligning the API to the new server/home/server-vault model.

MVP route identifiers are slug-based for filesystem-addressed entities. Stable internal IDs remain present in metadata and response bodies where useful, but they are not the canonical route keys in this feature.

## Authenticated Routes

### Server Metadata

#### `GET /api/server`

Returns the active server namespace metadata for the caller.

Response:

```json
{
  "id": "server-main",
  "slug": "server-main",
  "name": "Main Server",
  "role": "admin"
}
```

### Vault Routes

#### `GET /api/vaults`

List all vaults accessible to the caller:

- personal vaults in the caller's home
- shared server vaults

Response: `200 OK`

#### `POST /api/vaults`

Create a personal vault in the caller's home.

Request:

```json
{
  "slug": "journal",
  "name": "Journal",
  "description": "Private daily writing."
}
```

Response: `201 Created`

#### `PATCH /api/vaults/:vault_id`

Update vault metadata.

- personal vault: owner only
- server vault: `admin` only

Response: `200 OK`

#### `DELETE /api/vaults/:vault_id`

Delete a personal vault owned by the caller. Vault must be empty.

Response: `204 No Content`

### Server Vault Administration

#### `POST /api/server/vaults`

Create a shared server vault. `admin` only.

Request:

```json
{
  "slug": "handbook",
  "name": "Handbook",
  "description": "Shared team handbook."
}
```

Response: `201 Created`

#### `DELETE /api/server/vaults/:vault_id`

Delete a shared server vault. `admin` only. Vault must be empty.

Response: `204 No Content`

## Note Routes

The note routes remain vault-scoped:

- `GET /api/vaults/:vault_id/notes`
- `GET /api/vaults/:vault_id/notes/:id`
- `POST /api/vaults/:vault_id/notes`
- `PUT /api/vaults/:vault_id/notes/:id`
- `DELETE /api/vaults/:vault_id/notes/:id`
- `PATCH /api/vaults/:vault_id/notes/:id/permission`

Behavioral changes:

- server-vault access is gated by the caller's server role and the vault's permissions metadata
- personal home vaults remain owner-scoped

## Removed MVP Routes

These routes are removed from the active MVP contract:

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
