# Data Model: Global Note Browsing

## BrowseNoteCardModel

Represents the full card data needed by search results, favorites, and single-vault note grids.

### Fields

- `note_id`: Canonical note identifier
- `vault_id`: Canonical vault identifier
- `profile_id`: Workspace profile identifier for the source server context
- `title`: Display title for the note
- `description`: Distilled description from saved content, if available
- `tags`: Up to the full tag set supplied by note metadata
- `updated_at`: Canonical last-updated timestamp
- `updated_label`: User-facing relative or formatted time label
- `href`: Canonical application route to open the note
- `server_label`: Display label for the source server or local workspace
- `server_id`: Optional discovered server identifier
- `vault_name`: Display name for the source vault
- `vault_slug`: Stable vault slug for secondary identification
- `is_favorite`: Whether the note belongs to the persisted favorites collection

### Relationships

- Derived from `NoteMeta` plus workspace profile data and vault metadata
- Consumed by `NoteCard` in all browse grids

### Validation Rules

- `profile_id`, `vault_id`, and `note_id` must be present for every globally browsed note
- `href` must open the exact note represented by the card
- `server_label` and `vault_name` must always be present in rendered card data, with safe fallback values when discovery data is incomplete

## NoteOriginContext

Represents where a note comes from in user-facing terms.

### Fields

- `profile_id`: Workspace profile containing the note
- `server_label`: Human-readable server or workspace name
- `server_id`: Optional server identifier from auth discovery
- `vault_id`: Canonical vault identifier
- `vault_name`: Human-readable vault name
- `vault_slug`: Stable vault slug

### Relationships

- Composed into `BrowseNoteCardModel`
- Derived from the workspace-profile layer plus vault metadata

### Validation Rules

- Must distinguish notes with identical titles across locations
- Must remain renderable even when optional `server_id` is unavailable

## FavoriteNoteRef

Represents a persisted favorite without duplicating mutable note content.

### Fields

- `profile_id`: Workspace profile containing the note
- `vault_id`: Vault containing the note
- `note_id`: Favorited note identifier
- `created_at`: Timestamp when the note was favorited

### Relationships

- Joins to `BrowseNoteCardModel` during favorites view assembly
- Replaces the current bare note-ID favorite storage model

### Validation Rules

- The combination of `profile_id`, `vault_id`, and `note_id` must be unique
- Missing notes should not break the favorites view; orphaned references should be skipped or surfaced for cleanup

## GlobalSearchState

Represents the route-driven search state.

### Fields

- `query`: Current search text
- `results`: Aggregated `BrowseNoteCardModel[]`
- `is_loading`: Whether one or more profile/vault searches are in progress
- `error`: User-facing load error, if any
- `searched_at`: Timestamp of the latest completed search

### State Transitions

1. `idle` -> `loading` when the user submits or updates a global query
2. `loading` -> `loaded` when aggregated results finish successfully
3. `loading` -> `empty` when no eligible matches are found
4. `loading` -> `error` when all result loading fails or a fatal aggregation error occurs

## GlobalFavoritesState

Represents the aggregated favorites surface.

### Fields

- `favorite_refs`: Persisted `FavoriteNoteRef[]`
- `results`: Resolved `BrowseNoteCardModel[]`
- `is_loading`: Whether favorite resolution is in progress
- `error`: User-facing load error, if any

### State Transitions

1. `idle` -> `loading` when the favorites view opens or favorites change
2. `loading` -> `loaded` when favorite references are resolved into cards
3. `loading` -> `empty` when no favorite references remain
4. `loading` -> `error` when resolution fails in a way the UI cannot recover from
