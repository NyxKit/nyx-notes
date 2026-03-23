# Feature Spec: Vault Icons & Overview Redesign

**Branch**: `005-vault-icons-overview`
**Date**: 2026-03-23

---

## Summary

Users can assign a decorative icon to any vault from a curated set of 20 icons. The vault overview screen is redesigned: each vault card is a 1:1 aspect-ratio square with the vault name and slug aligned left and the icon rendered at large scale at the right side at low opacity, acting as a visual background accent.

---

## Clarifications

### Session 2026-03-23

- Q: Is setting icons on team vaults in scope, or personal vaults only? → A: Both — `PATCH /api/vaults/:vault_id` handles personal vaults (owner check) and team vaults (team owner/admin check).
- Q: Where does the icon picker appear in the vault creation flow? → A: Inside the inline create form card, alongside the slug and name fields.
- Q: Should vault cards without an assigned icon show a fallback icon or an empty right side? → A: Always render a background icon — fall back to `folder` for unassigned vaults.

---

## Goals

1. **Icon assignment** — Vault owners can choose one of 20 curated icons for a vault at create time or later via vault settings.
2. **Vault overview redesign** — Each vault card in `HomeView` becomes a square tile with the icon as a low-opacity background filler.
3. **Vault settings** — `VaultSettingsView` exposes an icon picker alongside the existing rename/delete controls.

---

## Non-Goals

- Full icon library (Lucide, FontAwesome, etc.) — limit to exactly 20 curated icons.
- Custom user-uploaded icons.

---

## Icon Set

A fixed list of 20 icons identified by slug. All icons are stroke-based inline SVG, consistent with the project's existing icon style.

| Slug | Meaning |
|---|---|
| `home` | Home / default |
| `book` | Writing, journal |
| `star` | Favorites, important |
| `briefcase` | Work |
| `code` | Programming |
| `pen` | Drafts, editing |
| `heart` | Personal |
| `globe` | Public, shared |
| `lock` | Private, secure |
| `rocket` | Projects |
| `lightbulb` | Ideas |
| `music` | Creative |
| `camera` | Photos, visual |
| `folder` | General, archive |
| `compass` | Travel, exploration |
| `flask` | Research, science |
| `graduation-cap` | Education |
| `chart` | Analytics, data |
| `leaf` | Nature, wellness |
| `diamond` | Premium, important |

---

## Data Model Changes

### Rust (`notes-core`)

Add optional `icon` field to `Vault`:

```rust
pub struct Vault {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub owner: VaultOwner,
    pub permission: NotePermission,
    pub icon: Option<String>,   // NEW — one of the 20 curated slugs; None = no icon
}
```

Add `VaultUpdate` struct for partial updates:

```rust
pub struct VaultUpdate {
    pub name: Option<String>,
    pub icon: Option<VaultIconUpdate>,
}

pub enum VaultIconUpdate {
    Set(String),    // set to a specific icon slug
    Clear,          // remove the icon
}
```

Extend `StorageBackend` trait:

```rust
fn update_vault(&self, vault_id: &str, update: &VaultUpdate) -> Result<(), StorageError>;
```

### Filesystem (`.vault.json`)

New optional `icon` field — backward compatible (missing = `None`):

```json
{
  "id": "vault-abc123",
  "name": "Work",
  "slug": "work",
  "icon": "briefcase"
}
```

### TypeScript (`frontend/src/types/index.ts`)

```typescript
export interface Vault {
  id: string
  slug: string
  name: string
  owner: VaultOwner
  permission: NotePermission
  icon?: string   // NEW
}

export interface CreateVaultRequest {
  slug: string
  name: string
  icon?: string   // NEW
}

export interface UpdateVaultRequest {   // NEW
  name?: string
  icon?: string | null   // null = clear icon
}
```

---

## API Changes

### New endpoint

`PATCH /api/vaults/:vault_id` — update a vault's name and/or icon. Covers both personal and team vaults.

- Auth (personal vault): caller must be `VaultOwner::User(caller_uid)` — same ownership check as DELETE
- Auth (team vault): caller must hold `TeamRole::Owner` or `TeamRole::Admin` in the owning team — same check as `PATCH …/permission`
- Body: `UpdateVaultRequest`
- Response: `200 OK` with updated `Vault` JSON

### Updated `CreateVaultRequest`

`POST /api/vaults` body now accepts optional `icon` field.

---

## UI Changes

### `HomeView` — vault card redesign

Each vault card:
- **Shape**: 1:1 aspect ratio (`aspect-ratio: 1 / 1`)
- **Layout**: text (name + slug) aligned to the bottom-left; icon absolutely positioned at the right side, vertically centered, very large (~70% card height), `opacity: 0.12`
- **Icon fallback**: vaults with no assigned icon always render the `folder` icon as the background — no card is ever icon-less
- **Background**: unchanged (`var(--nyx-c-bg-soft)`)
- **Hover**: unchanged

### New components

- `VaultIcon.vue` — renders the SVG icon by slug prop; falls back to a neutral `folder` icon if unknown slug
- `VaultIconPicker.vue` — grid of 20 icon options; emits `select` event with icon slug; used in the inline create form card (alongside slug + name fields) and in vault settings

### `VaultSettingsView` — icon picker

Add icon picker section above the rename form.

---

## Constraints

- No new npm packages — icons are individual SVG files in `frontend/src/assets/icons/{slug}.svg`, imported via Vite's `?raw` suffix
- `icon` slug is free text in the API (no enum) — validated against the 20-slug allowlist in the server handler
- Unknown slugs stored on disk are forwarded as-is; the frontend falls back to a default icon
- Personal vault `.vault.json` did not previously include `permission` — `icon` follows the same pattern (optional, omitted when absent)
