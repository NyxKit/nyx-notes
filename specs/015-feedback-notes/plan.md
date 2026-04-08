# Implementation Plan: Feedback Center

**Branch**: `[015-feedback-notes]` | **Date**: 2026-04-08 | **Spec**: [`spec.md`](./spec.md)
**Input**: Feature specification from `/specs/015-feedback-notes/spec.md`

## Summary

Add a bottom-left Feedback entry point that opens a large modal for regular users and a vault-like masonry overview for admins. Reuse the note editing experience from the notes domain, treat feedback as an elevated note with extra metadata, and store it in a separate top-level server namespace with submission metadata for type, location, console output, and future interaction history.

## Technical Context

**Language/Version**: Rust backend, Vue 3 + TypeScript frontend  
**Primary Dependencies**: Axum, `nyx-kit`, Vue Router, Vite, existing notes/core storage stack  
**Storage**: Filesystem-backed notes plus new top-level `feedback/` namespace with item folders and image subfolders  
**Testing**: Rust unit/integration tests, frontend unit tests, Playwright E2E  
**Target Platform**: Self-hosted web app with optional embedded desktop shell  
**Project Type**: Web application with Rust HTTP API and Vue SPA  
**Performance Goals**: Keep modal open/submit flows responsive and image handling transparent to the user  
**Constraints**: Preserve docs-first layering, keep permission checks in the API layer, and use nyx-kit primitives  
**Scale/Scope**: Single-user or small-team self-hosted deployments with note/feedback attachments

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Docs remain the source of truth; this feature requires doc updates before implementation because it adds a new top-level filesystem namespace.
- Layering stays intact if feedback handling lives in the API and UI layers, with storage remaining permission-agnostic.
- Storage changes are intentionally breaking because note items gain attached images and a folder-per-item layout.
- Testing must cover core domain, filesystem, API, and frontend flows for the new feedback path.
- No new dependencies are justified in this plan.

## Project Structure

### Documentation (this feature)

```text
specs/015-feedback-notes/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── feedback-api.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── notes-core/
├── notes-storage-fs/
├── notes-server-axum/
└── notes-cli/

app/
└── src/
    ├── auth/
    ├── comments/
    ├── feedback/
    ├── notes/
    ├── shared/
    └── vaults/
```

**Structure Decision**: Extend the existing Rust backend layers and keep the reusable note editor shell in the notes domain while adding a separate `feedback` domain that builds on note types, note-derived metadata, and view components.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Folder-per-item storage for notes/feedback | Image attachments need stable per-item asset paths | Flat `.md` files cannot hold item-local image directories cleanly |
| New top-level `feedback/` namespace | Feedback must be separate from homes and vaults | Reusing vaults would blur permissions and admin review workflows |
