# Implementation Plan: Unify Card Surfaces

**Branch**: `[008-adopt-nyxcard-cards]` | **Date**: 2026-03-26 | **Spec**: `/home/arnedecant/Projects/nyxkit/nyx-notes-core/specs/008-adopt-nyxcard-cards/spec.md`
**Input**: Feature specification from `/home/arnedecant/Projects/nyxkit/nyx-notes-core/specs/008-adopt-nyxcard-cards/spec.md`

## Summary

Unify the browse-card family across the frontend by moving the in-scope surfaces onto standalone `NyxCard`-based components: `VaultCard`, the inline create-vault card, and a dedicated `NoteCard`. `VaultCard` and `NoteCard` wrap internally in `RouterLink` anchors to preserve standard browser link behavior, vaults gain an optional description field, and note cards display a distilled description derived from the first actual Markdown paragraph on save.

## Technical Context

**Language/Version**: TypeScript 5.6.x, Vue 3.5.x single-file components  
**Primary Dependencies**: `vue`, `pinia`, `vue-router`, `nyx-kit` 2.0.3, `ofetch`  
**Storage**: N/A for this feature; frontend presentation only  
**Testing**: `npm run lint`, `npm run build`, targeted frontend unit coverage for card interaction behavior, and browser smoke verification of affected browse flows  
**Target Platform**: Responsive Vue SPA in modern desktop and mobile browsers  
**Project Type**: Frontend web application within a larger Rust + Vue monorepo  
**Performance Goals**: Preserve current dashboard and vault browsing responsiveness with no perceptible regression in render or interaction speed during normal vault and note browsing  
**Constraints**: Must follow `DESIGN.md`; must use `nyx-kit` primitives; must preserve browser-native link behavior for selectable cards; must update docs before implementation; must not restyle out-of-scope surfaces; must not introduce a shared `BrowseCardSurface` abstraction  
**Scale/Scope**: In scope: vault dashboard cards, inline create-vault card, note cards in the vault notes view; out of scope: VaultSwitcher, empty-state cards, comment threads, settings/modals, and navigation chrome

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Docs are source of truth**: PASS. Implementation begins with updates to `/home/arnedecant/Projects/nyxkit/nyx-notes-core/docs/interface/frontend.md` and, if needed, `/home/arnedecant/Projects/nyxkit/nyx-notes-core/DESIGN.md` to reflect the clarified in-scope and out-of-scope card family rules.
- **Strict layer boundaries**: PASS. This is a frontend-only presentation change; no backend, storage, or API layer boundaries move.
- **Filesystem is source of truth**: PASS. No on-disk content, frontmatter, or storage semantics change.
- **Test coverage per layer**: PASS WITH EXECUTION REQUIREMENT. Frontend work must include automated coverage for explicit activation behavior plus browser verification for the affected browse flows.
- **Security by design**: PASS. No auth or storage posture changes, but link behavior and keyboard access are explicit requirements so primary-target behavior does not regress.
- **Frontend constraints**: PASS. `nyx-kit` remains the only UI primitive source; `VaultCard` stays in `vaults/components`, `NoteCard` lives in `notes/components`, and no direct cross-domain imports are introduced.
- **Known divergence to resolve explicitly**: Earlier repo research preferred `NyxButton`-semantic selection cards over `NyxCard`. This feature supersedes that presentation decision, but preserves the earlier accessibility concern by making the user-facing interactive surface a `RouterLink` anchor rather than `NyxCard` itself.

**Post-Design Re-Check**

- PASS. `research.md` resolves the semantics question by making the interactive surface a link while keeping `NyxCard` visual.
- PASS. `data-model.md` now encodes both in-scope and out-of-scope surface roles so implementation scope is testable.
- PASS. `contracts/browse-card-surfaces.md` makes the in-scope and exclusion boundaries explicit for future implementation and review.
- PASS. `quickstart.md` preserves docs-first workflow and includes explicit validation for link behavior, responsive behavior, description handling, and exclusions.

## Project Structure

### Documentation (this feature)

```text
/home/arnedecant/Projects/nyxkit/nyx-notes-core/specs/008-adopt-nyxcard-cards/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── browse-card-surfaces.md
└── tasks.md
```

### Source Code (repository root)

```text
/home/arnedecant/Projects/nyxkit/nyx-notes-core/
├── DESIGN.md
├── docs/
│   └── interface/
│       └── frontend.md
└── frontend/
    ├── package.json
    └── src/
        ├── vaults/
        │   ├── components/
        │   │   ├── VaultCard.vue
        │   │   └── index.ts
        │   └── views/
        │       ├── HomeView.vue
        │       └── VaultView.vue
        ├── comments/
        │   └── components/
        │       └── CommentThread.vue
        ├── notes/
        │   ├── components/
        │   │   ├── NoteCard.vue
        │   │   └── index.ts
        │   └── views/
        │       └── NoteView.vue
        └── shared/
            └── assets/
                └── theme.css
```

**Structure Decision**: Treat this as a frontend feature inside the existing domain-based Vue app. `VaultCard` remains in `/home/arnedecant/Projects/nyxkit/nyx-notes-core/frontend/src/vaults/components/`, `NoteCard` lives in `/home/arnedecant/Projects/nyxkit/nyx-notes-core/frontend/src/notes/components/`, shared theme tokens stay in `/home/arnedecant/Projects/nyxkit/nyx-notes-core/frontend/src/shared/assets/theme.css`, and out-of-scope surfaces such as `/home/arnedecant/Projects/nyxkit/nyx-notes-core/frontend/src/vaults/components/VaultSwitcher.vue` and `/home/arnedecant/Projects/nyxkit/nyx-notes-core/frontend/src/comments/components/CommentThread.vue` are referenced only to document non-target areas.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
