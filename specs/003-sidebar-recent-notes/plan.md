# Implementation Plan: Recent Notes Sidebar (Persistent)

**Branch**: `003-sidebar-recent-notes` | **Date**: 2026-03-23 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-sidebar-recent-notes/spec.md`

## Summary

Extract the existing sidebar (VaultSwitcher, SidebarNav, NoteList, sidebar footer) from the per-view `app-shell` layouts into a shared `AppLayout.vue` component. Use Vue Router nested routes to mount `AppLayout` once for all authenticated views, making the sidebar persistent without any per-view coordination. No new UI components beyond `AppLayout` are introduced. `useNotes` is extended with a vault-keyed notes cache so the sidebar always reflects correct data when switching vaults.

## Technical Context

**Language/Version**: TypeScript 5.x / Vue 3.5 / Vite
**Primary Dependencies**: nyx-kit 2.x (NyxButton), Vue Router 5.0, Pinia 3.0
**Storage**: N/A — frontend-only; uses existing REST API endpoints
**Testing**: Manual navigation smoke test across all authenticated routes
**Target Platform**: Web SPA (desktop browser)
**Project Type**: Web application (Vue 3 SPA frontend)
**Performance Goals**: Sidebar persists without remounting on route changes
**Constraints**: No new npm packages; no backend changes; no new composable files
**Scale/Scope**: Structural refactor + composable extension — moves existing sidebar, extends `useNotes` with a vault cache

## Constitution Check

| Principle | Gate | Status |
|-----------|------|--------|
| I. Docs first | `docs/interface/frontend.md` updated before implementation | PASS |
| II. Layer boundaries | No layer changes — pure structural UI refactor | PASS |
| III. Filesystem | No backend or on-disk changes | N/A |
| IV. Testing | Manual smoke test sufficient; no new logic introduced | PASS |
| V. Security | No new dependencies or auth surface changes | PASS |
| VI. Frontend | Composition API; nyx-kit; no semicolons; single quotes | PASS |

## Project Structure

### Documentation (this feature)

```text
specs/003-sidebar-recent-notes/
├── plan.md              # This file
├── research.md          # Phase 0 — approach decisions
├── data-model.md        # Phase 1 — no new types
├── quickstart.md        # Phase 1 — build guide
├── contracts/
│   └── ui-contract.md   # Phase 1 — AppLayout interface
└── tasks.md             # Task list
```

### Source Code Changes

```text
app/src/
├── components/
│   ├── AppLayout.vue               # NEW — persistent authenticated shell; calls loadAll on mount
│   ├── NoteList.vue                # MODIFIED — reads from notesByVault cache; sort+limit; active state via route param
│   └── VaultSwitcher.vue           # MODIFIED — note count via notesFor()
├── composables/
│   ├── useNotes.ts                 # MODIFIED — notesByVault cache; notesFor(); loadAll()
│   └── useVaults.ts                # MODIFIED — setActive() accepts Vault | null
├── router/
│   └── index.ts                    # MODIFIED — authenticated routes nested under AppLayout
└── views/
    ├── HomeView.vue                # MODIFIED — remove sidebar + outer wrapper; setActive(null) on mount
    ├── NoteView.vue                # MODIFIED — remove sidebar + toggle logic
    └── VaultView.vue               # MODIFIED — remove sidebar + outer wrapper; uses notesFor()
```
