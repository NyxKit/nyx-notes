# Feature Specification: Replace Native Primitives with nyx-kit Components

**Feature Branch**: `002-nyx-kit-primitives`
**Created**: 2026-03-21
**Status**: Draft

## Overview

The frontend codebase contains a large number of hand-rolled HTML primitives — `<button>`, `<input>`, `<textarea>`, inline tag/badge spans, and card-like elements — that duplicate behaviour already provided by the `nyx-kit` component library. This violates the constitution principle "Use nyx-kit for all UI primitives" and creates maintenance risk: styles drift, behaviour diverges, and every bespoke element carries its own CSS.

This feature replaces every qualifying native primitive with its nyx-kit equivalent across all frontend components and views.

---

## User Scenarios & Testing

### User Story 1 — Interactive Controls Use nyx-kit (Priority: P1)

Every `<button>`, `<input>`, `<textarea>`, and `<select>` in the frontend is replaced by the corresponding nyx-kit component (`NyxButton`, `NyxInput`, `NyxTextarea`, `NyxSelect`). Custom CSS for those elements is removed.

**Why this priority**: Buttons and inputs are the most numerous and most visible native primitives. They appear in every view. Fixing them first eliminates the largest class of violations and establishes the correct pattern for all future work.

**Independent Test**: Open every view in the app. No native `<button>` or `<input>` element should be visible in the rendered DOM (excluding those inside third-party library output). All interactive controls should visually match the nyx-kit design system.

**Acceptance Scenarios**:

1. **Given** any view with a call-to-action button, **When** the DOM is inspected, **Then** no raw `<button>` element exists — only nyx-kit button components.
2. **Given** any form (vault creation, team member add, comment composer), **When** the form fields are inspected, **Then** all text inputs and textareas are nyx-kit components.
3. **Given** a replaced control, **When** interacted with, **Then** all existing behaviour (click handlers, v-model bindings, disabled states, loading states) is preserved exactly.

---

### User Story 2 — Labels, Tags, and Badges Use nyx-kit (Priority: P2)

All inline tag/label/badge elements currently rendered as hand-styled `<span>` elements are replaced with `NyxBadge`.

**Why this priority**: Tags appear on note cards (vault view), the note toolbar, and comment threads. They are low-risk to replace and high-value for visual consistency.

**Independent Test**: Open the vault notes masonry and the note editor toolbar. Tag chips should render as `NyxBadge` components. Inspect the DOM — no hand-rolled tag span should exist.

**Acceptance Scenarios**:

1. **Given** a note with tags in the vault masonry, **When** the cards render, **Then** each tag is a `NyxBadge` component, not a plain `<span>`.
2. **Given** the note toolbar with tags, **When** tags are displayed, **Then** they render using `NyxBadge`.
3. **Given** replaced badges, **When** displayed, **Then** visual appearance matches or improves on the previous hand-rolled style.

---

### User Story 3 — Forms Wrapped in NyxForm (Priority: P3)

All form containers (vault creation, add team member, add vault to team, comment submission) use `NyxForm` and `NyxFormField` wrappers, consistent with the existing `LoginView` pattern.

**Why this priority**: Form wrapping is structural rather than visual. `LoginView` already uses this pattern correctly; other forms do not. Fixing this ensures consistent submit-prevention behaviour and label association across the app.

**Independent Test**: Open the vault creation form (HomeView), the add-member form (TeamSettingsView), and the comment composer. Each should be wrapped in a `NyxForm` with fields wrapped in `NyxFormField`.

**Acceptance Scenarios**:

1. **Given** any create/edit form in the app, **When** the component tree is inspected, **Then** the form root is a `NyxForm` component.
2. **Given** a form field with a label, **When** inspected, **Then** the label is associated via `NyxFormField`.
3. **Given** a form submission, **When** the user submits, **Then** default browser form submission is prevented and the existing handler fires.

---

### Edge Cases

- A replaced component must not alter data flow: `v-model`, `@click`, `@submit`, `:disabled`, and all other bindings must work identically after replacement.
- `NyxButton` used as a card/tile (vault cards, note cards) must preserve the full clickable area and existing navigation behaviour.
- Disabled states on buttons during async operations (e.g., "Creating…" state) must remain functional via nyx-kit's `disabled` prop.
- Tags/badges that were previously truncated or clamped must continue to truncate correctly.
- The comment tab switcher in `CommentSidebar` should be replaced with `NyxTabs` if the component supports non-routed tab state.

---

## Requirements

### Functional Requirements

- **FR-001**: Every `<button>` element in the frontend MUST be replaced with `NyxButton`, preserving all variants (primary, ghost, danger), sizes, disabled states, and click handlers.
- **FR-002**: Every `<input>` (text and related types) MUST be replaced with `NyxInput`, preserving `v-model`, placeholder, type, and disabled attributes.
- **FR-003**: Every `<textarea>` MUST be replaced with `NyxTextarea`, preserving `v-model`, placeholder, and resize behaviour.
- **FR-004**: Every hand-rolled tag/badge `<span>` MUST be replaced with `NyxBadge`, preserving label text and any close/dismiss behaviour.
- **FR-005**: Every qualifying form container MUST be wrapped in `NyxForm`; labelled fields MUST use `NyxFormField`.
- **FR-006**: No hand-rolled scoped CSS for replaced primitives may remain; only structural or layout styles that nyx-kit does not cover are permitted.
- **FR-007**: The comment tab switcher in `CommentSidebar` MUST be replaced with `NyxTabs`.
- **FR-008**: All replacements MUST use the nyx-kit component directly — no wrapper elements that reapply the component's own styling.

### Key Entities

- **nyx-kit component inventory**: Available replacements confirmed in the installed package: `NyxButton`, `NyxInput`, `NyxTextarea`, `NyxSelect`, `NyxForm`, `NyxFormField`, `NyxBadge`, `NyxCard`, `NyxTabs`, `NyxSpinner`, `NyxModal`, `NyxCheckbox`, `NyxSwitch`, `NyxTooltip`.
- **Affected files**: `HomeView.vue`, `VaultView.vue`, `SidebarNav.vue`, `CommentComposer.vue`, `CommentSidebar.vue`, `CommentThread.vue`, `TeamSettingsView.vue`, `VaultSettingsView.vue`, `NoteList.vue`.
- **Reference implementation**: `LoginView.vue` already uses `NyxForm`, `NyxFormField`, and `NyxInput` correctly — it is the target pattern for all form replacements.

---

## Success Criteria

### Measurable Outcomes

- **SC-001**: Zero native `<button>` elements remain in any `.vue` file template across the frontend.
- **SC-002**: Zero native `<input>` or `<textarea>` elements remain in any `.vue` file template.
- **SC-003**: Zero hand-rolled tag/badge `<span>` elements remain — all replaced by `NyxBadge`.
- **SC-004**: All replaced controls pass a manual regression check: interactions, disabled states, and data bindings behave identically to before the refactor.
- **SC-005**: No new scoped CSS is introduced to style primitives that nyx-kit components already style internally.

---

## Assumptions

- `NyxButton` supports a danger theme and ghost variant sufficient to cover the delete and cancel button styles currently in use.
- `NyxBadge` supports the close/dismiss pattern used on note tags in the toolbar.
- `NyxTabs` can manage active state without a router — suitable for the comment sidebar's open/resolved tab toggle.
- Note and vault selection "cards" (currently `<button>` with card styling) are best served by `NyxButton` with an appropriate variant rather than `NyxCard`, since they are primarily navigation actions.
- `NyxSpinner` is available for replacing any inline loading text indicators.
