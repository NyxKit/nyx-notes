# Implementation Plan: Pinia Stores for Vaults and Notes

**Branch**: `004-pinia-stores` | **Date**: 2026-03-23 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/004-pinia-stores/spec.md`

## Summary

Replace the `useVaults` and `useNotes` module-level singleton composables with proper Pinia setup stores (`useVaultStore`, `useNotesStore`). Both stores use the existing Pinia `defineStore` setup-function pattern already established by `useEditorStore`. The migration also re-introduces the vault-keyed notes cache (`notesByVault`) — the right layer for this was always the store, not the composable. Ten caller files are updated with new import paths and function names. The two old composable files are deleted.

## Technical Context

**Language/Version**: TypeScript 5.x / Vue 3.5 / Vite
**Primary Dependencies**: Pinia 3.0 (already installed), nyx-kit 2.x, Vue Router 5.0
**Storage**: N/A — frontend-only; existing REST API endpoints unchanged
**Testing**: Manual smoke test + Vue DevTools Pinia panel verification
**Target Platform**: Web SPA (desktop browser)
**Project Type**: Web application (Vue 3 SPA frontend)
**Performance Goals**: No regressions; vault list and note list reactive as before
**Constraints**: No new npm packages; no backend changes; no type changes; no new API calls
**Scale/Scope**: Refactor only — 2 new store files, 2 composable files deleted, 10 call sites updated

## Constitution Check

| Principle | Gate | Status |
|-----------|------|--------|
| I. Docs first | `docs/interface/frontend.md` updated before implementation | PASS |
| II. Layer boundaries | No layer changes — pure frontend state management refactor | PASS |
| III. Filesystem | No backend or on-disk changes | N/A |
| IV. Testing | Manual smoke test + DevTools verification sufficient; no new logic | PASS |
| V. Security | No new dependencies; Pinia already present | PASS |
| VI. Frontend | Composition API; setup store pattern; no semicolons; single quotes | PASS |

## Project Structure

### Documentation (this feature)

```text
specs/004-pinia-stores/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 — approach decisions
├── data-model.md        # Phase 1 — store state and action contracts
├── quickstart.md        # Phase 1 — build guide
├── contracts/
│   └── store-contract.md # Phase 1 — public store API
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code Changes

```text
app/src/
├── stores/
│   ├── vaults.ts         # NEW — useVaultStore (replaces useVaults composable)
│   └── notes.ts          # NEW — useNotesStore (replaces useNotes composable)
├── composables/
│   ├── useVaults.ts      # DELETED
│   └── useNotes.ts       # DELETED
├── components/
│   ├── AppLayout.vue     # MODIFIED — useVaultStore, useNotesStore; add loadAll
│   ├── NoteList.vue      # MODIFIED — useVaultStore, useNotesStore; cache-based; sort+limit
│   ├── NoteEditor.vue    # MODIFIED — useNotesStore
│   ├── SidebarNav.vue    # MODIFIED — useVaultStore, useNotesStore
│   └── VaultSwitcher.vue # MODIFIED — useVaultStore, useNotesStore; notesFor()
└── views/
    ├── HomeView.vue          # MODIFIED — useVaultStore; setActive(null)
    ├── VaultView.vue         # MODIFIED — useVaultStore, useNotesStore; notesFor()
    ├── NoteView.vue          # MODIFIED — useVaultStore, useNotesStore
    ├── TeamSettingsView.vue  # MODIFIED — useVaultStore
    └── VaultSettingsView.vue # MODIFIED — useVaultStore
```
