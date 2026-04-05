# Tasks: User Management

**Input**: Design documents from `/specs/014-user-management/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Include backend integration tests, frontend unit tests, and E2E coverage because the implementation plan and constitution require layer-appropriate validation for this feature.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g. US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Update the authoritative docs and introduce the dependencies/configuration required for the embedded user store.

- [X] T001 Update user-management docs in `docs/architecture/core-domain.md`, `docs/architecture/authentication.md`, `docs/architecture/backend-api.md`, `docs/interface/frontend.md`, `docs/testing/README.md`, and `README.md` for `secret_key`-only MVP scope, required unique email, and self-management restrictions
- [X] T002 Add embedded user-store dependencies and configuration notes in `Cargo.toml`, `crates/notes-auth/Cargo.toml`, `crates/notes-server-axum/Cargo.toml`, and `.env.example`
- [X] T003 [P] Create the top-level users domain barrel files in `frontend/src/users/index.ts`, `frontend/src/users/api/index.ts`, `frontend/src/users/components/index.ts`, `frontend/src/users/composables/index.ts`, and `frontend/src/users/views/index.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core auth-store, route, and frontend scaffolding that MUST be complete before ANY user story can be implemented.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T004 Extend shared auth and user-management abstractions in `crates/notes-core/src/auth.rs` and `crates/notes-core/src/lib.rs` for `secret_key`-mode user management, required email, and self-management safeguards
- [X] T005 [P] Add SQLite-backed user store scaffolding in `crates/notes-auth/src/sqlite_user_store.rs` and export it from `crates/notes-auth/src/lib.rs`
- [X] T006 [P] Wire `SecretKeyAuthStore` bootstrap and storage lifecycle to the embedded store in `crates/notes-auth/src/secret_key.rs` and `crates/notes-server-axum/src/main.rs`
- [X] T007 [P] Add user-management request/response types and route registration scaffolding in `crates/notes-server-axum/src/types.rs`, `crates/notes-server-axum/src/routes/mod.rs`, and `crates/notes-server-axum/src/lib.rs`
- [X] T008 [P] Add frontend users route scaffolding in `frontend/src/shared/types/router.ts`, `frontend/src/shared/router/index.ts`, and `frontend/src/users/views/UsersView.vue`

**Checkpoint**: Foundation ready - user story implementation can now begin.

---

## Phase 3: User Story 1 - Review Existing Users (Priority: P1) 🎯 MVP

**Goal**: Let an administrator navigate to a dedicated users page and review the current managed accounts in a table.

**Independent Test**: Sign in as an administrator, open the sidebar, click `Users`, and confirm the page shows the current user list with loading, empty, and error states plus row actions.

### Tests for User Story 1

- [X] T009 [P] [US1] Add embedded-store list and bootstrap tests in `crates/notes-auth/tests/sqlite_user_store.rs` for `secret_key`-mode user listing metadata
- [X] T010 [P] [US1] Add admin-only list API tests in `crates/notes-server-axum/tests/users_api.rs`
- [X] T011 [P] [US1] Add users navigation and list rendering tests in `frontend/src/users/views/UsersView.spec.ts` and `frontend/src/shared/components/SidebarNav.spec.ts`

### Implementation for User Story 1

- [X] T012 [US1] Implement user listing and summary mapping in `crates/notes-core/src/auth.rs` and `crates/notes-auth/src/sqlite_user_store.rs` including required email and allowed-action metadata
- [X] T013 [US1] Implement `GET /api/users` and admin authorization in `crates/notes-server-axum/src/routes/users.rs` and `crates/notes-server-axum/src/routes/mod.rs`
- [X] T014 [US1] Implement users API client and list composable in `frontend/src/users/api/users.ts` and `frontend/src/users/composables/useUsers.ts`
- [X] T015 [US1] Build the users table view with `NyxTable` plus loading, empty, and error states in `frontend/src/users/views/UsersView.vue` and `frontend/src/users/components/UsersTable.vue`
- [X] T016 [US1] Add the `Users` sidebar entry and route integration in `frontend/src/shared/components/SidebarNav.vue`, `frontend/src/shared/router/index.ts`, `frontend/src/shared/types/router.ts`, and `frontend/src/users/views/index.ts`

**Checkpoint**: User Story 1 should now be fully functional and independently testable.

---

## Phase 4: User Story 2 - Create And Update Users (Priority: P1)

**Goal**: Let an administrator add new users and edit existing users from the users page without direct access to backend storage.

**Independent Test**: From the users page, create a new user, confirm it appears in the list, edit that user, and confirm the saved metadata and password behavior are reflected correctly.

### Tests for User Story 2

- [X] T017 [P] [US2] Add create/update validation tests for required email, unique email, duplicate username, and the 12-character plus 3-of-4-category password policy in `crates/notes-auth/tests/sqlite_user_store.rs`
- [X] T018 [P] [US2] Add `POST /api/users` and `PATCH /api/users/:user_id` contract tests for required email, duplicate email, the 12-character plus 3-of-4-category password policy, and self-demotion blocking in `crates/notes-server-axum/tests/users_api.rs` and `crates/notes-server-axum/tests/auth_api.rs`
- [X] T019 [P] [US2] Add create/edit modal workflow tests for required email, validation feedback, and self-demotion blocking in `frontend/src/users/components/CreateEditUser.spec.ts` and `frontend/src/users/views/UsersView.spec.ts`

### Implementation for User Story 2

- [X] T020 [US2] Implement create/update mutations with required email, duplicate-username, duplicate-email, and 12-character plus 3-of-4-category password-policy validation in `crates/notes-core/src/auth.rs` and `crates/notes-auth/src/sqlite_user_store.rs`
- [X] T021 [US2] Implement `POST /api/users` and `PATCH /api/users/:user_id` handlers with safe validation errors for required email, duplicate email, and self-demotion blocking in `crates/notes-server-axum/src/routes/users.rs`, `crates/notes-server-axum/src/types.rs`, and `crates/notes-server-axum/src/error.rs`
- [X] T022 [US2] Implement the shared create/edit modal using `NyxModal`, `NyxButton`, and nyx-kit form primitives in `frontend/src/users/components/CreateEditUser.vue` and `frontend/src/users/components/index.ts` with required email input, explicit 12-character plus 3-of-4-category password validation messaging, and self-demotion blocking
- [X] T023 [US2] Wire add/edit actions, modal state, and list refresh behavior in `frontend/src/users/views/UsersView.vue` and `frontend/src/users/composables/useUsers.ts`
- [X] T024 [US2] Ensure sign-in uses newly created users and rotated passwords in `crates/notes-auth/src/secret_key.rs` and `crates/notes-server-axum/tests/auth_api.rs`

**Checkpoint**: User Stories 1 and 2 should both work independently.

---

## Phase 5: User Story 3 - Remove Users Safely (Priority: P2)

**Goal**: Let an administrator delete eligible users while preventing administrative lockout and showing clear protection failures.

**Independent Test**: Delete an eligible user from the users page and confirm the account disappears and cannot sign in; then attempt to delete the last administrator and confirm the action is blocked with an explanatory message.

### Tests for User Story 3

- [X] T025 [P] [US3] Add protected-account, self-delete, and delete mutation tests in `crates/notes-auth/tests/sqlite_user_store.rs`
- [X] T026 [P] [US3] Add `DELETE /api/users/:user_id` protection tests for self-delete and last-admin conflicts in `crates/notes-server-axum/tests/users_api.rs`
- [X] T027 [P] [US3] Add delete confirmation, self-delete blocking, and protected-account UI tests in `frontend/src/users/views/UsersView.spec.ts`

### Implementation for User Story 3

- [X] T028 [US3] Implement delete mutations and last-admin/self-delete safeguards in `crates/notes-core/src/auth.rs` and `crates/notes-auth/src/sqlite_user_store.rs`
- [X] T029 [US3] Implement `DELETE /api/users/:user_id` conflict handling for self-delete and protected-account failures in `crates/notes-server-axum/src/routes/users.rs` and `crates/notes-server-axum/src/error.rs`
- [X] T030 [US3] Implement delete confirmation UX, disabled self-delete states, and protected-account messaging in `frontend/src/users/views/UsersView.vue` and `frontend/src/users/composables/useUsers.ts`

**Checkpoint**: All user stories should now be independently functional.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Finish validation, broad coverage, and feature-level hardening that spans multiple stories.

- [X] T031 [P] Add end-to-end admin user-management coverage in `frontend/tests/e2e/users-management.spec.ts`
- [X] T032 [P] Add cross-story backend auth and authorization edge-case coverage for auth-mode scope, duplicate email, self-demotion, and protected-account failures in `crates/notes-server-axum/tests/users_api.rs` and `crates/notes-server-axum/tests/auth_api.rs`
- [X] T033 Run full feature validation using `Cargo.toml`, `frontend/package.json`, and `specs/014-user-management/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational completion - MVP slice
- **User Story 2 (Phase 4)**: Depends on Foundational completion and reuses the Users page introduced in US1
- **User Story 3 (Phase 5)**: Depends on Foundational completion and reuses the Users page/actions introduced in US1
- **Polish (Phase 6)**: Depends on all implemented user stories being complete

### User Story Dependencies

- **US1**: Can start after Phase 2 - no dependency on other user stories
- **US2**: Can start after Phase 2, but is simplest to land after US1 because create/edit entry points live on the Users page
- **US3**: Can start after Phase 2, but is simplest to land after US1 because delete actions live on the Users page

### Within Each User Story

- Tests should be added before or alongside implementation and must fail before the feature is considered complete
- Shared store/domain mutations precede route handlers
- Route handlers precede frontend integration
- Frontend composables precede view wiring

### Parallel Opportunities

- `T003` can run alongside `T001-T002`
- `T005-T008` can run in parallel once Setup completes
- In each user story, backend store tests, API tests, and frontend tests can be written in parallel
- In each user story, frontend API/composable work can proceed in parallel with backend handler work once shared domain mutations are defined
- `T031` and `T032` can run in parallel after all stories are complete

---

## Parallel Example: User Story 1

```bash
# Parallel test work for US1
Task: "Add embedded-store list and bootstrap tests in crates/notes-auth/tests/sqlite_user_store.rs"
Task: "Add admin-only list API tests in crates/notes-server-axum/tests/users_api.rs"
Task: "Add users navigation and list rendering tests in frontend/src/users/views/UsersView.spec.ts and frontend/src/shared/components/SidebarNav.spec.ts"

# Parallel implementation work for US1 after shared listing contracts exist
Task: "Implement GET /api/users in crates/notes-server-axum/src/routes/users.rs and crates/notes-server-axum/src/routes/mod.rs"
Task: "Implement users API client and list composable in frontend/src/users/api/users.ts and frontend/src/users/composables/useUsers.ts"
```

## Parallel Example: User Story 2

```bash
# Parallel test work for US2
Task: "Add create/update validation tests in crates/notes-auth/tests/sqlite_user_store.rs"
Task: "Add POST/PATCH contract tests in crates/notes-server-axum/tests/users_api.rs and crates/notes-server-axum/tests/auth_api.rs"
Task: "Add create/edit modal workflow tests in frontend/src/users/components/CreateEditUser.spec.ts and frontend/src/users/views/UsersView.spec.ts"

# Parallel implementation work for US2 after mutation contracts exist
Task: "Implement POST/PATCH handlers in crates/notes-server-axum/src/routes/users.rs, crates/notes-server-axum/src/types.rs, and crates/notes-server-axum/src/error.rs"
Task: "Implement CreateEditUser modal in frontend/src/users/components/CreateEditUser.vue and frontend/src/users/components/index.ts"
```

## Parallel Example: User Story 3

```bash
# Parallel test work for US3
Task: "Add protected-account delete tests in crates/notes-auth/tests/sqlite_user_store.rs"
Task: "Add DELETE protection tests in crates/notes-server-axum/tests/users_api.rs"
Task: "Add delete confirmation UI tests in frontend/src/users/views/UsersView.spec.ts"

# Parallel implementation work for US3 after delete mutation contracts exist
Task: "Implement DELETE /api/users/:user_id handling in crates/notes-server-axum/src/routes/users.rs and crates/notes-server-axum/src/error.rs"
Task: "Implement delete confirmation UX in frontend/src/users/views/UsersView.vue and frontend/src/users/composables/useUsers.ts"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Validate that administrators can reach and load the Users page independently
5. Demo the MVP before adding mutation flows

### Incremental Delivery

1. Finish Setup + Foundational to establish docs, SQLite auth storage, and route scaffolding
2. Deliver US1 for navigation and visibility into existing accounts
3. Deliver US2 for create/edit flows and validation
4. Deliver US3 for safe deletion and protected-account enforcement
5. Finish with cross-cutting E2E and auth-edge validation

### Parallel Team Strategy

1. One engineer updates docs and dependencies while another scaffolds the frontend users domain
2. After Phase 2, backend and frontend work can split per story
3. Final validation can run with one engineer on E2E and another on backend auth edge cases

---

## Notes

- `[P]` tasks target different files and are safe to parallelize
- `[US1]`, `[US2]`, and `[US3]` labels map directly to the spec user stories
- Every story phase contains enough work to be completed and validated independently
- The suggested MVP scope is **User Story 1 only**
