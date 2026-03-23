# Research: Recent Notes Sidebar (Persistent)

**Feature**: `003-sidebar-recent-notes`
**Phase**: 0 — Research
**Date**: 2026-03-23

---

## 1. Approach: Persistent Shell via Nested Routes

**Decision**: Create `AppLayout.vue` as the shared authenticated shell; nest all authenticated routes under it in Vue Router using the `children` array.

**Rationale**: This is the canonical Vue 3 pattern for persistent layouts. The parent component (`AppLayout`) mounts once and never remounts on child route changes — `<RouterView />` swaps only the child content. No per-view coordination or event bus needed.

**Alternatives considered**:
- Keep sidebar in each view and synchronise state — rejected; doesn't solve the remounting problem and adds complexity.
- Move sidebar into `App.vue` with conditional rendering — rejected; conflates routing logic with app root.

---

## 2. Scope: Vault-Scoped, Not Cross-Vault

**Decision**: Use the existing `NoteList` component (vault-scoped) as the sidebar content. Cross-vault aggregation is explicitly deferred.

**Rationale**: The user confirmed they prefer the look of the existing `NoteList` UI. Adding a new cross-vault component created a duplicate list when both appeared in `NoteView`. The simplest correct solution is to make the existing component persistent rather than introduce a parallel one.

**What this means at runtime**:
- On `HomeView` with no active vault: `NoteList` shows nothing or its natural idle state.
- On `VaultView` / `NoteView`: `NoteList` shows notes for the active vault, sorted by `updated_at` desc.
- Behaviour is identical to the existing `NoteView` sidebar — it is now just always present.

---

## 3. Sidebar Toggle Removal

**Decision**: Remove the `isSidebarOpen` toggle and hamburger button from `NoteView`.

**Rationale**: The sidebar is now always visible (fixed width 288px, same as the `--open` state). A toggle that hides the persistent sidebar would undermine the feature's goal. The toggle was only needed when the sidebar was per-view and conditionally mounted.

---

## 4. `AppLayout` Load Strategy

**Decision**: `AppLayout.onMounted` calls `useVaults().load()`. Individual views that also call `loadVaults()` will trigger a redundant fetch — this is harmless (idempotent overwrite of module-level ref) and out of scope to fix here.

**Rationale**: Minimum-diff approach. Modifying each view to guard against double-loading is a separate cleanup task unrelated to this feature.

---

## 5. Notes Cache (`notesByVault` Map)

**Decision**: Replace the single `notes: Ref<NoteMeta[]>` in `useNotes` with `notesByVault: Ref<Record<string, NoteMeta[]>>`. `AppLayout` calls `loadAll(vaultIds)` on mount. `NoteList` reads `notesFor(activeVaultId)` from the cache.

**Rationale**: The flat ref was clobbered on each `loadList` call. Navigating WorkVault → HomeView left `activeVault` unchanged, so `NoteList` never re-fetched and kept showing the wrong (stale) notes. The map gives each vault its own slot — switching vaults reads correct pre-loaded data instantly.

**Alternatives considered**: Re-triggering `loadList` on every `activeVault` change — rejected; still produces a loading flash, and fails when `activeVault` doesn't change (e.g. WorkVault → HomeView).

---

## 6. HomeView Clears `activeVault`

**Decision**: `HomeView.onMounted` calls `setActive(null)`. `setActive` is widened to `Vault | null`.

**Rationale**: Without this, navigating from any vault to HomeView leaves `activeVault` pointing at the last vault. `NoteList` would show that vault's notes on the vaults overview. The home view has no active vault context, so the sidebar should be empty.

---

## 7. Active State Derived from Route Param

**Decision**: `NoteList` derives the active item from `route.params.id` instead of `useNotes().activeNote`.

**Rationale**: `activeNote` persists after leaving NoteView. On HomeView or VaultView the last-opened note would still appear highlighted in the sidebar. `route.params.id` is naturally `undefined` on non-note routes.

---

## Summary Table

| Unknown | Decision | Rationale |
|---------|----------|-----------|
| How to make sidebar persistent | `AppLayout.vue` with Vue Router nested routes | Canonical pattern; no per-view coordination |
| Cross-vault or vault-scoped | Vault-scoped — use existing `NoteList` | Avoids duplicate list; preserves existing UI |
| Sidebar toggle | Remove from NoteView | Sidebar is now always open; toggle is counterproductive |
| Double vault load | Accept as-is | Out of scope; harmless |
| Notes not repopulating on vault switch | `notesByVault` cache + `loadAll` in AppLayout | One fetch per vault on mount; slots never clobber each other |
| Sidebar showing stale active state | Derive from `route.params.id` | Naturally `undefined` on non-note routes |
| Sidebar content on HomeView | `setActive(null)` in HomeView.onMounted | No active vault = empty sidebar |
