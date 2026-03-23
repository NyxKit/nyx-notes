# Quickstart: Pinia Stores for Vaults and Notes

**Feature**: `004-pinia-stores`
**Phase**: 1 — Design
**Date**: 2026-03-23

---

## What is changing

| Area | Change |
|------|--------|
| `frontend/src/stores/vaults.ts` | **NEW** — `useVaultStore` (Pinia setup store, replaces `useVaults`) |
| `frontend/src/stores/notes.ts` | **NEW** — `useNotesStore` (Pinia setup store, replaces `useNotes`) |
| `frontend/src/composables/useVaults.ts` | **DELETED** |
| `frontend/src/composables/useNotes.ts` | **DELETED** |
| 10 components/views | Import path + function name updated |
| `frontend/src/components/NoteList.vue` | Use `notesFor()`, sort+limit, active state from route, no loadList watch |
| `frontend/src/components/AppLayout.vue` | Call `useNotesStore().loadAll()` after loading vaults |
| `frontend/src/views/HomeView.vue` | Call `useVaultStore().setActive(null)` on mount |

---

## Build order

### Step 1: Create `stores/vaults.ts`

```typescript
import { ref } from 'vue'
import { defineStore, acceptHMRUpdate } from 'pinia'
// ... same imports as useVaults.ts

export const useVaultStore = defineStore('vaults', () => {
  // same refs as module-level refs in useVaults.ts
  // same functions (now inside the store)
  // add $reset()
  return { vaults, activeVault, loading, error, load, create, remove, setActive, ... $reset }
})

if (import.meta.hot) acceptHMRUpdate(useVaultStore, import.meta.hot)
```

Key changes from `useVaults`:
- `setActive()` accepts `Vault | null`
- Add `$reset()` function
- Wrap with `defineStore` + `acceptHMRUpdate`
- Module-level refs become store-local refs

### Step 2: Create `stores/notes.ts`

```typescript
import { ref } from 'vue'
import { defineStore, acceptHMRUpdate } from 'pinia'
// ... same imports as useNotes.ts

export const useNotesStore = defineStore('notes', () => {
  const notesByVault = ref<Record<string, NoteMeta[]>>({})  // vault cache (not flat notes)
  // ... other refs
  function notesFor(vaultId: string): NoteMeta[] {
    return notesByVault.value[vaultId] ?? []
  }
  async function loadAll(vaultIds: string[]) { ... }  // parallel fetch, no listLoading
  // ... all other functions from useNotes.ts, updated to use notesByVault[vaultId]
  function $reset() { ... }
  return { notesByVault, activeNote, listLoading, loading, saving, error, notesFor, loadAll, loadList, ... $reset }
})

if (import.meta.hot) acceptHMRUpdate(useNotesStore, import.meta.hot)
```

### Step 3: Update all 10 callers

For each file listed in data-model.md §Call Site Migration:

```typescript
// Before
import { useVaults } from '@/composables/useVaults'
const { vaults, activeVault } = useVaults()

// After
import { useVaultStore } from '@/stores/vaults'
const { vaults, activeVault } = useVaultStore()
```

```typescript
// Before
import { useNotes } from '@/composables/useNotes'
const { notes, loadList } = useNotes()

// After
import { useNotesStore } from '@/stores/notes'
const { notesFor, loadList } = useNotesStore()
```

### Step 4: Update `AppLayout.vue`

Add `loadAll` call after vault load:
```typescript
const vaultStore = useVaultStore()
const notesStore = useNotesStore()
onMounted(async () => {
  await vaultStore.load()
  await notesStore.loadAll(vaultStore.vaults.map(v => v.id))
})
```

### Step 5: Update `NoteList.vue`

- Remove `watch(activeVault, loadList)` — AppLayout pre-loads all vaults
- Use `notesStore.notesFor(activeVault?.id ?? '')` filtered by null guard
- Sort by `updated_at` desc, limit to 20
- Active state: `route.params.id === note.id`

### Step 6: Update `HomeView.vue`

```typescript
const vaultStore = useVaultStore()
onMounted(async () => {
  vaultStore.setActive(null)
  await vaultStore.load()
  ...
})
```

### Step 7: Delete composable files

```
rm frontend/src/composables/useVaults.ts
rm frontend/src/composables/useNotes.ts
```

---

## Key constraints

- No semicolons; single quotes in all frontend TS/Vue files
- No new npm packages (Pinia is already installed)
- `notesByVault` cache is the source of truth for all note lists
- `loadList` still used by `VaultView` for per-visit freshness (writes into cache)
- `loadAll` does NOT set `listLoading` — it is a background prefetch; `VaultView`'s skeleton is tied to `loadList` only
- `$reset()` must be called on logout (caller is `useAuth`; out of scope for this feature — just expose it)

---

## How to verify

1. Open Vue DevTools → Pinia panel → `vaults` and `notes` stores are listed
2. Navigate to HomeView — sidebar is empty (null active vault)
3. Navigate to a vault — sidebar shows that vault's notes immediately
4. Switch to a vault with 0 notes and back — notes repopulate from cache, no loading flash
5. Open a note — it is highlighted in the sidebar; navigate away — highlight clears
6. Check browser console — no import errors from deleted composable files
