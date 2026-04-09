# Feature Specification: Feedback Center

**Feature Branch**: `[015-feedback-notes]`  
**Created**: 2026-04-08  
**Status**: Draft  
**Input**: User description: "Add a feedback entry point for users and an admin feedback overview, with note-like editing and image attachments for notes and feedback."

## Clarifications

### Session 2026-04-08

- Q: Who can view submitted feedback? → A: Only admins can view submitted feedback.

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
  
  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Submit Feedback From Current Page (Priority: P1)

As a regular user, I can open a feedback modal from the bottom-left menu, enter a title and description, attach images, choose whether I am sending feedback or reporting a bug, and submit it without leaving the page I was on.

**Why this priority**: This is the primary value of the feature and the main reason users need the feedback entry point.

**Independent Test**: Can be tested by opening feedback from any page, submitting a complete entry, and confirming the user remains on the same page afterward.

**Acceptance Scenarios**:

1. **Given** a signed-in user on any page, **When** they select Feedback from the bottom-left menu, **Then** a large modal opens with fields for title, description, images, and feedback type, while app location, storage path, and console output are captured automatically.
2. **Given** a user has completed the form, **When** they submit it, **Then** the feedback is stored and the user stays on the same page they started from.
3. **Given** a user attaches multiple images, **When** they submit the form, **Then** the images are preserved with the feedback entry.

---

### User Story 2 - Review Feedback as Admin (Priority: P2)

As an admin, I can open Feedback from the same menu and see a feedback overview that behaves like the existing vault overview, so I can browse submitted feedback and add my own items.

**Why this priority**: Admin review and management is the second major workflow and is needed to make feedback actionable.

**Independent Test**: Can be tested by opening Feedback as an admin and verifying the overview appears with browse and create behaviors.

**Acceptance Scenarios**:

1. **Given** an admin user selects Feedback, **When** the section loads, **Then** they see a masonry-style overview of feedback items.
2. **Given** an admin wants to add feedback, **When** they create a new item, **Then** it is added to the same feedback collection as user submissions.

---

### User Story 3 - Manage Feedback Like a Note (Priority: P3)

As an admin, I can open a feedback item and manage it with the same capabilities as a note, including comments, title changes, edits, and image attachments.

**Why this priority**: This makes feedback actionable after collection and keeps the experience consistent with notes.

**Independent Test**: Can be tested by opening a feedback item from the overview and confirming it supports the note-style detail workflow.

**Acceptance Scenarios**:

1. **Given** an admin opens a feedback item, **When** the item loads, **Then** it presents the same detail interactions available for notes.
2. **Given** an admin updates a feedback item, **When** they save changes, **Then** the updated content remains associated with that feedback item.

---

### Edge Cases

- Submitting feedback with no images should still succeed.
- Large or numerous images should not prevent submission.
- If optional diagnostic context is unavailable, feedback should still be accepted.
- If no feedback items exist yet, admins should still reach an empty but usable overview.
- Future session history metadata should be reservable without blocking current submissions.

## Assumptions

- Feedback is a first-class top-level area alongside homes and vaults.
- The future UX-history payload will be referred to as interaction trail in this spec.
- Notes and feedback items share the same editor experience and image model.
- Console output is captured in redacted, bounded form so the submission remains safe to store and review.
- Feedback items are elevated notes: they inherit the note structure and add feedback-specific metadata.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST expose a Feedback entry point in the bottom-left menu.
- **FR-002**: Regular users MUST open Feedback in a large modal and remain on their current page after opening or submitting it.
- **FR-003**: Only admin users MUST be able to view submitted feedback, and they MUST see Feedback as a masonry-style overview that matches the existing vault overview behavior.
- **FR-004**: Feedback items MUST extend the note structure as elevated notes, adding feedback-specific metadata while retaining the note content model and image support.
- **FR-005**: The system MUST compress attached images before storing them.
- **FR-006**: Note images MUST be stored inside an images folder within the note folder.
- **FR-007**: Feedback items MUST be stored in a top-level feedback area separate from homes and vaults.
- **FR-008**: A feedback submission MUST capture the feedback type, the user’s current app location, the corresponding storage path, and the console output available at submission time without asking the user to enter those diagnostics manually.
- **FR-009**: Console output capture MUST be bounded and redacted so secrets, tokens, and other sensitive values are not retained in the submitted record.
- **FR-010**: The feedback data model MUST allow additional diagnostic context, including a future interaction trail field, without changing the user flow.
- **FR-011**: Admins MUST be able to open a feedback item into a note-style detail view that supports comments, title changes, and editing.
- **FR-012**: The system MUST allow admins to create feedback items of their own from the feedback overview.

### Key Entities *(include if feature involves data)*

- **Note**: A user-authored content item with a title, description, comments, and zero or more images.
- **FeedbackItem**: A note-derived item used for feedback or bug reports, with feedback type, location context, console output, and optional extra diagnostic context.
- **ImageAttachment**: An image associated with a note or feedback item.
- **SubmissionContext**: The location and diagnostic information captured when feedback is submitted.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: At least 9 out of 10 test users can open feedback from a page and submit it without losing their place.
- **SC-002**: Users can attach multiple images to a note or feedback item and retrieve those images after submission.
- **SC-003**: Admins can reach a usable feedback overview and open a submitted item in a note-style detail view within 3 clicks from the menu.
- **SC-004**: At least 95% of submitted feedback records include the issue type, current app location, storage path, and console output.
- **SC-005**: Reviewers can distinguish feedback from bugs consistently across all submitted items.
