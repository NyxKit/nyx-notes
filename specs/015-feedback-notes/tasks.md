# Tasks: Feedback Center

**Input**: Design documents from `/specs/015-feedback-notes/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Layer-specific test tasks are included to satisfy the constitution and validate each affected layer.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Align docs and shared foundations before feature work begins

- [ ] T001 Update active storage and domain docs for the feedback namespace and shared image support in `docs/architecture/file-system.md`, `docs/architecture/filesystem-storage.md`, `docs/architecture/core-domain.md`, and `docs/architecture/backend-api.md`
- [ ] T002 Update frontend architecture guidance for the feedback domain and shared create/edit note flow in `docs/interface/frontend.md`
- [ ] T003 Update the feature summary in `README.md` to mention feedback and image attachments

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data and routing changes that every user story depends on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 Define the note-derived feedback domain types in `crates/notes-core/` and document the storage contract for item-local `images/` folders
- [ ] T005 [P] Extend filesystem storage to support item folders with image subdirectories for notes and feedback in `crates/notes-storage-fs/`
- [ ] T006 [P] Add backend route scaffolding for feedback collection and admin-only access in `crates/notes-server-axum/`
- [ ] T007 [P] Add the feedback domain barrel files and route constants in `app/src/feedback/index.ts`, `app/src/feedback/api/index.ts`, `app/src/feedback/components/index.ts`, `app/src/feedback/composables/index.ts`, `app/src/feedback/views/index.ts`, and `app/src/notes/components/index.ts`
- [ ] T008 [P] Add `notes-core` unit tests for elevated note inheritance, image support, and feedback metadata in `crates/notes-core/`
- [ ] T009 [P] Add `notes-storage-fs` integration tests for note and feedback image persistence in `crates/notes-storage-fs/`
- [ ] T010 [P] Add `notes-server-axum` integration tests for admin-only feedback visibility and creation in `crates/notes-server-axum/`
- [ ] T011 [P] Add frontend Vitest tests for the feedback modal and reusable note editor shell in `app/src/feedback/` and `app/src/notes/components/`
- [ ] T012 [P] Add Playwright E2E tests for user submission and admin review flows in `app/src/feedback/`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Submit Feedback From Current Page (Priority: P1) 🎯 MVP

**Goal**: Let regular users open a large feedback modal, enter note-like content with images, and submit without leaving their current page.

**Independent Test**: Open Feedback as a regular user, submit a feedback item with title, description, feedback type, and images, and confirm the current page stays visible.

### Implementation for User Story 1

- [ ] T013 [P] [US1] Add the reusable create/edit note editor shell in `app/src/notes/components/CreateEditNote.vue`
- [ ] T014 [P] [US1] Add the regular-user feedback modal view that reuses the notes-domain create/edit shell in `app/src/feedback/views/FeedbackModal.vue`
- [ ] T015 [US1] Wire the bottom-left Feedback menu item to open the feedback modal from `app/src/shared/components/SidebarNav.vue` and `app/src/App.vue`
- [ ] T016 [US1] Add feedback submission state and API calls in `app/src/feedback/api/feedback.ts` and `app/src/feedback/composables/useFeedback.ts`
- [ ] T017 [US1] Implement backend feedback creation with image compression and note-derived metadata capture in `crates/notes-server-axum/`
- [ ] T018 [US1] Persist feedback items, note inheritance data, and item-local `images/` folders in `crates/notes-storage-fs/`

**Checkpoint**: Regular users can submit feedback independently of the admin overview.

---

## Phase 4: User Story 2 - Review Feedback as Admin (Priority: P2)

**Goal**: Let admins open Feedback as a masonry-style overview and create their own feedback items there.

**Independent Test**: Open Feedback as an admin and confirm it loads a vault-like masonry overview with the ability to add items.

### Implementation for User Story 2

- [ ] T019 [P] [US2] Add the admin feedback overview view and masonry layout in `app/src/feedback/views/FeedbackVaultView.vue`
- [ ] T020 [P] [US2] Add admin feedback list and create flows in `app/src/feedback/composables/useFeedbackVault.ts`
- [ ] T021 [US2] Add admin-only feedback listing and overview routes in `crates/notes-server-axum/`
- [ ] T022 [US2] Add feedback item card and empty-state components in `app/src/feedback/components/`

**Checkpoint**: Admins can browse and create feedback independently of the detail view.

---

## Phase 5: User Story 3 - Manage Feedback Like a Note (Priority: P3)

**Goal**: Let admins open a feedback item into a note-style detail view with comments and editing.

**Independent Test**: Open a feedback item from the admin overview and confirm it supports note-style editing, title changes, and comments.

### Implementation for User Story 3

- [ ] T023 [P] [US3] Add the feedback detail route and note-style editor view in `app/src/feedback/views/FeedbackItemView.vue`
- [ ] T024 [P] [US3] Add feedback detail API and update flows in `app/src/feedback/api/feedback.ts`
- [ ] T025 [US3] Reuse the existing comment sidebar and note editing components for feedback items in `app/src/comments/` and `app/src/notes/components/`
- [ ] T026 [US3] Implement admin feedback update and comment handling in `crates/notes-server-axum/`
- [ ] T027 [US3] Ensure feedback item save/load preserves images and submission metadata in `crates/notes-storage-fs/`

**Checkpoint**: Admin feedback items now behave like notes in the detail view.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T028 [P] Update `specs/015-feedback-notes/quickstart.md` with the implemented feedback and admin flows if any visible behavior changes
- [ ] T029 Validate the feedback paths manually against `specs/015-feedback-notes/quickstart.md`
- [ ] T030 Review and tighten terminology consistency across `specs/015-feedback-notes/spec.md`, `specs/015-feedback-notes/data-model.md`, and `specs/015-feedback-notes/contracts/feedback-api.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable
- **User Story 3 (P3)**: Can start after User Story 2 - Depends on the admin overview and shared detail components

### Within Each User Story

- Shared types before API routes
- API routes before UI wiring
- UI shell before view-specific behavior
- Persisted data shape before detail reuse

### Parallel Opportunities

- T005, T006, and T007 can run in parallel after docs are updated
- T008 and T009 can run in parallel because they touch different frontend files
- T014 and T015 can run in parallel once the admin overview shape is clear
- T018 and T019 can run in parallel after the admin overview route exists

---

## Parallel Example: User Story 1

```bash
Task: "Extract the reusable create/edit note editor shell from `app/src/notes/views/NoteView.vue` into `app/src/notes/components/CreateEditNote.vue`"
Task: "Add the regular-user feedback modal view that reuses the shared create/edit note shell in `app/src/feedback/views/FeedbackModal.vue`"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. STOP and validate the regular-user feedback flow

### Incremental Delivery

1. Complete Setup + Foundational → shared data and routing ready
2. Add User Story 1 → regular users can submit feedback
3. Add User Story 2 → admins can browse and add feedback
4. Add User Story 3 → admins can manage feedback like notes
5. Finish with polish and terminology cleanup

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
