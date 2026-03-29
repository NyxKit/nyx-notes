# Research: Line-Based Comments

## Decision 1: Use nyx-kit annotations as the canonical editor integration

- Decision: Bind note comments to `NyxEditor` via `annotations`, `annotation:create`, `annotation:focus`, and `annotation:blur` rather than keeping a custom comment-mark flow.
- Rationale: nyx-kit 2.0.6 already exposes first-class annotation primitives, including attachment state, focus events, and status theming. Reusing that API lowers editor-side custom work and keeps the feature aligned with the shared component library.
- Alternatives considered:
  - Keep the current `quoted_text` comment flow and add a thin line-selection wrapper. Rejected because it ignores nyx-kit annotation state and leaves the editor/sidebar disconnected.
  - Build a custom TipTap mark extension in-app. Rejected because AGENTS prefers nyx-kit base components when available and nyx-kit already provides the relevant surface.

## Decision 2: Persist a richer anchor object, not just quoted text

- Decision: Extend the comment model to store a structured anchor with exact selected text, surrounding context, last-known range, and attachment state.
- Rationale: nyx-kit exposes `NyxAnnotationAnchor` with `text`, `context.prefix`, `context.suffix`, and `range`, which is enough to restore a discussion after edits and to distinguish detached threads from attached ones. `quoted_text` alone is too weak for reliable reattachment when the same text appears multiple times.
- Alternatives considered:
  - Keep `quoted_text` as the only persisted anchor. Rejected because ambiguous matches and line edits would continue to create incorrect restores.
  - Persist only numeric ranges. Rejected because range-only anchors drift as soon as users edit earlier content.

## Decision 3: Interpret "line-based" as exact selected text within a rendered line

- Decision: Treat a line comment as a discussion anchored to the exact selected text inside `NyxEditor`, while the sidebar displays the containing line as the visible context.
- Rationale: The clarified spec requires exact selected-text anchors for new comments, while still presenting the broader line in the sidebar so users can orient themselves quickly.
- Alternatives considered:
  - Snap every comment to the full line. Rejected because it loses precision and can create noisy anchors when only part of the line is under discussion.
  - Store raw Markdown line numbers as the canonical anchor. Rejected because nyx-kit does not expose source-line mapping and line numbers become unstable after edits.

## Decision 4: Hide non-convertible legacy comments by default, but retain them in storage

- Decision: Existing comments without a reliable line anchor remain stored in sidecar files but are hidden from the default line-discussion UI.
- Rationale: The clarified spec prefers a clean line-based experience over surfacing low-confidence legacy threads, while still requiring that historical comment data is not discarded.
- Alternatives considered:
  - Show legacy comments as detached threads. Rejected because the clarified product decision is to hide them from the default experience.
  - Attempt low-confidence auto-attachment. Rejected because incorrect placement is more misleading than omission.
  - Delete legacy comments. Rejected because it violates the requirement to retain prior history.

## Decision 5: Keep permission and resolution semantics unchanged

- Decision: The feature changes comment anchoring and rendering only; who may read, comment, resolve, reopen, and delete remains governed by current note permission rules.
- Rationale: The current permission model already separates restricted, comment, and edit access. Changing that would expand scope and complicate the storage/API redesign unnecessarily.
- Alternatives considered:
  - Introduce line-level permissions or per-thread ownership rules beyond current behavior. Rejected because the feature request is about anchoring and discussion UX, not access control redesign.

## Decision 6: Use a frontend adapter between backend comments and editor annotations

- Decision: Add an explicit mapping layer that converts persisted comments into `NyxAnnotation[]` for the editor and converts editor annotation events into comment-create payloads.
- Rationale: The backend model carries replies, authorship, and resolution state that the editor does not need directly, while `NyxEditor` needs attachment/interactions in its own shape. A dedicated adapter keeps both models clear.
- Alternatives considered:
  - Reuse backend comment objects directly as editor state. Rejected because it couples editor state too tightly to API shapes and makes focus/attachment behavior harder to manage.
  - Keep the editor unaware of comment state and let the sidebar manage everything. Rejected because the requested feature depends on visible line highlights and editor/sidebar synchronization.
