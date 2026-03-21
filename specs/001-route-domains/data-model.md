# Data Model: Route Domains

**Phase 1 output for `001-route-domains`**

## Existing Types (no new types introduced)

This feature introduces no new domain types. It operates entirely on existing frontend interfaces:

```ts
// types/index.ts — unchanged
interface Vault {
  id: string
  slug: string
  name: string
  owner: VaultOwner
  permission: string
}

interface NoteMeta {
  id: string
  vault_id: string
  title: string
  author_id: string
  tags: string[]
  category: string | null
  created_at: string  // ISO 8601 UTC
  updated_at: string  // ISO 8601 UTC
  is_encrypted: boolean
  permission: NotePermission
}
```

## State Per View

### HomeView (modified)

| State | Source | Purpose |
|-------|--------|---------|
| `vaults` | `useVaults().vaults` | Full vault list |
| `loading` | `useVaults().loading` | Loading skeleton |
| `showCreateForm` | local `ref<boolean>` | Toggle inline create form |
| `newSlug`, `newName` | local `ref<string>` | Controlled inputs for new vault |

**Derived logic**:
- `isSingleVault`: `computed(() => vaults.value.length === 1)`
- On mount: if `isSingleVault` → `router.replace('/vaults/:vault_id')`

### VaultView (new)

| State | Source | Purpose |
|-------|--------|---------|
| `notes` | `useNotes().notes` | Notes list for vault |
| `listLoading` | `useNotes().listLoading` | Loading state |
| `vault_id` | `useRoute().params.vault_id` | Current vault from URL |
| `activeVault` | `useVaults().activeVault` | Vault metadata for header |

**Derived logic**:
- `hasNotes`: `computed(() => notes.value.length > 0)`
- On mount: load vaults → set active → load notes for `vault_id`

## Route → View Mapping

| Route | View | Route Name | Auth Guard |
|-------|------|-----------|-----------|
| `/` | `HomeView.vue` | `home` | ✅ requiresAuth |
| `/vaults/:vault_id` | `VaultView.vue` (new) | `vault` | ✅ requiresAuth |
| `/vaults/:vault_id/notes/:id?` | `NoteView.vue` | — | ✅ requiresAuth |

## No Backend Changes

All data is fetched via existing API endpoints:
- `GET /api/vaults` → used by `useVaults().load()`
- `GET /api/vaults/:vault_id/notes` → used by `useNotes().loadList(vault_id)`
- `POST /api/vaults` → used by `useVaults().create(req)`
- `POST /api/vaults/:vault_id/notes` → used by `useNotes().create(vault_id, req)`
