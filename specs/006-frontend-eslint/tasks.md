# Tasks: Frontend ESLint Integration

**Input**: Design documents from `/specs/006-frontend-eslint/`
**Prerequisites**: plan.md, spec.md, research.md, quickstart.md

**Organization**: Tasks grouped by user story. Setup and Foundational phases are prerequisites for all stories.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to
- No test tasks — not requested in spec

---

## Phase 1: Setup

**Purpose**: Install tooling and create the ESLint config. Nothing else can proceed until this is done.

- [x] T001 Install ESLint peer devDependencies in `frontend/package.json`: `eslint ^9.18.0`, `eslint-plugin-vue ^9.32.0`, `@vue/eslint-config-typescript ^14.3.0`, `eslint-plugin-oxlint ^0.15.6` (run `pnpm add -D` in `frontend/`)
- [x] T002 Create `frontend/eslint.config.mjs` — import and spread nyx-kit shared config from `nyx-kit/eslint`, then add a project-level override block for `**/*.{ts,vue}` that sets `semi: ['error', 'never']` and `quotes: ['error', 'single', { avoidEscape: true }]` (base rules; `@typescript-eslint/semi`/`quotes` were removed in typescript-eslint v8)

---

## Phase 2: Foundational

**Purpose**: Register lint commands so all user stories can be executed and verified.

**⚠️ CRITICAL**: All user story verification depends on `pnpm lint` and `pnpm lint:fix` being available.

- [x] T003 Add `"lint": "eslint ./src"` and `"lint:fix": "eslint ./src --fix"` to the `scripts` block in `frontend/package.json`

**Checkpoint**: Run `pnpm lint --version` to confirm ESLint resolves. Foundation ready.

---

## Phase 3: User Story 1 — Lint runs and passes (Priority: P1) 🎯 MVP

**Goal**: Confirm the ESLint config loads correctly and the linter runs end-to-end, producing actionable output.

**Independent Test**: `pnpm lint` in `frontend/` executes without crashing, resolves the nyx-kit shared config, and reports violations (or exits 0 if none). Exit code confirms the gate is active.

### Implementation

- [x] T004 [US1] Run `pnpm lint` in `frontend/` for the first time and capture the full output — confirm the config loads without module-not-found errors and that rules from the nyx-kit shared config are active (e.g. a Vue or TypeScript rule fires or the run completes cleanly)
- [x] T005 [US1] If the `@typescript-eslint/semi` or `@typescript-eslint/quotes` overrides in `frontend/eslint.config.mjs` are silenced by the oxlint shim, switch those overrides to the base `semi` and `quotes` rules instead (verify by temporarily introducing a semicolon in any `.ts` file and confirming lint catches it, then revert)

**Checkpoint**: `pnpm lint` runs to completion. Config is confirmed working. US1 is independently verified.

---

## Phase 4: User Story 2 — Existing code adjusted to comply (Priority: P2)

**Goal**: All source files in `frontend/src/` pass the rule set with zero errors and zero warnings. No suppression comments introduced.

**Independent Test**: `pnpm lint` exits 0. Zero `eslint-disable` directives anywhere in `frontend/src/`.

### Implementation

- [x] T006 [US2] Run `pnpm lint:fix` in `frontend/` — apply all safe auto-fixes (quotes, semicolons, simple import ordering). Commit the auto-fixed changes as a single atomic commit before proceeding to manual fixes
- [x] T007 [P] [US2] Run `pnpm lint` and fix any residual violations in `frontend/src/api/` files manually — resolve each reported error at the source without adding `eslint-disable`
- [x] T008 [P] [US2] Run `pnpm lint` and fix any residual violations in `frontend/src/components/` files manually — if `vue/multi-word-component-names` fires on any single-word component file, rename the file and update its import sites
- [x] T009 [P] [US2] Run `pnpm lint` and fix any residual violations in `frontend/src/composables/`, `frontend/src/stores/`, and `frontend/src/utils/` files manually
- [x] T010 [P] [US2] Run `pnpm lint` and fix any residual violations in `frontend/src/views/` and `frontend/src/router/` files manually
- [x] T011 [P] [US2] Run `pnpm lint` and fix any residual violations in `frontend/src/types/`, `frontend/src/main.ts`, and `frontend/App.vue` manually
- [x] T012 [US2] Run `pnpm lint` to confirm zero errors and zero warnings across all files in `frontend/src/` — this is the acceptance gate for US2

**Checkpoint**: `pnpm lint` exits 0. No suppression comments added. US2 complete.

---

## Phase 5: User Story 3 — Lint integrated into dev workflow (Priority: P3)

**Goal**: Lint and lint:fix are discoverable in `package.json` and run cleanly from a clean clone.

**Independent Test**: A developer who has never run lint before can type `pnpm lint` and receive a clean result without any additional setup.

### Implementation

- [x] T013 [US3] Verify `pnpm lint` and `pnpm lint:fix` appear in `pnpm run` output alongside `dev`, `build`, `preview`, and `type-check` — confirm naming follows the same convention as existing scripts
- [x] T014 [US3] Simulate a clean install: run `pnpm install` in `frontend/` then `pnpm lint` — confirm it resolves correctly without requiring any additional manual steps

**Checkpoint**: Workflow integration confirmed. US3 complete.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation and final validation.

- [x] T015 [P] Update `docs/conventions/README.md` — add one sentence under the Vue/TS section noting that `pnpm lint` (in `frontend/`) enforces the no-semicolons and single-quote conventions defined in Constitution §VI
- [x] T016 Run the full quickstart from `specs/006-frontend-eslint/quickstart.md` top to bottom — confirm all commands work as documented and output matches expectations (also verifies SC-004: lint completes in under 30 seconds)
- [x] T017 Confirm no `eslint-disable` directives were introduced anywhere in `frontend/src/` — run `grep -r 'eslint-disable' frontend/src/` and verify empty output

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 (T001, T002 complete)
- **US1 (Phase 3)**: Depends on Foundational (Phase 2 complete)
- **US2 (Phase 4)**: Depends on US1 (Phase 3 complete — config confirmed working)
- **US3 (Phase 5)**: Depends on US2 (Phase 4 complete — clean baseline exists)
- **Polish (Phase 6)**: Depends on US3 complete

### User Story Dependencies

- **US1 (P1)**: Requires Setup + Foundational. No dependency on US2 or US3.
- **US2 (P2)**: Requires US1 confirmed working. Code fixes are safe once config is validated.
- **US3 (P3)**: Requires US2 clean baseline. Workflow verification is meaningful only with a passing lint.

### Parallel Opportunities

Within Phase 4 (US2), once `lint:fix` has run (T006), the manual fix tasks T007–T011 target different directory trees and can be worked in parallel:

```bash
# Parallel manual fix pass (after T006 lint:fix commit):
Task T007: fix api/ violations
Task T008: fix components/ violations
Task T009: fix composables/, stores/, utils/ violations
Task T010: fix views/, router/ violations
Task T011: fix types/, main.ts, App.vue violations
```

---

## Implementation Strategy

### MVP First (US1 only — ~3 tasks)

1. Complete Phase 1: T001, T002 (install deps, create config)
2. Complete Phase 2: T003 (add scripts)
3. Complete Phase 3: T004, T005 (first lint run, config verified)
4. **STOP and VALIDATE**: `pnpm lint` runs without errors and config is confirmed active

### Incremental Delivery

1. Setup + Foundational → lint is runnable
2. US1 → config confirmed working (MVP)
3. US2 → clean baseline established
4. US3 → workflow verified end-to-end
5. Polish → docs updated, quickstart validated

---

## Notes

- T006 (`lint:fix` auto-fix pass) MUST be committed before T007–T011 manual fixes, so the diff is clean and reviewable
- T007–T011 are marked [P] but only after T006 completes — they target different file groups so can be done in any order or in parallel if multiple agents are running
- If `vue/multi-word-component-names` fires on any file in T008, that file must be renamed — update all `import` statements and `router/index.ts` references accordingly
- If the oxlint shim turns out to disable the `@typescript-eslint/semi` or `@typescript-eslint/quotes` rules, T005 resolves this before the fix pass begins
