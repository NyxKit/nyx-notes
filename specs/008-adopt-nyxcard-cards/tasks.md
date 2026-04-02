# Tasks: Unify Card Surfaces

**Input**: Design documents from `/home/arnedecant/Projects/nyxkit/nyx-notes/specs/008-adopt-nyxcard-cards/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/browse-card-surfaces.md`, `quickstart.md`

**Tests**: Include automated frontend coverage because the plan and quickstart explicitly require interaction coverage and browser verification for the affected browse-card surfaces.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Maps the task to a user story (`[US1]`, `[US2]`, `[US3]`)
- Every task includes an exact file path

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish validation and test scaffolding required for the clarified browse-card feature.

- [X] T001 Update frontend validation and test scripts in `frontend/package.json`
- [X] T002 [P] Add frontend unit test configuration in `frontend/vitest.config.ts` and `frontend/src/test/setup.ts`
- [X] T003 [P] Add browser test configuration scaffold in `frontend/playwright.config.ts` and `frontend/tests/e2e/.gitkeep`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Complete docs-first scope updates and shared browse-card groundwork before any user-story implementation starts.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T004 Update in-scope and out-of-scope browse-card guidance in `docs/interface/frontend.md`
- [X] T005 [P] Update shared browse-card visual rules and exclusions in `DESIGN.md`
- [X] T006 Add standalone card-family shell styles in `frontend/src/shared/assets/theme.css`
- [X] T007 [P] Add optional description support to frontend request and domain types in `frontend/src/shared/types/index.ts`
- [X] T008 [P] Add Rust domain and API support for vault and note descriptions in `crates/notes-core/src/domain.rs`, `crates/notes-storage-fs/src/lib.rs`, and `crates/notes-server-axum/src/routes/notes.rs`

**Checkpoint**: Docs, shared browse-card scaffold, and validation tooling are ready; user stories can begin.

---

## Phase 3: User Story 1 - Browse Vaults Consistently (Priority: P1) 🎯 MVP

**Goal**: Present vault dashboard tiles with one consistent browse-card surface while preserving explicit pointer and keyboard activation behavior.

**Independent Test**: Open the home dashboard with multiple vaults and confirm each vault tile uses the shared card treatment, still shows vault identity, and still opens the selected vault through the explicit activation target by pointer and keyboard.

### Tests for User Story 1

- [X] T009 [P] [US1] Add vault-card interaction coverage in `frontend/src/vaults/components/VaultCard.spec.ts`
- [X] T010 [P] [US1] Add vault dashboard browser verification in `frontend/tests/e2e/vault-dashboard-cards.spec.ts`

### Implementation for User Story 1

- [X] T011 [US1] Refactor `VaultCard` into a standalone `NyxCard` + `RouterLink` component in `frontend/src/vaults/components/VaultCard.vue`
- [X] T012 [US1] Wire the updated vault card behavior and descriptions into `frontend/src/vaults/views/HomeView.vue`
- [X] T013 [US1] Update vault component exports for the browse-card refactor in `frontend/src/vaults/components/index.ts`

**Checkpoint**: User Story 1 is fully functional and independently testable.

---

## Phase 4: User Story 2 - Create Vaults Inside the Same Card Language (Priority: P2)

**Goal**: Make the inline create-vault surface feel like part of the same card family without turning it into a misleading single-action tile.

**Independent Test**: Open the inline create flow and confirm the form uses the same card family, preserves form controls and create/cancel behavior, and returns the dashboard grid to a consistent state after cancel or successful creation.

### Tests for User Story 2

- [X] T014 [P] [US2] Add inline create-card state coverage in `frontend/src/vaults/views/HomeView.create-card.spec.ts`
- [X] T015 [P] [US2] Add create-vault browser verification in `frontend/tests/e2e/create-vault-card.spec.ts`

### Implementation for User Story 2

- [X] T016 [US2] Refactor the inline create-vault surface into a `NyxCard` form with description input in `frontend/src/vaults/views/HomeView.vue`
- [X] T017 [US2] Extend vault create/update payload types for descriptions in `frontend/src/shared/types/index.ts` and `frontend/src/vaults/api/vaults.ts`

**Checkpoint**: User Stories 1 and 2 both work independently and the dashboard card family remains coherent.

---

## Phase 5: User Story 3 - Reuse the Card Pattern in Similar Surfaces (Priority: P3)

**Goal**: Apply the shared browse-card family to note masonry cards while preserving note-specific content, explicit activation behavior, and out-of-scope exclusions.

**Independent Test**: Open a vault with notes and confirm note tiles use the shared card family, retain note-specific title/tags/date content, still open notes correctly through the explicit activation target, and do not alter excluded surfaces.

### Tests for User Story 3

- [X] T018 [P] [US3] Add note-card rendering and link behavior coverage in `frontend/src/vaults/views/VaultView.note-cards.spec.ts`
- [X] T019 [P] [US3] Add note masonry browser verification in `frontend/tests/e2e/vault-note-cards.spec.ts`

### Implementation for User Story 3

- [X] T020 [US3] Create `NoteCard` and refactor note masonry tiles to use it in `frontend/src/notes/components/NoteCard.vue` and `frontend/src/vaults/views/VaultView.vue`
- [X] T021 [US3] Add distilled note-description handling to frontend and Rust note save flows in `frontend/src/shared/types/index.ts`, `crates/notes-core/src/domain.rs`, and `crates/notes-server-axum/src/routes/notes.rs`

**Checkpoint**: All user stories are independently functional with shared browse-card behavior.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Finalize docs alignment, exclusions, responsive behavior, and full validation across the feature.

- [X] T022 [P] Reconcile final browse-card docs wording in `docs/interface/frontend.md` and `DESIGN.md`
- [X] T023 [P] Verify excluded surfaces remain unchanged in `frontend/src/vaults/components/VaultSwitcher.vue` and `frontend/src/comments/components/CommentThread.vue`
- [X] T024 Run full frontend validation from `frontend/package.json` and resolve issues in `frontend/package.json`, `frontend/vitest.config.ts`, and `frontend/playwright.config.ts`
- [X] T025 Perform final responsive and accessibility cleanup in `frontend/src/vaults/components/VaultCard.vue`, `frontend/src/notes/components/NoteCard.vue`, `frontend/src/vaults/views/HomeView.vue`, and `frontend/src/vaults/views/VaultView.vue`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies; can start immediately.
- **Phase 2 (Foundational)**: Depends on Phase 1; blocks all user stories.
- **Phase 3 (US1)**: Depends on Phase 2; MVP and highest priority.
- **Phase 4 (US2)**: Depends on Phase 2; should follow US1 in practice because both modify `frontend/src/vaults/views/HomeView.vue`.
- **Phase 5 (US3)**: Depends on Phase 2; can proceed once the shared browse-card scaffold is stable, but should land after US1 if shared surface behavior is still changing.
- **Phase 6 (Polish)**: Depends on all selected user stories being complete.

### User Story Dependencies

- **US1**: No dependency on other stories after foundational work.
- **US2**: Independent in behavior, but shares `frontend/src/vaults/views/HomeView.vue` with US1, so sequence it after US1 to avoid file conflicts.
- **US3**: Independent in behavior after foundational work; it reuses the shared browse-card family but does not depend on the create-vault flow.

### Within Each User Story

- Write story tests before implementation and confirm they fail for the intended reason.
- Update or extend the shared browse-card surface before finishing story-specific wiring.
- Validate each story independently before moving to the next one.

### Parallel Opportunities

- `T002` and `T003` can run in parallel after `T001`.
- `T005`, `T007`, and `T008` can run in parallel after `T004` starts the docs-first pass.
- `T009` and `T010` can run in parallel for US1.
- `T014` and `T015` can run in parallel for US2.
- `T018` and `T019` can run in parallel for US3.
- `T022` and `T023` can run in parallel during polish.

---

## Parallel Example: User Story 1

```bash
# Launch User Story 1 verification tasks together:
Task: "Add vault-card interaction coverage in frontend/src/vaults/components/VaultCard.spec.ts"
Task: "Add vault dashboard browser verification in frontend/tests/e2e/vault-dashboard-cards.spec.ts"
```

## Parallel Example: User Story 2

```bash
# Launch User Story 2 verification tasks together:
Task: "Add inline create-card state coverage in frontend/src/vaults/views/HomeView.create-card.spec.ts"
Task: "Add create-vault browser verification in frontend/tests/e2e/create-vault-card.spec.ts"
```

## Parallel Example: User Story 3

```bash
# Launch User Story 3 verification tasks together:
Task: "Add note-card rendering and activation coverage in frontend/src/vaults/views/VaultView.note-cards.spec.ts"
Task: "Add note masonry browser verification in frontend/tests/e2e/vault-note-cards.spec.ts"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Validate vault dashboard behavior independently before expanding scope

### Incremental Delivery

1. Finish setup and foundational browse-card work
2. Deliver US1 for the vault dashboard as the MVP
3. Add US2 to unify the inline create-vault surface
4. Add US3 to extend the card family to note masonry cards
5. Finish with exclusion checks, responsive cleanup, and full validation

### Parallel Team Strategy

1. One developer completes setup and docs-first foundational work
2. After the shared browse-card scaffold is stable:
   - Developer A implements US1 in `frontend/src/vaults/components/VaultCard.vue` and `frontend/src/vaults/views/HomeView.vue`
   - Developer B prepares US3 tests and note-card refactor in `frontend/src/vaults/views/VaultView.vue`
3. US2 follows once `frontend/src/vaults/views/HomeView.vue` is free of US1 merge conflicts

---

## Notes

- All tasks use the required checklist format with task ID, optional `[P]`, story label for story phases, and file path.
- The clarified interaction rule is preserved throughout: `NyxCard` is the visual shell, while selectable cards use internal `RouterLink` anchors.
- Out-of-scope card-like containers such as `frontend/src/vaults/components/VaultSwitcher.vue` and `frontend/src/comments/components/CommentThread.vue` are intentionally excluded and explicitly verified in polish.
