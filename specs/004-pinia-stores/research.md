# Research: Pinia Stores for Vaults and Notes

**Feature**: `004-pinia-stores`
**Phase**: 0 — Research
**Date**: 2026-03-23

---

## 1. Setup Store vs Options Store

**Decision**: Use **Pinia setup store** syntax (`defineStore('id', () => { ... })`).

**Rationale**: Both composables already use `ref()` and plain functions — the setup store pattern maps directly with minimal diff. The code inside `defineStore` is identical to what the composables have now: module-level refs become store-local refs, functions stay as-is. Options store would require restructuring into `state`/`getters`/`actions` objects.

**Alternatives considered**: Options store — rejected; would require restructuring all methods, no benefit for this codebase.

---

## 2. Store Naming Convention

**Decision**:
- Function name: `useVaultStore`, `useNotesStore` (camelCase, `Store` suffix)
- Store ID: `'vaults'`, `'notes'` (lowercase, matches existing Pinia store ID patterns)
- File name: `stores/vaults.ts`, `stores/notes.ts` (matches composable naming, joins `stores/editor.ts`)

**Rationale**: The `Store` suffix makes it immediately clear at call sites that this is a Pinia store, not a composable. Pinia convention is `useXStore`. File names match the composables they replace.

---

## 3. Module-Level Refs vs Pinia: Why the Migration Is Worth It

The current pattern works but has concrete costs:

| Concern | Module-level refs | Pinia store |
|---------|------------------|-------------|
| Vue DevTools visibility | None | Full state + action tracking |
| Per-test isolation | Not possible without module mocking | `createPinia()` gives each test a fresh instance |
| HMR state preservation | Lost on module reload | `acceptHMRUpdate` preserves it |
| Logout state reset | Manual `ref.value = initial` scattered everywhere | Single `$reset()` call |
| Plugin support (persist, etc.) | Must reimplement | Built-in |

---

## 4. `$reset()` in Setup Stores

**Decision**: Implement `$reset()` manually in each store. Save initial state as constants before the store body, restore them in the reset function.

**Rationale**: Pinia's built-in `$reset()` only works with options stores. The manual approach is explicit and readable. A plugin approach (deep-cloning `$state`) is overkill for two stores.

```typescript
// Example pattern
export const useVaultStore = defineStore('vaults', () => {
  const vaults = ref<Vault[]>([])
  const activeVault = ref<Vault | null>(null)

  function $reset() {
    vaults.value = []
    activeVault.value = null
    loading.value = false
    error.value = null
  }

  return { vaults, activeVault, $reset, ... }
})
```

---

## 5. Vault-Keyed Notes Cache

**Decision**: `useNotesStore` uses `notesByVault: Ref<Record<string, NoteMeta[]>>` instead of a flat `notes` ref.

**Rationale**: The flat `notes` ref gets clobbered on each `loadList` call. With the cache, each vault has its own slot that persists across navigation. `AppLayout` calls `loadAll(vaultIds)` on mount to pre-populate the cache for all vaults. This was prototyped in feature 003 but reverted due to the composable approach being the wrong layer. Pinia is the right place for this.

**`notesFor(vaultId)`**: A plain function (not a computed) that returns `notesByVault.value[vaultId] ?? []`. Reactive when called inside a template or computed because the reactivity system tracks `notesByVault.value`.

---

## 6. `setActive(null)` on HomeView

**Decision**: `useVaultStore.setActive()` accepts `Vault | null`. `HomeView.onMounted` calls `setActive(null)` before loading vaults.

**Rationale**: Without clearing `activeVault`, navigating from any vault to HomeView leaves the last vault active. `NoteList` would show that vault's notes on the vault overview. HomeView has no active vault context.

---

## 7. HMR Support

**Decision**: Add `if (import.meta.hot) acceptHMRUpdate(useVaultStore, import.meta.hot)` and equivalent for `useNotesStore`.

**Rationale**: This is the standard Pinia pattern for Vite HMR. Without it, editing the store file during development resets all state. Already used in `stores/editor.ts` implicitly (editor store is stateless enough that it doesn't matter, but it's good practice).

---

## Summary Table

| Question | Decision | Rationale |
|----------|----------|-----------|
| Store syntax | Setup store | Identical to current composable structure; minimal diff |
| Naming | `useVaultStore`, `useNotesStore` | Clear Pinia convention; `Store` suffix distinguishes from composables |
| `$reset()` | Manual implementation | Options store `$reset()` not available for setup stores |
| Notes cache | `notesByVault` Record | Flat ref clobbered on vault switch; cache fixes vault repopulation bug |
| HomeView vault | `setActive(null)` on mount | Clears stale vault context on vaults overview |
| HMR | `acceptHMRUpdate` | Preserves state during dev hot-reloads |
