# Research: Global Note Browsing

## Decision: Treat global search and favorites as cross-profile, cross-vault browse surfaces

**Rationale**: The feature spec requires search and favorites to ignore the current project or vault context and always show all eligible notes. In this repo, "project" maps most closely to workspace profiles and their server connections, while vaults remain server-scoped. Planning therefore treats global browse surfaces as spanning all accessible workspace profiles and all accessible vaults, not only the currently active vault.

**Alternatives considered**:
- Limit the feature to the active vault: rejected because it contradicts the feature spec.
- Limit the feature to the active workspace profile: rejected because it still makes results depend on current context.
- Add a new backend endpoint that aggregates across servers: rejected for this phase because the frontend already owns multi-profile state and server aggregation is a client orchestration problem, not a single-server API concern.

## Decision: Update docs before implementation because current frontend docs describe vault-scoped favorites and active-profile-only behavior

**Rationale**: `docs/interface/frontend.md` currently documents favorites as a vault-scoped route and documents a single active profile bootstrapping model. That diverges from the feature spec. The constitution and `AGENTS.md` require docs to be the source of truth, so implementation must start with doc updates that define the new global browse behavior and the revised routes.

**Alternatives considered**:
- Implement first and backfill docs later: rejected by constitution.
- Keep the old docs and treat the feature as a one-off exception: rejected because it would formalize drift.

## Decision: Move global search and favorites into dedicated notes-domain views

**Rationale**: Search and favorites are note-browsing features, not shell-level shared infrastructure and not vault-owned behavior. Dedicated `notes/views` pages keep ownership inside the notes domain while allowing the shared shell to route to them. `NoteView` should stop acting as a placeholder page for non-editor surfaces.

**Alternatives considered**:
- Keep favorites in `NoteView`: rejected because it couples editor concerns to browse views and preserves the current placeholder pattern.
- Put search/favorites views in `shared/`: rejected because the behavior is domain-specific, not shared infrastructure.
- Put them in `vaults/views/`: rejected because the feature is explicitly not vault-scoped.

## Decision: Use global routes under the authenticated shell

**Rationale**: Global browse pages should not require a vault identifier in the URL. The plan therefore uses `/notes/search` and `/notes/favorites` as canonical routes. Existing vault-scoped favorites links should redirect to the global route to avoid stale URLs.

**Alternatives considered**:
- Keep `/vaults/:vault_id/favorites`: rejected because the URL implies vault scoping.
- Put routes under `/browse/*`: rejected because this repo already uses the `notes` domain for note-focused views.

## Decision: Keep canonical API/domain note types unchanged and introduce frontend-derived browse models

**Rationale**: `NoteMeta` is a shared domain/API type. Server labels and resolved vault names are frontend presentation context derived from workspace profiles and vault stores, not canonical note fields. A derived browse model keeps layering clean while giving `NoteCard` the origin context it now needs.

**Alternatives considered**:
- Add `server_name` and `vault_name` to shared `NoteMeta`: rejected because those are not canonical note fields and would blur API boundaries.
- Pass many separate props into `NoteCard`: rejected because it spreads origin assembly logic across multiple callers.

## Decision: Replace favorite storage based on bare note IDs with compound references

**Rationale**: The current `localStorage` favorite model stores only note IDs, which is insufficient once the same note ID can appear in multiple vaults or profiles. A compound favorite reference must include enough origin data to reload and distinguish the target note.

**Alternatives considered**:
- Keep `string[]` note IDs: rejected because of collisions and missing origin reconstruction.
- Persist the full note object in local storage: rejected because it duplicates mutable note data and risks stale labels/content.

## Decision: Keep search and favorites state in a dedicated notes browsing store

**Rationale**: Search query state, favorites membership, aggregated result sets, and global loading/error handling are shared across the sidebar, the new browse views, and note-level favorite toggles. A dedicated notes browsing store keeps those concerns out of the CRUD/editor stores while remaining in the notes domain.

**Alternatives considered**:
- Fold browsing state into `notes/stores/notes.ts`: rejected because it mixes CRUD cache responsibilities with cross-profile browse concerns.
- Use only local component state: rejected because multiple routes and components need the same global browse state.

## Decision: Respect the existing active API client for normal navigation, but plan profile-scoped loading for global aggregation

**Rationale**: The current shared API client is a mutable singleton tied to the active profile. Global browse views need to load notes without changing the active profile or polluting the main app context. Planning therefore assumes profile-scoped loading helpers for aggregation work while leaving the active profile API context untouched for the rest of the app.

**Alternatives considered**:
- Reuse the singleton API client and mutate active profile context during aggregation: rejected because it risks stale UI and request races.
- Require users to switch profiles before searching: rejected because it breaks the feature goal.

## Decision: Cover the feature with frontend unit tests and focused route/view integration checks

**Rationale**: The constitution requires frontend Vitest tests for composables/stores and Playwright E2E for browser flows. This feature changes route ownership, browse-state derivation, and note card rendering, so tests must cover aggregation behavior, origin labels, favorite persistence, and removal of the sidebar-wide new-note action.

**Alternatives considered**:
- Rely on manual verification only: rejected by the testing rules.
- Limit testing to `NoteCard` rendering only: rejected because the highest risk is route/state behavior, not card markup alone.
