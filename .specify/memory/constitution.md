<!--
SYNC IMPACT REPORT
==================
Version change: 1.0.1 → 1.0.2 (PATCH — added barrel-export rule to Principle VI)
Modified principles: VI. Frontend Constraints (added one bullet on index-based subdirectory exports/imports)
Added sections: N/A
Removed sections: N/A
Templates requiring updates:
  - .specify/templates/plan-template.md ✅ — no outdated references found; constitution-aligned
  - .specify/templates/spec-template.md ✅ — no misaligned constraints
  - .specify/templates/tasks-template.md ✅ — task categories (testing, security, layer) consistent
  - docs/interface/frontend.md ✅ — Application Layout updated with barrel export/import rules
  - docs/conventions/README.md ✅ — Frontend Directory Structure updated with barrel export/import rules
Follow-up TODOs: none
-->

# Nyx Notes Core Constitution

## Core Principles

### I. Docs Are the Source of Truth

The `docs/` folder is the authoritative specification for every feature, type, and design decision.
Implementations MUST match the docs. Code that diverges from `docs/` MUST be flagged before any
attempt to reconcile it.

- Update `docs/` first; implement second.
- Cross-doc consistency MUST be validated on every change: a type change in `core-domain.md`
  cascades to `filesystem-storage.md`, `backend-api.md`, and `interface/frontend.md`.
- `README.md` is a human-facing summary and MUST be kept in sync with `docs/`.
- Docs-only changes MUST use targeted edits; full rewrites are not permitted without an explicit
  request.

### II. Strict Layer Boundaries

The backend is a layered system. Each layer MUST depend only on the layer below it.

```
notes-server-axum   →  HTTP API, auth middleware, permission enforcement
notes-storage-fs    →  Filesystem implementation of StorageBackend
notes-core          →  Domain types and traits — zero IO, zero runtime dependencies
```

- Permission checks MUST live in the API layer (`notes-server-axum`). `FsStorage` MUST NOT
  enforce permissions.
- Domain types belong in `notes-core`, not in `notes-server-axum`.
- `Arc<dyn Trait>` MUST be used over generic parameters on `AppState`.
- The native Tauri app embeds the Axum server; no separate IPC contract is permitted.

### III. Filesystem Is the Content Source of Truth

Notes are plain `.md` files with YAML frontmatter. There is no database.

- The on-disk directory layout and frontmatter format are defined in
  `docs/architecture/filesystem-storage.md`. Changes to either are BREAKING and MUST be treated as
  migrations.
- Unknown frontmatter keys MUST be preserved on round-trip.
- Frontmatter dates MUST be ISO 8601 UTC strings; permission values MUST be lowercase strings.

### IV. Test Coverage Per Layer (NON-NEGOTIABLE)

Every layer has a mandatory testing approach, defined in `docs/testing/README.md`.

- `notes-core`: plain `#[test]`, no IO, no filesystem.
- `notes-storage-fs`: `#[tokio::test]` with a real `FsStorage` on a `tempdir`.
- `notes-server-axum`: HTTP integration tests using `axum::test` or `reqwest` with a stub
  `AuthStore`.
- Frontend: Vitest for composables (mocked fetch); Playwright for E2E.
- CLI: subprocess invocation tests against a temp `NOTES_ROOT`.
- The OIDC provider MUST be mocked; `serde_yaml` MUST be trusted, not retested.

### V. Security by Design

The backend handles private notes on personal servers. Security MUST be treated as a first-class
concern, not a post-hoc addition.

- Auth extractors MUST return the correct user in every auth mode, including `local`.
- Path traversal risks in vault/note file resolution MUST be explicitly mitigated.
- `AppError` MUST NOT leak internal file paths or stack traces to HTTP clients.
- `StorageError` variants MUST NOT silently swallow detail needed for debugging.
- No new dependency (Rust crate or npm package) may be introduced without explicit justification.

### VI. Frontend Constraints

The Vue 3 frontend MUST follow the Composition API exclusively.

- All API calls MUST go through composables; components MUST NOT call `fetch` directly.
- `nyx-kit` MUST be used for all UI primitives; no additional component libraries are permitted.
- TypeScript types MUST match the Rust types (field names, nullability, optional fields).
- No semicolons in frontend TypeScript/Vue files; single quotes for strings.
- `frontend/src/` MUST use a domain-based layout (`vaults/`, `notes/`, `comments/`, `auth/`,
  `teams/`, `shared/`). A file belongs in its domain folder if used by one domain; in `shared/`
  if used by two or more. Direct imports between two domain folders are NOT permitted — extract
  shared code to `shared/` instead.
- Every importable frontend subdirectory MUST expose an `index.ts` barrel file that re-exports
  every module in that subdirectory. Imports MUST target the directory barrel (for example,
  `@/notes/components`) rather than reaching into a specific module file.

## Development Workflow

- **Plan before editing.** For any non-trivial change, identify all affected files before touching
  any of them. State the plan; confirm if scope is larger than expected.
- **Minimize diff size.** The smallest change that achieves the goal. A targeted edit is preferred
  over rewriting a section.
- **No broad rewrites** unless that is the explicit request. Reformatting beyond the stated task
  introduces noise.
- **Flag divergences.** If code or docs conflict, stop and surface the conflict; do not silently
  reconcile in favour of either.

## Code Conventions

Conventions are defined in `docs/conventions/README.md`. Key rules:

- Rust: `snake_case` identifiers, `PascalCase` types/traits/enums; `thiserror` for error types;
  no `unwrap()`/`expect()` in library code.
- Vue: `PascalCase.vue` components, `useX.ts` composables; Composition API only.
- Docs: `kebab-case.md` filenames.
- Git: imperative mood commit messages; `feat/`, `fix/`, `chore/`, `docs/` branch prefixes;
  one logical change per commit.

## Governance

- This constitution supersedes all other implicit practices.
- Amendments require: a clear rationale, a version bump (semantic), and propagation across all
  affected `docs/` and template files.
- All implementation work MUST verify compliance with this constitution before a change is
  considered complete.
- The checklist in `AGENTS.md` (§ What Must Be Validated) is the compliance gate.

**Version**: 1.0.2 | **Ratified**: 2026-03-21 | **Last Amended**: 2026-03-25
