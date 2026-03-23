# Quickstart: Recent Notes Sidebar (Persistent)

**Feature**: `003-sidebar-recent-notes`
**Phase**: 1 — Design
**Date**: 2026-03-23

This guide summarises what is changing, what to build, and the order to build it in.

---

## What is changing

| Area | Change |
|------|--------|
| `frontend/src/composables/useNotes.ts` | Replace flat `notes` ref with `notesByVault` cache; add `notesFor()` and `loadAll()` |
| `frontend/src/composables/useVaults.ts` | Widen `setActive()` to accept `Vault \| null` |
| `frontend/src/components/AppLayout.vue` | **NEW** — persistent authenticated shell; calls `loadAll` on mount |
| `frontend/src/components/NoteList.vue` | Read from cache; sort by `updated_at`; limit to 20; active state from route param |
| `frontend/src/components/VaultSwitcher.vue` | Note count via `notesFor()` |
| `frontend/src/router/index.ts` | Authenticated routes become children of the `AppLayout` parent route |
| `frontend/src/views/HomeView.vue` | Remove sidebar; call `setActive(null)` on mount |
| `frontend/src/views/VaultView.vue` | Remove sidebar; use `notesFor()` for masonry |
| `frontend/src/views/NoteView.vue` | Remove left sidebar and toggle logic |
| `docs/interface/frontend.md` | Update component table, routing section, composables section |

No new types. No new composable files. `SidebarNav.vue` is unchanged.

---

## Build order

Follow this order to keep the codebase buildable at each step.

### Step 0: Update docs first

Update `docs/interface/frontend.md` to document:
- `AppLayout.vue` as the new persistent authenticated shell
- The router nesting change (authenticated routes as children of `AppLayout`)
- `useNotes` notesByVault cache and new exports
- Updated `NoteView` description (left panel now owned by `AppLayout`)

### Step 1: Extend `useNotes.ts`

File: `frontend/src/composables/useNotes.ts`

- Replace `const notes = ref<NoteMeta[]>([])` with `const notesByVault = ref<Record<string, NoteMeta[]>>({})`
- Add `notesFor(vaultId: string): NoteMeta[]` — returns `notesByVault.value[vaultId] ?? []`
- Add `loadAll(vaultIds: string[])` — `Promise.allSettled` over `fetchNotes(id)` per vault; writes results directly into `notesByVault` without touching `listLoading`
- Update `loadList` to write into `notesByVault.value[vaultId]` instead of the flat ref
- Update `create`, `save`, `remove`, `updatePermission` to mutate `notesByVault[vaultId]`

### Step 2: Widen `useVaults.setActive`

File: `frontend/src/composables/useVaults.ts`

Change signature from `setActive(vault: Vault)` to `setActive(vault: Vault | null)`.

### Step 3: Create `AppLayout.vue`

File: `frontend/src/components/AppLayout.vue`

Key behaviours:
- On `onMounted`: `await loadVaults()` then `await loadAll(vaults.value.map(v => v.id))`
- Renders the persistent left sidebar: `VaultSwitcher`, `SidebarNav`, `NoteList`
- Renders `<RouterView />` as the main content area
- Includes sidebar footer (Settings link bound to `activeVault`, Help link)
- Sidebar is always open at 288px — no toggle

### Step 4: Update `router/index.ts`

Wrap authenticated routes under an `AppLayout` parent entry:

```typescript
{
  path: '/',
  component: () => import('@/components/AppLayout.vue'),
  meta: { requiresAuth: true },
  children: [
    { path: '', component: () => import('@/views/HomeView.vue') },
    { path: 'vaults/:vault_id', component: () => import('@/views/VaultView.vue') },
    { path: 'vaults/:vault_id/notes/:id?', component: () => import('@/views/NoteView.vue') },
    // ... other authenticated routes
  ]
}
```

`LoginView` (`/login`) remains a top-level sibling. `meta: { requiresAuth: true }` moves to the parent; child routes inherit it.

### Step 5: Update `NoteList.vue`

- Remove `watch(activeVault, vault => loadList(vault.id))`
- Replace `notes` with `notesFor(activeVault.value.id)`
- Guard: return `[]` when `activeVault.value` is null
- Sort by `updated_at` desc, limit to `RECENT_LIMIT = 20`
- Active state: `route.params.id === note.id` (not `activeNote`)

### Step 6: Simplify existing views

- **`HomeView.vue`**: Remove sidebar. Add `setActive(null)` in `onMounted` before `loadVaults`. Root becomes `<div class="app-shell__main">`.
- **`VaultView.vue`**: Remove sidebar. Change `notes.value` to `notesFor(vaultId.value)` in `sortedNotes`. Root becomes `<div class="app-shell__main">`.
- **`NoteView.vue`**: Remove `<aside class="app-shell__sidebar">`, outer `app-shell` wrapper, `isSidebarOpen` ref, toggle button, and unused imports. Root becomes `<div class="app-shell__main">`.

### Step 7: Update `VaultSwitcher.vue`

Change note count from `notes.value.length` to `notesFor(activeVault.value?.id ?? '').length`.

---

## Key constraints to keep in mind

- No semicolons; single quotes in all frontend TS/Vue files.
- No new npm packages.
- No backend changes.
- No changes to `NoteList.vue`, `VaultSwitcher.vue`, or `SidebarNav.vue`.
- The `AppLayout` route must have `meta: { requiresAuth: true }` so the existing `beforeEach` guard still redirects unauthenticated users.

---

## How to verify the feature is complete

1. Visit `/` (HomeView) — the sidebar with `NoteList` is visible in the left panel.
2. Visit `/vaults/:id` (VaultView) — the sidebar is still visible.
3. Visit `/vaults/:id/notes/:id` (NoteView) — the sidebar is still visible (and there is no second note list).
4. Click a note entry in the sidebar from any view — you land on the correct note editor.
5. Navigate between all views without noticing a sidebar flicker (it does not remount).
