# Implementation Plan: Multi-Instance Access Profiles

**Branch**: `011-add-server-profiles` | **Date**: 2026-03-30 | **Spec**: [/home/arnedecant/Projects/nyxkit/nyx-notes-core/specs/011-add-server-profiles/spec.md](/home/arnedecant/Projects/nyxkit/nyx-notes-core/specs/011-add-server-profiles/spec.md)
**Input**: Feature specification from `/specs/011-add-server-profiles/spec.md`

## Summary

Add a client-side workspace profile layer that lets one Nyx Notes client switch between a singleton local workspace and multiple remote server profiles, with each remote profile carrying its own URL, username/password sign-in flow, and isolated auth/session state. The implementation should preserve existing backend layer boundaries by treating profile management as a frontend/native-client concern, with only additive auth-discovery contract support if server identity or compatibility metadata is needed.

## Technical Context

**Language/Version**: Rust (workspace crates), TypeScript, Vue 3 Composition API  
**Primary Dependencies**: Axum server stack, Vue Router, ofetch/fetch, nyx-kit, Tauri shell for native mode  
**Storage**: Filesystem-backed notes remain unchanged; client profile metadata persists in client app configuration and per-profile secret storage  
**Testing**: `cargo test`, frontend Vitest composable tests, Playwright E2E, Axum HTTP integration tests  
**Target Platform**: Linux/macOS/Windows local desktop app, browser client for self-hosted server, NAS/self-hosted Docker deployment  
**Project Type**: Layered web application with native desktop shell  
**Performance Goals**: Profile switching should clear stale data immediately and re-enter a usable state within 2 seconds on a healthy local or LAN server; first-time remote profile setup should fit the spec target of under 2 minutes  
**Constraints**: Docs must be updated before implementation; filesystem remains the content source of truth; no permission logic in storage; remote profiles are username/password only in this feature; no unnecessary new dependencies; frontend must use domain folders, composables, and nyx-kit primitives  
**Scale/Scope**: One local profile plus multiple remote profiles per client installation; profile isolation must hold across at least three remote profiles in acceptance testing

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Docs are the source of truth**: PASS with prerequisite. Implementation must begin by updating `docs/architecture/authentication.md`, `docs/architecture/deployment-modes.md`, `docs/interface/frontend.md`, and `README.md` to describe multi-profile client behavior and the narrowed username/password scope for this feature.
- **Strict layer boundaries**: PASS. Profile persistence and switching stay in the frontend/native-client layer. Backend changes, if any, are limited to additive auth discovery metadata and existing auth endpoints.
- **Filesystem source of truth**: PASS. This feature does not alter note, vault, or comment storage layout.
- **Test coverage per layer**: PASS. Plan includes frontend unit/E2E coverage, server integration coverage for discovery/login contracts, and no unnecessary storage-layer changes.
- **Security by design**: PASS. Per-profile auth/session isolation, explicit failure classification, and no token sharing across profiles are required.
- **Frontend constraints**: PASS. Work will use Vue Composition API, composables for API access, nyx-kit primitives, and domain-based folders with barrel exports.

## Phase 0: Research

- Confirm that workspace profiles belong entirely to client configuration and not to the Rust domain/storage layers.
- Confirm uniqueness and identity rules for remote profiles and the singleton local profile.
- Confirm whether profile switching should fully reset active auth/data stores.
- Confirm the minimal HTTP contract needed for remote discovery and compatibility checks.

Research output is captured in `research.md`.

## Phase 1: Design & Contracts

- Define client-side entities for workspace profiles, remote connection profiles, and per-profile session state.
- Define validation and lifecycle rules for adding, editing, activating, invalidating, and removing profiles.
- Document the external contracts for auth-mode discovery and username/password login as used by the multi-profile client.
- Document a quickstart that exercises local setup, remote connection, same-server multi-account setup, and switch isolation.
- Update agent context after artifacts are written.

## Post-Design Constitution Check

- **Docs-first prerequisite still required**: PASS. Design keeps docs updates as the first implementation task.
- **Layer boundaries preserved**: PASS. No domain/storage responsibility drift introduced by the design artifacts.
- **Security posture preserved**: PASS. Contracts and data model keep tokens and per-profile state isolated.
- **Testing coverage preserved**: PASS. Design artifacts define concrete frontend and server validation points.

## Project Structure

### Documentation (this feature)

```text
specs/011-add-server-profiles/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── client-profile-behavior.md
│   └── server-auth-discovery.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── notes-core/
├── notes-storage-fs/
├── notes-server-axum/
└── notes-cli/

frontend/
└── src/
    ├── auth/
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

**Structure Decision**: This feature primarily affects `frontend/src/auth/` and `frontend/src/shared/` by introducing client-side profile selection and per-profile bootstrapping above the existing API client. `notes-server-axum` may receive additive discovery metadata or response-shape hardening, but `notes-core` and `notes-storage-fs` should remain unchanged unless docs updates reveal a broader architectural gap.

## Complexity Tracking

No constitution violations currently require justification.
