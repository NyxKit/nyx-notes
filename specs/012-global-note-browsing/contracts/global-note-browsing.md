# Contract: Global Note Browsing

## Purpose

Defines the user-facing route and view contract for global search, global favorites, and the reusable note card across browse surfaces.

## Routes

### `GET /notes/search`

Main browse route for global note search.

#### Query Parameters

- `q`: Search query string

#### View Contract

- The page is available from any authenticated application context
- Results are aggregated across all accessible workspace profiles and all accessible vaults
- Results render in the main content area using the shared global browse layout and note browse grid pattern
- Each result card shows note title, supporting metadata, server label, and vault label
- Results default to most recent ordering and expose the shared sort control for best match, most recent, and grouped by server then vault
- The view updates live from the sidebar search bar as the user types, using debounce or a similar mechanism to avoid excessive refreshes
- If `q` is empty, the page shows the search surface without stale results from a previous vault page
- If no matches are found, the page shows a dedicated search empty state

### `GET /notes/favorites`

Main browse route for global favorites.

#### View Contract

- The page is available from any authenticated application context
- Results are aggregated across all persisted favorites from all accessible workspace profiles and vaults
- Results render in the main content area using the same shared global browse layout, note browse grid pattern, and sort controls as global search
- Each result card shows note title, supporting metadata, server label, and vault label
- If no favorites exist, the page shows a dedicated favorites empty state

### Legacy Route Behavior

- `GET /vaults/:vault_id/favorites` redirects to `/notes/favorites`

## Note Card Contract

The same note card family is used in:

- global search results
- global favorites
- single-vault note browse grid

### Required Card Content

- note title
- note description when available
- updated label
- up to the supported browse-card metadata set
- source server label
- source vault label
- link target that opens the represented note directly

### Interaction Contract

- The whole card remains a navigable link target
- The card must preserve standard browser link affordances
- Visual treatment remains within the browse-card family defined by the design docs

## Sidebar Navigation Contract

- The sidebar no longer exposes a context-free `New Note` action
- Favorites navigation points to the global favorites route, not a vault-scoped route
- Search interactions route results into the main window rather than leaving them isolated in the sidebar
