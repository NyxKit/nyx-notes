# Data Model: Line-Based Comments

## Overview

The feature extends the existing note comment model from quote-only anchors to structured line anchors that preserve the exact selected text for new comments, display the containing line in the sidebar, and support hidden legacy comments that remain stored but are not shown in the default line-based discussion UI.

## Entities

### CommentThread

- Purpose: Represents one discussion thread attached to an exact text selection within a note line.
- Fields:
  - `id`: stable thread identifier
  - `note_id`: parent note identifier
  - `author_id`: creator user identifier
  - `author_name`: creator display name
  - `body`: root discussion message
  - `anchor`: structured `CommentAnchor`
  - `resolved`: whether the thread is resolved
  - `visibility`: whether the thread is shown in the default line-discussion UI (`visible` or `hidden_legacy`)
  - `created_at`: creation timestamp
  - `updated_at`: last thread-level state change timestamp
  - `replies`: ordered collection of `CommentReply`
- Validation rules:
  - `body` must be non-empty after trimming
  - `anchor` is required for newly created threads
  - `visibility` defaults to `visible` for newly created line-based threads
  - `author_id`, `author_name`, `note_id`, and `id` are required
  - `replies` preserve insertion order
- Relationships:
  - Belongs to one note
  - Has one `CommentAnchor`
  - Has many `CommentReply`

### CommentAnchor

- Purpose: Captures enough context to reattach a thread to the correct rendered selection after note edits while showing broader line context in the sidebar.
- Fields:
  - `text`: exact selected text for the anchor
  - `prefix`: text immediately before the selection
  - `suffix`: text immediately after the selection
  - `range_from`: last known start offset in the rendered document
  - `range_to`: last known end offset in the rendered document
  - `attachment`: current anchor attachment state (`attached` or `detached`)
  - `line_preview`: full containing line shown in the sidebar
  - `last_matched_at`: timestamp for the latest successful reattachment attempt
- Validation rules:
  - `text` must be non-empty for new comments
  - `range_from` must be less than or equal to `range_to`
  - `attachment` must be one of the supported states
  - `line_preview` must remain available even when `attachment` is `detached`

### CommentReply

- Purpose: Represents a reply inside a thread.
- Fields:
  - `id`: stable reply identifier
  - `author_id`: reply author identifier
  - `author_name`: reply author display name
  - `body`: reply body
  - `created_at`: creation timestamp
- Validation rules:
  - `body` must be non-empty after trimming
  - Replies do not carry their own anchor; they inherit the thread anchor

### LegacyCommentRecord

- Purpose: Represents older comment history that lacks a reliable line anchor and is preserved for future recovery or migration.
- Fields:
  - `id`: stable thread identifier
  - `quoted_text`: historical quoted text, if present
  - `body`: root discussion message
  - `replies`: ordered collection of `CommentReply`
  - `hidden_reason`: why it is excluded from the default line-discussion UI
- Validation rules:
  - Must remain readable from existing sidecar history
  - Must not be deleted implicitly during migration to the new model

### EditorAnnotationView

- Purpose: Frontend-only projection of a visible `CommentThread` into the shape required by `NyxEditor`.
- Fields:
  - `id`: thread identifier
  - `anchor`: annotation anchor for the editor
  - `status`: unresolved/resolved UI state
  - `attachment`: attached/detached UI state
  - `interaction`: default/hover/focus UI state
  - `tone`: optional visual theme token
- Validation rules:
  - Derived only from `visible` comment threads
  - Never treated as the source of truth

## Relationships

- One `Note` has many `CommentThread`
- One `CommentThread` has one `CommentAnchor`
- One `CommentThread` has many `CommentReply`
- One `CommentThread` maps to one `EditorAnnotationView` when visible
- One `Note` may also contain many hidden `LegacyCommentRecord` entries retained in storage

## State Transitions

### Thread State

- `open` -> `resolved`: note author resolves the discussion
- `resolved` -> `open`: note author reopens the discussion
- `open/resolved` -> `deleted`: thread author or note author deletes the thread

### Anchor State

- `attached` -> `attached`: anchor reattaches successfully after load or edit
- `attached` -> `detached`: no confident match is found after note edits
- `detached` -> `attached`: a later load finds a confident match using stored context

### Visibility State

- `visible` -> `visible`: thread is a new line-based comment and participates in the default UI
- `hidden_legacy` -> `hidden_legacy`: legacy thread remains stored but excluded from the default UI
- `hidden_legacy` -> `visible`: only through an explicit future migration or recovery flow

### Interaction State

- `default` -> `focus`: user activates a thread in the sidebar or editor
- `focus` -> `default`: user leaves the active thread
- `default` -> `hover`: user hovers an annotation or sidebar thread

## Migration and Compatibility Rules

- Existing sidecar entries with `quoted_text` only remain readable.
- New writes use the structured anchor shape consistently.
- Legacy entries that cannot be converted into reliable line anchors are retained as hidden legacy records and omitted from the default line-based discussion UI.
- No implicit cleanup step may delete or overwrite hidden legacy history.
