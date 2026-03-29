# Tasks: Line-Based Comments

**Input**: Design documents from `/specs/010-add-line-comments/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/comments-api.md, quickstart.md

**Tests**: Include tests because the plan and constitution require coverage across Rust domain/storage/API layers plus Vitest and Playwright.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g. US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Refresh the docs and shared type definitions that the rest of the feature depends on.

- [X] T001 Update comment and annotation behavior in `docs/interface/frontend.md`
- [X] T002 [P] Update comment domain types and permission notes in `docs/architecture/core-domain.md`
- [X] T003 [P] Update comment route and payload documentation in `docs/architecture/backend-api.md`
- [X] T004 [P] Update sidecar storage format and migration notes in `docs/architecture/filesystem-storage.md`
- [X] T005 [P] Update testing coverage guidance for line-based comments in `docs/testing/README.md`
- [X] T006 [P] Update high-level feature summary for line-based comments in `README.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core comment-anchor infrastructure that MUST be complete before ANY user story can be implemented.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T007 Add canonical comment anchor, visibility state, and hidden legacy comment domain types in `crates/notes-core/src/domain.rs`
- [X] T008 [P] Extend shared frontend comment types to match the new anchor model in `frontend/src/shared/types/index.ts`
- [X] T009 [P] Update request/response payload types for comment routes in `crates/notes-server-axum/src/types.rs`
- [X] T010 Implement backward-compatible sidecar read/write handling for visible anchored threads and hidden legacy comment records in `crates/notes-storage-fs/src/lib.rs`
- [X] T011 [P] Add shared comment-to-annotation mapping utilities in `frontend/src/comments/composables/useCommentAnnotations.ts`
- [X] T012 [P] Add foundational storage coverage for structured anchors and hidden legacy records in `crates/notes-storage-fs/src/tests.rs`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel.

---

## Phase 3: User Story 1 - Start a Discussion From a Line (Priority: P1) 🎯 MVP

**Goal**: Let a user create a new discussion from exact selected text in the editor and see it appear in the sidebar with containing-line context.

**Independent Test**: Open a note, select text within a line, create a comment, and confirm the new thread appears in the sidebar with the containing line shown while the selected text is highlighted in the editor.

### Tests for User Story 1

- [X] T013 [P] [US1] Add domain tests for comment anchor validation in `crates/notes-core/src/domain.rs`
- [X] T014 [P] [US1] Add HTTP integration tests for comment creation payloads in `crates/notes-server-axum/tests/comments_api.rs`
- [X] T015 [P] [US1] Add Vitest coverage for comment-to-annotation mapping in `frontend/src/comments/composables/useCommentAnnotations.spec.ts`
- [ ] T016 [P] [US1] Add Playwright coverage for creating a line-based comment in `frontend/e2e/line-comments.spec.ts`

### Implementation for User Story 1

- [X] T017 [US1] Implement structured anchor handling for comment creation in `crates/notes-server-axum/src/routes/comments.rs`
- [X] T018 [P] [US1] Update frontend comment create API payloads in `frontend/src/comments/api/comments.ts`
- [X] T019 [P] [US1] Extend comment composable create/load behavior in `frontend/src/comments/composables/useComments.ts`
- [X] T020 [US1] Integrate `NyxEditor` annotation creation events in `frontend/src/notes/components/NoteEditor.vue`
- [X] T021 [US1] Update comment composer flow for exact selected-text anchors in `frontend/src/comments/components/CommentComposer.vue`
- [X] T022 [US1] Render new line-based threads with line context in `frontend/src/comments/components/CommentSidebar.vue`
- [X] T023 [US1] Connect note editor and sidebar create flow in `frontend/src/notes/views/NoteView.vue`

**Checkpoint**: User Story 1 should now be fully functional and testable independently.

---

## Phase 4: User Story 2 - Follow and Manage a Line Discussion (Priority: P2)

**Goal**: Let collaborators follow existing line-based discussions in the sidebar, focus related annotations, reply, and resolve threads while staying oriented to the selected line context.

**Independent Test**: Load a note with line-based discussions, focus a thread, confirm the matching annotation activates, add a reply, resolve the thread, and confirm open/resolved behavior remains clear.

### Tests for User Story 2

- [ ] T024 [P] [US2] Add HTTP integration tests for reply and resolve flows in `crates/notes-server-axum/tests/comments_api.rs`
- [ ] T025 [P] [US2] Add Vitest coverage for focus and ordering behavior in `frontend/src/comments/composables/useComments.spec.ts`
- [ ] T026 [P] [US2] Extend Playwright discussion-management scenarios in `frontend/e2e/line-comments.spec.ts`

### Implementation for User Story 2

- [X] T027 [US2] Implement resolve and reply updates against the new thread shape in `crates/notes-server-axum/src/routes/comments.rs`
- [ ] T028 [P] [US2] Update reply and resolve client calls in `frontend/src/comments/api/comments.ts`
- [X] T029 [P] [US2] Track active/focused annotations and stable ordering in `frontend/src/comments/composables/useComments.ts`
- [X] T030 [US2] Add active thread and resolved state handling in `frontend/src/comments/components/CommentThread.vue`
- [X] T031 [US2] Synchronize sidebar focus with editor annotation focus in `frontend/src/comments/components/CommentSidebar.vue`
- [X] T032 [US2] Wire annotation focus and blur events in `frontend/src/notes/components/NoteEditor.vue`

**Checkpoint**: User Stories 1 and 2 should both work independently.

---

## Phase 5: User Story 3 - Keep Discussions Understandable After Note Edits (Priority: P3)

**Goal**: Preserve comment history after note edits by reattaching when possible, showing detached visible threads clearly, and hiding non-convertible legacy comments while retaining them in storage.

**Independent Test**: Edit a commented line, reload the note, and confirm threads either reattach correctly or show detached with line preview; load legacy quote-only comments and confirm they remain stored but hidden from the default sidebar.

### Tests for User Story 3

- [X] T033 [P] [US3] Add storage tests for legacy comment retention and hidden visibility in `crates/notes-storage-fs/src/tests.rs`
- [X] T034 [P] [US3] Add HTTP integration tests for hidden legacy filtering in `crates/notes-server-axum/tests/comments_api.rs`
- [X] T035 [P] [US3] Add Vitest coverage for detached and hidden legacy annotation mapping in `frontend/src/comments/composables/useCommentAnnotations.spec.ts`
- [ ] T036 [P] [US3] Extend Playwright restore and legacy scenarios in `frontend/e2e/line-comments.spec.ts`

### Implementation for User Story 3

- [X] T037 [US3] Implement legacy filtering and detached-thread response handling in `crates/notes-server-axum/src/routes/comments.rs`
- [X] T038 [P] [US3] Finalize sidecar migration behavior for hidden legacy comments in `crates/notes-storage-fs/src/lib.rs`
- [X] T039 [P] [US3] Update annotation mapping for detached and hidden legacy states in `frontend/src/comments/composables/useCommentAnnotations.ts`
- [X] T040 [US3] Render detached visible threads and omit hidden legacy threads in `frontend/src/comments/components/CommentSidebar.vue`
- [X] T041 [US3] Refresh note load behavior for restored annotations in `frontend/src/notes/views/NoteView.vue`

**Checkpoint**: All user stories should now be independently functional.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final validation and cleanup across all user stories.

- [X] T042 [P] Export new comment composables from `frontend/src/comments/composables/index.ts`
- [ ] T043 [P] Export new comment components from `frontend/src/comments/components/index.ts`
- [ ] T044 Run end-to-end validation steps from `specs/010-add-line-comments/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational completion
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Starts after Foundational - MVP for creating line-based comments
- **User Story 2 (P2)**: Starts after Foundational and builds on the visible thread model from US1
- **User Story 3 (P3)**: Starts after Foundational and depends on the anchor model introduced in US1

### Within Each User Story

- Tests should be written before implementation and fail first where practical
- Backend/domain contract changes precede frontend integration for the same flow
- API/composable updates precede component wiring
- Story checkpoint should pass before moving to the next priority for release

### Parallel Opportunities

- `T002`, `T003`, `T004`, and `T005` can run in parallel
- `T008`, `T009`, `T011`, and `T012` can run in parallel after `T007`
- Test tasks marked `[P]` within each user story can run in parallel
- Frontend API/composable tasks marked `[P]` can run in parallel with backend test work in the same story

---

## Parallel Example: User Story 1

```bash
# Launch US1 test work together:
Task: "Add HTTP integration tests for comment creation payloads in crates/notes-server-axum/tests/comments_api.rs"
Task: "Add Vitest coverage for comment-to-annotation mapping in frontend/src/comments/composables/useCommentAnnotations.spec.ts"
Task: "Add Playwright coverage for creating a line-based comment in frontend/e2e/line-comments.spec.ts"

# Launch US1 client-side support together after backend contract work starts:
Task: "Update frontend comment create API payloads in frontend/src/comments/api/comments.ts"
Task: "Extend comment composable create/load behavior in frontend/src/comments/composables/useComments.ts"
```

## Parallel Example: User Story 2

```bash
# Launch US2 tests together:
Task: "Add HTTP integration tests for reply and resolve flows in crates/notes-server-axum/tests/comments_api.rs"
Task: "Add Vitest coverage for focus and ordering behavior in frontend/src/comments/composables/useComments.spec.ts"

# Launch US2 frontend behavior together:
Task: "Track active/focused annotations and stable ordering in frontend/src/comments/composables/useComments.ts"
Task: "Update reply and resolve client calls in frontend/src/comments/api/comments.ts"
```

## Parallel Example: User Story 3

```bash
# Launch US3 regression tests together:
Task: "Add storage tests for legacy comment retention and hidden visibility in crates/notes-storage-fs/src/tests.rs"
Task: "Add HTTP integration tests for hidden legacy filtering in crates/notes-server-axum/tests/comments_api.rs"
Task: "Add Vitest coverage for detached and hidden legacy annotation mapping in frontend/src/comments/composables/useCommentAnnotations.spec.ts"

# Launch US3 implementation pieces together:
Task: "Finalize sidecar migration behavior for hidden legacy comments in crates/notes-storage-fs/src/lib.rs"
Task: "Update annotation mapping for detached and hidden legacy states in frontend/src/comments/composables/useCommentAnnotations.ts"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Validate exact selected-text anchoring, sidebar line context, and creation flow

### Incremental Delivery

1. Finish Setup + Foundational to establish the shared anchor model
2. Deliver User Story 1 for end-to-end line comment creation
3. Deliver User Story 2 for discussion management and focus syncing
4. Deliver User Story 3 for restore, detachment, and hidden legacy handling
5. Finish with Polish and quickstart validation

### Parallel Team Strategy

1. One developer updates docs and contracts while another prepares shared domain/frontend types
2. After Foundational completes:
   - Developer A: User Story 1 backend/editor creation flow
   - Developer B: User Story 2 sidebar/reply/focus behavior
   - Developer C: User Story 3 migration and legacy handling

---

## Notes

- Every task follows the required checkbox, ID, label, and file path format
- `[P]` tasks target different files and can be worked concurrently
- User story phases are structured for independent validation
- Suggested MVP scope is Phase 3 / User Story 1 only
