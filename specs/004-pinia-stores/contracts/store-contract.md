# Store Contract: Pinia Stores for Vaults and Notes

**Feature**: `004-pinia-stores`
**Phase**: 1 — Design
**Date**: 2026-03-23

---

## 1. `useVaultStore` — `stores/vaults.ts`

### Public interface

```typescript
import { useVaultStore } from '@/stores/vaults'

const store = useVaultStore()

// State (reactive, read directly)
store.vaults        // Vault[]
store.activeVault   // Vault | null
store.loading       // boolean
store.error         // string | null

// Actions
await store.load()
const vault = await store.create(body)
await store.remove(vaultId)
store.setActive(vault)   // Vault | null
await store.patchPermission(teamId, vaultId, permission)
const vault = await store.addTeamVault(teamId, body)
await store.removeTeamVault(teamId, vaultId)
store.$reset()
```

### Invariants

- `activeVault` is always either `null` or an element of `vaults`
- After `remove(id)`, `activeVault` is null if it matched the removed vault
- `$reset()` sets `vaults = []`, `activeVault = null`, `loading = false`, `error = null`

---

## 2. `useNotesStore` — `stores/notes.ts`

### Public interface

```typescript
import { useNotesStore } from '@/stores/notes'

const store = useNotesStore()

// State
store.notesByVault  // Record<string, NoteMeta[]>
store.activeNote    // Note | null
store.listLoading   // boolean
store.loading       // boolean
store.saving        // boolean
store.error         // string | null

// Cache accessor (reactive when called inside computed/template)
store.notesFor(vaultId)   // NoteMeta[]

// Actions
await store.loadAll(vaultIds)
await store.loadList(vaultId)
await store.loadNote(vaultId, id)
const meta = await store.create(vaultId, body)
const meta = await store.save(vaultId, id, body)
await store.remove(vaultId, id)
const meta = await store.updatePermission(vaultId, id, permission)
store.$reset()
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
