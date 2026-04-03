# Data Model: File System Architecture Alignment

## Overview

This feature replaces the MVP team-centric model with a server/home/server-vault model while preserving note and comment payload formats.

## Entities

### Server Namespace

Represents one logical server namespace under `NOTES_ROOT`.

Fields:

- `id: String`
- `slug: String`
- `name: String`
- `roles: Vec<ServerRole>`

Validation:

- `slug` must match `^[a-z0-9_-]{1,64}$`
- `slug` is immutable in MVP
- `roles` contains only `admin` and `user` in MVP

Relationships:

- has many `Home`
- has many server-owned `Vault`

### Home

Represents one user's personal namespace inside a server.

Fields:

- `id: String`
- `slug: String`
- `name: String`
- `owner_user_id: String`

Validation:

- `slug` must match `^[a-z0-9_-]{1,64}$`
- unique within a server
- username/home-slug change is not supported in MVP

Relationships:

- belongs to one `Server Namespace`
- owns many personal `Vault`

### Vault

Represents a note container. Vault slugs are the main note-scoping identifiers exposed to the MVP API, while stable internal IDs remain stored in metadata.

Fields:

- `id: String`                # stable internal identifier retained in metadata
- `slug: String`
- `name: String`
- `description: Option<String>`
- `icon: Option<String>`
- `kind: VaultKind`
- `owner: VaultOwner`
- `permissions: Option<ServerVaultPermissions>`

Validation:

- `slug` must match `^[a-z0-9_-]{1,64}$`
- personal vault slugs are unique within a home
- server vault slugs are unique within a server
- local vault slugs are unique within `local/`
- `permissions` is required for server vaults and absent for personal vaults in MVP

Relationships:

- belongs to either a `Home`, a `Server Namespace`, or the local namespace
- has many `Note`

### VaultKind

Enum:

- `Personal`
- `Server`
- `Local`
- `LocalSync` (reserved for future work)

### VaultOwner

Enum:

- `Home { home_slug: String, server_slug: String }`
- `Server { server_slug: String }`
- `Local`

### ServerRole

Enum:

- `Admin`
- `User`

Meaning:

- `Admin` can create and delete server vaults
- `User` can use existing server vaults but cannot create or delete them

### ServerVaultPermissions

Capability-based permissions stored in `.vault.json` for server-owned shared vaults.

Fields:

- `admin: Vec<ServerVaultCapability>`
- `user: Vec<ServerVaultCapability>`

### ServerVaultCapability

Enum values used in metadata:

- `view`
- `edit`
- `create_note`
- `delete_note`
- `create_vault`
- `delete_vault`

MVP default:

- `admin`: all capabilities above
- `user`: `view`, `edit`, `create_note`, `delete_note`

### NoteMeta

Unchanged from current MVP unless a separate feature changes note payload format.

Fields retained:

- `id`
- `vault_id`
- `title`
- `description`
- `author_id`
- `tags`
- `category`
- `created_at`
- `updated_at`
- `is_encrypted`
- `permission`

Validation retained:

- note slug/file stem matches `id`
- note slugs are unique within a vault
- timestamps are ISO 8601 UTC strings

### Comment Sidecar

Unchanged by this feature.

## Relationships Summary

- `Server Namespace` -> many `Home`
- `Server Namespace` -> many server `Vault`
- `Home` -> many personal `Vault`
- `Vault` -> many `Note`
- `Note` -> optional comment sidecar

## State Transitions

### Create personal vault

1. Resolve caller home from server namespace + user identity
2. Validate slug uniqueness inside that home
3. Create vault directory and `.vault.json`

### Create server vault

1. Confirm caller role is `admin`
2. Validate slug uniqueness inside `<server-slug>/vaults/`
3. Create vault directory and `.vault.json` with server permissions

### Delete server vault

1. Confirm caller role is `admin`
2. Confirm vault is empty of note files
3. Remove vault directory and metadata file

### Resolve vault by ID

1. Scan namespace metadata and vault metadata under the configured root
2. Match on stable `id`
3. Return vault path and ownership metadata

### Resolve vault by slug for MVP API operations

1. Determine whether the target is a personal or server vault from the caller context and route shape
2. Resolve the directory path directly from validated slugs
3. Read `.vault.json` for metadata validation and internal ID continuity

## Removed MVP Entities

These are removed from the active MVP model and should be retired from active docs and code paths:

- `Team`
- `TeamMember`
- `TeamRole`
- `VaultOwner::Team`
