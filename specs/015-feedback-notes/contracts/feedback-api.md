# Feedback API Contract

## Overview

Feedback is a top-level resource separate from notes and vaults. Submitted feedback is admin-visible only. Feedback items are note-derived records with extra feedback metadata.

## Endpoints

### `POST /api/feedback`

Creates a new feedback item.

Request fields:
- `title`
- `description`
- `feedback_type`
- `app_location` (auto-captured by the client)
- `storage_path` (auto-captured by the client)
- `console_output` (auto-captured, bounded, and redacted)
- `interaction_trail` (optional)
- `images` (optional)

### `GET /api/feedback`

Lists feedback items for admins.

### `GET /api/feedback/:id`

Loads one feedback item with its editable content and attached images.

### `PUT /api/feedback/:id`

Updates the title, description, content, and images for an existing feedback item.

### Comment actions

Feedback detail views support the same comment interaction model as notes.

## Response expectations

- Feedback create returns a stable item identifier.
- List and detail responses include images and submission context.
- Non-admin callers must not receive submitted feedback data.
