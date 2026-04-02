# Implementation Plan: Global Note Browsing

**Branch**: `012-global-note-browsing` | **Date**: 2026-04-01 | **Spec**: [/home/arnedecant/Projects/nyxkit/nyx-notes/specs/012-global-note-browsing/spec.md](/home/arnedecant/Projects/nyxkit/nyx-notes/specs/012-global-note-browsing/spec.md)
**Input**: Feature specification from `/specs/012-global-note-browsing/spec.md`

## Summary

Promote search and favorites from vault-scoped placeholders into global note-browsing surfaces that always report into the main window, share the same browse layout and sorting controls, reuse the same note card across browse grids, expose server and vault origin on every note card, and remove the obsolete sidebar-wide `New Note` action. Search is fed live from the sidebar search bar and updates the main window on keypresses using debounce or a similar mechanism. The work is frontend-led but begins with doc updates because the current frontend docs still describe vault-scoped favorites and active-profile-only browse behavior.

## Technical Context

**Language/Version**: TypeScript with Vue 3 Composition API  
**Primary Dependencies**: Vue Router, Pinia, `ofetch`, `nyx-kit`  
**Storage**: Filesystem-backed notes through the existing backend APIs; browser local storage for workspace profiles, sessions, and favorites persistence  
**Testing**: Vitest for frontend unit tests and Playwright for frontend E2E, per `docs/testing/README.md`  
**Target Platform**: Vite-based web SPA used against local and remote Nyx Notes servers  
**Project Type**: Web application frontend plus documentation updates  
**Performance Goals**: Global browse views should feel immediate for cached data and keep result rendering usable across multiple profiles and vaults without blocking normal navigation  
**Constraints**: Docs must be updated before code, `nyx-kit` primitives only, Composition API only, no direct cross-domain imports, no new dependencies, preserve link-based browse cards, do not mutate active profile context while aggregating global results  
**Scale/Scope**: Frontend routes, notes-domain browse state, shared browse-surface presentation, sidebar navigation, and supporting docs/tests across the existing multi-profile frontend

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Docs are the source of truth**: PASS with required first step to update `docs/interface/frontend.md` and `docs/testing/README.md` before implementation because current docs still describe vault-scoped favorites and active-profile-only browse behavior.
- **Strict layer boundaries**: PASS. The plan is frontend-focused and does not move permission or storage responsibilities into the wrong backend layer.
- **Filesystem is the content source of truth**: PASS. No on-disk storage or frontmatter changes are planned.
- **Test coverage per layer**: PASS with required frontend Vitest coverage for browse-state derivation and card rendering, plus Playwright coverage for global browse flows.
- **Security by design**: PASS. The plan avoids leaking data across active-profile context switches by using explicit profile-scoped loading and retaining existing auth/session boundaries.
- **Frontend constraints**: PASS. Ownership stays in `notes/` with shared-only UI reuse, all API activity remains behind stores/composables, and the plan uses `nyx-kit` browse-card primitives rather than custom replacements.

## Project Structure

### Documentation (this feature)

```text
specs/012-global-note-browsing/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── global-note-browsing.md
└── tasks.md
```

### Source Code (repository root)

```text
docs/
├── interface/frontend.md
└── testing/README.md

frontend/src/
├── notes/
│   ├── composables/
│   │   ├── useGlobalNoteBrowsing.ts    # planned
│   │   └── index.ts
│   ├── components/
│   │   ├── NoteCard.vue
│   │   ├── GlobalNoteBrowseView.vue    # planned
│   │   └── index.ts
│   ├── stores/
│   │   ├── notes.ts
│   │   ├── editor.ts
│   │   ├── index.ts
│   │   └── noteBrowsing.ts            # planned
│   └── views/
│       ├── NoteView.vue
│       ├── GlobalSearchView.vue       # planned
│       ├── FavoritesView.vue          # planned
│       └── index.ts
├── shared/
│   ├── components/
│   │   ├── AppLayout.vue
│   │   ├── SidebarNav.vue
│   │   └── index.ts
│   ├── router/index.ts
│   └── types/
│       └── index.ts
└── vaults/
    ├── stores/vaults.ts
    └── views/VaultView.vue
```

**Structure Decision**: Keep feature ownership in `frontend/src/notes/` because global search and favorites are note-browsing concerns. Reuse `shared/` only for shell navigation and common types, keep `vaults/` limited to vault-scoped browsing, and implement one shared browse surface component for search and favorites so the layout and sort-control presentation stay identical. Profile-scoped API loading lives in a notes-domain composable, while the browsing store owns derived state and UI-facing state only. Docs are updated first because they currently describe the older route and scope model.

## Phase 0: Research Outcomes

- Canonical global routes should be `/notes/search` and `/notes/favorites`.
- `NoteView.vue` should return to editor ownership only; global browse surfaces should move into dedicated notes-domain views.
- Search and favorites should render through the same browse-surface layout component, differing only in the notes they pass into the grid.
- Profile-scoped global browse loading should live in a notes-domain composable rather than directly in the browsing store.
- `NoteMeta` should remain unchanged as the shared domain/API type; origin labels belong in a frontend-derived browse-card model.
- Favorite persistence must move from bare note IDs to compound references containing profile, vault, and note identity.
- Global aggregation must not rely on mutating the active profile API client context while loading results.
- Sidebar search input should drive global search live in the main window using debounce or a similar mechanism.

## Phase 1: Design Outputs

- `research.md`: records route ownership, origin-model, persistence, and aggregation decisions.
- `data-model.md`: defines `BrowseNoteCardModel`, `NoteOriginContext`, `FavoriteNoteRef`, and global browse state models.
- `contracts/global-note-browsing.md`: defines the route, card, and sidebar behavior contract.
- `quickstart.md`: defines doc-first and frontend verification workflow.

## Phase 2: Implementation Planning

1. Update docs to redefine search and favorites as global browse surfaces and document route ownership changes.
2. Add a notes-domain composable for profile-scoped global browse loading.
3. Add notes-domain browse state for global search query/results, shared sorting state, and persisted favorites.
4. Add a shared browse surface component plus dedicated search and favorites views under `frontend/src/notes/`, and register global routes.
5. Rework sidebar navigation so favorites is global and the generic `New Note` CTA is removed.
6. Route sidebar search keypresses into live global search updates with debounce or a similar mechanism.
7. Extend `NoteCard` inputs to render server and vault labels and reuse it in global and vault-scoped grids.
8. Add or update Vitest coverage for composable loading, browse-state derivation, shared sort behavior, favorite persistence, and note-card rendering.
9. Add or update Playwright coverage for live search behavior and global browse consistency.

## Post-Design Constitution Check

- **Docs-first requirement**: PASS. The design explicitly treats doc updates as the first implementation step.
- **No new dependency requirement**: PASS. The plan uses existing frontend stack elements only.
- **Domain-boundary requirement**: PASS. Notes-domain behavior stays in `notes/`; shared code remains shell- or type-level only.
- **Testing requirement**: PASS. Required unit and E2E coverage areas are identified before implementation.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |
