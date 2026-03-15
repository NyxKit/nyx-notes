# Vaults and Teams

## Concepts

### Vault

A **vault** is a named, isolated collection of notes. It is the primary organizational unit above individual notes.

- Every user has a personal vault named `home` created automatically
- Users can create additional personal vaults (e.g. `work`, `journal`)
- A vault maps directly to a directory on the filesystem
- Personal vaults have no permission system — the owner controls everything

### Team

A **team** is a group of users. Teams can own vaults.

- Users are not required to belong to a team
- A team has one default vault named `home`, created automatically
- Teams can have additional vaults
- Team vaults have a permission level that governs what non-owner members can do

### Vault Ownership

A vault is owned by either a user (personal) or a team.

```rust
pub enum VaultOwner {
    User(String),   // user id
    Team(String),   // team id
}
```

Personal vault permissions are not configurable — only the owner accesses them.
Team vault permissions use the same `NotePermission` enum (`restricted`, `comment`, `edit`), applied at the vault level to all team members who are not the team owner.

> Individual notes inside a team vault can still carry their own `permission` field, but the vault-level permission acts as the **floor** — a note cannot be more permissive than its vault.

---

## Domain Types

```rust
pub struct Vault {
    pub id: String,             // stable identifier (uuid or slug)
    pub slug: String,           // filesystem/URL-safe name, unique per owner (e.g. "home", "work")
    pub name: String,           // display name
    pub owner: VaultOwner,
    /// Only meaningful for team vaults.
    /// Personal vaults are always implicitly `Restricted` to the owner.
    pub permission: NotePermission,
}

pub struct Team {
    pub id: String,
    pub name: String,
    pub members: Vec<TeamMember>,
}

pub struct TeamMember {
    pub user_id: String,        // user id
    pub role: TeamRole,
}

pub enum TeamRole {
    /// Full control: manage members, vaults, and vault permissions.
    Owner,
    /// Can manage vaults and their permissions, cannot manage members.
    Admin,
    /// Access governed by vault-level permissions.
    Member,
}
```

### Team Role × Vault Permission Matrix

For team vaults, the effective access is determined by combining the user's `TeamRole` and the vault's `NotePermission`:

| Action | Team Owner | Team Admin | Member (`edit` vault) | Member (`comment` vault) | Member (`restricted` vault) |
|---|---|---|---|---|---|
| View notes | ✓ | ✓ | ✓ | ✓ | ✗ |
| Edit notes | ✓ | ✓ | ✓ | ✗ | ✗ |
| Add comments | ✓ | ✓ | ✓ | ✓ | ✗ |
| Delete notes | ✓ | ✓ | ✗ | ✗ | ✗ |
| Change vault permission | ✓ | ✓ | ✗ | ✗ | ✗ |
| Add/remove members | ✓ | ✗ | ✗ | ✗ | ✗ |
| Delete vault | ✓ | ✗ | ✗ | ✗ | ✗ |

---

## Filesystem Layout

```
$NOTES_ROOT/
  users/
    <uid>/                     # one directory per user
      home/                    # default personal vault (always exists)
        .vault.json            # vault metadata (id, name, slug — no permission)
        note-slug.md
      <vault-slug>/            # additional personal vaults
        .vault.json
        note-slug.md
  teams/
    <team-id>/
      .team.json               # team metadata (name, members, roles)
      home/                    # default team vault (always exists)
        .vault.json            # vault metadata (id, name, slug, permission)
        note-slug.md
      <vault-slug>/
        .vault.json
        note-slug.md
```

### `.team.json`

```json
{
  "id": "team-abc123",
  "name": "Engineering",
  "members": [
    { "user_id": "uid-1", "role": "owner" },
    { "user_id": "uid-2", "role": "admin" },
    { "user_id": "uid-3", "role": "member" }
  ]
}
```

### `.vault.json`

Present in **all** vault directories (personal and team). This allows `FsStorage` to resolve a vault by its `id` without maintaining a separate index.

Personal vault example (no `permission` field — personal vaults are always restricted to the owner):

```json
{
  "id": "vault-abc123",
  "name": "Home",
  "slug": "home"
}
```

Team vault example:

```json
{
  "id": "vault-xyz456",
  "name": "Home",
  "slug": "home",
  "permission": "comment"
}
```

---

## API Routes

### Vault Routes

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/vaults` | List all vaults accessible to the user (personal + team) |
| `POST` | `/api/vaults` | Create a new personal vault |
| `DELETE` | `/api/vaults/:vault_id` | Delete a personal vault (must be empty) |
| `PATCH` | `/api/teams/:team_id/vaults/:vault_id/permission` | Change a team vault's permission (owner or admin) |

### Team Routes

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/teams` | List teams the user belongs to |
| `POST` | `/api/teams` | Create a team (caller becomes owner) |
| `GET` | `/api/teams/:team_id` | Get team details and member list |
| `DELETE` | `/api/teams/:team_id` | Delete a team (owner only) |
| `POST` | `/api/teams/:team_id/members` | Add a member |
| `PATCH` | `/api/teams/:team_id/members/:user_id` | Change a member's role |
| `DELETE` | `/api/teams/:team_id/members/:user_id` | Remove a member |
| `POST` | `/api/teams/:team_id/vaults` | Create a team vault |
| `DELETE` | `/api/teams/:team_id/vaults/:vault_id` | Delete a team vault (must be empty; owner or admin) |

### Note Routes (vault-scoped)

All note operations are scoped to a vault:

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/vaults/:vault_id/notes` | List notes in a vault |
| `GET` | `/api/vaults/:vault_id/notes/:id` | Fetch a note |
| `POST` | `/api/vaults/:vault_id/notes` | Create a note in a vault |
| `PUT` | `/api/vaults/:vault_id/notes/:id` | Update a note |
| `DELETE` | `/api/vaults/:vault_id/notes/:id` | Delete a note |
| `PATCH` | `/api/vaults/:vault_id/notes/:id/permission` | Change note-level permission |

---

## Defaults

- On first login, a personal `home` vault is created for the user if it doesn't exist.
- On team creation, a team `home` vault is created automatically with `permission: restricted`.
  - The home vault's `id` is derived as `"{team_id}-home"` (e.g. `"team-abc123-home"`). This convention is used by `FsStorage::save_team` when initialising the default vault.
- New notes created within a team vault inherit the vault's permission as their default.
