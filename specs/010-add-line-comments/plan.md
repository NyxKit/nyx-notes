# Implementation Plan: Line-Based Comments

**Branch**: `010-add-line-comments` | **Date**: 2026-03-28 | **Spec**: `/specs/010-add-line-comments/spec.md`
**Input**: Feature specification from `/specs/010-add-line-comments/spec.md`

## Summary

Introduce line-based discussion threads by anchoring each new comment to the exact selected text inside a rendered note line while showing the containing line as sidebar context. The plan binds the existing comment system to nyx-kit annotation primitives, extends the persisted comment model with structured anchor data, hides legacy comments without reliable anchors from the default line-discussion UI, and keeps comment history stored for possible recovery or later migration.

## Technical Context

**Language/Version**: Rust workspace backend + TypeScript 5.6 / Vue 3.5 frontend  
**Primary Dependencies**: Axum, serde/serde_json, chrono, uuid, Vue 3 Composition API, Pinia, ofetch, nyx-kit 2.0.6  
**Storage**: Filesystem-backed Markdown notes plus JSON comment sidecars (`<note>.comments.json`)  
**Testing**: Rust unit/integration tests, HTTP integration tests, Vitest composable/unit tests, Playwright E2E  
**Target Platform**: Self-hosted web app: Axum server with Vue SPA in modern desktop browsers  
**Project Type**: Full-stack web application with Rust API and Vue frontend  
**Performance Goals**: No additional performance target beyond maintaining normal note-loading and annotation interaction behavior for typical notes  
**Constraints**: Docs must be updated first; filesystem remains source of truth; permission checks stay in API handlers; no new component library; no new dependency without explicit justification; exact selected-text anchors must be preserved for new comments; legacy comments without reliable anchors are hidden from the default line-discussion UI but retained in storage  
**Scale/Scope**: Per-note discussion threads for current single-user/small-team usage, with dozens of annotations/replies per note and tens of vaults per installation

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Docs First**: PASS - implementation work begins with doc updates across `docs/interface/frontend.md`, `docs/architecture/core-domain.md`, `docs/architecture/backend-api.md`, and `docs/architecture/filesystem-storage.md`; `README.md` updated if user-facing behavior changes.
- **Strict Layer Boundaries**: PASS - anchor types live in `notes-core`; sidecar persistence lives in `notes-storage-fs`; permission enforcement remains in `notes-server-axum`; Vue only consumes API/composables.
- **Filesystem Source of Truth**: PASS - comments remain in `.comments.json` sidecars; shape changes are treated as a storage/documentation migration with backward-compatible reads and hidden legacy threads.
- **Test Coverage Per Layer**: PASS - plan includes Rust domain/storage/API tests plus Vitest and Playwright coverage for annotation creation, restore, detachment, hidden legacy behavior, and permission enforcement.
- **Security by Design**: PASS - no auth model change; API continues to gate comment actions by note permission; storage changes do not widen path access; no new dependency introduced.
- **Frontend Constraints**: PASS - Composition API only, nyx-kit editor/annotation primitives reused, domain barrel imports preserved, API access remains in composables.

## Project Structure

### Documentation (this feature)

```text
specs/010-add-line-comments/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── comments-api.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── notes-core/
│   └── src/
│       └── domain.rs
├── notes-storage-fs/
│   └── src/
│       ├── lib.rs
│       └── tests.rs
└── notes-server-axum/
    └── src/
        ├── routes/comments.rs
        └── types.rs

app/
└── src/
    ├── comments/
    │   ├── api/
    │   ├── components/
    │   └── composables/
    ├── notes/
    │   ├── components/
    │   └── views/
    └── shared/
        └── types/

docs/
├── architecture/
│   ├── backend-api.md
│   ├── core-domain.md
│   └── filesystem-storage.md
└── interface/
    └── frontend.md
```

**Structure Decision**: Use the existing full-stack Rust + Vue structure. The feature spans domain types, sidecar storage, comment routes, frontend shared types, the note editor integration, and comment composables/components. Shared annotation mapping logic should live under `app/src/comments/` or `app/src/shared/` only if reused by multiple domains.

## Phase 0 Research Outcomes

- Reuse nyx-kit 2.0.6 annotation primitives instead of building custom editor comment marks.
- Promote comment anchors from `quoted_text` to a richer persisted anchor object that mirrors nyx-kit selection data.
- Treat "line-based" comments as exact selected-text anchors within rendered lines, with the containing line used as sidebar context.
- Hide legacy comments without reliable line anchors from the default line-discussion UI while retaining them in sidecar storage.

## Phase 1 Design Outcomes

- Introduce a canonical `CommentAnchor` domain object with selected text, surrounding context, last-known range, and attachment state.
- Extend comment contracts so create/list/update payloads expose anchor data and enough context for annotation rendering and sidebar display.
- Add a frontend adapter between backend comment records and `NyxAnnotation[]` for the editor.
- Keep resolved/open discussion behavior and permission semantics unchanged while changing how threads are anchored and rendered.
- Separate new line-based threads from hidden legacy threads in frontend behavior without deleting historical data.

## Post-Design Constitution Check

- **Docs First**: PASS - design explicitly requires doc updates before code changes.
- **Strict Layer Boundaries**: PASS - no permission logic or editor-specific behavior leaks into storage.
- **Filesystem Source of Truth**: PASS - sidecar evolution remains backward-compatible and documented as a sidecar schema update.
- **Test Coverage Per Layer**: PASS - design artifacts specify coverage additions at every affected layer.
- **Security by Design**: PASS - no new dependency, path, or auth risk introduced by the design.
- **Frontend Constraints**: PASS - nyx-kit annotations are used directly; components continue to rely on composables and barrel exports.

## Complexity Tracking

No constitution violations requiring justification.
