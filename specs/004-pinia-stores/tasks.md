# Tasks: Pinia Stores for Vaults and Notes

**Input**: Design documents from `/specs/004-pinia-stores/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/store-contract.md ✓, quickstart.md ✓

**Organization**: Tasks are grouped by user story. No new npm packages. No tests explicitly requested — test tasks omitted.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3)

---

## Phase 1: Setup

**Purpose**: Update docs before touching any source files.

- [X] T001 Update `docs/interface/frontend.md` — replace `useVaults.ts` and `useNotes.ts` composable descriptions with `useVaultStore` and `useNotesStore` Pinia store entries; update composables section to reflect only `useAuth`, `useComments`, `useTeams` remain; add `stores/` tree entry alongside existing `stores/editor.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Create both stores. All user story work and call site updates depend on these existing first.

- [X] T002 Create `frontend/src/stores/vaults.ts` — Pinia setup store `useVaultStore` with store ID `'vaults'`; move all module-level refs from `useVaults.ts` into the store body: `vaults`, `activeVault`, `loading`, `error`; preserve all existing actions (`load`, `create`, `remove`, `patchPermission`, `addTeamVault`, `removeTeamVault`); widen `setActive` to accept `Vault | null`; add `$reset()` that restores all refs to initial values; add `if (import.meta.hot) acceptHMRUpdate(useVaultStore, import.meta.hot)` at the bottom; see contracts/store-contract.md §1
- [X] T003 Create `frontend/src/stores/notes.ts` — Pinia setup store `useNotesStore` with store ID `'notes'`; use `notesByVault: ref<Record<string, NoteMeta[]>>({})` (vault-keyed cache, not a flat notes ref); add `notesFor(vaultId: string): NoteMeta[]` returning `notesByVault.value[vaultId] ?? []`; add `loadAll(vaultIds: string[])` using `Promise.allSettled` writing results into the cache without touching `listLoading`; preserve all existing actions (`loadList`, `loadNote`, `create`, `save`, `remove`, `updatePermission`) updated to read/write `notesByVault[vaultId]` instead of a flat ref; add `$reset()`; add `acceptHMRUpdate`; see contracts/store-contract.md §2

**Checkpoint**: Both stores exist and compile. Vue DevTools Pinia panel should now show `vaults` and `notes` stores.

---

## Phase 3: User Story 1 — Vault State is Inspectable and Resettable (Priority: P1)

**Goal**: `useVaultStore` is wired everywhere `useVaults` was used; app works identically to before.

**Independent Test**: Open Vue DevTools Pinia panel — `vaults` store is listed with full state. Navigate between views; vault list and activeVault update as expected.

- [X] T004 [P] [US1] Update `frontend/src/components/AppLayout.vue` — replace `import { useVaults }` with `import { useVaultStore } from '@/stores/vaults'`; replace `import { useNotes }` with `import { useNotesStore } from '@/stores/notes'`; call `const vaultStore = useVaultStore()` and `const notesStore = useNotesStore()`; update `onMounted` to `await vaultStore.load()` then `await notesStore.loadAll(vaultStore.vaults.map(v => v.id))`; update template refs (`activeVault` → `vaultStore.activeVault`)
- [X] T005 [P] [US1] Update `frontend/src/components/VaultSwitcher.vue` — replace `useVaults` import with `useVaultStore` from `@/stores/vaults`; replace `useNotes` / `notes` with `useNotesStore().notesFor(activeVault?.id ?? '')` for the count label; update all `setActive`, `vaults`, `activeVault` references to use the store
- [X] T006 [P] [US1] Update `frontend/src/components/SidebarNav.vue` — replace `useVaults` with `useVaultStore` from `@/stores/vaults`; replace `useNotes` with `useNotesStore` from `@/stores/notes`; update all destructured references accordingly
- [X] T007 [P] [US1] Update `frontend/src/views/HomeView.vue` — replace `useVaults` import with `useVaultStore` from `@/stores/vaults`; call `vaultStore.setActive(null)` at start of `onMounted` before `vaultStore.load()`; update all destructured references
- [X] T008 [P] [US1] Update `frontend/src/views/VaultView.vue` — replace `useVaults` and `useNotes` imports with store equivalents; update `onMounted` to use store actions; change `sortedNotes` computed to use `notesStore.notesFor(vaultId.value)`
- [X] T009 [P] [US1] Update `frontend/src/views/NoteView.vue` — replace `useVaults` and `useNotes` imports with store equivalents; update all references (`loadVaults`, `setActive`, `loadNote`, `activeNote`, `saving`, `remove`) to use store instances
- [X] T010 [P] [US1] Update `frontend/src/views/VaultSettingsView.vue` — replace `useVaults` import with `useVaultStore` from `@/stores/vaults`; update all destructured references (`vaults`, `load`, `remove`, `patchPermission`)
- [X] T011 [P] [US1] Update `frontend/src/views/TeamSettingsView.vue` — replace `useVaults` import with `useVaultStore` from `@/stores/vaults`; update all destructured references (`vaults`, `load`, `patchPermission`, `addTeamVault`, `removeTeamVault`)

**Checkpoint**: All vault-related call sites use `useVaultStore`. App navigates correctly. DevTools shows vault state.

---

## Phase 4: User Story 2 — Notes Cache Survives Vault Switching (Priority: P1)

**Goal**: `useNotesStore` is wired everywhere `useNotes` was used; `NoteList` reads from the cache.

**Independent Test**: Switch from a 0-note vault to a 2-note vault — sidebar shows correct notes instantly. Navigate to HomeView — sidebar is empty.

- [X] T012 [US2] Update `frontend/src/components/NoteList.vue` — replace `useVaults` with `useVaultStore`; replace `useNotes` with `useNotesStore`; remove `watch(activeVault, loadList)` (AppLayout pre-loads); use `notesStore.notesFor(activeVault?.id ?? '')` guarded by `if (!activeVault) return []`; sort by `updated_at` desc; slice to `RECENT_LIMIT = 20`; active state: `route.params.id === note.id`; see contracts/store-contract.md §5
- [X] T013 [US2] Update `frontend/src/components/NoteEditor.vue` — replace `useNotes` import with `useNotesStore` from `@/stores/notes`; update `save` and `updatePermission` call sites to use the store

**Checkpoint**: US2 complete. Sidebar correctly reflects active vault's notes from cache. No loading flash on vault switch. HomeView shows empty sidebar.

---

## Phase 5: User Story 3 — Notes State is Inspectable and Resettable (Priority: P2)

**Goal**: `useNotesStore` appears in Vue DevTools; `$reset()` clears all note state.

**Independent Test**: Open Vue DevTools Pinia panel — `notes` store listed with `notesByVault`, `activeNote`, loading flags visible.

- [X] T014 [US3] Delete `frontend/src/composables/useVaults.ts` — remove the file entirely once all callers in T004–T011 are confirmed updated
- [X] T015 [US3] Delete `frontend/src/composables/useNotes.ts` — remove the file entirely once all callers in T012–T013 are confirmed updated

**Checkpoint**: US3 complete. Both composable files are gone. No dangling imports. `notes` store visible in DevTools.

---

## Phase 6: Polish

- [X] T016 Verify `frontend/src/main.ts` — confirm `createPinia()` is already passed to the app (should already be present since `useEditorStore` works); no change needed if present
- [X] T017 [P] Update `CLAUDE.md` active technologies entry for `004-pinia-stores` — replace references to `useNotes` / `useVaults` composables with `useNotesStore` / `useVaultStore` Pinia stores

---

## Dependencies & Execution Order

- **Phase 1** (T001): No dependencies
- **Phase 2** (T002, T003): Depends on Phase 1; T002 and T003 can run in parallel
- **Phase 3** (T004–T011): All depend on T002 (useVaultStore must exist); T004–T011 are fully parallel with each other
- **Phase 4** (T012, T013): Depends on T003 (useNotesStore must exist); T012 and T013 can run in parallel
- **Phase 5** (T014, T015): T014 depends on T004–T011 complete; T015 depends on T012–T013 complete; T014 and T015 can run in parallel
- **Phase 6** (T016, T017): No blocking dependencies; T016 and T017 can run in parallel

### Parallel Opportunities

```
Phase 2:  T002 ‖ T003
Phase 3:  T004 ‖ T005 ‖ T006 ‖ T007 ‖ T008 ‖ T009 ‖ T010 ‖ T011
Phase 4:  T012 ‖ T013
Phase 5:  T014 ‖ T015  (after all callers done)
Phase 6:  T016 ‖ T017
```

---

## Implementation Strategy

### MVP (All stories are P1/P2 — do all in order)

1. T001 — docs
2. T002 ‖ T003 — create both stores
3. T004–T011 — update all vault call sites
4. T012–T013 — update notes call sites (NoteList cache behaviour)
5. T014–T015 — delete old composable files
6. T016–T017 — polish

---

## Notes

- No semicolons; single quotes in all frontend TS/Vue files
- No new npm packages — Pinia is already installed
- `loadAll` must NOT set `listLoading` — it is a background prefetch; `VaultView`'s loading skeleton is controlled by `loadList` only
- `$reset()` in both stores is manually implemented (setup stores don't get built-in `$reset()`)
- `acceptHMRUpdate` must be the last line in both store files
- Search in NoteList applies before sort+limit (searches all vault notes, not just top 20)
