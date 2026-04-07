# Tasks: Multi-Instance Access Profiles

**Input**: Design documents from `/specs/011-add-server-profiles/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: Frontend Vitest and Playwright coverage plus Axum auth-route integration coverage are included because the constitution and plan require verification at the affected layer boundaries.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (`[US1]`, `[US2]`, `[US3]`)
- Every task includes exact file paths

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Update source-of-truth docs and create the shared type/storage surface the implementation will build on.

- [X] T001 Update feature docs in `docs/architecture/authentication.md`, `docs/architecture/deployment-modes.md`, `docs/interface/frontend.md`, and `README.md`
- [X] T002 [P] Add workspace profile domain types in `app/src/shared/types/profile.ts` and export them from `app/src/shared/types/index.ts`
- [X] T003 [P] Add client profile persistence helpers in `app/src/shared/utils/profileStorage.ts`, `app/src/shared/utils/profileSecrets.ts`, and `app/src/shared/utils/index.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core auth, routing, and API infrastructure that must exist before any story can be completed.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T004 Extend auth discovery/login client helpers in `app/src/auth/api/auth.ts` and `app/src/auth/api/index.ts`
- [X] T005 [P] Add optional discovery metadata support in `crates/notes-server-axum/src/routes/auth.rs` and `crates/notes-server-axum/src/types.rs`
- [X] T006 [P] Add auth discovery/login integration coverage in `crates/notes-server-axum/tests/auth_api.rs`
- [X] T007 Create workspace profile composable scaffolding in `app/src/shared/composables/useWorkspaceProfiles.ts` and `app/src/shared/composables/index.ts`
- [X] T008 Wire active-profile-aware API client behavior in `app/src/shared/api/client.ts` and `app/src/shared/api/index.ts`
- [X] T009 Connect profile bootstrapping to auth state in `app/src/auth/composables/useAuth.ts` and `app/src/auth/composables/index.ts`
- [X] T010 Update shell and router bootstrap for active profile loading in `app/src/shared/router/index.ts`, `app/src/shared/components/AppLayout.vue`, and `app/src/App.vue`

**Checkpoint**: Foundation ready. User story implementation can now proceed.

---

## Phase 3: User Story 1 - Choose an Initial Operating Mode (Priority: P1) 🎯 MVP

**Goal**: Let a first-time user choose local or server-oriented setup and complete local setup or remote connection onboarding from a clean install.

**Independent Test**: Start with no saved profiles, complete onboarding once in local mode and once in server mode using `Connect to an existing server`, and verify the app lands in the correct workspace without cross-mode leakage.

### Tests for User Story 1

- [X] T011 [P] [US1] Add first-run setup composable tests in `app/src/auth/composables/useAuth.spec.ts`
- [X] T012 [P] [US1] Add onboarding E2E coverage for local and connect-existing flows in `app/tests/e2e/profile-onboarding.spec.ts`

### Implementation for User Story 1

- [X] T013 [P] [US1] Create onboarding view-state types in `app/src/auth/types/profileSetup.ts` and export them from `app/src/auth/index.ts`
- [X] T014 [P] [US1] Build onboarding form components in `app/src/auth/components/InstallationModeStep.vue`, `app/src/auth/components/RemoteProfileForm.vue`, and `app/src/auth/components/index.ts`
- [X] T015 [US1] Implement first-run setup state transitions in `app/src/auth/composables/useAuth.ts`
- [X] T016 [US1] Implement local/server onboarding UI in `app/src/auth/views/LoginView.vue`
- [X] T017 [US1] Wire first-run profile creation and app entry behavior in `app/src/main.ts`, `app/src/App.vue`, and `app/src/shared/router/index.ts`

**Checkpoint**: User Story 1 should support fresh-install local setup and initial remote connection onboarding on its own.

---

## Phase 4: User Story 2 - Manage Multiple Server Connections (Priority: P2)

**Goal**: Let the client save multiple remote profiles, including multiple accounts on the same server URL, and switch between them and the local profile.

**Independent Test**: Add one local profile plus at least two remote profiles, including two profiles that share the same server URL but use different usernames, then switch among them and confirm the correct workspace loads each time.

### Tests for User Story 2

- [X] T018 [P] [US2] Add workspace profile persistence and duplicate-rule tests in `app/src/shared/composables/useWorkspaceProfiles.spec.ts`
- [X] T019 [P] [US2] Add profile switching E2E coverage in `app/tests/e2e/profile-switching.spec.ts`

### Implementation for User Story 2

- [X] T020 [US2] Implement profile create/edit/remove logic in `app/src/shared/composables/useWorkspaceProfiles.ts`
- [X] T021 [P] [US2] Add profile switcher UI in `app/src/shared/components/ProfileSwitcher.vue` and export it from `app/src/shared/components/index.ts`
- [X] T022 [US2] Integrate profile switcher and profile management entry points in `app/src/shared/components/AppLayout.vue` and `app/src/shared/components/SidebarNav.vue`
- [X] T023 [US2] Implement add-edit-remove remote profile flows in `app/src/auth/views/LoginView.vue` and `app/src/auth/components/RemoteProfileForm.vue`
- [X] T024 [US2] Persist last-route and activation metadata per profile in `app/src/shared/composables/useWorkspaceProfiles.ts` and `app/src/shared/router/index.ts`

**Checkpoint**: User Story 2 should allow multiple saved remote profiles and clear switching between local and remote contexts independent of US3 failure handling.

---

## Phase 5: User Story 3 - Keep Authentication Bound to Each Server (Priority: P3)

**Goal**: Isolate auth/session state per profile so sign-in failures, sign-out, and credential updates affect only the targeted server profile.

**Independent Test**: Save two remote profiles with different credentials, sign into both, invalidate one profile's credentials, and verify the failure, sign-out, and retry behavior remains isolated to that profile while the other stays usable.

### Tests for User Story 3

- [X] T025 [P] [US3] Add auth session isolation unit tests in `app/src/auth/composables/useAuth.spec.ts`
- [X] T026 [P] [US3] Add auth failure isolation E2E coverage in `app/tests/e2e/profile-auth-isolation.spec.ts`
- [X] T027 [P] [US3] Extend auth route integration coverage for unsupported mode and invalid credentials in `crates/notes-server-axum/tests/auth_api.rs`

### Implementation for User Story 3

- [X] T028 [US3] Implement per-profile session lifecycle and sign-out behavior in `app/src/auth/composables/useAuth.ts`
- [X] T029 [US3] Add profile-scoped credential update and error handling UI in `app/src/auth/views/LoginView.vue` and `app/src/auth/components/RemoteProfileForm.vue`
- [X] T030 [US3] Abort in-flight requests and clear stale data on profile changes in `app/src/shared/api/client.ts`, `app/src/vaults/stores/vaults.ts`, `app/src/notes/stores/notes.ts`, `app/src/teams/composables/useTeams.ts`, and `app/src/comments/composables/useComments.ts`
- [X] T031 [US3] Enforce unsupported-mode and isolated-session cleanup rules in `app/src/shared/composables/useWorkspaceProfiles.ts` and `app/src/auth/api/auth.ts`

**Checkpoint**: All user stories should now work independently, with remote auth and failure handling fully scoped per profile.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Finalize shared validation, barrels, and test/document alignment across the whole feature.

- [X] T032 [P] Update backend/testing docs in `docs/architecture/backend-api.md` and `docs/testing/README.md`
- [X] T033 [P] Clean up and export new shared/auth modules in `app/src/shared/index.ts`, `app/src/auth/index.ts`, `app/src/shared/components/index.ts`, `app/src/shared/composables/index.ts`, and `app/src/auth/components/index.ts`
- [ ] T034 Validate quickstart coverage against `app/tests/e2e/profile-onboarding.spec.ts`, `app/tests/e2e/profile-switching.spec.ts`, `app/tests/e2e/profile-auth-isolation.spec.ts`, and `specs/011-add-server-profiles/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1: Setup**: No dependencies; starts immediately
- **Phase 2: Foundational**: Depends on Phase 1 and blocks all user stories
- **Phase 3: US1**: Depends on Phase 2; recommended MVP slice
- **Phase 4: US2**: Depends on Phase 2 and benefits from US1 UI scaffolding but remains independently testable
- **Phase 5: US3**: Depends on Phase 2 and uses the profile/session surfaces introduced in US1 and US2
- **Phase 6: Polish**: Depends on all desired user stories being complete

### User Story Dependencies

- **US1 (P1)**: Starts after Foundational; no dependency on US2 or US3
- **US2 (P2)**: Starts after Foundational; can reuse US1 onboarding components but delivers independent saved-profile management
- **US3 (P3)**: Starts after Foundational; depends on the existence of saved remote profiles but isolates auth behavior independently of the onboarding flow

### Within Each User Story

- Tests should be added before or alongside implementation and must validate the story-specific acceptance path
- Shared types/forms before composable logic
- Composable logic before router/shell integration
- Profile-scoped failure handling before final polish

### Parallel Opportunities

- `T002` and `T003` can run in parallel after `T001`
- `T005` and `T006` can run in parallel with `T007` during foundational work
- In US1, `T013` and `T014` can run in parallel before `T015`
- In US2, `T018`, `T019`, and `T021` can run in parallel once profile composable scaffolding exists
- In US3, `T025`, `T026`, and `T027` can run in parallel while session-isolation implementation is underway
- `T032` and `T033` can run in parallel during polish

---

## Parallel Example: User Story 1

```bash
Task: "Add first-run setup composable tests in app/src/auth/composables/useAuth.spec.ts"
Task: "Add onboarding E2E coverage for local and connect-existing flows in app/tests/e2e/profile-onboarding.spec.ts"
Task: "Build onboarding form components in app/src/auth/components/InstallationModeStep.vue, app/src/auth/components/RemoteProfileForm.vue, and app/src/auth/components/index.ts"
```

## Parallel Example: User Story 2

```bash
Task: "Add workspace profile persistence and duplicate-rule tests in app/src/shared/composables/useWorkspaceProfiles.spec.ts"
Task: "Add profile switching E2E coverage in app/tests/e2e/profile-switching.spec.ts"
Task: "Add profile switcher UI in app/src/shared/components/ProfileSwitcher.vue and app/src/shared/components/index.ts"
```

## Parallel Example: User Story 3

```bash
Task: "Add auth session isolation unit tests in app/src/auth/composables/useAuth.spec.ts"
Task: "Add auth failure isolation E2E coverage in app/tests/e2e/profile-auth-isolation.spec.ts"
Task: "Extend auth route integration coverage for unsupported mode and invalid credentials in crates/notes-server-axum/tests/auth_api.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Validate fresh-install local setup and initial remote connection onboarding
5. Demo or ship the MVP onboarding slice

### Incremental Delivery

1. Finish Setup + Foundational to establish docs, auth discovery, and profile-aware client bootstrapping
2. Deliver US1 for first-run mode selection and initial remote connection
3. Deliver US2 for saved multi-profile management and switching
4. Deliver US3 for isolated auth/session behavior and failure handling
5. Finish with polish and quickstart validation

### Parallel Team Strategy

1. One engineer updates docs and backend discovery coverage while another builds shared profile types/storage
2. After Foundational, one engineer can own onboarding (`US1`) while another prepares switching surfaces (`US2`)
3. Session isolation and failure handling (`US3`) can proceed once saved-profile infrastructure exists

---

## Notes

- All tasks use the required checklist format
- Story labels are applied only to user-story phases
- Every user story includes independent validation criteria
- The docs-first rule is captured as the first executable task
