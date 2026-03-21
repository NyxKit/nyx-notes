# Feature Spec: Route Domains

**Branch**: `001-route-domains` | **Date**: 2026-03-21

## Overview

Establish three clear, purpose-driven route domains for the frontend SPA. Every view shares a persistent left sidebar for navigation.

| Domain | Route Pattern | Purpose |
|--------|--------------|---------|
| **Home** | `/` | Vault management dashboard |
| **Vault** | `/vaults/:vault_id` | Notes masonry for one vault |
| **Note** | `/vaults/:vault_id/notes/:id?` | Note editor |

## Shared Layout

All authenticated views use the `app-shell` layout:

- **Left sidebar** (always visible on Home and Vault; toggleable on Note): contains `VaultSwitcher`, `SidebarNav`, and a settings footer link.
- **Main column**: header bar + scrollable body + footer status bar.

### SidebarNav

The sidebar navigation contains two sections:

**App section** (always visible):
- **Vaults** → `/` — active when on the Home view.

**Workspace section** (visible only when a vault is active):
- **All Notes** → `/vaults/:vault_id` — navigates to the Vault view; active when on the Vault view.
- **Favorites** → `/vaults/:vault_id/favorites`
- **Drafts** → `/vaults/:vault_id/drafts`

### VaultSwitcher

Accepts a `dest` prop (`'notes'` | `'vault'`, default `'notes'`):
- `dest="vault"` — selecting a vault navigates to `/vaults/:vault_id` (used in Home and Vault views).
- `dest="notes"` — selecting a vault navigates to `/vaults/:vault_id/notes` (used in Note view, existing behaviour).

## Behavioural Requirements

### Home (`/`)

1. On load, fetch all vaults for the current user.
2. **Initial page load with exactly one vault** → redirect immediately to `/vaults/:vault_id` using `router.replace`. Detection: `window.history.state.back === null` (no prior navigation history entry). This redirect does **not** fire when the user navigates intentionally back to `/` from within the app.
3. **Any other case** → render a masonry of vault cards; each card navigates to `/vaults/:vault_id` on click.
4. **Vault creation** → "New Vault" button in the header reveals an inline create-vault form (name + slug fields) as an additional masonry card. On success, redirect to the new vault.
5. Show a loading skeleton while vaults are fetching.
6. Sidebar is always open. `VaultSwitcher` uses `dest="vault"`.

### Vault (`/vaults/:vault_id`)

1. Fetch all notes in the vault (`GET /api/vaults/:vault_id/notes`).
2. **If notes exist** → render a masonry of note cards, sorted by `updated_at` descending. Each card navigates to `/vaults/:vault_id/notes/:id`.
3. **If no notes exist** → render a getting-started prompt with a "New Note" CTA.
4. Header shows vault name and a "New Note" button (visible when notes exist).
5. Show a loading skeleton while notes are fetching.
6. Sidebar is always open. `VaultSwitcher` uses `dest="vault"`.

### Note (`/vaults/:vault_id/notes/:id?`)

No changes to `NoteView.vue` content or editor behaviour. Sidebar remains toggleable as before. `VaultSwitcher` uses default `dest="notes"`.

## Non-Goals

- No changes to the note editor (`NoteView.vue`) content or saving logic.
- No changes to authentication, settings views, or team/comment features.
- No server-side routing changes; purely frontend SPA routing.

## Acceptance Criteria

- [ ] On initial app load with one vault, `/` redirects to `/vaults/:vault_id` automatically.
- [ ] Navigating back to `/` from within the app (e.g. via "Vaults" sidebar link) shows the vault management page — no redirect fires.
- [ ] `/` shows a masonry of vault cards when multiple vaults exist.
- [ ] `/` "New Vault" button opens an inline create form; submitting creates the vault and redirects to it.
- [ ] `/vaults/:vault_id` with notes shows a note masonry sorted by `updated_at` desc.
- [ ] `/vaults/:vault_id` with no notes shows getting-started prompt with "New Note" CTA.
- [ ] Sidebar "All Notes" navigates to `/vaults/:vault_id` (Vault view), not to the note editor.
- [ ] Sidebar "Vaults" navigates to `/` and is active when on the Home view.
- [ ] Sidebar "Vaults" is always visible; "Workspace" section (All Notes, Favorites, Drafts) only renders when a vault is active.
- [ ] Switching vault via `VaultSwitcher` on Home or Vault view navigates to `/vaults/:new_vault_id`.
- [ ] Auth guard (`requiresAuth: true`) is applied to both `/` and `/vaults/:vault_id`.
- [ ] All existing routes (`/vaults/:vault_id/notes/:id`, `/login`, settings) remain functional.
- [ ] `docs/interface/frontend.md` routing table reflects the new route structure.
