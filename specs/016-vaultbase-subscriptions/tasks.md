# Tasks: Centralized Subscription Data Flow

**Input**: Design documents from `/specs/016-vaultbase-subscriptions/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Include targeted frontend and backend tests because the plan explicitly requires frontend subscription lifecycle tests and Axum integration tests for live routes.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the documentation and code scaffolding required for live subscription work.

- [X] T001 Update live subscription architecture notes in docs/interface/frontend.md, docs/architecture/backend-api.md, docs/architecture/core-domain.md, docs/architecture/README.md, and README.md
- [X] T002 Create frontend shared subscription scaffolding barrels in app/src/shared/api/index.ts, app/src/shared/composables/index.ts, and app/src/shared/types/index.ts
- [X] T003 [P] Create backend live subscription support module scaffolding in crates/notes-server-axum/src/routes/subscriptions.rs and crates/notes-server-axum/src/live/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the core live subscription infrastructure that every user story depends on.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T004 Define canonical live subscription domain types and event envelopes in crates/notes-core/src/live.rs and crates/notes-core/src/lib.rs
- [X] T005 [P] Add server-side live subscription request/response types in crates/notes-server-axum/src/types.rs
- [X] T006 Implement backend broker state and fanout primitives in crates/notes-server-axum/src/live/broker.rs and crates/notes-server-axum/src/live/mod.rs
- [X] T007 Implement scope normalization and authorization helpers for live subscriptions in crates/notes-server-axum/src/live/scopes.rs
- [X] T008 Wire the live subscription route into the router in crates/notes-server-axum/src/routes/mod.rs and crates/notes-server-axum/src/routes/subscriptions.rs
- [X] T009 [P] Implement frontend live subscription enums and query identity types in app/src/shared/types/live.ts and app/src/shared/types/index.ts
- [X] T010 Implement the shared frontend subscription manager in app/src/shared/api/subscriptionManager.ts
- [X] T011 Implement the NyxBase data-access layer beneath domain stores in app/src/shared/api/nyxBase.ts and app/src/shared/api/index.ts
- [X] T012 Implement reusable composable lifecycle helpers for calling store-level acquire/release subscription behavior in app/src/shared/composables/useSubscription.ts and app/src/shared/composables/index.ts

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel.

---

## Phase 3: User Story 1 - Live Vault And Note Updates (Priority: P1) 🎯 MVP

**Goal**: Deliver live auto-updating vault lists, note lists, and note documents without manual refresh.

**Independent Test**: Open a vault view or note view, trigger a vault/note change from another action path, and confirm the visible state updates automatically without reloading the route.

### Tests for User Story 1

- [X] T013 [P] [US1] Add Axum integration coverage for vault-list, note-list, and note live subscription scopes in crates/notes-server-axum/tests/subscriptions_api.rs
- [X] T014 [P] [US1] Add frontend subscription manager snapshot/reconnect tests in app/src/shared/api/subscriptionManager.spec.ts
- [X] T015 [P] [US1] Add NyxBase live scope tests in app/src/shared/api/nyxBase.spec.ts

### Implementation for User Story 1

- [X] T016 [US1] Implement SSE snapshot delivery for typed live scopes in crates/notes-server-axum/src/routes/subscriptions.rs
- [X] T017 [US1] Publish vault and note updates from write handlers into the broker in crates/notes-server-axum/src/routes/vaults.rs and crates/notes-server-axum/src/routes/notes.rs
- [X] T018 [US1] Add latest-snapshot replay and reconnect handling to the shared manager in app/src/shared/api/subscriptionManager.ts
- [X] T019 [US1] Adapt the vault store to own live vault list subscriptions through NyxBase in app/src/vaults/stores/vaults.ts
- [X] T020 [US1] Adapt the notes store to own live note list and note document subscriptions through NyxBase in app/src/notes/stores/notes.ts
- [X] T021 [US1] Update useVaults lifecycle behavior to call store-level subscribe/unsubscribe methods on mount/unmount in app/src/vaults/composables/useVaults.ts
- [X] T022 [US1] Update note-facing composables and views to call store-level live note subscription behavior in app/src/notes/composables/useGlobalNoteBrowsing.ts, app/src/vaults/views/VaultView.vue, and app/src/notes/views/NoteView.vue

**Checkpoint**: User Story 1 should now deliver live vault and note updates and be testable on its own.

---

## Phase 4: User Story 2 - Shared Subscription Reuse Across Screens (Priority: P2)

**Goal**: Ensure equivalent live queries are deduplicated and reused across multiple consumers.

**Independent Test**: Mount multiple consumers for the same vault or note scope and verify one underlying subscription serves all of them until the last consumer unsubscribes.

### Tests for User Story 2

- [X] T023 [P] [US2] Add frontend ref-counting and dedupe tests in app/src/shared/api/subscriptionManager.dedupe.spec.ts
- [X] T024 [P] [US2] Add backend broker fanout tests for duplicate listeners in crates/notes-server-axum/tests/subscriptions_api.rs

### Implementation for User Story 2

- [X] T025 [US2] Implement canonical query-key normalization and registry ownership in app/src/shared/api/subscriptionManager.ts
- [X] T026 [US2] Add shared handle reference tracking and delayed release behavior in app/src/shared/api/subscriptionManager.ts
- [X] T027 [US2] Replay latest shared snapshots to late subscribers in app/src/shared/api/subscriptionManager.ts and app/src/shared/api/nyxBase.ts
- [X] T028 [US2] Ensure backend broker reuses one upstream scope per normalized key in crates/notes-server-axum/src/live/broker.rs and crates/notes-server-axum/src/live/scopes.rs
- [X] T029 [US2] Integrate shared subscription reuse into vault and note store/composable orchestration in app/src/vaults/stores/vaults.ts, app/src/notes/stores/notes.ts, app/src/vaults/composables/useVaults.ts, and app/src/notes/composables/useGlobalNoteBrowsing.ts

**Checkpoint**: User Stories 1 and 2 should both work independently, with shared subscriptions deduplicated across consumers.

---

## Phase 5: User Story 3 - Predictable Composable Lifecycle Management (Priority: P3)

**Goal**: Make live subscription lifecycle behavior reusable and safe for future composables.

**Independent Test**: Attach a composable-backed consumer to a live scope, switch scopes, unmount it, and confirm lifecycle cleanup plus stale-update protection behave correctly without custom component code.

### Tests for User Story 3

- [X] T030 [P] [US3] Add composable lifecycle and query-switch tests in app/src/shared/composables/useSubscription.spec.ts
- [X] T031 [P] [US3] Add stale-event rejection tests in app/src/shared/api/subscriptionManager.generation.spec.ts

### Implementation for User Story 3

- [X] T032 [US3] Add generation fencing and stale-event rejection to the subscription manager in app/src/shared/api/subscriptionManager.ts
- [X] T033 [US3] Implement reusable mount/unmount and query-switch lifecycle helpers in app/src/shared/composables/useSubscription.ts
- [X] T034 [US3] Refactor useVaults to rely only on shared lifecycle helpers and store subscribe/select patterns in app/src/vaults/composables/useVaults.ts
- [X] T035 [US3] Refactor note-related composables to rely only on shared lifecycle helpers and store subscribe/select patterns in app/src/notes/composables/useGlobalNoteBrowsing.ts and app/src/notes/composables/index.ts
- [ ] T036 [US3] Document the composable adoption pattern for future domains in docs/interface/frontend.md and specs/016-vaultbase-subscriptions/quickstart.md

**Checkpoint**: All user stories should now be independently functional.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final hardening across multiple user stories.

- [ ] T037 [P] Add view-level Vitest validation coverage for live vault and note updates in app/src/vaults/views/VaultView.spec.ts and app/src/notes/views/NoteView.spec.ts
- [X] T038 Harden transient interruption and error-state handling across backend and frontend in crates/notes-server-axum/src/routes/subscriptions.rs and app/src/shared/api/subscriptionManager.ts
- [X] T039 [P] Clean up obsolete direct-fetch flows superseded by NyxBase in app/src/vaults/api/vaults.ts, app/src/notes/api/notes.ts, and related composables/stores
- [X] T040 Run and document quickstart validation outcomes in specs/016-vaultbase-subscriptions/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story phases (Phase 3-5)**: Depend on Foundational completion
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Starts after Phase 2 and defines the MVP
- **User Story 2 (P2)**: Starts after Phase 2 and builds on the same manager/broker foundation, but remains independently testable through duplicate-consumer scenarios
- **User Story 3 (P3)**: Starts after Phase 2 and depends conceptually on the shared manager existing, but remains independently testable through lifecycle switching and cleanup scenarios

### Within Each User Story

- Tests should be written before or alongside implementation and used to validate the story independently
- Backend live scope delivery should be in place before frontend consumers rely on the scope
- Store integration should be completed before view/composable adoption
- Lifecycle cleanup and stale-update guards should land before broad composable rollout

### Parallel Opportunities

- **Setup**: T003 can run in parallel with T001-T002 after structure is known
- **Foundational**: T005 and T009 can run in parallel with backend/frontend scaffolding work; T010 and T011 can proceed in parallel after shared live types exist
- **US1**: T013-T015 can run in parallel; T019 and T020 can run in parallel after T018; T021 and T022 can run in parallel after store integration is ready
- **US2**: T023 and T024 can run in parallel; T028 and T029 can run in parallel once T025-T027 establish the shared query-key behavior
- **US3**: T030 and T031 can run in parallel; T034 and T035 can run in parallel after T032-T033
- **Polish**: T037 and T039 can run in parallel; T040 follows validation completion

---

## Parallel Example: User Story 1

```bash
# Parallel test preparation for User Story 1:
Task: "Add Axum integration coverage for vault-list, note-list, and note live subscription scopes in crates/notes-server-axum/tests/subscriptions_api.rs"
Task: "Add frontend subscription manager snapshot/reconnect tests in app/src/shared/api/subscriptionManager.spec.ts"
Task: "Add NyxBase live scope tests in app/src/shared/api/nyxBase.spec.ts"

# Parallel store integration for User Story 1 after manager replay logic exists:
Task: "Adapt the vault store to consume live vault list snapshots from NyxBase in app/src/vaults/stores/vaults.ts"
Task: "Adapt the notes store to consume live note list and note document snapshots from NyxBase in app/src/notes/stores/notes.ts"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Confirm live vault and note updates work without manual refresh
5. Demo the centralized live data flow with vault and note screens only

### Incremental Delivery

1. Setup + Foundational establish shared live infrastructure
2. Add User Story 1 to deliver the first live product slice
3. Add User Story 2 to remove redundant subscriptions and improve reuse
4. Add User Story 3 to standardize composable lifecycle behavior for future adoption
5. Finish with polish, error hardening, and quickstart verification

### Parallel Team Strategy

With multiple developers:

1. One developer drives backend broker/live route infrastructure
2. One developer builds frontend `NyxBase` and subscription manager
3. One developer prepares store/composable adoption and tests
4. After Phase 2, developers can split across US1-US3 by story while coordinating on shared interfaces

---

## Notes

- All tasks follow the required checklist format with IDs, optional `[P]`, story labels where required, and exact file paths.
- User story phases are designed to remain independently testable.
- Suggested MVP scope: **User Story 1** only.
