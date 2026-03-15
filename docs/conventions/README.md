# Conventions

Coding conventions, naming rules, and file organization standards for Nyx Notes.

## Rust

- Use `snake_case` for all identifiers: fields, functions, variables, modules
- Use `PascalCase` for types, traits, and enums
- Prefer `Arc<dyn Trait>` over generic parameters on shared state structs (e.g. `AppState`)
- Error types are enums; use `thiserror` for derivable `Display` and `From` impls
- No `unwrap()` or `expect()` in library code (`notes-core`, `notes-storage-fs`); allowed only in binary entrypoints with a descriptive message

## Frontmatter

- All frontmatter fields use `snake_case` keys
- Dates are ISO 8601 UTC strings: `"2026-03-14T10:00:00Z"`
- `permission` values are lowercase strings: `"restricted"`, `"comment"`, `"edit"`
- Unknown frontmatter keys must be preserved on round-trip (do not strip unrecognised fields)

## File Naming

| Context | Convention | Example |
|---|---|---|
| Rust source files | `snake_case.rs` | `filesystem_storage.rs` |
| Vue components | `PascalCase.vue` | `NoteEditor.vue` |
| Vue composables | `camelCase.ts`, prefixed `use` | `useNotes.ts` |
| Docs | `kebab-case.md` | `backend-api.md` |
| Note files on disk | `kebab-case.md` (slug) | `weekly-sync.md` |

## Vue / TypeScript

- Use the Composition API exclusively; no Options API
- Composables return plain `ref`/`computed` values — avoid returning reactive objects with nested refs
- All API calls go through composables, not directly from components
- Use `nyx-kit` for all UI primitives; do not introduce additional component libraries

## Git

- Commit messages: imperative mood, present tense (`add vault switcher`, not `added` or `adds`)
- Branch names: `kebab-case`, prefixed by type (`feat/`, `fix/`, `chore/`, `docs/`)
- One logical change per commit; avoid mixing refactors with feature work
