# Feature Specification: Global Note Browsing

**Feature Branch**: `012-global-note-browsing`  
**Created**: 2026-04-01  
**Status**: Draft  
**Input**: User description: "let's finalize some loose ends:
- search notes: should search all projects, all vaults, always (= no matter what context the user is in) and report findings in the main window (also a grid of notes, just like vaultview)
- favorite notes: works the same way - project/vault agnostic - show all, always
- make the note card include server & vault => reuse in \"search\", \"favorites\" and the note grid in the single vault view
- \"new note\" in sidebar is obsolete, users should create a new note inside the correct vault"

## Clarifications

### Session 2026-04-01

- Q: What should global search match? → A: Titles, note content, and tags.
- Q: Which servers count as "accessible" for global search and favorites? → A: All configured profiles that are currently reachable and authenticated.
- Q: How should global search results be ordered by default? → A: Most recent first by default, with a sort control that toggles between best match, recent, and grouped by server then vault.
- Q: When some configured profiles are excluded because they are offline or signed out, how should the UI communicate that? → A: Show a non-blocking notice with the excluded profile count.
- Q: How should search react to user input from the sidebar search bar? → A: The search view updates live on keypresses from the sidebar search bar, using debounce or a similar mechanism to avoid excessive refreshes.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Search Across All Notes (Priority: P1)

As a user, I can search my notes from anywhere in the app and always see matching notes from every connected server and every vault in the main content area, so I do not need to switch context before searching.

**Why this priority**: Search is a primary discovery action and loses most of its value if it is limited by the current vault or project context.

**Independent Test**: Can be fully tested by searching while viewing different vaults or screens and confirming that the same cross-context result set appears in the main window.

**Acceptance Scenarios**:

1. **Given** the user is currently inside a specific vault, **When** they run a search, **Then** the results include matches from all accessible servers and vaults rather than only the current vault.
2. **Given** the search returns notes from multiple locations, **When** the results are shown, **Then** they appear in the main window as a note grid and each result identifies its server and vault.
3. **Given** the search returns no matches, **When** the results view loads, **Then** the main window clearly shows an empty state for search rather than leaving the prior vault content visible.
4. **Given** a query matches a note title, body content, or tag, **When** the user searches, **Then** that note appears in the global search results.
5. **Given** some configured profiles are unreachable or not authenticated, **When** the user opens global search or favorites, **Then** only reachable and authenticated profiles contribute notes to the result set.
6. **Given** the user opens global search results, **When** the results first appear, **Then** they are ordered by most recently updated notes first.
7. **Given** the user wants a different result organization, **When** they use the search sort control, **Then** they can switch between best match, most recent, and grouping by server then vault.
8. **Given** one or more configured profiles are excluded from global search or favorites because they are offline or signed out, **When** the user views the results, **Then** the interface shows a non-blocking notice with the count of excluded profiles.
9. **Given** the user types into the sidebar search bar, **When** they press keys, **Then** the search view updates live in the main window using a debounce or similar mechanism that avoids excessive refreshes.

---

### User Story 2 - Browse Favorites Globally (Priority: P2)

As a user, I can open my favorited notes and always see every favorite from every accessible server and vault in one place, so favorites act as a global shortcut list rather than a local vault filter.

**Why this priority**: Favorites are a saved discovery surface and should remain dependable regardless of the current navigation context.

**Independent Test**: Can be fully tested by marking notes as favorites in multiple vaults, navigating to the favorites view from different contexts, and confirming that all favorites appear together in the main window.

**Acceptance Scenarios**:

1. **Given** the user has favorites in more than one vault or server, **When** they open favorites from any context, **Then** the main window shows all favorites in a single grid.
2. **Given** the favorites grid contains notes from multiple locations, **When** it is rendered, **Then** each note card identifies its server and vault.
3. **Given** the user has no favorited notes, **When** they open favorites, **Then** the main window shows a dedicated empty state for favorites.
4. **Given** the user moves between global search and global favorites, **When** each view is shown, **Then** both use the same browse layout and sort-control presentation, with only the note collection and favorites-specific sort options changing.

---

### User Story 3 - Understand Note Origin At A Glance (Priority: P3)

As a user, I can tell which server and vault a note belongs to wherever note grids are shown, so I can confidently open the right note without guessing its origin.

**Why this priority**: Cross-vault search and favorites depend on note cards exposing origin information consistently.

**Independent Test**: Can be fully tested by viewing note grids in search, favorites, and a single-vault note list and confirming that the same note card presentation includes server and vault labels in each place.

**Acceptance Scenarios**:

1. **Given** a note appears in search results, favorites, or a single-vault grid, **When** its card is displayed, **Then** the card includes the note's server and vault.
2. **Given** the user is in the sidebar, **When** they look for a generic "new note" action there, **Then** that action is not shown.
3. **Given** the user wants to create a note, **When** they are outside a specific vault context, **Then** the product does not imply that a note can be created without first choosing the correct vault.

### Edge Cases

- What happens when a search result set includes notes with the same title from different servers or vaults? Each result must still be distinguishable by its server and vault labels.
- How does the system handle servers or vaults with no matching search results or no favorites? The view must still show the complete global result set from other locations.
- What happens when the current context becomes unavailable while viewing global search or favorites? The global view must remain usable and continue to represent all still-accessible notes.
- How does the system handle long server or vault names? Cards must still expose origin information clearly without making the note impossible to identify.
- What happens when a query matches only note body content or tags and not the title? The note must still appear in search results.
- What happens when some configured profiles are offline or signed out? They must be excluded from the global result set until they become reachable and authenticated again.
- What happens when users want a different search-result ordering? They must be able to switch sorting between best match, most recent, and grouped by server then vault without changing the search scope.
- What happens when excluded profiles make results incomplete? The interface must show a non-blocking notice stating how many configured profiles are currently excluded.
- What happens when the user types quickly in the sidebar search bar? The search view must update live without refreshing on every keystroke immediately.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST treat note search and global favorites as context-independent surfaces that operate across all accessible servers and all accessible vaults.
- **FR-001a**: Global note search MUST match note titles, note body content, and tags.
- **FR-001b**: For global search and favorites, "accessible" MUST mean configured profiles that are currently reachable and authenticated.
- **FR-001c**: Changing vault or project context MUST NOT change the scope of global search or global favorites.
- **FR-002**: The system MUST present search results in the main content area rather than limiting them to sidebar-only feedback.
- **FR-003**: The system MUST present search results using a note grid view consistent with the application's vault note browsing experience.
- **FR-003a**: Global search results MUST default to sorting by most recently updated notes first.
- **FR-003b**: Global search MUST provide a sort control that allows switching between best match, most recent, and grouping by server then vault.
- **FR-003c**: Global search MUST update live from the sidebar search bar as the user types, using debounce or a similar mechanism to avoid excessive refreshes.
- **FR-004**: The system MUST treat favorites as a global collection that includes favorited notes from all accessible servers and all accessible vaults, regardless of the user's current navigation context.
- **FR-004a**: When one or more configured profiles are excluded from global search or favorites because they are unreachable or not authenticated, the system MUST show a non-blocking notice with the count of excluded profiles.
- **FR-004b**: Global favorites MUST use the same browse layout and sort-control presentation as global search.
- **FR-004c**: In global favorites, the sort control MUST support recent and grouping by server then vault.
- **FR-004d**: The shared sort control MUST hide best match while viewing global favorites.
- **FR-005**: The system MUST present favorites in the main content area using the same note grid pattern used for search results.
- **FR-006**: The system MUST use a shared note card presentation for search results, favorites, and the single-vault note grid.
- **FR-007**: Each note card in search results, favorites, and the single-vault note grid MUST display the note's server and vault.
- **FR-008**: The system MUST preserve the ability to open a note from any note card shown in these grids.
- **FR-009**: The system MUST provide a dedicated empty state in the main content area when global search returns no results.
- **FR-010**: The system MUST provide a dedicated empty state in the main content area when the global favorites collection is empty.
- **FR-011**: The system MUST remove the obsolete generic "new note" action from the sidebar.
- **FR-012**: The system MUST make note creation available only from within a specific vault context so the destination vault is explicit before creation begins.

### Key Entities *(include if feature involves data)*

- **Note Card**: A reusable note summary shown in grid views, including the note title and its origin context.
- **Origin Context**: The server and vault a note belongs to, used to distinguish similarly named notes across the product.
- **Global Search Result Set**: The collection of note matches returned from all accessible servers and vaults for a search query.
- **Global Favorites Collection**: The collection of all favorited notes gathered from all accessible servers and vaults.

## Assumptions

- Users may have access to multiple servers and multiple vaults at the same time.
- "Project agnostic" and "vault agnostic" both mean the feature ignores the currently viewed location when determining search or favorites scope.
- Only configured profiles that are currently reachable and authenticated participate in global search and global favorites.
- The existing vault note grid is the reference interaction model for how search and favorites results should be presented.
- Search and favorites share the same browse layout and sort-control presentation; favorites only expose the sort modes that are meaningful for a non-search result set.
- Removing the sidebar "new note" action does not remove note creation itself; it only removes a context-free entry point.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In validation tests, 100% of searches started from different vault or project contexts return the same set of eligible matches for the same query.
- **SC-002**: In validation tests, 100% of favorites views opened from different vault or project contexts show the same set of favorited notes.
- **SC-003**: In usability checks, users can identify the originating server and vault for 95% of displayed notes in search, favorites, and single-vault grids without opening the note first.
- **SC-004**: In task-based testing, 90% of users can locate a note stored outside their current vault using global search in under 30 seconds.
- **SC-005**: In navigation checks, the sidebar no longer presents a context-free note creation action in any standard app state.
- **SC-006**: In validation tests, search results in the main window react to sidebar search-bar input during typing without requiring an explicit submit action.
