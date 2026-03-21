# Implementation Plan: nyx-kit Primitives Refactor

**Branch**: `002-nyx-kit-primitives` | **Date**: 2026-03-21 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-nyx-kit-primitives/spec.md`

## Summary

Replace every native HTML primitive (`<button>`, `<input>`, `<textarea>`, hand-rolled badge spans) across 11 frontend files with the corresponding nyx-kit component (`NyxButton`, `NyxInput`, `NyxTextarea`, `NyxBadge`, `NyxForm`, `NyxFormField`, `NyxTabs`). Remove all scoped CSS that duplicated nyx-kit's own styling. No new dependencies, no backend changes.

## Technical Context

**Language/Version**: TypeScript 5.x / Vue 3.5 / Vite
**Primary Dependencies**: nyx-kit 1.4.4 (`NyxButton`, `NyxInput`, `NyxTextarea`, `NyxBadge`, `NyxForm`, `NyxFormField`, `NyxTabs` — all already installed)
**Storage**: N/A — UI-only refactor
**Testing**: Manual regression (Playwright E2E for critical flows)
**Target Platform**: Web SPA (served by Axum; Tauri desktop)
**Project Type**: Frontend UI refactor
**Performance Goals**: No regressions in render or interaction behaviour
**Constraints**: No new npm packages; no semicolons; single quotes; Composition API only
**Scale/Scope**: 11 `.vue` files affected; ~30 button replacements, ~5 input replacements, ~1 textarea, ~6 badge replacements, ~3 form wrappers, ~1 tab replacement

## Constitution Check

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Docs Are the Source of Truth | ✅ GATE | `docs/interface/frontend.md` component library section must be updated |
| II. Strict Layer Boundaries | ✅ PASS | Frontend-only; no backend touched |
| III. Filesystem Is the Content Source of Truth | ✅ N/A | No storage changes |
| IV. Test Coverage Per Layer | ✅ REQUIRED | Manual regression required per file; Playwright for critical flows (note editor, comment submit, vault creation) |
| V. Security by Design | ✅ PASS | No auth or permission logic touched |
| VI. Frontend Constraints | ✅ THIS IS THE GATE | This refactor enforces VI: nyx-kit only, no native primitives |

**Post-design re-check**: All gates confirmed satisfied. The refactor is the direct enforcement of Principle VI.

## Project Structure

### Documentation (this feature)

```text
specs/002-nyx-kit-primitives/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 — nyx-kit API decisions
├── data-model.md        # Phase 1 — complete replacement map
├── quickstart.md        # Phase 1 — patterns and verification
├── contracts/
│   └── ui-primitives.md # UI primitive standard contract
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (affected files only)

```text
frontend/src/
├── components/
│   ├── SidebarNav.vue          # <button> → NyxButton gradient
│   ├── NoteList.vue            # <button>s + tag spans → NyxButton, NyxBadge
│   ├── NoteToolbar.vue         # tag spans → NyxBadge hasClose
│   ├── CommentComposer.vue     # <textarea>, <button>s, form → NyxTextarea, NyxButton, NyxForm
│   ├── CommentThread.vue       # <button>s → NyxButton (ghost/danger)
│   └── CommentSidebar.vue      # tab buttons → NyxTabs; add-comment → NyxButton
├── views/
│   ├── HomeView.vue            # <button>s, <input>s, form div → NyxButton, NyxInput, NyxForm
│   ├── VaultView.vue           # <button>s, tag spans → NyxButton, NyxBadge
│   ├── NoteView.vue            # icon <button>s → NyxButton ghost square
│   ├── VaultSettingsView.vue   # <button>s → NyxButton (ghost, danger)
│   └── TeamSettingsView.vue    # <button>s, <input>s, forms → NyxButton, NyxInput, NyxForm

docs/
└── interface/
    └── frontend.md             # Update component library enforcement rule

NOT changed: LoginView.vue (already correct), VaultSwitcher.vue (already uses NyxSelect), NoteEditor.vue, App.vue
```

**Structure Decision**: Single web-application project, frontend-only changes. Implementation order: components first (shared across views), then views.

## Complexity Tracking

*No constitution violations — table not required.*
