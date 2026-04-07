# Implementation Plan: User Management

**Branch**: `014-user-management` | **Date**: 2026-04-03 | **Spec**: [/home/arnedecant/Projects/nyxkit/nyx-notes/specs/014-user-management/spec.md](/home/arnedecant/Projects/nyxkit/nyx-notes/specs/014-user-management/spec.md)
**Input**: Feature specification from `/specs/014-user-management/spec.md`

## Summary

Add administrator-managed user accounts for the `secret_key` authentication mode, backed by a server-local embedded credential store, with all authentication and account administration remaining inside the Rust backend. The feature introduces authenticated user-management endpoints, a new `users` frontend domain with a sidebar entry and management page, and explicit safeguards around password hashing, duplicate usernames and emails, a 12-character minimum password policy requiring 3 of 4 character categories, self-management restrictions, and protected administrator deletion.

## Technical Context

**Language/Version**: Rust 2021 workspace crates, TypeScript 5.x, Vue 3 Composition API  
**Primary Dependencies**: Axum, notes-core/auth/storage crates, Argon2 password hashing, Vue Router, Pinia, ofetch, nyx-kit  
**Storage**: Filesystem for notes remains unchanged; user credentials move to a server-local embedded SQLite store with non-reversible password hashes  
**Testing**: `cargo test`, Axum HTTP integration tests, frontend Vitest unit tests, Playwright E2E, targeted auth-store integration tests  
**Target Platform**: Self-hosted Linux server/runtime plus Vue web client and Tauri-hosted desktop shell  
**Project Type**: Layered web application with Rust backend and Vue frontend  
**Performance Goals**: Users page loads current accounts in under 1 second for up to 500 managed users on typical self-hosted hardware; create, edit, and delete actions complete in one round-trip and surface results immediately  
**Constraints**: Docs must be updated before implementation; notes filesystem layout must not change; MVP user management applies only to `secret_key` mode; `local` mode is out of scope and `oidc` is future work; all user/auth access goes through backend only; permission checks stay in API layer; no plaintext or reversible password storage; every managed user requires a unique email address; passwords must be at least 12 characters and include at least 3 of 4 categories (lowercase, uppercase, digit, symbol); use nyx-kit primitives only; keep frontend code under a top-level `users` domain with barrel exports  
**Scale/Scope**: Single server-local user directory for one Nyx Notes instance in `secret_key` mode, supporting bootstrap admin plus at least hundreds of managed accounts without introducing external infrastructure

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Docs are the source of truth**: PASS with prerequisite. Implementation must begin by updating `docs/architecture/core-domain.md`, `docs/architecture/authentication.md`, `docs/architecture/backend-api.md`, `docs/interface/frontend.md`, `docs/testing/README.md`, and `README.md` because current docs still describe `.users.json` and do not define user-management routes or the `users` frontend domain.
- **Strict layer boundaries**: PASS. Plan keeps credential persistence in the auth backend crate, shared abstractions in `notes-core`, and permission enforcement for user administration in `notes-server-axum`.
- **Filesystem is the content source of truth**: PASS. Note and vault storage stay filesystem-backed; the embedded database is limited to auth/user metadata and does not replace content storage.
- **Test coverage per layer**: PASS. Plan adds auth-store integration coverage, server HTTP contract tests, frontend unit tests for the new users domain, and E2E coverage for admin flows.
- **Security by design**: PASS. Plan requires Argon2 hashing, backend-only credential access, protected admin deletion rules, and sanitized error mapping.
- **Frontend constraints**: PASS. Plan uses a new `app/src/users/` domain, composable-driven API access, nyx-kit primitives, and barrel-based imports.

## Phase 0: Research

- Confirm the embedded credential-store choice and reject weaker alternatives currently implied by `.users.json`.
- Confirm the backend abstraction shape needed for user administration without violating the current layer model.
- Confirm the HTTP contract and authorization rules for listing, creating, updating, and deleting users in `secret_key` mode only.
- Confirm how the frontend should integrate a new users domain into the existing authenticated shell, routing, and sidebar behavior.

Research output is captured in `research.md`.

## Phase 1: Design & Contracts

- Define server-side entities for user accounts, password credentials, and protected-account rules.
- Define validation and lifecycle rules for required unique emails, bootstrap admin creation, self-edit restrictions, password rotation, and deletion safeguards.
- Document the backend contract for `secret_key`-mode user-management endpoints and the frontend behavior contract for the users page and modal flows.
- Document a quickstart that exercises bootstrap state, list view, create, edit, duplicate validation, and protected deletion.
- Update agent context after artifacts are written.

## Post-Design Constitution Check

- **Docs-first prerequisite still required**: PASS. Design artifacts explicitly depend on docs updates before code changes.
- **Layer boundaries preserved**: PASS. Data model keeps admin authorization in the server layer and credential persistence in the auth store implementation.
- **Filesystem/content boundary preserved**: PASS. Contracts and data model do not move notes or vault metadata into the embedded auth store.
- **Security posture preserved**: PASS. Design uses non-reversible password hashes, required unique account identifiers, self-management safeguards, and backend-only credential handling.
- **Testing coverage preserved**: PASS. Quickstart, contracts, and data model define concrete server and frontend verification points.

## Project Structure

### Documentation (this feature)

```text
specs/014-user-management/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── users-api.md
│   └── users-ui.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── notes-core/
├── notes-auth/
├── notes-server-axum/
├── notes-storage-fs/
└── notes-cli/

app/
└── src/
    ├── auth/
    ├── users/
    ├── shared/
    ├── vaults/
    ├── notes/
    ├── comments/
    └── teams/

docs/
├── architecture/
├── interface/
├── conventions/
└── testing/
```

**Structure Decision**: This feature spans `notes-core` for shared auth/user abstractions, `notes-auth` for the embedded credential store implementation used by `secret_key` mode, `notes-server-axum` for admin-only user-management routes and auth integration, and `app/src/users/` plus `app/src/shared/` for the management UI and navigation wiring. `notes-storage-fs` remains untouched unless docs uncover a cross-layer mismatch.

## Complexity Tracking

No constitution violations currently require justification.
