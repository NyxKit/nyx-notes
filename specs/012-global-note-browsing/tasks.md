# Tasks: Global Note Browsing

**Input**: Design documents from `/specs/012-global-note-browsing/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/global-note-browsing.md

**Tests**: Include Vitest and Playwright coverage because the plan and constitution require frontend unit and E2E validation for the changed browse flows.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (`[US1]`, `[US2]`, `[US3]`)
- Include exact file paths in every task description

## Path Conventions

- Frontend application code lives under `app/src/`
- Feature docs live under `docs/` and `specs/012-global-note-browsing/`
- Playwright coverage for this feature should live under `app/tests/e2e/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Align repo docs with the approved feature behavior before code changes

- [x] T001 Update global browse behavior, routes, and sidebar navigation docs in `/home/arnedecant/Projects/nyxkit/nyx-notes/docs/interface/frontend.md`
- [x] T002 [P] Update frontend unit and E2E expectations for global search and favorites in `/home/arnedecant/Projects/nyxkit/nyx-notes/docs/testing/README.md`
- [x] T003 [P] Update the user-facing feature summary for global browse/navigation changes in `/home/arnedecant/Projects/nyxkit/nyx-notes/README.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core browse infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Add frontend browse model types for origin context and favorite references in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/shared/types/index.ts`
- [x] T005 [P] Create profile-scoped global browse loading composable in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/composables/useGlobalNoteBrowsing.ts`
- [x] T006 Update composable barrel exports in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/composables/index.ts`
- [x] T007 [P] Create the global notes browsing store scaffold in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/stores/noteBrowsing.ts`
- [x] T008 Update store barrel exports for the browsing store in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/stores/index.ts`
- [x] T009 [P] Add shared browse surface and placeholder view exports in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/components/index.ts` and `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/views/index.ts`
- [x] T010 Add route entries for `/notes/search`, `/notes/favorites`, and the legacy favorites redirect in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/shared/router/index.ts`

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Search Across All Notes (Priority: P1) 🎯 MVP

**Goal**: Let users search from any context and always see a live-updating global results grid in the main window.

**Independent Test**: From two different vault or screen contexts, type the same query into the sidebar search bar and confirm the main window updates live to the same aggregated results set, defaults to recent sorting, and shows a dedicated empty state when there are no matches.

### Tests for User Story 1

- [x] T011 [P] [US1] Add Vitest coverage for profile-scoped global browse loading in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/composables/useGlobalNoteBrowsing.spec.ts`
- [x] T012 [P] [US1] Add Vitest coverage for global search aggregation, live query updates, and shared sort state in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/stores/noteBrowsing.spec.ts`
- [x] T013 [P] [US1] Add Vitest coverage for the shared browse surface states and search sorting controls in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/components/GlobalNoteBrowseView.spec.ts`
- [x] T014 [P] [US1] Add Playwright coverage for cross-context live global search in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/tests/e2e/global-search.spec.ts`

### Implementation for User Story 1

- [x] T015 [US1] Implement profile-scoped global browse loading in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/composables/useGlobalNoteBrowsing.ts`
- [x] T016 [US1] Implement derived browse state, live query updates, shared sort behavior, and empty-state handling in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/stores/noteBrowsing.ts`
- [x] T017 [P] [US1] Create the shared global browse surface layout, sort controls, and excluded-profile notice in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/components/GlobalNoteBrowseView.vue`
- [x] T018 [US1] Wire the global search route and view export in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/shared/router/index.ts` and `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/views/index.ts`
- [x] T019 [US1] Route sidebar search input into live debounced global search updates from `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/components/NoteList.vue`
- [x] T020 [US1] Connect the search route to the shared browse surface in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/views/GlobalSearchView.vue`

**Checkpoint**: User Story 1 should now be fully functional and independently testable as the MVP

---

## Phase 4: User Story 2 - Browse Favorites Globally (Priority: P2)

**Goal**: Let users open one global favorites grid from any context and see all favorited notes across accessible profiles and vaults.

**Independent Test**: Favorite notes from multiple locations, open favorites from different contexts, and confirm the same aggregated favorites grid appears in the main window with a dedicated empty state when no favorites exist.

### Tests for User Story 2

- [x] T021 [P] [US2] Add Vitest coverage for compound favorite reference persistence and aggregation in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/stores/noteBrowsing.spec.ts`
- [x] T022 [P] [US2] Add Vitest coverage for the favorites view using the shared browse surface in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/views/FavoritesView.spec.ts`
- [x] T023 [P] [US2] Add Playwright coverage for global favorites navigation, shared sorting controls, and rendering in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/tests/e2e/global-favorites.spec.ts`

### Implementation for User Story 2

- [x] T024 [US2] Replace bare note-ID favorites with compound favorite references in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/stores/noteBrowsing.ts`
- [x] T025 [US2] Update note favorite toggling to use the shared browsing store in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/views/NoteView.vue`
- [x] T026 [P] [US2] Connect the favorites route to the shared browse surface in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/views/FavoritesView.vue`
- [x] T027 [US2] Wire the favorites route, legacy redirect behavior, and view export in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/shared/router/index.ts` and `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/views/index.ts`
- [x] T028 [US2] Point sidebar favorites navigation at the global route in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/shared/components/SidebarNav.vue`

**Checkpoint**: User Stories 1 and 2 should both work independently, with favorites no longer scoped to the active vault

---

## Phase 5: User Story 3 - Understand Note Origin At A Glance (Priority: P3)

**Goal**: Reuse one note card family that shows server and vault origin in search, favorites, and single-vault grids, while removing the obsolete sidebar-wide `New Note` action.

**Independent Test**: Open search, favorites, and a single-vault notes grid and confirm every card shows server and vault origin; confirm the sidebar no longer exposes a context-free `New Note` CTA.

### Tests for User Story 3

- [x] T029 [P] [US3] Add Vitest coverage for note-card origin labels and link targets in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/components/NoteCard.spec.ts`
- [x] T030 [P] [US3] Extend note-grid coverage for origin-aware card reuse in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/vaults/views/VaultView.note-cards.spec.ts`
- [x] T031 [P] [US3] Add Playwright coverage for origin labels and sidebar CTA removal in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/tests/e2e/note-origin-context.spec.ts`

### Implementation for User Story 3

- [x] T032 [US3] Refactor `NoteCard` to render server and vault origin labels from the browse-card model in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/components/NoteCard.vue`
- [x] T033 [US3] Update the single-vault note grid to supply origin-aware card data in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/vaults/views/VaultView.vue`
- [x] T034 [US3] Remove the obsolete sidebar-wide `New Note` CTA and keep only context-aware navigation in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/shared/components/SidebarNav.vue`
- [x] T035 [US3] Ensure both global browse views render the shared origin-aware note card consistently through `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/components/GlobalNoteBrowseView.vue`, `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/views/GlobalSearchView.vue`, and `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/views/FavoritesView.vue`

**Checkpoint**: Search, favorites, and single-vault browsing should now share the same origin-aware note card behavior

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final verification and cleanup across all user stories

- [x] T036 Run and fix Vitest coverage for the feature with `pnpm --dir /home/arnedecant/Projects/nyxkit/nyx-notes/frontend test`
- [x] T037 Run and fix linting for the feature with `pnpm --dir /home/arnedecant/Projects/nyxkit/nyx-notes/frontend lint`
- [x] T038 Validate the quickstart scenarios against `/home/arnedecant/Projects/nyxkit/nyx-notes/specs/012-global-note-browsing/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - starts immediately and must complete before implementation because docs are the source of truth
- **Foundational (Phase 2)**: Depends on Phase 1 - blocks all user stories
- **User Story 1 (Phase 3)**: Depends on Phase 2 - this is the MVP
- **User Story 2 (Phase 4)**: Depends on Phase 2 and reuses the browsing store scaffold from Phase 2
- **User Story 3 (Phase 5)**: Depends on Phases 3 and 4 because it verifies consistent card behavior across search, favorites, and vault grids
- **Polish (Phase 6)**: Depends on all completed story phases

### User Story Dependencies

- **US1**: No dependencies beyond Setup and Foundational
- **US2**: No dependency on US1 behavior, but shares the same foundational browsing store and route infrastructure
- **US3**: Depends on US1 and US2 being present so the shared note-card behavior can be verified across all browse surfaces

### Within Each User Story

- Tests should be added before or alongside implementation and must fail meaningfully before the final behavior is complete
- Store/state updates precede route and view wiring
- Shared card and navigation changes come after the affected browse surfaces exist
- Each story should be validated independently at its checkpoint before moving on

### Parallel Opportunities

- T002 and T003 can run in parallel during docs setup
- T005 and T007 can run in parallel during foundational work
- T011, T012, T013, and T014 can run in parallel for US1
- T021, T022, and T023 can run in parallel for US2
- T029, T030, and T031 can run in parallel for US3
- T017 can proceed in parallel with T016 once the browsing store shape is agreed
- T026 can proceed in parallel with T024 and T025 once the favorites contract is fixed

---

## Parallel Example: User Story 1

```bash
# Launch US1 test work together
Task: "Add Vitest coverage for profile-scoped global browse loading in app/src/notes/composables/useGlobalNoteBrowsing.spec.ts"
Task: "Add Vitest coverage for global search aggregation, live query updates, and shared sort state in app/src/notes/stores/noteBrowsing.spec.ts"
Task: "Add Vitest coverage for the shared browse surface states and search sorting controls in app/src/notes/components/GlobalNoteBrowseView.spec.ts"
Task: "Add Playwright coverage for cross-context live global search in app/tests/e2e/global-search.spec.ts"

# Launch US1 implementation work with separate files
Task: "Implement profile-scoped global browse loading in app/src/notes/composables/useGlobalNoteBrowsing.ts"
Task: "Implement derived browse state, live query updates, shared sort behavior, and empty-state handling in app/src/notes/stores/noteBrowsing.ts"
Task: "Create the shared global browse surface layout, sort controls, and excluded-profile notice in app/src/notes/components/GlobalNoteBrowseView.vue"
```

## Parallel Example: User Story 2

```bash
# Launch US2 test work together
Task: "Add Vitest coverage for compound favorite reference persistence and aggregation in app/src/notes/stores/noteBrowsing.spec.ts"
Task: "Add Vitest coverage for the favorites view using the shared browse surface in app/src/notes/views/FavoritesView.spec.ts"
Task: "Add Playwright coverage for global favorites navigation, shared sorting controls, and rendering in app/tests/e2e/global-favorites.spec.ts"

# Launch US2 implementation work with separate files
Task: "Update note favorite toggling to use the shared browsing store in app/src/notes/views/NoteView.vue"
Task: "Connect the favorites route to the shared browse surface in app/src/notes/views/FavoritesView.vue"
```

## Parallel Example: User Story 3

```bash
# Launch US3 test work together
Task: "Add Vitest coverage for note-card origin labels and link targets in app/src/notes/components/NoteCard.spec.ts"
Task: "Extend note-grid coverage for origin-aware card reuse in app/src/vaults/views/VaultView.note-cards.spec.ts"
Task: "Add Playwright coverage for origin labels and sidebar CTA removal in app/tests/e2e/note-origin-context.spec.ts"

# Launch US3 implementation work with separate files
Task: "Refactor NoteCard to render server and vault origin labels from the browse-card model in app/src/notes/components/NoteCard.vue"
Task: "Update the single-vault note grid to supply origin-aware card data in app/src/vaults/views/VaultView.vue"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Validate global search independently from multiple contexts
5. Stop for review if only the MVP is needed

### Incremental Delivery

1. Finish Setup and Foundational work
2. Deliver US1 for global search
3. Deliver US2 for global favorites
4. Deliver US3 for consistent origin-aware note cards and sidebar cleanup
5. Finish with lint, tests, and quickstart validation

### Parallel Team Strategy

1. One developer updates docs while another prepares the foundational browse store scaffold after docs land
2. After Phase 2, one developer can take US1 while another prepares US2 tests and favorites view scaffolding
3. US3 starts once US1 and US2 surfaces exist so the shared card refactor lands once

---

## Notes

- All tasks follow the required checklist format: checkbox, task ID, optional `[P]`, required story label for story phases, and exact file path
- The suggested MVP scope is Phase 3 / User Story 1 only
- Avoid merging US3 card refactors into US1 or US2 early; keep the shared-card sweep as one focused pass after both global surfaces exist
