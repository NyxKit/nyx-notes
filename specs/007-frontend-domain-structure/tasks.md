# Tasks: Frontend Domain-Based Structure

**Input**: Design documents from `/specs/007-frontend-domain-structure/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, quickstart.md ✅

**Tests**: Not requested — this is a pure structural refactor. Verification is by successful build and manual route check.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (US1, US2, US3)
- All paths are relative to `app/src/`

---

## Phase 1: Setup

**Purpose**: Update docs before any code changes (constitution requirement: docs-first)

- [x] T001 Update `docs/interface/frontend.md` to document the new domain-based `src/` structure — replace the current flat layout description with the domain tree from `specs/007-frontend-domain-structure/plan.md`

---

## Phase 2: Foundational — Shared Layer Migration

**Purpose**: Move cross-domain shared code first so domain files can import from it correctly after their own moves.

**⚠️ CRITICAL**: No domain file moves can begin until the shared layer is in place — domain API modules import `client.ts`; all domain files import `types/index.ts`.

- [x] T002 Move `api/client.ts` → `shared/api/client.ts` (no import changes needed in this file)
- [x] T003 [P] Move `types/index.ts` → `shared/types/index.ts` (no import changes needed in this file)
- [x] T004 [P] Move `utils/time.ts` → `shared/utils/time.ts` (no import changes needed in this file)
- [x] T005 [P] Move `components/AppLayout.vue` → `shared/components/AppLayout.vue` (`git mv` only — do NOT update internal imports yet; domain targets don't exist until Phase 3)
- [x] T006 [P] Move `components/SidebarNav.vue` → `shared/components/SidebarNav.vue` (`git mv` only — do NOT update internal imports yet)
- [x] T007 [P] Move `assets/theme.css` and `assets/vue.svg` → `shared/assets/`
- [x] T008 [P] Move the 20 generic outline SVGs from `assets/icons/*.svg` → `shared/assets/icons/` (do not move `assets/icons/vaults/` — that goes to the vaults domain in Phase 3)
- [x] T009 Move `router/index.ts` → `shared/router/index.ts` (`git mv` only — do NOT update import paths yet; `useAuth` and all view imports still point to flat paths, which still resolve since flat domain files haven't moved yet)
- [x] T010 Update `main.ts` (stays at `src/main.ts`): change `@/router` → `@/shared/router`; change `@/assets/theme.css` → `@/shared/assets/theme.css`

**Checkpoint**: Shared layer files are at their new locations. `main.ts` compiles. Internal imports in T005/T006/T009 are intentionally stale (they still resolve because domain files haven't moved yet). Domain file moves can now begin in parallel.

---

## Phase 3: User Story 1 — All Domain Code Colocated (Priority: P1) 🎯 MVP

**Goal**: Every domain-specific file lives inside its domain folder. A developer opening `vaults/`, `notes/`, `comments/`, `auth/`, or `teams/` finds all files for that domain.

**Independent Test**: Open `app/src/` — the only top-level entries are `main.ts`, `App.vue`, `vite-env.d.ts`, the five domain folders, and `shared/`. No `components/`, `stores/`, `views/`, `api/`, or `composables/` folders remain.

### Vaults domain

- [x] T011 [P] [US1] Move `api/vaults.ts` → `vaults/api/vaults.ts`; update `client` import: `@/api/client` → `@/shared/api/client`
- [x] T012 [P] [US1] Move `stores/vaults.ts` → `vaults/stores/vaults.ts`; update `@/types` → `@/shared/types`
- [x] T013 [P] [US1] Move the 20 solid vault SVGs from `assets/icons/vaults/*.svg` → `vaults/assets/icons/`
- [x] T014 [P] [US1] Move vault components to `vaults/components/`: `VaultCard.vue`, `VaultIcon.vue`, `VaultIconPicker.vue`, `VaultSwitcher.vue`; in each file update `@/types` → `@/shared/types`, `@/stores/vaults` → `@/vaults/stores/vaults`, and dynamic SVG asset paths `assets/icons/vaults/` → correct path under `@/vaults/assets/icons/`
- [x] T015 [P] [US1] Move vault views to `vaults/views/`: `HomeView.vue`, `VaultView.vue`, `VaultSettingsView.vue`; in each file update `@/stores/vaults` → `@/vaults/stores/vaults`, `@/api/vaults` → `@/vaults/api/vaults`, `@/types` → `@/shared/types`, component imports to `@/vaults/components/...`

### Notes domain

- [x] T016 [P] [US1] Move `api/notes.ts` → `notes/api/notes.ts`; update `@/api/client` → `@/shared/api/client`
- [x] T017 [P] [US1] Move `stores/notes.ts` and `stores/editor.ts` → `notes/stores/`; update `@/types` → `@/shared/types` in both files
- [x] T018 [P] [US1] Move notes components to `notes/components/`: `NoteEditor.vue`, `NoteList.vue`, `NoteToolbar.vue`; in each file update `@/types` → `@/shared/types`, store imports → `@/notes/stores/...`
- [x] T019 [P] [US1] Move `views/NoteView.vue` → `notes/views/NoteView.vue`; update `@/api/notes` → `@/notes/api/notes`, `@/stores/notes` → `@/notes/stores/notes`, `@/stores/editor` → `@/notes/stores/editor`, component imports → `@/notes/components/...`, `@/types` → `@/shared/types`

### Comments domain

- [x] T020 [P] [US1] Move `api/comments.ts` → `comments/api/comments.ts`; update `@/api/client` → `@/shared/api/client`
- [x] T021 [P] [US1] Move `composables/useComments.ts` → `comments/composables/useComments.ts`; update `@/api/comments` → `@/comments/api/comments`, `@/types` → `@/shared/types`
- [x] T022 [P] [US1] Move comment components to `comments/components/`: `CommentComposer.vue`, `CommentSidebar.vue`, `CommentThread.vue`; in each file update `@/types` → `@/shared/types`, `@/composables/useComments` → `@/comments/composables/useComments`, `@/api/comments` → `@/comments/api/comments`

### Auth domain

- [x] T023 [P] [US1] Move `api/auth.ts` → `auth/api/auth.ts`; update `@/api/client` → `@/shared/api/client`
- [x] T024 [P] [US1] Move `composables/useAuth.ts` → `auth/composables/useAuth.ts`; update `@/api/auth` → `@/auth/api/auth`, `@/types` → `@/shared/types`
- [x] T025 [P] [US1] Move `views/LoginView.vue` → `auth/views/LoginView.vue`; update `@/composables/useAuth` → `@/auth/composables/useAuth`, `@/types` → `@/shared/types`

### Teams domain

- [x] T026 [P] [US1] Move `api/teams.ts` → `teams/api/teams.ts`; update `@/api/client` → `@/shared/api/client`
- [x] T027 [P] [US1] Move `composables/useTeams.ts` → `teams/composables/useTeams.ts`; update `@/api/teams` → `@/teams/api/teams`, `@/types` → `@/shared/types`
- [x] T028 [P] [US1] Move `views/TeamSettingsView.vue` → `teams/views/TeamSettingsView.vue`; update `@/composables/useTeams` → `@/teams/composables/useTeams`, `@/types` → `@/shared/types`

### Shared layer import consolidation

- [x] T029 [US1] Update all import paths in the three shared files that were moved in Phase 2 without import updates: in `shared/router/index.ts` update `@/composables/useAuth` → `@/auth/composables/useAuth`, `@/components/AppLayout.vue` → `@/shared/components/AppLayout.vue`, and all `@/views/...` → their new domain paths (see data-model.md import table); in `shared/components/AppLayout.vue` update any vault/notes component imports to `@/vaults/components/...` etc.; in `shared/components/SidebarNav.vue` update any domain imports similarly

**Checkpoint**: All domain files are in their domain folders. All import paths are updated. Run `pnpm build` — fix any remaining import error before proceeding.

---

## Phase 4: User Story 2 — Shared Layer Integrity (Priority: P2)

**Goal**: Every file in `shared/` is genuinely cross-domain; every cross-domain import everywhere in the codebase resolves to `@/shared/...`.

**Independent Test**: A developer can open `app/src/shared/` and confirm every file there is used by more than one domain. A developer can `grep -r "@/types\|@/api/client\|@/utils/time\|@/router" app/src/` and find zero results (all such imports now use `@/shared/`).

- [x] T030 [US2] Audit `app/src/` with `grep -r "@/types\|@/api/client\|@/utils/time\|@/composables/\|@/stores/\|@/views/\|@/components/" app/src/` and fix any stale `@/` imports that were not updated during Phase 3 moves
- [x] T031 [P] [US2] Verify `shared/router/index.ts` resolves: confirm every dynamic `import(...)` path points to an existing file under the new domain structure
- [x] T032 [P] [US2] Verify `shared/components/AppLayout.vue` and `shared/components/SidebarNav.vue` use correct `@/vaults/...` or `@/shared/...` imports for any sub-components they reference

**Checkpoint**: `grep` audit returns zero stale imports. `pnpm build` passes cleanly.

---

## Phase 5: User Story 3 — Clean Up and Verify Isolation (Priority: P3)

**Goal**: Old flat directories are removed. Each domain is self-contained — removing one domain folder requires changes only to the router and shared layer, not to other domain folders.

**Independent Test**: Delete or stub out `app/src/comments/` — the only files outside `comments/` that reference it are `shared/router/index.ts` (the route entry) and any `shared/components/` that use comment components (e.g., `AppLayout.vue`). No `vaults/` or `notes/` file should import from `comments/`.

- [x] T033 [US3] Delete the now-empty flat directories: `app/src/api/`, `app/src/components/`, `app/src/composables/`, `app/src/stores/`, `app/src/views/`, `app/src/router/`, `app/src/types/`, `app/src/utils/`, `app/src/assets/icons/`, `app/src/assets/` (the `vaults/` subfolder was already moved in T013; the generic icons were moved in T008; `assets/` itself is empty after T007/T008)
- [x] T034 [US3] Run `pnpm build` from `app/` — confirm zero TypeScript and import errors
- [x] T035 [P] [US3] Verify cross-domain isolation: `grep -r "@/vaults\|@/notes\|@/comments\|@/auth\|@/teams" app/src/notes app/src/vaults app/src/comments app/src/auth app/src/teams` — confirm no domain imports another domain (cross-domain imports should only appear in `shared/`)

**Checkpoint**: Build passes. Old flat directories are gone. Domain isolation verified.

---

## Final Phase: Polish & Cross-Cutting Concerns

- [x] T036 [P] Update `README.md` if it contains a description or tree of the `app/src/` layout — sync it with the new domain structure
- [x] T037 Start the dev server (`pnpm dev`) and manually verify all routes: `/` (vault list), `/vaults/:id` (vault view), `/vaults/:id/notes/:id` (note editor), `/vaults/:id/settings` (vault settings), `/teams/:id/settings` (team settings), `/login` (login page)
- [x] T038 [P] Verify vault icons render correctly in the UI — they reference SVGs moved from `assets/icons/vaults/` to `vaults/assets/icons/`
- [x] T039 Run `git status` to confirm no untracked or dangling files remain at old flat directory paths

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — BLOCKS all domain moves
- **US1 (Phase 3)**: Depends on Phase 2 completion — all domain tasks can then run in parallel
- **US2 (Phase 4)**: Depends on Phase 3 completion — audit requires all files to be in final locations
- **US3 (Phase 5)**: Depends on Phase 4 completion — cleanup requires imports to be correct
- **Polish (Final Phase)**: Depends on Phase 5 completion

### User Story Dependencies

- **US1 (P1)**: Depends on Phase 2 (shared layer in place). All domain moves are independent of each other — vaults, notes, comments, auth, teams can be moved in parallel.
- **US2 (P2)**: Depends on US1 completion (all files in final locations before auditing).
- **US3 (P3)**: Depends on US2 completion (imports correct before deleting old directories).

### Within Each Domain (US1)

For each domain, the internal order is:
1. Move API module (update `client` import)
2. Move stores / composables in parallel (update `types` import)
3. Move components in parallel (update store, API, types imports)
4. Move views (update store, API, component imports)

These are collapsed into 3–5 tasks per domain in Phase 3 for clarity.

---

## Parallel Example: US1 Domain Moves

After Phase 2 completes, these Phase 3 task groups can all run simultaneously (one developer per domain, or one AI agent per domain):

```
Group A — Vaults:  T011, T012, T013, T014, T015
Group B — Notes:   T016, T017, T018, T019
Group C — Comments: T020, T021, T022
Group D — Auth:    T023, T024, T025
Group E — Teams:   T026, T027, T028
Then (sequential): T029 — shared import consolidation
```

Within each group, tasks marked [P] can also run in parallel.

---

## Implementation Strategy

### MVP First (US1 Only)

1. Complete Phase 1: Update docs
2. Complete Phase 2: Move shared layer
3. Complete Phase 3: Move all domain files
4. **STOP and VALIDATE**: Run `pnpm build` — all imports should resolve
5. Demo: All routes work with the new structure

### Incremental Delivery

1. Phase 1 + 2 → Shared layer ready
2. Phase 3 → All domains colocated → **US1 satisfied** (MVP)
3. Phase 4 → Import audit clean → **US2 satisfied**
4. Phase 5 → Old dirs removed, isolation verified → **US3 satisfied**
5. Final Phase → Docs and manual smoke test

---

## Notes

- All file moves should use `git mv` to preserve git history
- Each task or logical group should be committed separately
- After each domain group (T011–T015, T016–T019, etc.) run `pnpm build` to catch import errors early
- The `@/` alias does NOT change — `vite.config.ts` is untouched
- No logic changes: this is a pure structural refactor
- `main.ts`, `App.vue`, `vite-env.d.ts` remain at `src/` root — do not move them
