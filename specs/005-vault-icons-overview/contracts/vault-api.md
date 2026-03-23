# API Contract: Vault Updates

**Branch**: `005-vault-icons-overview`

---

## Updated: `POST /api/vaults`

Create a personal vault. `icon` is now accepted as an optional field.

**Request**

```http
POST /api/vaults
Authorization: Bearer <token>
Content-Type: application/json

{
  "slug": "work",
  "name": "Work",
  "icon": "briefcase"
}
```

| Field | Type | Required | Validation |
|---|---|---|---|
| `slug` | `string` | ✓ | filesystem-safe, unique per user |
| `name` | `string` | ✓ | non-empty |
| `icon` | `string` | ✗ | must be in the 20-slug allowlist if provided |

**Response** `201 Created`

```json
{
  "id": "vault-abc123",
  "slug": "work",
  "name": "Work",
  "owner": { "type": "user", "id": "uid-1" },
  "permission": "restricted",
  "icon": "briefcase"
}
```

The `icon` field is omitted from the response if not set.

---

## New: `PATCH /api/vaults/:vault_id`

Partial update of a personal vault's name and/or icon.

**Request**

```http
PATCH /api/vaults/:vault_id
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "My Work Notes",
  "icon": "briefcase"
}
```

| Field | Type | Notes |
|---|---|---|
| `name` | `string \| undefined` | If present, replace vault name (must be non-empty) |
| `icon` | `string \| null \| undefined` | `string` = set icon; `null` = clear icon; absent = leave unchanged |

At least one field must be present (400 if body is empty `{}`).

**Auth**: Vault must be owned by `VaultOwner::User(caller_uid)`. Returns `403` for team-owned vaults (use the team admin routes for those).

**Responses**

| Status | Condition |
|---|---|
| `200 OK` | Update applied; returns updated `Vault` JSON |
| `400 Bad Request` | Empty body, invalid icon slug, or empty name |
| `403 Forbidden` | Caller does not own the vault, or vault is team-owned |
| `404 Not Found` | Vault not found |

**Response body** `200 OK`

```json
{
  "id": "vault-abc123",
  "slug": "work",
  "name": "My Work Notes",
  "owner": { "type": "user", "id": "uid-1" },
  "permission": "restricted",
  "icon": "briefcase"
}
```

**Clear icon example**

```json
{ "icon": null }
```

Response omits the `icon` field entirely when cleared.

---

## Updated: `GET /api/vaults`

Vault objects in the response list now include the optional `icon` field.

```json
[
  {
    "id": "vault-abc123",
    "slug": "home",
    "name": "Home",
    "owner": { "type": "user", "id": "uid-1" },
    "permission": "restricted"
  },
  {
    "id": "vault-def456",
    "slug": "work",
    "name": "Work",
    "owner": { "type": "user", "id": "uid-1" },
    "permission": "restricted",
    "icon": "briefcase"
  }
]
```

---

## Unchanged Routes

All other vault and team routes are unaffected by this feature.
