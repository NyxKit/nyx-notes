# Tasks: File System Architecture Alignment

**Input**: Design documents from `/home/arnedecant/Projects/nyxkit/nyx-notes/specs/013-file-system/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`

**Tests**: Include test updates because the spec explicitly requires relevant tests to be updated for the new layout and role model.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (`US1`, `US2`, `US3`)
- Every task includes an exact file path

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Refresh the feature docs and task scaffolding that all implementation work depends on.

- [X] T001 Reconcile the clarified feature contract in `/home/arnedecant/Projects/nyxkit/nyx-notes/specs/013-file-system/spec.md`, `/home/arnedecant/Projects/nyxkit/nyx-notes/specs/013-file-system/plan.md`, `/home/arnedecant/Projects/nyxkit/nyx-notes/specs/013-file-system/research.md`, `/home/arnedecant/Projects/nyxkit/nyx-notes/specs/013-file-system/data-model.md`, `/home/arnedecant/Projects/nyxkit/nyx-notes/specs/013-file-system/contracts/filesystem-metadata.md`, and `/home/arnedecant/Projects/nyxkit/nyx-notes/specs/013-file-system/contracts/http-api.md`
- [X] T002 [P] Create a divergence entry for the old filesystem/team model in `/home/arnedecant/Projects/nyxkit/nyx-notes/AGENTS.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish the shared contracts and configuration assumptions that block all story work.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T003 Update `/home/arnedecant/Projects/nyxkit/nyx-notes/docs/architecture/README.md` to reference `file-system.md` as the active storage source document and remove the old `users/` + `teams/` summary
- [X] T004 [P] Update `/home/arnedecant/Projects/nyxkit/nyx-notes/README.md` to describe the new server/home/server-vault layout, slug-based API identifiers, and deferred `local/` runtime support
- [X] T005 [P] Update `/home/arnedecant/Projects/nyxkit/nyx-notes/.env.example` to replace the old `users/<NOTES_USER_ID>/` assumptions with the new MVP configuration model
- [X] T006 [P] Update `/home/arnedecant/Projects/nyxkit/nyx-notes/docs/testing/README.md` to define the new storage, API, CLI, and frontend test expectations for slug-based identifiers and server roles

**Checkpoint**: Shared docs and test expectations are aligned; user story work can now proceed.

---

## Phase 3: User Story 1 - Persist notes in the new server-first filesystem layout (Priority: P1) 🎯 MVP

**Goal**: As a maintainer, I can store and resolve personal and shared vault content using the new server/home/server-vault filesystem layout without team directories.

**Independent Test**: With a temp `NOTES_ROOT`, storage can create and load personal vaults under `<server>/homes/<home>/<vault>`, create and load shared server vaults under `<server>/vaults/<vault>`, and preserve note/comment sidecars and metadata under the new structure.

### Tests for User Story 1

- [X] T007 [P] [US1] Update storage integration tests for the new namespace layout in `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-storage-fs/src/tests.rs`
- [X] T008 [P] [US1] Add core-domain unit coverage for the new ownership and role types in `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-core/src/domain.rs`

### Implementation for User Story 1

- [X] T009 [P] [US1] Rewrite the canonical storage specification in `/home/arnedecant/Projects/nyxkit/nyx-notes/docs/architecture/filesystem-storage.md` to match `docs/architecture/file-system.md`
- [X] T010 [P] [US1] Replace the old team-centric architecture contract in `/home/arnedecant/Projects/nyxkit/nyx-notes/docs/architecture/vaults-and-teams.md` with the new server/home/server-vault ownership model or an explicitly renamed replacement document
- [X] T011 [US1] Update domain ownership, role, and storage trait definitions in `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-core/src/domain.rs` and `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-core/src/storage.rs`
- [X] T012 [US1] Implement `.server.json`, `.home.json`, `.vault.json`, and `.local.json` metadata codecs in `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-storage-fs/src/meta.rs`
- [X] T013 [US1] Replace the old `users/` and `teams/` path helpers, vault discovery, and namespace resolution logic in `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-storage-fs/src/lib.rs`
- [X] T014 [US1] Update storage-side exports and compile-time plumbing for the new domain/storage API in `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-storage-fs/src/lib.rs` and `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-core/src/lib.rs`

**Checkpoint**: The filesystem contract and storage implementation work for the new layout independently of HTTP, CLI, and frontend concerns.

---

## Phase 4: User Story 2 - Use the new layout and roles through server and CLI surfaces (Priority: P2)

**Goal**: As an authenticated user or admin, I can use the backend API and CLI against the new server/home/server-vault model with slug-based route identifiers and without team routes.

**Independent Test**: Server integration tests pass for slug-based vault routes, admin-only server-vault management, and owner-scoped personal vaults; CLI integration tests pass using the new namespace resolution and without team commands.

### Tests for User Story 2

- [X] T015 [P] [US2] Update auth and vault HTTP integration tests for slug-based routes and server roles in `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-server-axum/tests/auth_api.rs`, `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-server-axum/tests/comments_api.rs`, and related server test files under `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-server-axum/tests/`
- [X] T016 [P] [US2] Update CLI subprocess coverage for the new namespace layout and removed team flows in `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-cli/tests/` or the existing CLI test module locations

### Implementation for User Story 2

- [X] T017 [P] [US2] Rewrite the API contract in `/home/arnedecant/Projects/nyxkit/nyx-notes/docs/architecture/backend-api.md` for server metadata, server-vault administration, slug-based route identifiers, and removal of `/api/teams/*`
- [X] T018 [P] [US2] Update auth and deployment docs in `/home/arnedecant/Projects/nyxkit/nyx-notes/docs/architecture/authentication.md` and `/home/arnedecant/Projects/nyxkit/nyx-notes/docs/architecture/deployment-modes.md` for server slug derivation, `NOTES_USER_ID` home lookup, and `admin` / `user` roles
- [X] T019 [P] [US2] Update CLI docs in `/home/arnedecant/Projects/nyxkit/nyx-notes/docs/interface/cli.md` to remove team addressing and describe slug-based personal/server vault usage
- [X] T020 [US2] Replace team request/response types and route registrations in `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-server-axum/src/types.rs`, `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-server-axum/src/routes/mod.rs`, and `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-server-axum/src/storage_adapter.rs`
- [X] T021 [US2] Implement the new server-vault and slug-based authorization behavior in `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-server-axum/src/routes/vaults.rs`, `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-server-axum/src/routes/notes.rs`, and `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-server-axum/src/routes/comments.rs`
- [X] T022 [US2] Remove team route handlers and replace them with server metadata/admin behavior in `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-server-axum/src/routes/teams.rs` and any successor route module files under `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-server-axum/src/routes/`
- [X] T023 [US2] Update server startup and local auth role/home resolution in `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-server-axum/src/main.rs`, `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-auth-local/src/local.rs`, `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-auth-local/src/secret_key.rs`, and `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-auth-local/src/user_store.rs`
- [X] T024 [US2] Remove team-centric CLI context and commands in `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-cli/src/config.rs`, `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-cli/src/context.rs`, `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-cli/src/cmd/vault.rs`, `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-cli/src/cmd/team.rs`, `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-cli/src/cmd/notes.rs`, and `/home/arnedecant/Projects/nyxkit/nyx-notes/crates/notes-cli/src/main.rs`

**Checkpoint**: The backend and CLI expose the new filesystem and role model independently of frontend cleanup.

---

## Phase 5: User Story 3 - Align frontend contracts and docs with the new storage model (Priority: P3)

**Goal**: As a frontend contributor, I can rely on the documented frontend routes, shared types, and stores to match the new server/home/server-vault model and the removal of teams from the MVP.

**Independent Test**: Frontend unit tests pass with the updated shared types and route contract, no active team route remains in the MVP flow, and vault UI uses slug-based identifiers and server-role-aware behavior.

### Tests for User Story 3

- [X] T025 [P] [US3] Update frontend unit tests affected by owner/type and route changes in `/home/arnedecant/Projects/nyxkit/nyx-notes/frontend/src/auth/composables/useAuth.spec.ts`, `/home/arnedecant/Projects/nyxkit/nyx-notes/frontend/src/vaults/views/VaultSettingsView.spec.ts`, and related spec files under `/home/arnedecant/Projects/nyxkit/nyx-notes/frontend/src/`

### Implementation for User Story 3

- [X] T026 [P] [US3] Rewrite the frontend interface spec in `/home/arnedecant/Projects/nyxkit/nyx-notes/docs/interface/frontend.md` for server/home/server-vault navigation and retirement of team settings from the MVP
- [X] T027 [US3] Update shared frontend domain types and route names in `/home/arnedecant/Projects/nyxkit/nyx-notes/frontend/src/shared/types/index.ts`, `/home/arnedecant/Projects/nyxkit/nyx-notes/frontend/src/shared/types/router.ts`, and `/home/arnedecant/Projects/nyxkit/nyx-notes/frontend/src/shared/router/index.ts`
- [X] T028 [P] [US3] Remove team-specific frontend API and composable contracts in `/home/arnedecant/Projects/nyxkit/nyx-notes/frontend/src/teams/api/teams.ts`, `/home/arnedecant/Projects/nyxkit/nyx-notes/frontend/src/teams/composables/useTeams.ts`, and `/home/arnedecant/Projects/nyxkit/nyx-notes/frontend/src/teams/views/TeamSettingsView.vue`
- [X] T029 [US3] Update vault UI state and API usage for server-owned vaults in `/home/arnedecant/Projects/nyxkit/nyx-notes/frontend/src/vaults/api/vaults.ts`, `/home/arnedecant/Projects/nyxkit/nyx-notes/frontend/src/vaults/stores/vaults.ts`, `/home/arnedecant/Projects/nyxkit/nyx-notes/frontend/src/vaults/components/VaultSwitcher.vue`, and `/home/arnedecant/Projects/nyxkit/nyx-notes/frontend/src/vaults/views/VaultSettingsView.vue`

**Checkpoint**: Frontend contracts, docs, and tests align with the new MVP architecture.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Finish cross-story validation and cleanup.

- [X] T030 [P] Run Rust validation in `/home/arnedecant/Projects/nyxkit/nyx-notes` with `cargo test`
- [X] T031 [P] Run frontend unit validation in `/home/arnedecant/Projects/nyxkit/nyx-notes/frontend` with the repository test command used for Vitest
- [X] T032 Run the manual validation checklist from `/home/arnedecant/Projects/nyxkit/nyx-notes/specs/013-file-system/quickstart.md` and update any remaining doc mismatches in `/home/arnedecant/Projects/nyxkit/nyx-notes/docs/` and `/home/arnedecant/Projects/nyxkit/nyx-notes/README.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies; start immediately.
- **Foundational (Phase 2)**: Depends on Setup completion; blocks all user stories.
- **User Story 1 (Phase 3)**: Depends on Phase 2 completion.
- **User Story 2 (Phase 4)**: Depends on Phase 3 because the server and CLI need the new core/storage contracts.
- **User Story 3 (Phase 5)**: Depends on Phase 4 because frontend contracts must match the finalized API and route surface.
- **Polish (Phase 6)**: Depends on all desired user stories being complete.

### User Story Dependencies

- **US1 (P1)**: No dependency on later stories; this is the MVP slice.
- **US2 (P2)**: Depends on US1 domain and storage work.
- **US3 (P3)**: Depends on US2 API contract finalization.

### Within Each User Story

- Tests should be updated before or alongside the implementation they verify.
- Docs and contracts should be updated before code that depends on them.
- Domain/storage changes must land before server/auth/CLI changes.
- Server/auth/CLI changes must land before frontend contract cleanup.

### Parallel Opportunities

- T004, T005, and T006 can run in parallel after T003.
- T007 and T008 can run in parallel within US1.
- T009 and T010 can run in parallel within US1 before the code changes.
- T015 and T016 can run in parallel within US2.
- T017, T018, and T019 can run in parallel within US2 before route implementation.
- T025 and T026 can run in parallel within US3.
- T028 can run in parallel with T027 before T029 integrates the new vault flows.
- T030 and T031 can run in parallel in the final phase.

---

## Parallel Example: User Story 1

```bash
# Update the US1 tests in parallel:
Task: "Update storage integration tests in crates/notes-storage-fs/src/tests.rs"
Task: "Add core-domain unit coverage in crates/notes-core/src/domain.rs"

# Update the US1 docs in parallel:
Task: "Rewrite docs/architecture/filesystem-storage.md"
Task: "Replace docs/architecture/vaults-and-teams.md with the new ownership model"
```

## Parallel Example: User Story 2

```bash
# Update the US2 docs/contracts in parallel:
Task: "Rewrite docs/architecture/backend-api.md"
Task: "Update docs/architecture/authentication.md and docs/architecture/deployment-modes.md"
Task: "Update docs/interface/cli.md"

# Update the US2 tests in parallel:
Task: "Update server integration tests under crates/notes-server-axum/tests/"
Task: "Update CLI integration tests under crates/notes-cli/tests/"
```

## Parallel Example: User Story 3

```bash
# Update the US3 contracts in parallel:
Task: "Rewrite docs/interface/frontend.md"
Task: "Remove team-specific frontend API/composable contracts under frontend/src/teams/"

# Update the US3 tests in parallel:
Task: "Update frontend unit tests under frontend/src/ for owner/type and route changes"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup.
2. Complete Phase 2: Foundational.
3. Complete Phase 3: User Story 1.
4. **STOP and VALIDATE**: Verify storage works with the new server/home/server-vault layout on a temp `NOTES_ROOT`.

### Incremental Delivery

1. Finish Setup + Foundational.
2. Deliver US1 to establish the new filesystem contract.
3. Deliver US2 to expose the new model through server and CLI surfaces.
4. Deliver US3 to align frontend contracts and docs with the backend model.
5. Finish with cross-cutting validation.

### Parallel Team Strategy

1. One contributor updates shared docs and test expectations in Phases 1-2.
2. One contributor drives US1 core/storage changes.
3. After US1 lands, one contributor handles US2 server/auth/CLI work while another prepares US3 frontend doc/test updates.
4. Finish with shared validation in Phase 6.

---

## Notes

- All tasks follow the required checklist format.
- Every user story is independently testable at its checkpoint.
- The suggested MVP scope is **US1 only**.
- `local/` remains documented but is not runtime-active in this feature.
