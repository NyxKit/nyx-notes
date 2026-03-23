# Feature Specification: Recent Notes Sidebar (Persistent)

**Feature Branch**: `003-sidebar-recent-notes`
**Created**: 2026-03-23
**Status**: Implemented
**Input**: User description: "the recent notes are currently only listed when the note view is open. we want it to always show recent notes in the sidebar, no matter what view is open."

**Scope note**: The initial implementation makes the existing vault-scoped note list (`NoteList`) persistent across all authenticated views, rather than introducing cross-vault aggregation. Cross-vault filtering was considered but deferred — the existing UI look and feel is preserved.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Access Recent Notes from Any View (Priority: P1)

A user is on the vault overview (`VaultView`) or home dashboard (`HomeView`) and wants to quickly jump to a recently edited note without first navigating into the specific view where the note list was previously available. The sidebar should always show the note list for the active vault, allowing one-click navigation from anywhere.

**Why this priority**: This is the core of the feature. Without it, the feature does not exist.

**Independent Test**: Navigate to the home view or vault overview; verify the sidebar note list is visible and that clicking any note navigates to the correct editor.

**Acceptance Scenarios**:

1. **Given** a user is on the home view, **When** the page loads, **Then** the sidebar displays the note list for the active vault (or an empty state if no vault is active).
2. **Given** a user is on a vault overview, **When** the page loads, **Then** the sidebar note list is still visible.
3. **Given** a user clicks a note in the sidebar, **When** they are on any view, **Then** they are navigated to the correct editor for that note.

---

### User Story 2 - Recent Notes Persist Across Navigation (Priority: P2)

A user navigates between views (home, vault overview, note editor, settings) and expects the sidebar to remain visible and consistent throughout, without disappearing or resetting.

**Why this priority**: The sidebar should feel like a persistent, always-available panel. Disappearing between views would erode trust in the feature.

**Independent Test**: Navigate between all major views in sequence and verify the sidebar note list remains present on each.

**Acceptance Scenarios**:

1. **Given** a user is viewing the sidebar, **When** they navigate to a different view, **Then** the sidebar remains visible with the same list.
2. **Given** a user saves a note (updating its last modified time), **When** they navigate to any other view, **Then** the note list reflects the updated state.

---

### Edge Cases

- What happens when the user has no active vault? The sidebar shows `NoteList` in its natural empty/no-vault state.
- What happens when a vault is deleted that the user was viewing? The note list empties naturally as `activeVault` becomes null.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The note list sidebar MUST be visible on all authenticated views, not only the note editor view.
- **FR-002**: The note list MUST show notes for the currently active vault, sorted by last modified date descending (existing `NoteList` behaviour is preserved unchanged).
- **FR-003**: Clicking any entry in the sidebar MUST navigate the user directly to the editor for that note.
- **FR-004**: The sidebar MUST display the note list in its natural empty state when no vault is active.

### Key Entities

- **`NoteList`**: Existing component — vault-scoped note list, already labelled "Recent Notes" in the UI. No changes to this component.
- **`AppLayout`**: New persistent shell component that owns the sidebar and renders child views via `<RouterView />`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The sidebar note list is present on every authenticated view — verified by visiting each route (home, vault overview, note editor, settings pages).
- **SC-002**: A user can reach the editor for any recent note in exactly one click from any view.
- **SC-003**: The sidebar does not flicker or remount during navigation between authenticated views.

## Assumptions

- The existing `NoteList` component is used as-is — its vault-scoped behaviour, search filter, and "Recent Notes" label are preserved.
- Sidebar visibility on HomeView (when no vault is active) shows an empty or idle `NoteList` state — this is acceptable.
- No new composable or component is introduced; the change is structural (extracting the existing per-view sidebar into a shared `AppLayout`).
- Cross-vault aggregation is explicitly deferred to a future feature iteration.
