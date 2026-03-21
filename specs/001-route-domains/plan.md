# Implementation Plan: Route Domains

**Branch**: `001-route-domains` | **Date**: 2026-03-21 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-route-domains/spec.md`

## Summary

Introduce three clear route domains — Home, Vault, Note — by creating a new `VaultView.vue` (notes masonry + getting-started state), reworking `HomeView.vue` to redirect to the single vault or display a multi-vault masonry, and registering the new `/vaults/:vault_id` route. No backend changes required.

## Technical Context

**Language/Version**: TypeScript 5.x / Vue 3.5 / Vite
**Primary Dependencies**: Vue Router 5.0, Pinia 3.0, nyx-kit 1.4.4, ofetch 1.5
**Storage**: Filesystem (Rust backend) — no frontend storage changes
**Testing**: Vitest (composables), Playwright (E2E)
**Target Platform**: Web SPA (served by Axum; also Tauri desktop)
**Project Type**: web-application (frontend SPA)
**Performance Goals**: Route transitions < 100 ms; first paint < 1 s
**Constraints**: nyx-kit only (no new component libraries), no new npm packages, no semicolons, single quotes
**Scale/Scope**: 3 primary views, ~8 route definitions, personal notes app

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Docs Are the Source of Truth | ✅ GATE | `docs/interface/frontend.md` routing table MUST be updated before implementation |
| II. Strict Layer Boundaries | ✅ PASS | Frontend-only change; no backend layer touched |
| III. Filesystem Is the Content Source of Truth | ✅ N/A | No storage format changes |
| IV. Test Coverage Per Layer | ⚠️ REQUIRED | `VaultView.vue` needs Vitest or Playwright coverage |
| V. Security by Design | ✅ GATE | Auth guard (`requiresAuth: true`) must be applied to new `/vaults/:vault_id` route |
| VI. Frontend Constraints | ✅ GATE | Composition API, nyx-kit only, no semicolons, single quotes |

**Post-design re-check**: All gates confirmed satisfied by design artifacts (see `research.md`, `data-model.md`, `contracts/routing.md`).

## Project Structure

### Documentation (this feature)

```text
specs/001-route-domains/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/
│   └── routing.md       # URL schema contract
└── tasks.md             # Phase 2 output (/speckit.tasks — NOT created here)
```

### Source Code (repository root)

```text
frontend/src/
├── router/
│   └── index.ts              # modified: add /vaults/:vault_id route
├── views/
│   ├── HomeView.vue           # modified: single-vault redirect + multi-vault masonry
│   └── VaultView.vue          # new: notes masonry + getting-started state
└── (all other files unchanged)

docs/
└── interface/
    └── frontend.md            # modified: routing table + VaultView description
```

**Structure Decision**: Single web-application project (Option 1 variant). Frontend-only changes in `frontend/src/`. Docs update in `docs/interface/`.

## Complexity Tracking

*No constitution violations — table not required.*
