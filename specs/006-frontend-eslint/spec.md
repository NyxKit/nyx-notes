# Feature Specification: Frontend ESLint Integration

**Feature Branch**: `006-frontend-eslint`
**Created**: 2026-03-24
**Status**: Complete
**Input**: User description: "integrate nyx-kit's eslint rules into this project's frontend and adjust code accordingly"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Lint runs and passes on the frontend codebase (Priority: P1)

A developer runs a single lint command from the frontend directory and gets a clear pass/fail result against the shared nyx-kit rule set. All existing source files conform to the rules, so the command exits cleanly with no violations.

**Why this priority**: Without a passing baseline, lint is noise rather than a gate. Every subsequent story depends on this working first.

**Independent Test**: Run `pnpm lint` in `frontend/` and observe zero errors and zero warnings.

**Acceptance Scenarios**:

1. **Given** the frontend has no ESLint config, **When** the integration is complete, **Then** `pnpm lint` executes successfully using the nyx-kit shared rule set.
2. **Given** a source file contains a rule violation (e.g. unused variable), **When** the linter runs, **Then** it reports the violation with file path and line number.
3. **Given** all source files are compliant, **When** the linter runs, **Then** it exits with code 0 and no output beyond a summary.

---

### User Story 2 — Existing code is adjusted to comply with the rule set (Priority: P2)

All existing `.ts` and `.vue` files in `frontend/src/` are updated to satisfy the rules the shared config introduces. No pre-existing violations remain after the adjustment pass.

**Why this priority**: A lint config with known suppressions or ignored violations is misleading. The baseline must be clean so future violations stand out.

**Independent Test**: `pnpm lint` passes with zero errors after code adjustments, without any ESLint suppression comments added.

**Acceptance Scenarios**:

1. **Given** a file contains a violation flagged by the nyx-kit rule set, **When** the adjustment pass is complete, **Then** the file is corrected without disabling the rule.
2. **Given** the codebase has style inconsistencies (semicolons, quote style), **When** lint runs, **Then** violations are resolved at the source rather than suppressed.
3. **Given** no violations remain, **When** a developer introduces a new violation in any file, **Then** lint fails on that file only.

---

### User Story 3 — Lint is integrated into the dev workflow (Priority: P3)

The lint command is part of the standard scripts in `frontend/package.json`, consistent with how the build and type-check commands are registered.

**Why this priority**: Discoverability matters. Developers should find lint the same way they find `dev`, `build`, and `type-check`.

**Independent Test**: Both `pnpm lint` and `pnpm lint:fix` are listed in `package.json` scripts and run without additional flags or setup.

**Acceptance Scenarios**:

1. **Given** a developer clones the repo and installs dependencies, **When** they run `pnpm lint`, **Then** linting executes without any additional configuration steps.
2. **Given** the project already has `dev`, `build`, and `type-check` scripts, **When** lint is added, **Then** it follows the same naming and invocation pattern as the existing scripts.
3. **Given** a developer wants to auto-fix mechanical violations, **When** they run `pnpm lint:fix`, **Then** ESLint applies all safe auto-fixes and reports any residual violations that require manual attention.

---

### Edge Cases

- What happens when a rule conflict exists between the nyx-kit config and a project-level override? Project-level overrides appended after the shared config take precedence — this is the standard flat-config composition model.
- How does the system handle files under `frontend/src/`? The lint script targets `./src` explicitly, so all committed source files — including `vite-env.d.ts` — are subject to the rule set. Only `dist/`, `dist-ssr/`, and `coverage/` are excluded via the nyx-kit config's built-in ignore patterns.
- What happens if a peer dependency required by the ESLint config is not yet installed? The install step must be completed before the config is loaded; missing peers produce a clear module-not-found error.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The frontend MUST have an ESLint flat config that extends the nyx-kit shared rule set.
- **FR-002**: All required peer dependencies for the shared ESLint config MUST be installed as dev dependencies.
- **FR-003**: A `lint` script MUST be added to `frontend/package.json` that runs ESLint over all TypeScript and Vue source files. A companion `lint:fix` script MUST also be provided that runs ESLint with auto-fix enabled, applying safe mechanical fixes and reporting residual violations.
- **FR-004**: All existing source files in `frontend/src/` MUST pass the rule set with zero errors and zero warnings.
- **FR-005**: No ESLint suppression directives (`eslint-disable`) MUST be introduced — every violation must be resolved at the source.
- **FR-006**: The ESLint config MUST allow project-level overrides to be appended after the shared config block, so future rules can be added without modifying the shared config.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `pnpm lint` completes with zero errors and zero warnings across all source files in `frontend/src/`.
- **SC-002**: Zero suppression directives are introduced anywhere in the codebase as part of this feature.
- **SC-003**: Any new `.ts` or `.vue` file added after this feature ships that violates a rule causes `pnpm lint` to fail, confirming the gate is active.
- **SC-004**: The lint command completes in under 30 seconds on the current source tree.

## Clarifications

### Session 2026-03-24

- Q: Should an auto-fix convenience script exist alongside the base `lint` command? → A: Yes — add `lint:fix` alongside `lint`; auto-fix handles mechanical violations first, residuals fixed manually.
- Q: Should lint gate the build (`pnpm build` fails if lint fails)? → A: No — lint is a separate script; `build` is not modified; enforcement is a CI concern.

## Assumptions

- The nyx-kit shared ESLint config is the canonical rule set for all NyxKit projects; no project-specific deviation is needed beyond what can be appended as overrides.
- `lint` and `lint:fix` are independent scripts; `build` is not modified to include a lint step — lint enforcement in CI is out of scope for this feature.
- `oxlint` is not being run as a standalone tool in this project currently; the oxlint compatibility shim in the shared config is sufficient.
- TypeScript strict mode is already in use per the existing `tsconfig.json`, so TypeScript-recommended ESLint rules will not surface unexpected new errors beyond style and correctness issues already in the code.
- Code adjustments are limited to lint compliance only — no logic, behaviour, or component API changes are in scope.
