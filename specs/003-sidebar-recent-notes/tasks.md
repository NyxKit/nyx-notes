# Tasks: Recent Notes Sidebar (Persistent)

**Input**: Design documents from `/specs/003-sidebar-recent-notes/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ui-contract.md ✓, quickstart.md ✓

**Organization**: Tasks are grouped by phase. No new composable files or types are introduced.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2)
- Exact file paths are included in every task description

---

## Phase 1: Setup

**Purpose**: Update documentation before any source files are touched.

- [X] T001 Update `docs/interface/frontend.md` to document `AppLayout.vue`, the router nesting change, the `useNotes` vault cache (`notesByVault`, `notesFor`, `loadAll`), and the updated `NoteView` description

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Extend composables and create the shared shell. Must complete before view simplification.

- [X] T002 Extend `useNotes` in `frontend/src/composables/useNotes.ts` — replace `notes: Ref<NoteMeta[]>` with `notesByVault: Ref<Record<string, NoteMeta[]>>`; add `notesFor(vaultId)` (returns `notesByVault.value[vaultId] ?? []`) and `loadAll(vaultIds)` (Promise.allSettled over fetchNotes, writes results into notesByVault without touching listLoading); update `loadList`, `create`, `save`, `remove`, `updatePermission` to target notesByVault[vaultId] instead of the flat ref
- [X] T003 Widen `setActive` in `frontend/src/composables/useVaults.ts` — change signature from `(vault: Vault)` to `(vault: Vault | null)`
- [X] T004 Create `AppLayout.vue` in `frontend/src/components/AppLayout.vue` — calls `useVaults().load()` then `useNotes().loadAll(vaultIds)` on `onMounted`; renders left sidebar (VaultSwitcher, SidebarNav, NoteList, footer with Settings/Help links) and `<RouterView />`; sidebar always open at 288px; see contracts/ui-contract.md §1
- [X] T005 Update `frontend/src/router/index.ts` — nest all authenticated routes under `AppLayout` parent with `meta: { requiresAuth: true }`; LoginView remains top-level sibling; see contracts/ui-contract.md §2

**Checkpoint**: `AppLayout` mounts, loads all vaults and all vault notes, and renders child route via `<RouterView />`.

---

## Phase 3: User Story 1 — Access Recent Notes from Any View (Priority: P1) 🎯 MVP

**Goal**: The vault-scoped note list is visible on every authenticated view with correct data.

**Independent Test**: Navigate to HomeView (`/`), VaultView (`/vaults/:id`), and NoteView (`/vaults/:id/notes/:id`); verify the "Recent Notes" sidebar is visible and shows correct notes; click an entry and verify you land on the correct note editor.

- [X] T006 [US1] Update `NoteList.vue` in `frontend/src/components/NoteList.vue` — remove `watch(activeVault, loadList)`; replace `notes` with `notesFor(activeVault.value.id)`; guard with `if (!activeVault.value) return []`; sort by `updated_at` desc; limit to `RECENT_LIMIT = 20`; derive active state from `route.params.id` instead of `activeNote`; see contracts/ui-contract.md §4
- [X] T007 [P] [US1] Simplify `HomeView.vue` in `frontend/src/views/HomeView.vue` — remove outer `app-shell` wrapper, `app-shell__sidebar` aside, sidebar CSS; call `setActive(null)` in `onMounted` before `loadVaults`; root becomes `<div class="app-shell__main">`
- [X] T008 [P] [US1] Simplify `VaultView.vue` in `frontend/src/views/VaultView.vue` — remove outer `app-shell` wrapper and sidebar aside; change `notes.value` to `notesFor(vaultId.value)` in `sortedNotes`; root becomes `<div class="app-shell__main">`
- [X] T009 [US1] Simplify `NoteView.vue` in `frontend/src/views/NoteView.vue` — remove `<aside class="app-shell__sidebar">` block, outer `app-shell` wrapper, `isSidebarOpen` ref, `sidebarVisible` computed, hamburger toggle, and unused imports; root becomes `<div class="app-shell__main">`
- [X] T010 [P] [US1] Update `VaultSwitcher.vue` in `frontend/src/components/VaultSwitcher.vue` — replace `notes.value.length` with `notesFor(activeVault.value?.id ?? '').length` for the note count label

**Checkpoint**: US1 complete. Sidebar present on all views. No duplicate list in NoteView. Switching vaults shows correct notes from cache.

---

## Phase 4: User Story 2 — Recent Notes Persist Across Navigation (Priority: P2)

**Goal**: The sidebar stays accurate across navigation; switching vaults shows the correct notes immediately without a loading flash.

**Independent Test**: Switch from a vault with 0 notes to a vault with 2 notes and back; verify the note count updates instantly without a blank/stale state.

- [X] T011 [US2] Verify cache behaviour — navigating between vaults reads from `notesByVault` cache populated by `AppLayout.onMounted`; no watcher or re-fetch needed; confirmed by manual smoke test across HomeView → VaultView → NoteView → HomeView

**Checkpoint**: US2 complete. The cache-backed sidebar reflects each vault's notes immediately on navigation.

---

## Dependencies & Execution Order

- **Phase 1**: No dependencies
- **Phase 2**: T002 → T003 → T004 → T005 (in order; each depends on previous)
- **Phase 3**: Depends on Phase 2; T007, T008, T010 are parallel; T006, T009 are sequential after T005
- **Phase 4**: Structurally satisfied by Phase 2+3; T011 is a validation step only

---

## Notes

- No semicolons; single quotes in all frontend TS/Vue files
- No new npm packages permitted
- `SidebarNav.vue` is unchanged
- `AppLayout` carries `meta: { requiresAuth: true }` for the existing `beforeEach` guard
- `loadAll` in `useNotes` does not set `listLoading` — it is a background prefetch; `VaultView` still calls `loadList` for per-visit freshness
- Search in `NoteList` applies before the sort+limit step (searches all vault notes, not just top 20)
