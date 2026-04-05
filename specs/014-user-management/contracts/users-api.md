# Contract: Users API

## Purpose

Defines the backend contract for server-scoped user administration in `secret_key` mode. All routes are authenticated and administrator-only unless stated otherwise.

## Route Family

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/users` | List managed user accounts |
| `POST` | `/api/users` | Create a new managed user |
| `PATCH` | `/api/users/:user_id` | Update account metadata, role, and optionally password |
| `DELETE` | `/api/users/:user_id` | Delete an eligible managed user |

## Response Entity: User Summary

```json
{
  "id": "user-123",
  "username": "alice",
  "display_name": "Alice",
  "email": "alice@example.com",
  "role": "user",
  "created_at": "2026-04-03T10:00:00Z",
  "updated_at": "2026-04-03T10:00:00Z",
  "can_edit": true,
  "can_delete": true
}
```

Rules:
- Password hashes and any credential fields must never appear in responses.
- `can_delete` must be `false` for protected accounts.

## `GET /api/users`

Returns all managed user accounts visible to the administrator.

Response: `200 OK`

```json
[
  {
    "id": "user-123",
    "username": "alice",
    "display_name": "Alice",
    "email": "alice@example.com",
    "role": "user",
    "created_at": "2026-04-03T10:00:00Z",
    "updated_at": "2026-04-03T10:00:00Z",
    "can_edit": true,
    "can_delete": true
  }
]
```

## `POST /api/users`

Creates a new managed user.

Request:

```json
{
  "username": "alice",
  "display_name": "Alice",
  "email": "alice@example.com",
  "role": "user",
  "password": "correct horse battery staple"
}
```

Behavior:
- Reject duplicate usernames.
- Reject duplicate email addresses.
- Require email for every managed user.
- Enforce a minimum password policy of 12 or more characters and at least 3 of these 4 categories: lowercase letters, uppercase letters, digits, and symbols.
- Make the created account available for later login immediately after success.

Response: `201 Created` with the created `User Summary`

Validation failures:
- `409 Conflict` for duplicate username
- `409 Conflict` for duplicate email address
- `422 Unprocessable Entity` for invalid field values, including passwords that do not meet the minimum length or category rules

## `PATCH /api/users/:user_id`

Updates an existing user.

Request:

```json
{
  "display_name": "Alice Example",
  "email": "alice@example.com",
  "role": "admin",
  "password": "new passphrase"
}
```

Behavior:
- Omitted password keeps the current credential unchanged.
- Email remains required on update.
- Duplicate email addresses must be rejected.
- Replacement passwords must meet the same minimum length and category rules.
- Self-demotion from the administrator role must be rejected.
- Protected-account rules apply to role changes that would remove the last administrator.

Response: `200 OK` with the updated `User Summary`

Validation failures:
- `404 Not Found` for unknown user
- `409 Conflict` for username collision when username edits are allowed
- `409 Conflict` for duplicate email address
- `409 Conflict` for self-demotion or other protected-role conflicts
- `422 Unprocessable Entity` for invalid field values, including replacement passwords that do not meet the minimum length or category rules

## `DELETE /api/users/:user_id`

Deletes an eligible managed user.

Behavior:
- Must reject deletion when the target is the currently signed-in administrator.
- Must reject deletion when the target is protected by administrative lockout rules.
- Deleted accounts must no longer be accepted for future sign-in.

Response: `204 No Content`

Validation failures:
- `404 Not Found` for unknown user
- `409 Conflict` when the account cannot be deleted because it is protected or is the currently signed-in administrator

## Authorization Rules

- Non-admin callers receive `403 Forbidden` for all routes in this contract.
- Unauthenticated callers receive `401 Unauthorized`.
- Permission enforcement lives in `notes-server-axum`, not in the embedded store implementation.
- This contract is part of the `secret_key` authentication mode MVP only.

## Error Contract

- Errors must return user-safe messages and must not leak database file paths, SQL details, or password-verifier content.
- Backend storage failures return a generic server error response while retaining enough internal detail for logs and debugging.
