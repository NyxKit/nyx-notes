# CLAUDE.md

Read `AGENTS.md` before doing anything else.

---

## Edit Style

**Plan before editing.**
For any non-trivial change, identify all affected files before touching any of them. State the plan and confirm if the scope is larger than expected.

**Minimize diff size.**
Make the smallest change that achieves the goal. A targeted `Edit` on three lines is better than rewriting a section. Prefer surgical edits over reconstructions.

**Avoid broad rewrites.**
Do not rewrite a file to clean it up unless that is the explicit request. Reformatting, restructuring, or "improving" content beyond the stated task introduces noise and risks losing intent.

**Ask before introducing dependencies.**
Do not add a new Rust crate, npm package, or external service without flagging it first. State what it is, why it is needed, and whether there is a lighter alternative already in scope.

## Active Technologies
- TypeScript 5.x / Vue 3.5 / Vite + Vue Router 5.0, Pinia 3.0, nyx-kit 2.0.2, ofetch 1.5 (001-route-domains)
- Filesystem (Rust backend) — no frontend storage changes (001-route-domains)
- TypeScript 5.x / Vue 3.5 / Vite + nyx-kit 2.0.2 (`NyxButton`, `NyxInput`, `NyxTextarea`, `NyxBadge`, `NyxForm`, `NyxFormField`, `NyxTabs` — all already installed) (002-nyx-kit-primitives)
- N/A — UI-only refactor (002-nyx-kit-primitives)
- TypeScript 5.x / Vue 3.5 / Vite + nyx-kit 2.0.2 (NyxButton, NyxBadge), ofetch 1.5, Vue Router 5.0, Pinia 3.0 (003-sidebar-recent-notes)
- N/A — frontend-only; uses existing REST API endpoints (003-sidebar-recent-notes)
- TypeScript 5.x / Vue 3.5 / Vite + Pinia 3.0 (already installed), nyx-kit 2.0.2, Vue Router 5.0 (004-pinia-stores)
- N/A — frontend-only; existing REST API endpoints unchanged (004-pinia-stores)
- TypeScript 5.x / Vue 3.5 (frontend); Rust (backend — stable toolchain) + Pinia 3.0, Vue Router 5.0, nyx-kit 2.0.2, ofetch 1.5 (frontend); Axum, serde/serde_json, notes-core, notes-storage-fs (backend) (005-vault-icons-overview)
- Filesystem — `.vault.json` in each vault directory gains an optional `icon` field (005-vault-icons-overview)
- TypeScript 5.6 / Vue 3.5 (frontend only) + `nyx-kit/eslint` (shared config); `eslint ^9.18`, `eslint-plugin-vue ^9.32`, `@vue/eslint-config-typescript ^14.3`, `eslint-plugin-oxlint ^0.15` (peer deps) (006-frontend-eslint)
- TypeScript 5.6 / Vue 3.5 / Vite + Vue Router 5.0, Pinia 3.0, nyx-kit 2.0.2, ofetch 1.5 (007-frontend-domain-structure)
- N/A — frontend-only refactor, no data model changes (007-frontend-domain-structure)
- TypeScript 5.6.x, Vue 3.5.x single-file components + `vue`, `pinia`, `vue-router`, `nyx-kit` 2.0.3, `ofetch` (008-adopt-nyxcard-cards)
- N/A for this feature; frontend presentation only (008-adopt-nyxcard-cards)
- Rust workspace backend + TypeScript 5.6 / Vue 3.5 frontend + Axum, serde/serde_json, chrono, uuid, Vue 3 Composition API, Pinia, ofetch, nyx-kit 2.0.6 (010-add-line-comments)
- Filesystem-backed Markdown notes plus JSON comment sidecars (`<note>.comments.json`) (010-add-line-comments)
- Rust (workspace crates), TypeScript, Vue 3 Composition API + Axum server stack, Vue Router, ofetch/fetch, nyx-kit, Tauri shell for native mode (011-add-server-profiles)
- Filesystem-backed notes remain unchanged; client profile metadata persists in client app configuration and per-profile secret storage (011-add-server-profiles)
- TypeScript with Vue 3 Composition API + Vue Router, Pinia, `ofetch`, `nyx-kit` (012-global-note-browsing)
- Filesystem-backed notes through the existing backend APIs; browser local storage for workspace profiles, sessions, and favorites persistence (012-global-note-browsing)
- Rust stable workspace, TypeScript 5, Vue 3 Composition API + `axum`, `tokio`, `serde`, `serde_yaml`, `serde_json`, `chrono`, `clap`, `dialoguer`, `comfy-table`, Vue Router, Pinia, `ofetch`, `nyx-kit` (013-file-system)
- Local filesystem under `NOTES_ROOT` with Markdown notes, YAML frontmatter, JSON metadata sidecars, and JSON namespace metadata files (013-file-system)
- Rust 2021 workspace crates, TypeScript 5.x, Vue 3 Composition API + Axum, notes-core/auth/storage crates, Argon2 password hashing, Vue Router, Pinia, ofetch, nyx-kit (014-user-management)
- Filesystem for notes remains unchanged; user credentials move to a server-local embedded SQLite store with non-reversible password hashes (014-user-management)

## Recent Changes
- 001-route-domains: Added TypeScript 5.x / Vue 3.5 / Vite + Vue Router 5.0, Pinia 3.0, nyx-kit 1.4.4, ofetch 1.5
- 004-pinia-stores: Replaced `useVaults` and `useNotes` module-level composables with `useVaultStore` (`stores/vaults.ts`) and `useNotesStore` (`stores/notes.ts`) Pinia setup stores; vault-keyed notes cache (`notesByVault`); both stores expose `$reset()` and `acceptHMRUpdate`
