# Research: Frontend Domain-Based Structure

**Feature**: 007-frontend-domain-structure
**Date**: 2026-03-25

---

## Q1: Should the `@/` Vite alias be changed after restructuring?

**Decision**: No change to the `@/` alias mapping.

**Rationale**: The alias `@` → `frontend/src/` remains valid after restructuring. Domain imports simply become `@/vaults/components/VaultCard.vue`, `@/shared/types/index.ts`, etc. Changing the alias would require touching every import in the app and would provide no benefit. The current alias resolves everything below `src/` by full path, which is exactly what we need.

**Alternatives considered**: Adding per-domain aliases (`@vaults/`, `@notes/`, etc.) — rejected because it requires `vite.config.ts` changes and adds cognitive overhead for minimal gain. The flat `@/domain/...` pattern is sufficient and immediately readable.

---

## Q2: Should domain folders expose a barrel index file (`index.ts`)?

**Decision**: No barrel files during this refactor. Individual files are imported by explicit path.

**Rationale**: Barrel files (`domain/index.ts`) are a common pattern but introduce a maintenance burden: every new file must be re-exported, and circular dependency risks increase as the app grows. The feature spec requires only that files be in the right domain folder — it does not require any particular import ergonomics beyond disambiguation.

**Alternatives considered**: One `index.ts` per domain re-exporting all public symbols — rejected for this refactor. It can be introduced incrementally per domain if developers find it useful. Not part of the migration scope.

---

## Q3: Which domain owns `HomeView.vue`?

**Decision**: `HomeView.vue` moves to `vaults/views/`.

**Rationale**: `HomeView.vue` is the vault overview screen (route `path: ''`, child of `/`). It displays vault cards and is the entry point for vault navigation. Its content, behaviour, and dependencies are entirely vault-domain concerns. Placing it in `shared/` would misrepresent its ownership; a `home/` domain would be a single-file domain with no other members.

**Alternatives considered**: `shared/views/HomeView.vue` — rejected because the view is vault-specific, not cross-domain. `home/` domain — rejected as premature; the concept of "home" is currently equivalent to "vault list".

---

## Q4: Which domain owns `editor.ts` (Pinia store)?

**Decision**: `stores/editor.ts` moves to `notes/stores/editor.ts`.

**Rationale**: The editor store manages the state of the note editing surface — current note content, dirty state, cursor position, etc. This is exclusively a notes-domain concern. No other domain reads or writes editor state.

**Alternatives considered**: `shared/stores/editor.ts` — rejected because nothing outside the notes domain depends on it.

---

## Q5: Which domain owns `SidebarNav.vue`?

**Decision**: `SidebarNav.vue` moves to `shared/components/`.

**Rationale**: The navigation sidebar (`SidebarNav.vue`) is part of the application shell — it renders navigation links across multiple domains (vaults, teams, settings). It is consumed by `AppLayout.vue` and serves no single-domain purpose. It belongs in shared alongside `AppLayout.vue`.

**Alternatives considered**: `vaults/components/` — rejected because the nav renders cross-domain links and is not vault-specific.

---

## Q6: Where do generic icon SVGs vs vault-specific icon SVGs go?

**Decision**: `assets/icons/vaults/` (solid vault icons) → `vaults/assets/icons/`. Generic outline SVGs in `assets/icons/` → `shared/assets/icons/`.

**Rationale**: Vault icon SVGs are rendered exclusively by `VaultIcon.vue` and `VaultIconPicker.vue`. They are vault-domain assets. The generic outline SVGs may be used elsewhere in the app (UI elements, shared components). The spec explicitly calls this out in the Assumptions section.

**Alternatives considered**: All icons in `shared/assets/` — rejected because it mixes domain-owned assets into shared, weakening the domain boundary for the vaults domain.

---

## Q7: Should import paths in moved files be updated?

**Decision**: Yes — all internal `@/` imports must be updated to point to new file locations as part of the migration.

**Rationale**: After files move, their existing imports (e.g. `@/stores/vaults` in a vault component) will point to non-existent paths and break the build. Every moved file must have its imports audited and updated to the new paths. The router's dynamic imports must also be updated.

**Scope of updates required**:
- `shared/router/index.ts` — all `@/views/...` and `@/components/AppLayout` imports
- All domain components that import from `@/stores/...`, `@/api/...`, `@/types/...`, `@/composables/...`, `@/utils/...`
- `shared/api/client.ts` — no internal imports, no changes needed
- `shared/types/index.ts` — no internal imports, no changes needed
- `main.ts` and `App.vue` — import `@/shared/router/...` and `@/shared/assets/theme.css`

---

## Q8: What is the correct approach for cross-domain type imports?

**Decision**: All TypeScript types remain in `shared/types/index.ts`. All domains import from `@/shared/types`.

**Rationale**: The current `types/index.ts` defines types for all domains (`Vault`, `Note`, `Comment`, `Team`, `Auth`). Splitting types per-domain (e.g., `vaults/types/vault.ts`) would require cross-domain type imports for types like `VaultOwner` that span domains, creating circular dependency risk. Keeping all types in `shared/types` is the safest approach for a project at this scale.

**Alternatives considered**: Split types per domain — possible but higher risk for cross-domain types (e.g., `NotePermission` used in both `Vault` and `NoteMeta`). Can be evaluated in a future iteration once domain boundaries are more settled.

---

## Summary: No blockers. All questions resolved.

| Question | Resolution |
|----------|-----------|
| `@/` alias change | No change needed |
| Barrel index files | Not introduced in this refactor |
| `HomeView.vue` domain | `vaults/` |
| `editor.ts` domain | `notes/` |
| `SidebarNav.vue` domain | `shared/` |
| Icon asset split | Vault icons → `vaults/assets/`; generic → `shared/assets/` |
| Import path updates | All internal imports must be updated in every moved file |
| Cross-domain types | Remain in `shared/types/index.ts` |
