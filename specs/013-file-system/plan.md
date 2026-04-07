# Implementation Plan: File System Architecture Alignment

**Branch**: `013-file-system` | **Date**: 2026-04-03 | **Spec**: `/home/arnedecant/Projects/nyxkit/nyx-notes/specs/013-file-system/spec.md`
**Input**: Feature specification from `/home/arnedecant/Projects/nyxkit/nyx-notes/specs/013-file-system/spec.md`

## Summary

Replace the current user/team-based filesystem contract with the new server-first layout defined in `docs/architecture/file-system.md`, then align the domain model, storage layer, API, CLI, frontend types/routes, and all affected docs to that structure. The implementation keeps stable internal IDs in metadata, uses immutable slugs for filesystem paths, uses slugs as the canonical MVP API identifiers, removes teams from the MVP, and introduces the MVP server-role model (`admin`, `user`) for shared server vault administration.

## Technical Context

**Language/Version**: Rust stable workspace, TypeScript 5, Vue 3 Composition API  
**Primary Dependencies**: `axum`, `tokio`, `serde`, `serde_yaml`, `serde_json`, `chrono`, `clap`, `dialoguer`, `comfy-table`, Vue Router, Pinia, `ofetch`, `nyx-kit`  
**Storage**: Local filesystem under `NOTES_ROOT` with Markdown notes, YAML frontmatter, JSON metadata sidecars, and JSON namespace metadata files  
**Testing**: `cargo test`, Rust integration tests on temp dirs, Vitest, Playwright  
**Target Platform**: Linux server binaries, CLI, browser frontend, and Tauri-backed local/native flows  
**Project Type**: Full-stack monorepo with Rust crates plus Vue SPA  
**Performance Goals**: Maintain human-scale self-hosted performance with flat file IO and no material regression from the current linear vault discovery approach; slug-based API identifiers should avoid mandatory metadata scans on the hot path  
**Constraints**: Docs-first change, no database, filesystem remains the content source of truth, permission enforcement stays in API layer, immutable slug rule `^[a-z0-9_-]{1,64}$`, single-server MVP, `local/` reserved but not runtime-active, no migration requirement for existing test data  
**Scale/Scope**: Single-server MVP, tens of vaults per install, hundreds to low-thousands of notes, future multi-server and local-sync support reserved in the layout but not implemented now

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Principle I. Docs Are the Source of Truth: PASS
  - This feature starts from `docs/architecture/file-system.md`, updates affected docs before implementation, and includes README/config alignment.
- Principle II. Strict Layer Boundaries: PASS
  - The plan updates `notes-core` first for domain/storage contracts, then `notes-storage-fs`, then API/auth/CLI/frontend adapters without moving permission logic into storage.
- Principle III. Filesystem Is the Content Source of Truth: PASS
  - The filesystem contract remains primary; only the directory model and namespace metadata change.
- Principle IV. Test Coverage Per Layer: PASS
  - Storage, server, CLI, and frontend test updates are part of the plan.
- Principle V. Security by Design: PASS
  - The plan includes slug/path validation, path traversal hardening in path resolution, and explicit role-resolution updates.
- Principle VI. Frontend Constraints: PASS
  - Frontend changes are limited to shared types, routes, and domain modules while preserving Composition API and `nyx-kit` requirements.

Post-Phase-1 re-check:

- PASS. The proposed design keeps docs authoritative, preserves layer boundaries, retains filesystem truth, requires layer-appropriate tests, and identifies path-resolution and auth-role changes as explicit security work rather than incidental refactors.

## Project Structure

### Documentation (this feature)

```text
specs/013-file-system/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── filesystem-metadata.md
│   └── http-api.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── notes-core/
│   └── src/
├── notes-storage-fs/
│   └── src/
├── notes-server-axum/
│   ├── src/
│   └── tests/
├── notes-auth/
│   └── src/
└── notes-cli/
    └── src/

app/
└── src/
    ├── auth/
    ├── comments/
    ├── notes/
    ├── servers/
    ├── shared/
    ├── teams/
    └── vaults/

docs/
├── architecture/
├── interface/
├── conventions/
└── testing/
```

**Structure Decision**: Keep the existing multi-crate Rust backend plus Vue SPA layout. This feature is cross-cutting and touches the domain contract (`notes-core`), storage implementation (`notes-storage-fs`), auth/server/CLI composition, frontend shared types and routing, and the architecture/interface/testing docs.

## Phase 0 Research Outcomes

- Stable internal IDs remain stored in metadata, but slugs are the canonical MVP API identifiers and path keys.
- MVP sharing removes teams and uses a server/home/server-vault model.
- `NOTES_USER_ID` remains the MVP identity and home lookup key to minimize configuration churn while the new home metadata model is introduced.
- Server roles are `admin` and `user`; server-vault administration is role-based while personal home vaults remain owner-scoped.
- Team routes are replaced by server-vault administration routes and flat vault discovery for the caller.
- `local/` stays in the filesystem contract but runtime support is deferred for this feature.

## Phase 1 Design Outputs

- `data-model.md` defines the new server, home, vault, and user-role model.
- `contracts/filesystem-metadata.md` defines on-disk namespace metadata.
- `contracts/http-api.md` defines the replacement API surface for the MVP.
- `quickstart.md` defines the implementation and validation sequence.

## Complexity Tracking

No constitution violations require exception handling for this plan.
