# Feature Specification: Line-Based Comments

**Feature Branch**: `010-add-line-comments`  
**Created**: 2026-03-28  
**Status**: Draft  
**Input**: User description: "i want to introduce line-based comments. the current comment system only works on the note itself, i want to allow users to select a line and be able to comment on that line and have a discussion around that in the sidebar. nyx-kit already introduced a lot of interfaces we can use and even base our own classes off of. research into what nyx-kit introduced in NyxEditor regarding annotations etc. It already exposes a lot, we just need to bind it to the internals and our data."

## Clarifications

### Session 2026-03-28

- Q: Should line-based comments snap to the entire line or stay anchored to the exact selected text? → A: Anchor to the exact selected text, but treat the containing line as the visible context in the sidebar.
- Q: For existing note-level comments that have no reliable line anchor, what should the product show by default after this feature ships? → A: Hide old comments.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Start a Discussion From a Line (Priority: P1)

A reader or editor selects text within a line in a note and starts a comment thread that stays tied to that exact selection so collaborators can discuss a precise passage without editing the note itself.

**Why this priority**: The feature only creates value if users can anchor feedback to exact selected text within a line and immediately start a discussion from that context.

**Independent Test**: Can be fully tested by opening a note, selecting text within a line, creating a comment, and confirming that the new thread appears in the sidebar linked to the exact selected text and shown with containing-line context.

**Acceptance Scenarios**:

1. **Given** a user can comment on a note, **When** the user selects visible text within a line and submits a comment, **Then** a new discussion thread is created, anchored to the exact selected text, and shown in the sidebar with the containing line as context.
2. **Given** a line already has one or more discussion threads, **When** the user selects that same line, **Then** the system allows the user to add another thread or continue the existing discussion without losing the original line association.
3. **Given** a user does not have comment permission, **When** the user selects text within a line, **Then** the system does not allow the user to create a line-based discussion.

---

### User Story 2 - Follow and Manage a Line Discussion (Priority: P2)

A collaborator opens the comment sidebar, sees discussions grouped by the selected text they refer to, and can reply, resolve, or revisit a thread while staying oriented through the containing line context in the note.

**Why this priority**: Once comments exist, users need to understand which line each discussion belongs to and manage the conversation without confusion.

**Independent Test**: Can be fully tested by loading a note with existing line discussions, opening the sidebar, replying to a thread, resolving it, and confirming the thread still points back to the correct line.

**Acceptance Scenarios**:

1. **Given** a note contains line-based discussions, **When** the sidebar opens, **Then** each thread shows the containing line for its selected-text anchor and displays its discussion history in a stable, readable order.
2. **Given** a user selects or focuses a thread in the sidebar, **When** the thread becomes active, **Then** the related line in the note is visibly highlighted so the user can see the discussion context.
3. **Given** a thread is resolved, **When** the user views the sidebar, **Then** the thread remains available in a resolved state without being confused with open discussions.
4. **Given** a legacy comment has no reliable line anchor, **When** the note loads after line-based comments ship, **Then** that legacy comment is not shown in the standard line-discussion experience.

---

### User Story 3 - Keep Discussions Understandable After Note Edits (Priority: P3)

A collaborator edits the note after comments already exist, and previously created line discussions remain understandable even if the original line has moved, changed, or can no longer be matched exactly.

**Why this priority**: Notes change over time, so anchored discussions must degrade gracefully instead of becoming misleading or disappearing.

**Independent Test**: Can be fully tested by creating a line-based discussion from selected text, editing the related line, reloading the note, and confirming the thread either reattaches correctly or is shown as detached with enough context for the user to understand it.

**Acceptance Scenarios**:

1. **Given** a commented line still exists after edits, **When** the note reloads, **Then** the discussion remains attached to the best matching line.
2. **Given** the original line can no longer be matched after edits, **When** the note reloads, **Then** the discussion is clearly shown as detached and still displays its saved context in the sidebar.
3. **Given** multiple nearby lines contain similar text, **When** the system restores a discussion, **Then** it uses stored context to avoid attaching the thread to the wrong line.

---

### Edge Cases

- What happens when a user selects an empty line, whitespace-only line, or a cursor position with no selectable content?
- What happens when a user selects only part of a line and the sidebar must show the broader containing line as context?
- How does the system handle multiple discussions attached to the same line without obscuring the line or making the sidebar ambiguous?
- What happens when a line is split into several lines or several lines are merged into one after comments already exist?
- How does the system present detached discussions when the referenced line was deleted entirely?
- What happens to legacy note-level comments that cannot be converted into reliable line-based anchors?
- What happens when a user opens a note that contains line discussions but only has read access and cannot participate?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST allow a user with comment permission to create a discussion thread from selected text within a line in a note.
- **FR-002**: The system MUST store enough anchor information about the selected line for the discussion to be restored after the note is reopened or refreshed.
- **FR-003**: The system MUST associate each discussion thread with one line anchor at a time and present that association consistently in the note view and the sidebar.
- **FR-004**: The system MUST show line-based discussions in the sidebar with the containing line context of the selected text.
- **FR-005**: The system MUST allow users with comment permission to reply to, reopen, and resolve line-based discussion threads according to existing note permissions.
- **FR-006**: The system MUST visually distinguish the active line discussion from inactive discussions in both the note body and the sidebar.
- **FR-007**: The system MUST preserve discussion history even when the original line text changes, moves, or is removed.
- **FR-008**: The system MUST mark a discussion as detached when it cannot be confidently re-associated with a line and MUST present the saved line context to the user.
- **FR-009**: The system MUST prevent users without comment permission from creating, replying to, resolving, or reopening line-based discussions.
- **FR-010**: The system MUST allow users to load an existing note and see all of its open and resolved line-based discussions without editing the note content.
- **FR-011**: The system MUST support multiple discussion threads on the same line and keep them separately identifiable in the sidebar.
- **FR-012**: The system MUST maintain a stable ordering for discussions in the sidebar so users can quickly scan them by note position, with detached discussions clearly separated from attached ones.
- **FR-013**: The system MUST retain existing note-level discussion data during the transition to line-based comments and MUST not silently discard prior comment history, even when those comments are hidden from the default line-discussion UI.
- **FR-014**: The system MUST preserve the exact selected-text anchor for each new discussion even when the sidebar displays the broader containing line as context.
- **FR-015**: The system MUST hide legacy note-level comments without reliable line anchors from the default line-based discussion view and retain them as hidden legacy records.

### Key Entities *(include if feature involves data)*

- **Line Discussion Thread**: A conversation attached to a specific text selection within a line in a note, including its author, status, timestamps, saved line context, replies, and visibility state in the default discussion UI.
- **Line Anchor**: The saved reference to the exact selected text, plus enough surrounding line context to locate the selection again after later note edits.
- **Discussion Reply**: A message within a line discussion thread, including its author, body, and timestamp.
- **Hidden Legacy Comment Record**: Previously stored note-level comment history without a reliable line anchor that remains retained but is excluded from the default line-based discussion view.

### Assumptions

- Line-based discussions replace free-floating note-level creation as the primary way to start a new comment in the editor.
- Existing permission rules for who may view, comment, and edit notes remain unchanged.
- Resolved discussions remain visible in a separate resolved state rather than being deleted.
- Users can create discussions only from non-empty text selections within a line.
- Hidden legacy comments remain stored and recoverable later even though they are not shown in the default line-based discussion view.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In usability testing, 90% of users who have comment access can create a line-based discussion from a note and see it in the sidebar within 30 seconds.
- **SC-002**: In validation using edited notes, at least 95% of existing line discussions either reattach to the correct line or are clearly shown as detached with readable saved context.
- **SC-003**: In task-based review, 90% of users can identify which line a sidebar discussion refers to on their first attempt.
- **SC-004**: In regression testing, all existing comment permissions continue to behave correctly for view-only, comment-only, and edit-capable users.
