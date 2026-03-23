# UI Contract: Recent Notes Sidebar (Persistent)

**Feature**: `003-sidebar-recent-notes`
**Phase**: 1 — Design
**Date**: 2026-03-23

---

## 1. `AppLayout.vue`

**Purpose**: Persistent authenticated shell. Renders the left sidebar (VaultSwitcher, SidebarNav, NoteList, sidebar footer) and `<RouterView />` for page content. Mounted once per authenticated session.

### Props

None.

### Slots

| Slot | Description |
|------|-------------|
| *(default via RouterView)* | The active authenticated view |

### Emits

None.

### Responsibilities

- Load vault list on mount (`useVaults().load()`)
- Load notes for all vaults on mount (`useNotes().loadAll(vaultIds)`)
- Render left sidebar always open (width 288px, no toggle)
- Render `<RouterView />` for the active child route
- Render sidebar footer (Settings link bound to `activeVault`, Help link)

### Does NOT own

- Note editor, comments sidebar, per-note toolbar — remain in `NoteView.vue`
- Route-specific headers — each view renders its own header

---

## 2. Router Contract Change

All authenticated routes become children of an `AppLayout` parent route.

### Before (flat)

```
/                          → HomeView       (meta: { requiresAuth: true })
/vaults/:vault_id          → VaultView      (meta: { requiresAuth: true })
/vaults/:vault_id/notes/:id? → NoteView    (meta: { requiresAuth: true })
...
/login                     → LoginView
```

### After (nested)

```
/                          → AppLayout      (meta: { requiresAuth: true })
  /                        → HomeView
  /vaults/:vault_id        → VaultView
  /vaults/:vault_id/notes/:id? → NoteView
  ...
/login                     → LoginView
```

**Guard contract**: `meta.requiresAuth` moves to the `AppLayout` parent. Child routes inherit it. The existing `beforeEach` guard logic is unchanged.

---

## 3. `useNotes` Composable Changes

The module-level `notes` ref is replaced with a vault-keyed cache.

### New interface additions

| Export | Type | Description |
|--------|------|-------------|
| `notesByVault` | `Ref<Record<string, NoteMeta[]>>` | Cache keyed by vault ID |
| `notesFor(vaultId)` | `(string) => NoteMeta[]` | Returns cached notes for one vault; `[]` if not yet loaded |
| `loadAll(vaultIds)` | `(string[]) => Promise<void>` | Fetches all vaults in parallel; does not set `listLoading` |

All mutation methods (`create`, `save`, `remove`, `updatePermission`) keep the same signatures but now update `notesByVault[vaultId]` instead of the removed flat `notes` ref.

---

## 4. `NoteList.vue` Changes

| Behaviour | Before | After |
|-----------|--------|-------|
| Data source | `useNotes().notes` (flat ref) | `notesFor(activeVault.id)` (cache lookup) |
| Load trigger | `watch(activeVault, loadList)` | None — `AppLayout` pre-loads all vaults |
| Null vault guard | No guard | Returns `[]` when `activeVault` is null |
| Sort | Not sorted (relied on API order) | Sorted by `updated_at` descending |
| Limit | No limit | Capped at `RECENT_LIMIT = 20` entries |
| Active item | `activeNote?.meta.id === note.id` | `route.params.id === note.id` |

Search applies before the sort+limit step, so it searches all vault notes rather than only the top 20.

---

## 5. View Simplification Contract

Each view's root element changes from the full `app-shell` wrapper to just the main content column.

| View | Root before | Root after |
|------|-------------|------------|
| `HomeView.vue` | `<div class="app-shell">` (sidebar + main) | `<div class="app-shell__main">` |
| `VaultView.vue` | `<div class="app-shell">` (sidebar + main) | `<div class="app-shell__main">` |
| `NoteView.vue` | `<div class="app-shell">` (sidebar + main) | `<div class="app-shell__main">` |

`HomeView` additionally calls `setActive(null)` on mount, clearing the active vault when the user is on the vaults overview. `useVaults.setActive` accepts `Vault | null`.
