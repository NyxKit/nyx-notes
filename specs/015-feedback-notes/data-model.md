# Data Model: Feedback Center

## Note

- `id`: stable item identifier
- `vault_id`: owning vault identifier
- `title`: display title
- `description`: summary text
- `content`: full note body
- `images`: ordered list of attached images
- `author_id`: creator identity
- `created_at`: creation timestamp
- `updated_at`: last change timestamp

## FeedbackNote

- `id`: stable item identifier
- `note`: inherited note fields and behavior
- `title`: display title
- `description`: summary text
- `content`: full feedback body
- `images`: ordered list of attached images
- `feedback_type`: feedback or bug
- `app_location`: current app location at submission time, auto-captured
- `storage_path`: stored location context, auto-captured
- `console_output`: captured console dump, auto-captured and redacted
- `interaction_trail`: optional future usage trail payload
- `author_id`: submitting user identity
- `created_at`: creation timestamp
- `updated_at`: last change timestamp

## ImageAttachment

- `id`: stable attachment identifier
- `filename`: stored file name
- `content_type`: image MIME type
- `size`: stored file size
- `path`: relative path inside the item folder

## SubmissionContext

- `app_location`: relative URL or route path where the user was when feedback started
- `storage_path`: storage-side path context associated with the item
- `console_output`: bounded, redacted console dump captured at submission time
- `interaction_trail`: future in-app history payload

## Relationships

- A note has zero or more image attachments.
- A feedback item extends a note and has zero or more image attachments.
- Each feedback item carries one submission context snapshot.
- Image attachments live under an item-local `images/` directory.

## Validation Rules

- Titles are required for notes and feedback items.
- Feedback type is required for feedback items.
- Images are optional but must be preserved in submission order.
- Item folders must be stable so attachment paths do not change when titles change.
