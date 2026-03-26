# Data Model: Unify Card Surfaces

## Entity: Browse Card Surface

**Purpose**: Represents a reusable visual and interaction shell for browse-and-select items shown in card grids.

**Fields**:
- `kind`: identifies the card family member (`vault`, `note`, `creation`)
- `title`: primary visible label shown on the card
- `supporting_text`: secondary identifier or metadata shown below or beside the title
- `description`: optional summary text shown beneath primary identity content
- `icon`: optional decorative or contextual iconography
- `primary_action`: the outcome triggered when the card is activated
- `activation_model`: internal anchor for selectable cards, or form controls for form-hosting cards
- `state`: current visual state (`loading`, `default`, `hover`, `focus`, `pressed`, `empty`, `disabled`)
- `content_density`: whether the card shows minimal identity data or richer metadata such as tags and timestamps

**Validation Rules**:
- `title` must remain visible or gracefully truncated at supported viewport sizes.
- `primary_action` must remain available by pointer and keyboard in all interactive states.
- `activation_model` must expose standard link behavior for selectable cards and must not depend on the `NyxCard` container alone for primary selection behavior.
- `state` must never hide required identifying information.

## Entity: Vault Card Surface

**Purpose**: Browse card representing a single vault on the dashboard.

**Fields**:
- `vault_name`
- `vault_slug`
- `vault_description`
- `vault_icon`
- `open_vault_action`
- `layout_variant`: square tile layout used in the vault dashboard grid

**Relationships**:
- Inherits the shared browse-card rules from **Browse Card Surface**.

**Validation Rules**:
- Must preserve the current ability to identify and open a vault.
- Must support missing custom icon data by using the existing fallback icon behavior.
- Must support missing descriptions without collapsing layout incorrectly.

## Entity: Creation Card Surface

**Purpose**: Inline dashboard card that hosts the create-vault form while remaining visually part of the same card family.

**Fields**:
- `form_title`
- `name_input`
- `slug_input`
- `description_input`
- `icon_picker`
- `submit_action`
- `cancel_action`
- `state`: `editing`, `submitting`, `dismissed`

**Relationships**:
- Shares visual rules with **Vault Card Surface**.
- Differs in interaction model because the surface hosts form controls rather than a single open action.

**Validation Rules**:
- Must preserve existing create and cancel outcomes.
- Must not visually imply that clicking the outer surface alone submits or opens anything.

**State Transitions**:
- `editing` -> `submitting` when the user submits a valid form
- `editing` -> `dismissed` when the user cancels
- `submitting` -> `dismissed` after successful creation
- `submitting` -> `editing` if creation fails and the form remains visible

## Entity: Note Card Surface

**Purpose**: Browse card representing a note within a vault's masonry grid.

**Fields**:
- `note_title`
- `note_description`
- `note_tags`
- `updated_label`
- `open_note_action`
- `layout_variant`: variable-height masonry tile

**Relationships**:
- Inherits the shared browse-card rules from **Browse Card Surface**.

**Validation Rules**:
- Must preserve the current ability to open a note from the card surface.
- Must handle empty titles with the existing fallback label.
- Must show the distilled description when one exists and omit it cleanly when no paragraph can be distilled.
- Must support note tags being absent without collapsing spacing incorrectly.

## Entity: Card Family Rules

**Purpose**: Shared constraints that define when a surface belongs to this feature.

**Fields**:
- `surface_role`: browse-and-select, form-hosting, or out-of-scope container
- `scope_membership`: in-scope or out-of-scope
- `typography_rules`
- `spacing_rules`
- `tonal_layering_rules`
- `interaction_rules`
- `responsive_rules`

**Validation Rules**:
- Browse-and-select members must share a recognisable visual family.
- Members with different interaction models must still feel related without becoming behaviorally misleading.
- Out-of-scope surfaces must not be restyled merely because they also use a card-like container.
