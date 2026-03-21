# Research: Route Domains

**Phase 0 output for `001-route-domains`**

## Resolved Decisions

### 1. Redirect strategy for single-vault Home

**Decision**: Use `onMounted` + `useRouter().replace()` in `HomeView.vue` after `loadVaults()` resolves.

**Rationale**: The vault count is only known after an async API call, so a static router-guard redirect isn't clean. Performing the redirect inside `onMounted` keeps the logic colocated with the view's data-loading. `replace()` (not `push()`) avoids polluting the history stack with a `/` entry the user can't usefully go back to.

**Alternatives considered**:
- Navigation guard `beforeEach`: would require injecting vault state into the guard, coupling it to composable internals. Rejected — adds complexity for no benefit.
- Redirect route config: static, cannot react to runtime data. Rejected.

---

### 2. VaultView as a separate file vs. extending HomeView

**Decision**: New `VaultView.vue` file. `HomeView.vue` is reworked to be vault-list aware.

**Rationale**: Home and Vault have different concerns — Home is about picking a vault, Vault is about picking a note. Merging them into one component would require conditional rendering at every layer and make future evolution harder. A separate view keeps each domain clean.

**Alternatives considered**:
- Single adaptive view: rejected due to logic branching complexity and coupling unrelated domains.

---

### 3. Vault card masonry layout

**Decision**: Reuse the same CSS `columns` masonry pattern already established in `HomeView.vue` for note cards. Vault cards follow the same structure: name, slug, and a relative-date or note-count badge.

**Rationale**: Pattern is already in the codebase, consistent with the "Silent Atelier" design system. No additional dependencies required.

**Alternatives considered**:
- CSS Grid: works, but CSS columns already used and produces good variable-height results. No reason to deviate.

---

### 4. Vault creation flow from Home

**Decision**: Inline form in `HomeView.vue` (a conditional expand, no modal) with `slug` and `name` fields. On submit, call `useVaults().create()` and redirect to the new vault.

**Rationale**: The existing `useVaults` composable already exposes `create(req: CreateVaultRequest)`. No new composable or API layer needed. An inline form avoids a modal dependency and keeps the flow lightweight.

**Alternatives considered**:
- NyxModal dialog: valid and aligns with the pattern used in NoteView delete confirmation. Acceptable but adds a dependency on user interaction choreography. Deferred — inline form is simpler for v1.

---

### 5. Empty state / getting-started prompt in VaultView

**Decision**: Reuse the `home__welcome-card` pattern from the existing `HomeView.vue` empty state. The CTA creates a new note in the current vault via `useNotes().create()`.

**Rationale**: The design already exists and is visually consistent. The note-creation flow is identical to the existing `createFirst()` in `HomeView.vue` — just scoped to a known `vault_id` from the route param.

**Alternatives considered**:
- Separate design treatment: unnecessary divergence for a first-pass.

---

## No NEEDS CLARIFICATION items

All unknowns in Technical Context resolved through codebase analysis. No external research required.
