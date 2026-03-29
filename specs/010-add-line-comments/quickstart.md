# Quickstart: Line-Based Comments

## Goal

Verify that line-based discussions work end to end: creation from the editor, sidebar discussion flow, persistence, restore after edits, permission enforcement, and hidden legacy comment behavior.

## Prerequisites

- Repo dependencies installed
- Frontend uses `nyx-kit@2.0.6`
- A runnable backend and frontend environment with at least two test users or one owner and one collaborator path

## Validation Flow

### 1. Start the app

- Start the backend server for the local notes root
- Start the frontend app
- Open a note with existing edit or comment access

### 2. Create a line-based discussion

- Select non-empty text within a rendered line in the note editor
- Trigger comment creation from the editor annotation flow
- Submit a root comment
- Confirm the sidebar shows a new thread attached to the selected text
- Confirm the sidebar displays the containing line as the visible context
- Confirm the selected text is visibly annotated in the editor

### 3. Follow a discussion from the sidebar

- Focus the thread in the sidebar
- Confirm the related editor annotation becomes active
- Add a reply
- Resolve the thread as the note owner
- Confirm the thread moves to the resolved state without losing its line context

### 4. Verify restore after note edits

- Edit content before or inside the commented line
- Save and reload the note
- Confirm the thread either reattaches to the correct selected text or shows as detached with the saved line preview still visible
- Confirm multiple threads on one line remain separately identifiable

### 5. Verify legacy comment behavior

- Load a note with pre-existing quote-only comments that cannot be converted into reliable line anchors
- Confirm those legacy comments are not shown in the default line-discussion sidebar
- Confirm the underlying comment sidecar data remains present and unchanged after load/save operations

### 6. Verify permissions

- Open the same note as a user with `comment` access
- Confirm the user can create and reply to line-based discussions but cannot edit note content
- Open as a user with read-only access
- Confirm the user can view visible threads but cannot create, reply, resolve, or reopen discussions

## Suggested Test Commands

```bash
cargo test
npm --prefix frontend run test:unit:run
npm --prefix frontend run build
npm --prefix frontend run test:e2e
```

## Expected Outcomes

- Comment creation originates from editor selection, not a free-floating sidebar input
- New comments preserve exact selected-text anchors while the sidebar shows the containing line
- Sidebar and editor stay synchronized on active/focused threads
- Detached visible threads are understandable instead of disappearing
- Legacy comments without reliable anchors stay hidden from the default line-based UI but remain stored
