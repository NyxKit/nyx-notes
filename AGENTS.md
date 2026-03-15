# AGENTS.md — Working Guide for AI Agents

## The Docs Are the Source of Truth

The `docs/` folder is the **authoritative specification** for this project. It is a living document.

- If the user asks for a feature change, a design decision, or a new capability: **update `docs/` first**, then implement.
- If code diverges from `docs/`, **flag it to the user before proceeding** — do not silently reconcile in favour of the code.
- If `docs/` diverges from itself (e.g. a type in `core-domain.md` doesn't match what `backend-api.md` assumes), flag it.
- `README.md` is a human-facing summary; keep it in sync with `docs/` when making changes, but `docs/` wins on conflicts.

---

## What This Repo Is

**Nyx Notes** is a self-hosted, Markdown-first notes application. It is not yet implemented — this repository currently holds architecture, design specs, and workspace scaffolding only.

The stack:

| Layer | Technology | Location |
|---|---|---|
| Domain logic | Rust (`notes-core`) | `crates/notes-core/` |
| Storage | Rust (`notes-storage-fs`) | `crates/notes-storage-fs/` |
| HTTP API | Rust/Axum (`notes-server-axum`) | `crates/notes-server-axum/` |
| CLI | Rust (`notes-cli`) | `crates/notes-cli/` |
| Frontend | Vue 3 + nyx-kit + TipTap | `frontend/` |
| Auth | Pluggable (`local`, `secret_key`, `oidc`) | — |
| Content storage | Plain `.md` files on disk | `$NOTES_ROOT/` |

The filesystem is the source of truth for content. Auth is pluggable via `AUTH_MODE`. The backend is stateless with respect to auth.

---

## How to Navigate This Repo

```
nyx-notes-core/
  README.md           # high-level summary (keep in sync with docs/)
  AGENTS.md           # this file
  CLAUDE.md           # Claude Code behaviour settings
  Cargo.toml          # workspace manifest (not yet written)
  crates/             # Rust crates (not yet implemented)
  frontend/           # Vue 3 SPA (not yet implemented)
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

## Divergence Log

When you notice that something in the codebase or docs is out of sync, record it here before fixing it.

| Noticed | Location | Description | Status |
|---|---|---|---|
| — | — | No known divergences. | — |
