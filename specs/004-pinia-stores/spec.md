# Feature Specification: Pinia Stores for Vaults and Notes

**Feature Branch**: `004-pinia-stores`
**Created**: 2026-03-23
**Status**: In Design
**Input**: "let's move some composables we have now to actual stores, we have useNotes and useVaults that definitely need to be their own store with their own state management"

---

## Context

`useVaults` and `useNotes` are currently implemented as composables with **module-level `ref()` singletons** — they act as global stores but without any of the Pinia guarantees. This pattern:

- Is invisible to Vue DevTools (no state inspection, no action tracking)
- Cannot be reset on logout
- Cannot be isolated per test
- Does not support HMR state preservation
- Duplicates what Pinia already provides correctly

`useEditorStore` (already a proper Pinia store) is the template for what these should be.

Additionally, the `useNotes` composable still uses a flat `notes` ref that gets clobbered on vault switches. The migration to a Pinia store is the right time to introduce the vault-keyed `notesByVault` cache that was previously prototyped in feature 003.

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Vault State is Inspectable and Resettable (Priority: P1)

A developer opens Vue DevTools and can inspect `useVaultStore` state (vault list, active vault), observe mutations as they happen, and replay actions. On logout, `$reset()` clears all vault state cleanly.

**Acceptance Scenarios**:
1. **Given** the app is running, **When** a developer opens Vue DevTools Pinia panel, **Then** `vaults` store is listed with its full state visible.
2. **Given** a user logs out, **When** `useVaultStore().$reset()` is called, **Then** `vaults` is `[]`, `activeVault` is `null`, `loading` is `false`.

---

### User Story 2 — Notes Cache Survives Vault Switching (Priority: P1)

A user has two vaults: "Home" (2 notes) and "Work" (0 notes). When they navigate between vaults, the sidebar note list immediately reflects the correct vault's notes without any loading flash or stale state.

**Acceptance Scenarios**:
1. **Given** the user visits HomeView, **When** the page loads, **Then** no vault is active and the sidebar note list is empty.
2. **Given** the user switches from Work vault to Home vault, **When** the route changes, **Then** the sidebar immediately shows the Home vault's 2 notes (from cache).
3. **Given** the user navigates back to HomeView, **When** the route loads, **Then** `activeVault` is null and the sidebar is empty.

---

### User Story 3 — Notes State is Inspectable and Resettable (Priority: P2)

Same developer ergonomics as US1 but for `useNotesStore`.

**Acceptance Scenarios**:
1. `notes` store is visible in Vue DevTools with `notesByVault`, `activeNote`, loading flags.
2. `useNotesStore().$reset()` clears all note state cleanly.

---

## Requirements

### Functional Requirements

- **FR-001**: `useVaults` MUST be replaced by `useVaultStore` (Pinia setup store) in `frontend/src/stores/vaults.ts`
- **FR-002**: `useNotes` MUST be replaced by `useNotesStore` (Pinia setup store) in `frontend/src/stores/notes.ts`
- **FR-003**: Both stores MUST expose a `$reset()` action that restores initial state
- **FR-004**: `useNotesStore` MUST use a `notesByVault: Record<string, NoteMeta[]>` cache so per-vault note lists are not clobbered when switching vaults
- **FR-005**: `useVaultStore.setActive()` MUST accept `Vault | null`; `HomeView` MUST call `setActive(null)` on mount
- **FR-006**: `AppLayout` MUST call `useNotesStore().loadAll(vaultIds)` after loading vaults
- **FR-007**: All 10 caller files MUST be updated to import from `@/stores/vaults` / `@/stores/notes`
- **FR-008**: The old composable files (`composables/useNotes.ts`, `composables/useVaults.ts`) MUST be deleted
- **FR-009**: Both stores MUST call `acceptHMRUpdate` for dev ergonomics

### Key Entities

- **`useVaultStore`**: Pinia setup store owning all vault state and mutations
- **`useNotesStore`**: Pinia setup store owning all note state, cache, and mutations

## Success Criteria

- **SC-001**: Both stores appear in Vue DevTools Pinia panel
- **SC-002**: Navigating between vaults shows correct notes from cache — no loading flash on second visit
- **SC-003**: HomeView shows empty sidebar (no active vault)
- **SC-004**: `$reset()` on both stores restores initial state (verified in unit tests or manual DevTools reset)
- **SC-005**: All 10 caller files compile without errors after import path updates
