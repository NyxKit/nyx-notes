# Feature Specification: Frontend Domain-Based Structure

**Feature Branch**: `007-frontend-domain-structure`
**Created**: 2026-03-25
**Status**: Draft
**Input**: User description: "move the frontend src directory to a domain-based structure. Use domains like 'vaults', 'notes', 'api', perhaps even 'comments' (or find a better suiting name) and find a place to store common/shared logic"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Find All Domain Code in One Place (Priority: P1)

A developer working on vault functionality opens the project and immediately knows where to look. All vault-related views, components, state, and data access live together under a single `vaults/` domain folder. No cross-referencing between unrelated directories is needed.

**Why this priority**: The primary value of domain-based structure is locality — reducing the number of directories a developer must navigate to understand or modify a domain. This is the foundational benefit everything else builds on.

**Independent Test**: A developer unfamiliar with the project can locate every file relevant to "vault listing" or "note editing" without reading an index file or grep-ing the codebase.

**Acceptance Scenarios**:

1. **Given** a developer needs to change how vaults are displayed, **When** they open the `vaults/` domain folder, **Then** they find the view, components, store, and API module for vaults all in that folder — nothing is spread across `components/`, `stores/`, and `views/` simultaneously.
2. **Given** a developer adds a new vault feature, **When** they decide where to put new files, **Then** the domain structure makes the correct location unambiguous.
3. **Given** a developer is reviewing the notes domain, **When** they look at its folder, **Then** they find only note-related files — no unrelated components or stores leak in.

---

### User Story 2 - Find Shared and Cross-Cutting Code (Priority: P2)

A developer needs to use a shared utility, the HTTP client, or a layout component. There is a clearly named `shared/` folder that holds everything used by more than one domain: layout, routing, global types, utilities, and the base HTTP client.

**Why this priority**: Without a defined home for shared code, it gravitates back into a flat pile, undoing the domain structure over time. The shared boundary must be explicit.

**Independent Test**: A developer can identify within 30 seconds whether a given file belongs to a domain or to the shared layer, and knows exactly where to add a new shared utility.

**Acceptance Scenarios**:

1. **Given** a developer needs the HTTP client used by all API modules, **When** they look in the shared folder, **Then** they find it there alongside other cross-domain concerns.
2. **Given** a developer creates a new utility function used by both vaults and notes, **When** they decide where to put it, **Then** the shared folder is the obvious and only correct location.
3. **Given** a developer looks at the shared folder, **When** they review its contents, **Then** every file there is genuinely used by more than one domain.

---

### User Story 3 - Remove a Domain Without Side Effects (Priority: P3)

A developer can reason about the blast radius of removing or significantly changing a domain. Because domain code is colocated, the scope of a change is contained within the domain folder (plus any imports from shared).

**Why this priority**: This is the longer-term payoff of domain structure — changeability and isolation. Less immediately testable than stories 1 and 2, but represents the architectural goal.

**Independent Test**: Removing or stubbing out the `comments/` domain folder does not require changes to files inside the `vaults/` or `notes/` domain folders — only the router and cross-domain entry points need updating.

**Acceptance Scenarios**:

1. **Given** the comments domain is removed, **When** a developer checks what else needs to change, **Then** the only non-comments files that need updating are in the shared layer (e.g., router) — not inside other domain folders.
2. **Given** a new domain is added (e.g., `teams/`), **When** a developer creates its folder, **Then** they can model it directly on the structure of existing domains without special-casing.

---

### Edge Cases

- What is the correct domain for a component used by exactly two domains? → It belongs in `shared/`.
- What happens to files that are not clearly domain-specific (global types, router, app entry point)? → These live in `shared/` or remain at the `src/` root if they are true entry points (`main.ts`, `App.vue`).
- How are assets (icons, CSS) organized — per-domain or shared? → Assets belonging to a specific domain (e.g., vault icon SVGs) move with that domain; global assets (theme CSS) stay in `shared/`.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The `src/` directory MUST be reorganized into domain folders: `vaults/`, `notes/`, `comments/`, `auth/`, and `teams/`.
- **FR-002**: Each domain folder MUST contain all files exclusively belonging to that domain: views, components, store, and composables.
- **FR-003**: A `shared/` folder MUST exist to hold files used by more than one domain: the HTTP client, global types, layout components, router, utilities, and global assets.
- **FR-004**: The application MUST continue to build and run correctly after the restructure — no broken imports, no missing routes, no runtime errors.
- **FR-005**: The router MUST remain functional and all existing routes MUST resolve to the correct views after the move.
- **FR-006**: Path aliases MUST continue to work or be updated consistently so that all imports resolve correctly across the new structure.
- **FR-007**: Each domain's API module MUST be colocated inside that domain under an `api/` subfolder; the base HTTP client moves to `shared/`.
- **FR-008**: Every domain folder MUST follow a consistent internal layout with the following subdirectories: `api/`, `classes/`, `components/`, `composables/`, `stores/`, `types/`, `utils/`, `views/`. Subdirectories that have no files for a given domain are omitted — the structure is additive, not enforced wholesale. Note: `classes/` has no instances in the current codebase and will be introduced in a subsequent feature; no action is required for it in this refactor.

### Key Entities

- **Domain**: A self-contained folder grouping all views, components, state, data access, and composables for one business concept (vaults, notes, comments, auth, teams).
- **Shared layer**: A folder holding code with no single-domain owner — the HTTP client, global type definitions, layout shell, router, and utility functions.
- **Entry points**: Files that remain at the `src/` root — `main.ts`, `App.vue`, `vite-env.d.ts` — as they bootstrap the application rather than belonging to any domain.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every domain-specific file is located inside its domain folder — zero vault components remain in a flat `components/` directory, zero note stores remain in a flat `stores/` directory.
- **SC-002**: The application builds without errors and all routes work correctly after the restructure.
- **SC-003**: A developer can identify the correct folder for any new file (domain or shared) without ambiguity — the structure is self-documenting.
- **SC-004**: The shared folder contains only genuinely cross-domain files — no domain-specific file is placed there as a shortcut.
- **SC-005**: The number of top-level folders inside `src/` is reduced from the current flat layout (components, composables, stores, views, api, utils, types) to domain folders plus shared, reducing navigational noise.

## Assumptions

- `App.vue`, `main.ts`, and `vite-env.d.ts` remain at the `src/` root as application entry points.
- The `router/` moves to `shared/` since routing references all domains.
- `types/index.ts` moves to `shared/` as it defines cross-domain types.
- `utils/time.ts` moves to `shared/` as it is a generic, domain-agnostic utility.
- The `comments/` domain name is retained — it is clear, consistent with the backend, and matches existing code naming.
- Each domain folder uses a consistent set of subdirectories mirroring the top-level structure that currently exists globally: `api/`, `classes/`, `components/`, `composables/`, `stores/`, `types/`, `utils/`, `views/`. Only subdirectories with actual files are created for a given domain.
- Vault-specific icon SVGs (currently in `assets/icons/vaults/`) move with the `vaults/` domain; the generic icon outline SVGs and global `theme.css` move to `shared/assets/`.
