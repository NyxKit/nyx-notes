# Contract: Users UI

## Purpose

Defines the user-visible behavior for the users administration surface in the authenticated application shell.

## Navigation

- A `Users` entry appears in the lower sidebar navigation directly above `Settings`.
- The route is available only inside the authenticated shell.
- Non-administrator users must not see a usable management surface.
- This management surface is available only when the server is using the `secret_key` authentication mode.

## Users Page

- The page presents a header with a primary `Add user` action.
- The main content is a table of managed users.
- Each row shows account identity fields and an actions column.
- Each row offers `Edit` and `Delete` actions when permitted for that account.

## Create/Edit Modal Behavior

- One shared modal flow supports both create and edit behavior.
- Create mode opens with empty fields and requires an initial password.
- Create and edit modes require an email field.
- Edit mode opens with existing account metadata and does not expose stored password content.
- Edit mode must not allow the currently signed-in administrator to remove their own administrator role.
- Password validation messaging must explain the 12-character minimum and the requirement to include at least 3 of 4 categories: lowercase letters, uppercase letters, digits, and symbols.
- Saving with invalid or incomplete data keeps the modal open and shows field-level or form-level feedback.

## Delete Behavior

- Delete requires an explicit confirmation step.
- Delete must not be offered as an allowed action for the currently signed-in administrator.
- Protected-account failures keep the user in context and explain why deletion is blocked.
- Successful deletion removes the row from the current list without requiring a full page reload.

## Empty And Error States

| State | Required Behavior |
|---|---|
| No manageable users yet | Show an empty-state message with visible `Add user` action |
| Users list loading | Show a loading state in the page body |
| List load failure | Show a retryable error state with explanation |
| Save failure | Keep modal open and show validation or server error message |
| Delete failure | Keep the list visible and show a non-destructive error message |
| Self-delete or self-demotion blocked | Keep the current user visible and explain that administrators cannot remove their own access |

## Visibility Rules

- Password values are never shown after submission.
- Protected accounts may still be visible in the table even when delete is disabled.
- Actions in the table must reflect backend authority, not inferred frontend-only rules.
- Email is required in create and edit forms.
