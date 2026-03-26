# Feature Specification: Unify Card Surfaces

**Feature Branch**: `[008-adopt-nyxcard-cards]`  
**Created**: 2026-03-26  
**Status**: Draft  
**Input**: User description: "i want to use NyxCard for VaultCard instead (and other possibly card-related stuff)"

## Clarifications

### Session 2026-03-26

- Q: Which card surfaces are in scope for this feature? → A: VaultCard, inline create-vault card, and note cards are in scope.
- Q: How should interactive behavior work when NyxCard is adopted? → A: `VaultCard` and `NoteCard` wrap their content in an internal `RouterLink` anchor so users retain standard link behavior.
- Q: Which surfaces should be explicitly out of scope? → A: VaultSwitcher, empty-state cards, comment threads, settings/modals, and navigation chrome are out of scope.
- Product direction update: Do not introduce a shared `BrowseCardSurface` abstraction; `VaultCard` and `NoteCard` remain standalone card components.
- Product direction update: `VaultCard` and `NoteCard` wrap their content in an internal `RouterLink` anchor so users keep the default link context menu and open-in-new-tab behavior.
- Product direction update: Vaults gain an optional user-authored description; notes gain an optional distilled description derived from the first paragraph of Markdown content on every save.
- Product direction update: `VaultCard` icons should be visually larger than before, but only overflow the card by roughly 10% on the bottom and right edges.
- Product direction update: Existing vaults must expose their description in vault settings so users can edit it after creation.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Browse Vaults Consistently (Priority: P1)

As a user landing on the vault dashboard, I want each vault to appear inside the same reusable card pattern with link-native behavior and richer summary content so the grid feels cohesive, predictable, and easier to scan.

**Why this priority**: Vault browsing is the primary card-based experience in the current product, and inconsistent vault cards make the dashboard feel less polished and harder to maintain.

**Independent Test**: Can be fully tested by opening the vault dashboard with multiple vaults and confirming every vault tile uses the same card treatment, shows title, slug, and description in the top-left, places a larger icon in the bottom-right with only slight overflow, and still behaves like a normal link to the selected vault.

**Acceptance Scenarios**:

1. **Given** a user has multiple vaults, **When** they open the vault dashboard, **Then** each vault is presented using the same reusable card surface and shows the vault title, slug, and description aligned to the top-left.
2. **Given** a vault has an icon, **When** its card is shown, **Then** the icon appears as a large decorative element in the bottom-right and overflows the card only slightly, at roughly 10% on the right and bottom edges.
3. **Given** a user interacts with a vault card, **When** they use the browser's default link affordances, **Then** the card behaves like a normal anchor that supports opening in a new tab, copying the link, and standard link context-menu behavior.

---

### User Story 2 - Create Vaults Inside the Same Card Language (Priority: P2)

As a user creating a new vault from the dashboard, I want the inline creation experience to feel like part of the same card family as the existing vault cards so the creation flow feels intentional rather than bolted on.

**Why this priority**: The creation card appears directly beside vault cards, so visual mismatch in this surface is immediately noticeable and weakens the dashboard experience.

**Independent Test**: Can be fully tested by opening the inline creation flow and confirming the form surface visually belongs to the same card family while retaining create, cancel, validation, and vault-description entry behavior.

**Acceptance Scenarios**:

1. **Given** the user starts the inline vault creation flow, **When** the create form appears in the dashboard grid, **Then** it uses the same card language as the browsing cards while preserving name, slug, description, icon, create, and cancel controls.
2. **Given** the user cancels or completes vault creation, **When** the grid updates, **Then** the dashboard returns to the same consistent card presentation without broken spacing or layout shifts beyond the expected card insertion or removal.
3. **Given** a user opens vault settings for an existing vault, **When** they edit the description and save, **Then** the updated description is persisted and shown on the vault card.

---

### User Story 3 - Reuse the Card Pattern in Similar Surfaces (Priority: P3)

As a user moving between card-based browsing surfaces, I want visually similar cards to follow the same shared pattern so the product feels coherent across related screens.

**Why this priority**: The user request explicitly calls out other card-related surfaces, but these are secondary to the vault dashboard and should only be aligned where they serve a similar browse-and-select role.

**Independent Test**: Can be fully tested by reviewing the dedicated `NoteCard` in the vault notes view and confirming it adopts the shared card pattern, behaves like a normal link, and shows the first actual paragraph from the note when one exists.

**Acceptance Scenarios**:

1. **Given** notes are displayed in the vault notes view, **When** the list is shown, **Then** each note is rendered by a dedicated `NoteCard` component that follows the shared card pattern while keeping note-specific content intact.
2. **Given** a surface is not part of the browse-and-select card family, **When** the feature is delivered, **Then** that surface remains unchanged by this effort.
3. **Given** a note is created or saved, **When** its content contains at least one paragraph, **Then** the note card description is distilled from the first actual paragraph rather than from headings, lists, or other non-paragraph blocks.
4. **Given** a note already exists and contains a valid paragraph, **When** the notes view is loaded, **Then** the note card shows that first-paragraph description instead of appearing blank.

---

### Edge Cases

- What happens when a vault has a very long name or slug that would normally overflow the card?
- What happens when a vault has no description?
- How does the system handle empty, loading, or skeleton states so they still feel part of the same card family?
- What happens when the dashboard is viewed on narrow screens where card width and spacing change significantly?
- How does the system behave when an in-scope card surface contains actions, badges, or icons that do not appear on every other card surface?
- What happens when a note contains no actual paragraph that can be distilled into a description?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The vault dashboard MUST present each existing vault within a shared card pattern that preserves the current ability to recognize and open a vault.
- **FR-002**: `VaultCard` MUST support the vault title, slug, optional description, and decorative iconography.
- **FR-003**: In `VaultCard`, the title, slug, and description MUST be aligned to the top-left of the card.
- **FR-004**: In `VaultCard`, the icon MUST be rendered as a large decorative element anchored to the bottom-right and allowed to overflow only slightly beyond the right and bottom edges of the card, at roughly 10% on each edge.
- **FR-005**: The inline vault creation surface MUST use the same card family as the vault browsing cards while preserving current create and cancel actions and adding support for editing the vault description.
- **FR-005a**: Existing vaults MUST expose the description field in vault settings so users can update or clear it after creation.
- **FR-006**: The system MUST introduce a dedicated `NoteCard` component for note cards in the vault notes view instead of keeping note-card markup inline in `VaultView`.
- **FR-007**: `VaultCard` and `NoteCard` MUST each wrap their content in an internal `RouterLink` anchor so users retain standard link behavior such as opening in a new tab and copying the destination link.
- **FR-008**: The system MUST NOT introduce a shared `BrowseCardSurface` abstraction for this feature; `VaultCard` and `NoteCard` remain standalone components.
- **FR-009**: `NoteCard` MUST support the note title, a distilled description, and note-specific supporting metadata.
- **FR-010**: The note description MUST be distilled on every note save from the first actual paragraph in the Markdown content and MUST ignore headings, lists, and other non-paragraph blocks.
- **FR-010a**: When a note already contains a valid first paragraph, the note-card description shown in the notes view MUST reflect that paragraph.
- **FR-011**: The user MUST NOT be prompted to manually enter or edit a note description in the UI.
- **FR-012**: The system MUST apply the shared card pattern only to the in-scope surfaces and MUST NOT force it onto out-of-scope surfaces, including the VaultSwitcher, empty-state cards, comment threads, settings/modals, and navigation chrome.
- **FR-013**: Users MUST be able to interact with all affected cards by pointer and keyboard using the same outcomes they have today.
- **FR-014**: Affected card surfaces MUST remain legible and visually consistent across loading, empty, populated, hover, focus, and pressed states.
- **FR-015**: The resulting card presentation MUST remain aligned with the project's documented visual design language for typography, spacing, tonal layering, and emphasis.
- **FR-016**: The card pattern MUST accommodate variable content lengths and different viewport sizes without obscuring key information or breaking the surrounding layout.

### Key Entities *(include if feature involves data)*

- **Vault Card Surface**: A selectable dashboard item representing a single vault, including its title, slug, optional description, decorative icon, and link destination.
- **Creation Card Surface**: The inline card-form surface used to create a new vault from within the dashboard grid.
- **Note Card Surface**: A selectable note item rendered by a dedicated `NoteCard` component, including the note title, a distilled description, supporting metadata, and link destination.
- **Card Family**: A shared visual and interaction pattern used by browse-and-select surfaces that should feel related across the product.
- **Card State**: The visible condition of a card surface, such as loading, empty, default, hover, focus, pressed, or disabled.

### Assumptions

- The in-scope surfaces for this feature are the vault dashboard cards, the inline create-vault card, and the note cards in the vault notes view.
- The out-of-scope surfaces for this feature are the VaultSwitcher, empty-state cards, comment threads, settings/modals, and navigation chrome.
- Vault descriptions are user-authored and optional.
- Note descriptions are stored as distilled summaries generated from saved note content rather than as user-authored fields.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In design and product review, 100% of in-scope card surfaces are judged to follow a single shared card family with no unresolved visual inconsistency issues.
- **SC-002**: In usability verification of the vault dashboard, 90% or more of participants can identify the vault destination as a standard link target on first attempt without guidance.
- **SC-003**: In keyboard-access verification, 100% of affected card surfaces can be reached and activated using the same successful outcomes as pointer interaction.
- **SC-004**: On supported desktop and mobile layouts, all required vault and note card content remains visible or gracefully truncated without overlap in 100% of reviewed states.
- **SC-005**: In review of saved note content, 100% of note-card descriptions are derived from the first actual paragraph when one exists and never from a heading or list item.
