# Data Model: Recent Notes Sidebar (Persistent)

**Feature**: `003-sidebar-recent-notes`
**Phase**: 1 — Design
**Date**: 2026-03-23

---

## New Types

None. This feature introduces no new TypeScript interfaces or types.

---

## New Components

### `AppLayout.vue`

A structural shell component. On mount it:

- Calls `useVaults().load()` to initialise the vault list
- Calls `useNotes().loadAll(vaultIds)` to populate the notes cache for every vault in parallel

It renders `VaultSwitcher`, `SidebarNav`, `NoteList`, sidebar footer, and `<RouterView />`. No new reactive state.

---

## Modified Composables

### `useNotes.ts`

The module-level `notes` ref is replaced with a vault-keyed cache:

```typescript
const notesByVault = ref<Record<string, NoteMeta[]>>({})
```

**New exports:**

| Export | Signature | Description |
|--------|-----------|-------------|
| `notesByVault` | `Ref<Record<string, NoteMeta[]>>` | The full cache — one entry per loaded vault |
| `notesFor` | `(vaultId: string) => NoteMeta[]` | Returns the cached notes for one vault ([] if not yet loaded) |
| `loadAll` | `(vaultIds: string[]) => Promise<void>` | Fetches all vaults in parallel via `Promise.allSettled`; writes each result into `notesByVault` without touching `listLoading` |

All mutation methods (`create`, `save`, `remove`, `updatePermission`) now target `notesByVault[vaultId]` instead of a flat array.

### `useVaults.ts`

`setActive` signature widened to accept `null`:

```typescript
function setActive(vault: Vault | null): void
```

`HomeView` calls `setActive(null)` on mount so the sidebar shows nothing (empty state) when the user is on the vaults overview.

---

## Modified Components

### `NoteList.vue`

- Removed: `watch(activeVault, vault => loadList(vault.id))` — loading is now done by `AppLayout`
- Reads: `notesFor(activeVault.value.id)` from the cache
- Guards: returns `[]` when `activeVault` is null (HomeView)
- Sorts: by `updated_at` descending before rendering
- Limits: to `RECENT_LIMIT = 20` entries
- Active state: derived from `route.params.id` (not `activeNote`) — naturally `undefined` on non-note routes

### `VaultSwitcher.vue`

Note count label now reads `notesFor(activeVault.value.id).length` instead of the removed flat `notes` ref.

### `VaultView.vue`

`sortedNotes` computed reads `notesFor(vaultId.value)` from the cache. Continues to call `loadList(vaultId)` on mount for freshness.

---

## State Flow

```
useVaults().vaults          → VaultSwitcher (dropdown options)
useVaults().activeVault     → NoteList (which vault's notes to show; null = show nothing)
                            → AppLayout sidebar footer (Settings link target)
useNotes().notesByVault     → notesFor(vaultId) → NoteList (list content, sorted, limited)
                                                 → VaultSwitcher (note count)
                                                 → VaultView (masonry grid)
useNotes().loadAll()        → called by AppLayout.onMounted after vaults load
useNotes().loadList()       → called by VaultView.onMounted for per-vault freshness
```
