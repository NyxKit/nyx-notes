# AGENTS.md — Working Guide for AI Agents

## Commits and Pushes

**Never auto-commit or auto-push.** Commits and pushes will be explicitly prompted by the user. Wait for explicit instruction before committing or pushing changes.

---

## Testing and Development

**Always clear ports after internal testing.** When testing the server or frontend locally, kill the processes on ports 4200 (backend) and 1420 (frontend) before returning control to the user:

```bash
fuser -k 4200/tcp 2>/dev/null
fuser -k 1420/tcp 2>/dev/null
```

---

## The Docs Are the Source of Truth

The `docs/` folder is the **authoritative specification** for this project. It is a living document.

- If the user asks for a feature change, a design decision, or a new capability: **update `docs/` first**, then implement.
- If code diverges from `docs/`, **flag it to the user before proceeding** — do not silently reconcile in favour of the code.
- If `docs/` diverges from itself (e.g. a type in `core-domain.md` doesn't match what `backend-api.md` assumes), flag it.
- `README.md` is a human-facing summary; keep it in sync with `docs/` when making changes, but `docs/` wins on conflicts.

---

## nyx-kit Components

**Always use nyx-kit base components** when a suitable one exists. Do not build custom equivalents.

Available components include (but are not limited to): `NyxButton`, `NyxInput`, `NyxSelect`, `NyxModal`, `NyxEditor`, `NyxCard`. Check `../nyx-kit/src/components/` for the full list.

If a needed UI primitive is **not** available in nyx-kit, flag it to the user before building a custom one.

---

## Design

The visual design of this application is governed by `DESIGN.md` at the repo root. Read it before making any frontend or UI changes.

`DESIGN.md` defines the design system: the "Silent Atelier" creative direction, color palette and surface hierarchy, typography (Manrope/Inter for UI, Newsreader for the writing canvas), elevation via tonal layering rather than shadows, and component-level guidelines for the editor, sidebars, tags, vault switcher, and comments.

Reference mockups and screen designs live in `design/`, organised by screen:

| Folder | Screen |
|---|---|
| `design/focused_writing_view/` | Default state — no sidebars, full-width writing canvas |
| `design/left_sidebar_open/` | Left sidebar (vault switcher + note list) open |
| `design/both_sidebars_open/` | Both sidebars open — notes left, comments right |
| `design/vault_overview/` | Vault overview screen |
| `design/empty_state/` | No notes yet — empty state |

Each folder contains `screen.png` (the visual mockup) and `code.html` (a static HTML prototype).

**When implementing frontend UI:**
- Follow `DESIGN.md` strictly. It overrides any pre-existing styles or component defaults.
- Refer to the `screen.png` files as the visual target for layout, spacing, and hierarchy.
- The `code.html` files are useful references for structure and class patterns but are not authoritative over `DESIGN.md`.

---

## What This Repo Is

**Nyx Notes** is a self-hosted, Markdown-first notes application. The Rust backend (domain, storage, HTTP API, CLI) is implemented. The Vue frontend is in progress.

The stack:

| Layer | Technology | Location | Status |
|---|---|---|---|
| Domain logic | Rust (`notes-core`) | `crates/notes-core/` | ✅ built |
| Storage | Rust (`notes-storage-fs`) | `crates/notes-storage-fs/` | ✅ built |
| HTTP API | Rust/Axum (`notes-server-axum`) | `crates/notes-server-axum/` | ✅ built |
| CLI | Rust (`notes-cli`) | `crates/notes-cli/` | ✅ built |
| Frontend | Vue 3 + nyx-kit | `app/src/` | 🚧 in progress |
| Native app | Tauri (embeds Axum server) | `app/native/` | 🔜 planned |
| Auth | Pluggable (`local`, `secret_key`, `oidc`) | — | 🔜 planned |
| Content storage | Plain `.md` files on disk | `$NOTES_ROOT/` | ✅ built |

The filesystem is the source of truth for content. Auth is pluggable via `AUTH_MODE`. The backend is stateless with respect to auth.

---

## How to Navigate This Repo

```
nyx-notes/
  README.md           # high-level summary (keep in sync with docs/)
  AGENTS.md           # this file
  CLAUDE.md           # Claude Code behaviour settings
  Cargo.toml          # workspace manifest (not yet written)
  crates/             # Rust crates (not yet implemented)
  app/           # Vue 3 SPA (not yet implemented)
  docs/
    architecture/     # system design, backend layers, data model
      README.md       # layer diagram + key design decisions
      core-domain.md  # canonical types, StorageBackend/AuthStore traits
      filesystem-storage.md  # directory layout, frontmatter format, FsStorage impl
      backend-api.md  # Axum routes, permission enforcement, error mapping
      authentication.md  # auth modes (local, secret_key, oidc), JWKS caching
      vaults-and-teams.md  # vault/team model, roles, permission matrices
    interface/        # user-facing surfaces
      README.md
      frontend.md     # Vue SPA: components, views, routing, editor, comments
      cli.md          # CLI: commands, vault context, configuration
    conventions/
      README.md       # Rust, frontmatter, file naming, Vue/TS, git conventions
    testing/
      README.md       # test strategy per layer (unit / integration / E2E)
```

**Start here for context:**
1. `README.md` — one-page overview
2. `docs/architecture/README.md` — layer diagram and key decisions
3. `docs/architecture/vaults-and-teams.md` — the central data model (vaults, teams, permissions)

---

## How to Change This Repo Safely

### For design or feature changes

1. **Read the relevant doc(s)** in `docs/` before proposing anything.
2. **Update `docs/` first** to reflect the agreed design.
3. **Check for cross-doc impact**: a change to a type in `core-domain.md` likely touches `filesystem-storage.md`, `backend-api.md`, and `interface/frontend.md`. Update all affected docs in the same pass.
4. Then implement in code, following `docs/conventions/README.md`.

### For code changes (once implementation begins)

1. Verify the change is within scope of what `docs/` specifies.
2. If implementation reveals a design gap or conflict in `docs/`, stop and flag it rather than making an undocumented decision.
3. Match the layer boundaries: permission checks belong in the API layer, not in `FsStorage`; domain types belong in `notes-core`, not in `notes-server-axum`.

### For docs-only changes

- Prefer targeted edits (`Edit` tool) over full rewrites.
- When renaming a concept, find and update every occurrence across all docs files.
- Keep the four `README.md` index files accurate — they are entry points, not copies of the content below them.

---

## What Must Be Validated

Before considering any change complete, verify:

- [ ] All affected `docs/` files are updated and internally consistent
- [ ] Cross-references between docs use correct relative paths (`./` for same folder, `../folder/` for cross-folder)
- [ ] `README.md` reflects any structural or API changes
- [ ] Types used in `backend-api.md` match what is defined in `core-domain.md`
- [ ] Filesystem paths in `filesystem-storage.md` match the layout in `vaults-and-teams.md`
- [ ] Any new dependency (Rust crate or npm package) is justified and noted in the relevant doc

---

## What Not to Touch Casually

| Thing | Why |
|---|---|
| `docs/architecture/core-domain.md` — the trait signatures | Everything depends on `StorageBackend` and `AuthStore`. Changes here cascade to storage, server, CLI, and tests. |
| `docs/architecture/vaults-and-teams.md` — the permission matrix | The matrix governs API enforcement, frontend read-only states, and CLI access. Changing a cell silently breaks multiple layers. |
| `docs/architecture/filesystem-storage.md` — the directory layout and frontmatter format | Changing the on-disk structure is a **migration** — existing notes files would break. Treat this as a breaking change. |
| `docs/architecture/authentication.md` — the auth mode interface | `GET /api/auth/mode` response shape and the four mode names are depended on by the frontend. Changing mode names or the response schema is a breaking change. |
| `docs/conventions/README.md` | Conventions are only useful if they're stable. Don't adjust them without a clear reason. |

---

## Audits

When asked to perform an audit, create a new file in `docs/audits/` named `YYYYMMDD-HHMM.md` using the current date and time (e.g. `20260315-1944.md`). Do not reuse or overwrite existing audit files.

### Audit standard

Every audit of nyx-notes must be performed as a **senior-level codebase review** — not a shallow summary. Treat this as production infrastructure: the backend runs on personal servers, stores private notes, and must be correct, secure, and maintainable.

**Role:** Act as a principal engineer / systems architect. Be highly critical, practical, and specific. Reference actual files, crates, modules, and patterns. Do not praise unnecessarily.

**Inspect all of the following where relevant:**
domain model design, trait signatures, crate boundaries and layering, `StorageBackend` / `AuthStore` implementations, filesystem layout correctness, frontmatter parsing robustness, Axum route structure and handler correctness, permission enforcement, auth flow and token handling, error mapping (`StorageError` → `AppError`), `AsyncStorageAdapter` correctness, CLI ergonomics and config handling, Vue composable design, API client patterns, route guards and auth flows, TypeScript type coverage, component API consistency, frontend state management, environment variable handling, build setup (Cargo workspace + Vite), dead code and duplication, docs coverage, test coverage, security posture (injection risks, auth bypasses, path traversal, data leakage).

**Evaluate whether the project is:**
1. Correct — does the implementation match what the docs specify?
2. Secure — are there auth bypasses, path traversal risks, or data leakage vectors?
3. Consistent — do the CLI, server, and frontend agree on user IDs, vault slugs, and data formats?
4. Maintainable — are crate boundaries clean, is logic in the right layer?
5. Resilient — are errors handled correctly at every layer boundary?
6. Developer-friendly — can a new contributor understand and run the project quickly?

**Explicitly check for:**
- Permission checks missing or in the wrong layer (should be in API handlers, not `FsStorage`)
- `NOTES_USER_ID` / `NOTES_ROOT` mismatches between CLI and server
- Path traversal risks in `find_vault_path` or note file resolution
- `local_user()` / hardcoded user IDs that ignore env config
- Auth extractor returning wrong user in `local` mode
- Frontmatter parsing edge cases (missing fields, malformed YAML)
- `StorageError` variants that silently swallow detail
- `AppError` variants that leak internal paths or stack info to the client
- Axum routes that return 200 where they should return 201 / 204
- `AsyncStorageAdapter` methods that are missing (coverage gaps vs. `StorageBackend` trait)
- Vue composables that share module-level state incorrectly across vault contexts
- Pinia store state destructured without `storeToRefs` (silently breaks reactivity — component never re-renders on state change)
- Frontend API calls that don't handle non-2xx responses
- `ofetch` error handling gaps in `api/client.ts`
- TypeScript types that diverge from the Rust types (field names, nullable vs. optional)
- Frontend routes that load stale vault/note state after navigation
- Missing `NOTES_USER_ID` export in `.env.example` or README mismatches
- Dead code, unreachable branches (e.g. `oidc` placeholder panic)
- Docs that describe a different interface than what is implemented
- Test coverage gaps at each layer (unit, integration, E2E)

**Output format — every audit must follow this structure:**

```
# Nyx Notes Core Audit — YYYYMMDD-HHMM

## 1. Executive summary
## 2. Severity overview (Critical / High / Medium / Low)
   Each finding: Title, Severity, Why it matters, Evidence (file:line), Recommended fix, Breaking or not
## 3. Backend architectural assessment (crate boundaries, trait design, layering)
## 4. Storage and filesystem assessment (FsStorage correctness, path handling, frontmatter)
## 5. API and auth assessment (routes, permission enforcement, auth extractor, error mapping)
## 6. CLI assessment (commands, config, user ID handling)
## 7. Frontend assessment (composables, API client, routing, TypeScript types)
## 8. Security assessment (auth bypass, path traversal, data leakage, input validation)
## 9. Testing assessment (coverage gaps, what is tested vs. assumed)
## 10. Docs assessment (accuracy, completeness, divergence from implementation)
## 11. Top 10 improvements (ranked by impact, with effort, risk, breaking flag)
## 12. Refactor roadmap (Phase 1: quick wins / Phase 2: structural / Phase 3: breaking)
## 13. Scorecard (1–10): Architecture, Security, API correctness, CLI ergonomics,
        Frontend quality, Type safety, Error handling, Test coverage, Docs accuracy, Operability
```

**Style:** Write like an expert reviewer addressing a senior engineer. Concise, structured, sharp, and practical. Distinguish between objectively risky issues, stylistic preferences, and possible future improvements. When uncertain, say so explicitly.

---

## Divergence Log

When you notice that something in the codebase or docs is out of sync, record it here before fixing it.

| Noticed | Location | Description | Status |
|---|---|---|---|
| 2026-04-03 | docs/architecture/*, README.md, crates/*, app/src/* | The new storage source of truth in `docs/architecture/file-system.md` diverged from the active docs and implementation, which still assumed `users/<uid>/` and `teams/<team-id>/` layout plus team-based MVP sharing. | In progress |
