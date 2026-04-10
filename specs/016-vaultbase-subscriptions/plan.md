# Implementation Plan: Centralized Subscription Data Flow

**Branch**: `016-vaultbase-subscriptions` | **Date**: 2026-04-09 | **Spec**: [/home/arnedecant/Projects/nyxkit/nyx-notes/specs/016-vaultbase-subscriptions/spec.md](./spec.md)
**Input**: Feature specification from `/specs/016-vaultbase-subscriptions/spec.md`

## Summary

Introduce a centralized `NyxBase` data-access layer beneath domain stores for live vault and note data, backed by a shared subscription manager that deduplicates equivalent queries across composables while keeping Pinia stores as the single read surface. The backend will expose typed live subscription scopes over server-sent events, with the existing write routes publishing scope updates into a broker so store-managed subscriptions can keep mounted consumers current without redundant fetch loops.

## Technical Context

**Language/Version**: TypeScript (Vue 3 SPA) and Rust (workspace crates on stable toolchain)  
**Primary Dependencies**: Vue 3 Composition API, Pinia, Vite, `ofetch`, Axum, Tokio, existing workspace auth/storage crates  
**Storage**: Filesystem-backed Markdown notes and JSON metadata under `NYX_ROOT`; in-memory subscription registries/brokers for active live queries  
**Testing**: Vitest for frontend units, `cargo test` for Rust unit/integration tests, Axum HTTP integration tests for live subscription endpoints  
**Target Platform**: Linux-hosted Axum server and Vue 3 web frontend  
**Project Type**: Web application with frontend + backend workspace  
**Performance Goals**: One underlying active subscription per canonical query key, immediate replay of latest snapshot to new consumers, release unused subscriptions within 2 seconds  
**Constraints**: Preserve Pinia stores as the only frontend read surface; keep `NyxBase` beneath stores; enforce auth in the API layer; avoid redundant backend scans for duplicate subscribers; preserve last known state during transient disconnects; no new dependencies unless justified  
**Scale/Scope**: Initial rollout limited to vault-list personal/shared scopes plus note-list and note document scopes, with extension points for broader user/server scopes, comments, and additional domains later

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Docs Are the Source of Truth**: PASS. Feature planning is captured under `specs/016-vaultbase-subscriptions/`; implementation will require coordinated doc updates in `docs/interface/frontend.md`, `docs/architecture/backend-api.md`, `docs/architecture/core-domain.md`, and `README.md` before code changes.
- **Strict Layer Boundaries**: PASS. Planned responsibilities keep permission/auth checks in `notes-server-axum`, transport/broker logic in API-layer/server support code, and storage concerns in `notes-storage-fs` only where existing write/read behavior is already defined.
- **Filesystem Is the Content Source of Truth**: PASS. Live subscriptions broadcast derived snapshots from filesystem-backed content; no alternate persistence source is introduced.
- **Test Coverage Per Layer**: PASS. Plan includes frontend unit tests for manager/composable lifecycle behavior, storage-safe backend integration tests for live subscription endpoints, and no reduction in existing layer-specific coverage.
- **Security by Design**: PASS. Subscription scopes will be typed and authorized before attachment, and long-lived delivery will use the same auth boundary as existing routes.
- **Frontend Constraints**: PASS. Components will continue reading from Pinia stores, stores own frontend API access through `NyxBase`, composables own subscribe/unsubscribe lifecycle, and no direct component fetch usage is introduced.

## Project Structure

### Documentation (this feature)

```text
specs/016-vaultbase-subscriptions/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── live-subscriptions.md
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
│   │   ├── routes/
│   │   ├── storage_adapter.rs
│   │   └── types.rs
│   └── tests/
└── notes-auth/
    └── src/

app/
└── src/
    ├── shared/
    │   ├── api/
    │   ├── composables/
    │   ├── router/
    │   ├── types/
    │   └── utils/
    ├── vaults/
    │   ├── composables/
    │   ├── stores/
    │   └── views/
    ├── notes/
    │   ├── composables/
    │   ├── stores/
    │   └── views/
    └── comments/
        ├── composables/
        └── stores/
```

**Structure Decision**: Use the existing web-application split. Add the subscription manager and `NyxBase` entry points under `app/src/shared/`, adapt vault/note domain stores to own that layer, adapt composables to orchestrate lifecycle around those stores, and add typed live subscription routes plus broker support inside `crates/notes-server-axum`.

## Phase 0: Research Summary

- Use a single app-level subscription manager keyed by canonical query identities.
- Keep Pinia stores as the only read surface; stores own `NyxBase` access; composables only acquire/release subscriptions through store methods.
- Use SSE as the initial live delivery channel, with typed scope requests and snapshot-style payloads.
- Use ref-counting and generation fencing to prevent duplicate subscriptions and stale updates.
- Publish updates from write routes into a broker and reserve filesystem watching for out-of-band change handling.

## Phase 1: Design Plan

1. Define canonical subscription entities, query identities, and lifecycle states.
2. Define the `NyxBase` contract as the single frontend entry point for list/document subscription acquisition.
3. Define live backend scope contracts and event envelopes for collection/document snapshots.
4. Define quickstart validation steps for shared subscriptions, lifecycle release, and route-level live updates.
5. Update agent context after artifacts are generated.

## Post-Design Constitution Check

- **Docs Are the Source of Truth**: PASS. Design artifacts identify the concrete docs that must be updated before implementation, including `docs/architecture/core-domain.md` and `README.md`.
- **Strict Layer Boundaries**: PASS. `NyxBase` stays in frontend shared infrastructure; server-side live delivery remains in the API layer; no permission logic is pushed into storage.
- **Filesystem Is the Content Source of Truth**: PASS. Broker snapshots derive from filesystem-backed reads and route-triggered invalidations, not a secondary datastore.
- **Test Coverage Per Layer**: PASS. Design includes frontend subscription lifecycle tests and Axum integration tests for live routes.
- **Security by Design**: PASS. Typed scopes plus API-layer authorization are preserved for long-lived delivery.
- **Frontend Constraints**: PASS. Stores own frontend API access, composables manage lifecycle, Pinia remains the single read surface, and cross-domain reuse stays under `shared/`.
