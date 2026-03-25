# Implementation Plan: Frontend Domain-Based Structure

**Branch**: `007-frontend-domain-structure` | **Date**: 2026-03-25 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/007-frontend-domain-structure/spec.md`

## Summary

Reorganise `frontend/src/` from a flat, concern-based layout (separate `components/`, `stores/`, `views/`, `api/` directories) into a domain-based layout (`vaults/`, `notes/`, `comments/`, `auth/`, `teams/`, `shared/`). Each domain folder collocates its views, components, stores, composables, and API module. Cross-domain code moves to `shared/`. Entry point files (`main.ts`, `App.vue`, `vite-env.d.ts`) stay at the `src/` root.

## Technical Context

**Language/Version**: TypeScript 5.6 / Vue 3.5 / Vite
**Primary Dependencies**: Vue Router 5.0, Pinia 3.0, nyx-kit 2.0.2, ofetch 1.5
**Storage**: N/A — frontend-only refactor, no data model changes
**Testing**: Vitest (unit), Playwright (E2E) — no new test infra introduced
**Target Platform**: Web browser (served by Vite dev server; proxied to Axum backend)
**Project Type**: Vue 3 SPA (single-page application)
**Performance Goals**: No change — this is a structural refactor, not a performance change
**Constraints**: Build must succeed after restructure; all existing routes must resolve correctly; `@/` alias must remain functional
**Scale/Scope**: ~70 files across 5 domains + 1 shared layer

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Check | Status |
|-----------|-------|--------|
| **VI – Composition API only** | No changes to component logic — only file moves | ✅ PASS |
| **VI – All API calls through composables** | API modules move with their domains; composable pattern unchanged | ✅ PASS |
| **VI – nyx-kit for all UI primitives** | No new components introduced | ✅ PASS |
| **VI – No semicolons / single quotes** | No new TypeScript authored; existing files moved as-is | ✅ PASS |
| **V – No new dependencies** | Zero new npm packages introduced | ✅ PASS |
| **IV – Frontend test coverage** | Vitest composable tests still work after alias updates; Playwright unaffected | ✅ PASS |
| **I – Docs are source of truth** | `docs/interface/frontend.md` must be updated to reflect the new structure | ⚠️ REQUIRED — update docs before shipping |
| **II – Layer boundaries** | Frontend-only change; no backend layers affected | ✅ PASS |

**Gate result**: No violations. One required docs update (`docs/interface/frontend.md`).

## Project Structure

### Documentation (this feature)

```text
specs/007-frontend-domain-structure/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output (file migration map)
├── quickstart.md        # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code — Target Layout

```text
frontend/src/
├── main.ts                     # entry point (stays at root)
├── App.vue                     # app shell (stays at root)
├── vite-env.d.ts               # Vite env types (stays at root)
│
├── vaults/
│   ├── api/
│   │   └── vaults.ts           # ← api/vaults.ts
│   ├── assets/
│   │   └── icons/              # ← assets/icons/vaults/ (solid vault icons)
│   ├── components/
│   │   ├── VaultCard.vue       # ← components/VaultCard.vue
│   │   ├── VaultIcon.vue       # ← components/VaultIcon.vue
│   │   ├── VaultIconPicker.vue # ← components/VaultIconPicker.vue
│   │   └── VaultSwitcher.vue   # ← components/VaultSwitcher.vue
│   ├── stores/
│   │   └── vaults.ts           # ← stores/vaults.ts
│   └── views/
│       ├── HomeView.vue        # ← views/HomeView.vue
│       ├── VaultView.vue       # ← views/VaultView.vue
│       └── VaultSettingsView.vue # ← views/VaultSettingsView.vue
│
├── notes/
│   ├── api/
│   │   └── notes.ts            # ← api/notes.ts
│   ├── components/
│   │   ├── NoteEditor.vue      # ← components/NoteEditor.vue
│   │   ├── NoteList.vue        # ← components/NoteList.vue
│   │   └── NoteToolbar.vue     # ← components/NoteToolbar.vue
│   ├── stores/
│   │   ├── notes.ts            # ← stores/notes.ts
│   │   └── editor.ts           # ← stores/editor.ts
│   └── views/
│       └── NoteView.vue        # ← views/NoteView.vue
│
├── comments/
│   ├── api/
│   │   └── comments.ts         # ← api/comments.ts
│   ├── components/
│   │   ├── CommentComposer.vue # ← components/CommentComposer.vue
│   │   ├── CommentSidebar.vue  # ← components/CommentSidebar.vue
│   │   └── CommentThread.vue   # ← components/CommentThread.vue
│   └── composables/
│       └── useComments.ts      # ← composables/useComments.ts
│
├── auth/
│   ├── api/
│   │   └── auth.ts             # ← api/auth.ts
│   ├── composables/
│   │   └── useAuth.ts          # ← composables/useAuth.ts
│   └── views/
│       └── LoginView.vue       # ← views/LoginView.vue
│
├── teams/
│   ├── api/
│   │   └── teams.ts            # ← api/teams.ts
│   ├── composables/
│   │   └── useTeams.ts         # ← composables/useTeams.ts
│   └── views/
│       └── TeamSettingsView.vue # ← views/TeamSettingsView.vue
│
└── shared/
    ├── api/
    │   └── client.ts           # ← api/client.ts (base HTTP client)
    ├── assets/
    │   ├── icons/              # ← assets/icons/ (generic outline SVGs)
    │   ├── theme.css           # ← assets/theme.css
    │   └── vue.svg             # ← assets/vue.svg
    ├── components/
    │   ├── AppLayout.vue       # ← components/AppLayout.vue
    │   └── SidebarNav.vue      # ← components/SidebarNav.vue
    ├── router/
    │   └── index.ts            # ← router/index.ts
    ├── types/
    │   └── index.ts            # ← types/index.ts (all cross-domain types)
    └── utils/
        └── time.ts             # ← utils/time.ts
```

**Structure Decision**: Single-project web application with domain-based `src/` layout. The `@/` alias continues to map to `frontend/src/`, so domain imports use `@/vaults/...`, `@/notes/...`, `@/shared/...`, etc. No alias changes required.

## Complexity Tracking

> No constitution violations — this section is empty.
