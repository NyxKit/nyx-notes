# Store Contract: Pinia Stores for Vaults and Notes

**Feature**: `004-pinia-stores`
**Phase**: 1 — Design
**Date**: 2026-03-23

---

## 0. Consuming Stores — `storeToRefs` Rule

Always use `storeToRefs` when destructuring reactive state. Actions (functions) are destructured directly from the store instance.

```typescript
import { storeToRefs } from 'pinia'
import { useVaultStore } from '@/stores/vaults'

const vaultStore = useVaultStore()
const { vaults, activeVault, loading, error } = storeToRefs(vaultStore)  // refs — reactive
const { load, create, remove, setActive } = vaultStore                    // actions — no wrapper needed
```

Omitting `storeToRefs` silently strips ref wrappers, breaking reactivity (component never re-renders on state change).

---

## 1. `useVaultStore` — `stores/vaults.ts`

### Public interface

```typescript
import { storeToRefs } from 'pinia'
import { useVaultStore } from '@/stores/vaults'

const vaultStore = useVaultStore()

// State — destructure via storeToRefs
const { vaults, activeVault, loading, error } = storeToRefs(vaultStore)

// Actions — destructure directly
const { load, create, remove, setActive, patchPermission, addTeamVault, removeTeamVault, $reset } = vaultStore
```

### Invariants

- `activeVault` is always either `null` or an element of `vaults`
- After `remove(id)`, `activeVault` is null if it matched the removed vault
- `$reset()` sets `vaults = []`, `activeVault = null`, `loading = false`, `error = null`

---

## 2. `useNotesStore` — `stores/notes.ts`

### Public interface

```typescript
import { storeToRefs } from 'pinia'
import { useNotesStore } from '@/stores/notes'

const notesStore = useNotesStore()

// State — destructure via storeToRefs
const { notesByVault, activeNote, listLoading, loading, saving, error } = storeToRefs(notesStore)

// Actions/functions — destructure directly
const { notesFor, loadAll, loadList, loadNote, create, save, remove, updatePermission, $reset } = notesStore
```

### Invariants

- `notesFor(vaultId)` always returns an array (never undefined)
- `loadAll` does not modify `listLoading` — it is a background cache fill
- `loadList` sets `listLoading = true` during fetch — used by VaultView skeleton
- After `remove(vaultId, id)`, entry is absent from `notesByVault[vaultId]`; `activeNote` is null if matched
- `$reset()` sets `notesByVault = {}`, `activeNote = null`, all flags `false`, `error = null`

---

## 3. AppLayout Behaviour

`AppLayout.onMounted` orchestrates the initial data load:

```
1. await useVaultStore().load()            → populates vaults
2. await useNotesStore().loadAll(vaultIds) → populates notesByVault for every vault
```

This ensures the sidebar has all data before the user interacts with any vault.

---

## 4. HomeView Behaviour

`HomeView.onMounted` clears the active vault:

```
1. useVaultStore().setActive(null)   → clears activeVault
2. await useVaultStore().load()      → refreshes vault list
```

`NoteList` reads `notesFor(activeVault?.id)` — with `activeVault = null` it returns `[]`, so the sidebar is empty on HomeView.

---

## 5. `NoteList` Cache Contract

`NoteList` reads from the store cache — it does **not** trigger any API calls:

```
activeVault = null          → filtered = []
activeVault = { id: 'x' }  → filtered = notesFor('x'), sorted by updated_at desc, sliced to 20
```

Active item: `route.params.id === note.id` (not `activeNote`) — `undefined` on non-note routes.
