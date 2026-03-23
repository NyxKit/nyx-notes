# Data Model: Vault Icons & Overview Redesign

**Branch**: `005-vault-icons-overview`

---

## Entities

### `Vault` (updated)

| Field | Type | Required | Notes |
|---|---|---|---|
| `id` | `String` | ✓ | Stable identifier |
| `slug` | `String` | ✓ | Filesystem/URL-safe name, unique per owner |
| `name` | `String` | ✓ | Display name |
| `owner` | `VaultOwner` | ✓ | `User(uid)` or `Team(team_id)` |
| `permission` | `NotePermission` | ✓ | Only meaningful for team vaults |
| `icon` | `Option<String>` | ✗ | **NEW** — curated icon slug; `None` = no icon |

### `VaultUpdate` (new — Rust only)

Used as the argument to `StorageBackend::update_vault`. Not persisted directly.

| Field | Type | Notes |
|---|---|---|
| `name` | `Option<String>` | If `Some`, replace vault name |
| `icon` | `Option<VaultIconUpdate>` | If `Some(Set(slug))`, set icon; if `Some(Clear)`, remove icon; if `None`, leave unchanged |

```rust
pub enum VaultIconUpdate {
    Set(String),
    Clear,
}
```

---

## Filesystem: `.vault.json` (updated)

### Personal vault (no permission, optional icon)

```json
{
  "id": "vault-abc123",
  "name": "Work",
  "slug": "work",
  "icon": "briefcase"
}
```

### Personal vault without icon (unchanged format)

```json
{
  "id": "vault-abc123",
  "name": "Home",
  "slug": "home"
}
```

### Team vault (with permission and optional icon)

```json
{
  "id": "vault-xyz456",
  "name": "Engineering Docs",
  "slug": "docs",
  "permission": "comment",
  "icon": "book"
}
```

---

## TypeScript Types (updated)

### `Vault`

```typescript
export interface Vault {
  id: string
  slug: string
  name: string
  owner: VaultOwner
  permission: NotePermission
  icon?: string   // optional — one of the 20 curated slugs
}
```

### `CreateVaultRequest`

```typescript
export interface CreateVaultRequest {
  slug: string
  name: string
  icon?: string   // optional — curated slug; omit for no icon
}
```

### `UpdateVaultRequest` (new)

```typescript
export interface UpdateVaultRequest {
  name?: string         // if provided, replace vault name
  icon?: string | null  // string = set icon; null = clear icon; undefined = no change
}
```

---

## StorageBackend Trait (updated)

```rust
pub trait StorageBackend: Send + Sync {
    // ... existing methods ...

    /// Update vault name and/or icon. Partial update: only fields present in
    /// `update` are applied; absent fields are left unchanged.
    fn update_vault(&self, vault_id: &str, update: &VaultUpdate) -> Result<(), StorageError>;
}
```

---

## Icon Allowlist

The 20 valid icon slugs. Validated by the server handler; stored as plain strings in `.vault.json`.

```rust
const VALID_ICONS: &[&str] = &[
    "home", "book", "star", "briefcase", "code",
    "pen", "heart", "globe", "lock", "rocket",
    "lightbulb", "music", "camera", "folder", "compass",
    "flask", "graduation-cap", "chart", "leaf", "diamond",
];
```

---

## Validation Rules

| Rule | Where enforced |
|---|---|
| `icon` slug must be in `VALID_ICONS` or absent | Server handler (`PATCH /api/vaults/:vault_id`, `POST /api/vaults`) |
| `icon` slug length ≤ 64 chars | Server handler (guards against oversized payloads) |
| `name` non-empty if provided | Server handler |
| Icon slugs stored on disk but not in `VALID_ICONS` | Forwarded as-is; frontend falls back to default icon |

---

## State Transitions

```
Vault (no icon)
  → PATCH with { icon: "briefcase" }  → Vault (icon: "briefcase")
  → PATCH with { icon: null }          → no-op (already no icon)

Vault (icon: "briefcase")
  → PATCH with { icon: "book" }       → Vault (icon: "book")
  → PATCH with { icon: null }         → Vault (no icon)
  → PATCH with { name: "New Name" }   → Vault (name: "New Name", icon unchanged)
```
