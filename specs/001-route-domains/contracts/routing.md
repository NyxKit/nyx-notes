# Contract: Frontend Routing

**Feature**: 001-route-domains
**Type**: UI routing contract (URL schema)

## Route Table

| Path | Component | Named Route | Auth | Notes |
|------|-----------|-------------|------|-------|
| `/` | `HomeView.vue` | `home` | required | Redirects to `/vaults/:id` if exactly 1 vault |
| `/vaults/:vault_id` | `VaultView.vue` | `vault` | required | New view — notes masonry or getting-started |
| `/vaults/:vault_id/notes/:id?` | `NoteView.vue` | — | required | Existing; unchanged |
| `/vaults/:vault_id/favorites` | `NoteView.vue` | — | required | Existing; unchanged |
| `/vaults/:vault_id/drafts` | `NoteView.vue` | — | required | Existing; unchanged |
| `/vaults/:vault_id/settings` | `VaultSettingsView.vue` | — | required | Existing; unchanged |
| `/teams/:team_id/settings` | `TeamSettingsView.vue` | — | required | Existing; unchanged |
| `/login` | `LoginView.vue` | — | public | Existing; unchanged |

## Navigation Invariants

1. **Single-vault redirect**: When `HomeView` mounts and the vault list resolves to exactly one vault, the router performs `replace('/vaults/:vault_id')`. The `/` entry is removed from the history stack.

2. **Multi-vault home**: When two or more vaults exist, `/` renders the vault masonry. No automatic redirect.

3. **Auth guard**: All routes except `/login` carry `meta: { requiresAuth: true }`. The existing `beforeEach` guard handles redirection to `/login` unchanged.

4. **Vault card navigation**: Clicking a vault card on `/` pushes `/vaults/:vault_id`.

5. **Note card navigation**: Clicking a note card on `/vaults/:vault_id` pushes `/vaults/:vault_id/notes/:id`.

## No API Contract Changes

This feature introduces no new backend routes. All data is fetched via existing endpoints.
