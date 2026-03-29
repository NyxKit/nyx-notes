# Contract: Line Comment API

## Purpose

Define the HTTP contract changes needed to support line-anchored comment threads while preserving historical comment data and hiding non-convertible legacy comments from the default line-discussion experience.

## Endpoints

### `GET /api/vaults/{vault_id}/notes/{note_id}/comments`

- Returns visible open and resolved line-based comment threads for the note.
- Response: `200 OK`

```json
[
  {
    "id": "comment_123",
    "note_id": "note_123",
    "author_id": "user_1",
    "author_name": "Arne",
    "body": "This line needs clarification.",
    "anchor": {
      "text": "selected phrase",
      "prefix": "Text before ",
      "suffix": " text after",
      "range_from": 128,
      "range_to": 143,
      "attachment": "attached",
      "line_preview": "A selected line of note text"
    },
    "resolved": false,
    "created_at": "2026-03-28T10:00:00Z",
    "updated_at": "2026-03-28T10:00:00Z",
    "replies": [
      {
        "id": "reply_456",
        "author_id": "user_2",
        "author_name": "Teammate",
        "body": "Agreed",
        "created_at": "2026-03-28T10:03:00Z"
      }
    ]
  }
]
```

Contract rules:

- Returned comments are the threads shown in the default line-discussion UI.
- Hidden legacy comments without reliable line anchors are not included in this default response.
- Detached visible threads must still include `line_preview` so the sidebar can explain the lost attachment.

### `POST /api/vaults/{vault_id}/notes/{note_id}/comments`

- Creates a new line-based discussion thread.
- Request: users with comment access or edit access only
- Response: `201 Created`

```json
{
  "body": "Please expand this thought.",
  "anchor": {
    "text": "selected phrase",
    "prefix": "Text before ",
    "suffix": " text after",
    "range_from": 128,
    "range_to": 143,
    "line_preview": "A selected line of note text"
  }
}
```

Validation:

- `body` must be non-empty
- `anchor.text` must be non-empty
- `anchor.range_from` must be less than or equal to `anchor.range_to`
- `anchor.line_preview` must reflect the containing line shown in the sidebar

### `PATCH /api/vaults/{vault_id}/notes/{note_id}/comments/{comment_id}`

- Updates thread state only.
- Request: note author only for resolve/reopen behavior
- Response: `200 OK`

```json
{
  "resolved": true
}
```

### `DELETE /api/vaults/{vault_id}/notes/{note_id}/comments/{comment_id}`

- Deletes a visible thread.
- Request: thread author or note author
- Response: `204 No Content`

### `POST /api/vaults/{vault_id}/notes/{note_id}/comments/{comment_id}/replies`

- Creates a reply on an existing visible thread.
- Request: users with comment access or edit access only
- Response: `201 Created`

```json
{
  "body": "I can take this update."
}
```

### `DELETE /api/vaults/{vault_id}/notes/{note_id}/comments/{comment_id}/replies/{reply_id}`

- Deletes a reply.
- Request: reply author or note author
- Response: `204 No Content`

## Permission Rules

- Users with `restricted` access cannot create or reply to threads unless they are the note author.
- Users with `comment` access can create threads and replies but cannot edit note body content.
- Users with `edit` access can create threads and replies under the same thread rules as comment-capable users.
- Resolve and reopen behavior remains owner-controlled unless the docs intentionally change this later.

## Frontend Integration Expectations

- The frontend maps each returned visible comment thread into a `NyxAnnotation` for `NyxEditor`.
- `resolved` maps to annotation status.
- `anchor.attachment` maps to annotation attachment state.
- The annotation anchor is based on exact selected text; the sidebar shows `line_preview` as the containing line context.
- Sidebar ordering uses attached note position first, then detached visible threads.
