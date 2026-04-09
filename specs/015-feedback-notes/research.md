# Research: Feedback Center

## 1. Namespace and storage shape

- Decision: Create a dedicated top-level `feedback/` namespace alongside `homes/` and `vaults/`, with each item stored in its own folder.
- Rationale: Feedback is not a vault and needs separate permissions, browsing, and diagnostics while still supporting item-local image storage.
- Alternatives considered: Reusing vault storage; keeping flat `.md` files with sibling asset folders.

## 2. Shared content model

- Decision: Extend the note content model so notes and feedback both support zero or more images.
- Rationale: The user experience is intentionally note-like, so one shared editor path reduces duplication and keeps behavior consistent.
- Alternatives considered: A separate feedback-only editor; separate image handling for notes and feedback.

## 3. Submission metadata

- Decision: Capture feedback type, current app location, storage path, console output, and a future interaction trail field.
- Rationale: These fields match the support/debugging goal of the feature while leaving room for later UX-history capture. The UI should auto-capture the diagnostic fields so users only provide the feedback itself. Console output is stored in bounded, redacted form to avoid retaining secrets or oversized payloads.
- Alternatives considered: Capturing only title and description; postponing diagnostics until later; storing raw console output.

## 4. Image compression

- Decision: Use the existing backend image-processing path for compression and keep the implementation dependency-neutral in the plan.
- Rationale: The feature needs compression, but the plan should avoid adding a new dependency unless implementation proves it is necessary.
- Alternatives considered: Adding a new image library immediately; storing original images without compression.

## 5. Admin workflow

- Decision: Treat admin feedback as a masonry-style overview plus note-style detail view.
- Rationale: This matches the existing vault browsing mental model and preserves the full comment/edit workflow for admins.
- Alternatives considered: A separate custom admin page; a plain list view.

## 6. UX naming for future history

- Decision: Use `interaction trail` as the placeholder name for later UX-history capture.
- Rationale: The phrase is understandable, neutral, and broad enough to include navigation and in-app actions.
- Alternatives considered: `event log`, `session history`, `activity trail`.
