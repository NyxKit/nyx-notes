# Authentication

## Principle

Auth is pluggable. The active implementation is selected with `AUTH_MODE` and always resolves a `User` that includes:

- `id`
- `email`
- `display_name`
- `role` (`admin` or `user`)

`role` is used by the API layer to enforce shared server-vault administration.

## Modes

| Mode | Use case | Notes |
|---|---|---|
| `local` | Single-user/local machine | Always resolves one local `admin` user |
| `secret_key` | Self-hosted server | Uses Argon2-hashed credentials and signed JWTs |
| `oidc` | Future cloud/IdP deployments | Still not implemented in the current code |

## `local`

- No login screen
- `NOTES_USER_ID` is the user ID and home lookup key in the MVP
- `NOTES_LOCAL_USER_NAME` controls the display name
- The resolved local user has server role `admin`
- Auth mode and server metadata endpoints should still expose the configured server name so the frontend can label the active built-in server correctly

## `secret_key`

- MVP server-managed user records live in a local SQLite database under `$NOTES_ROOT`
- Passwords are hashed with Argon2
- JWTs are signed with a server-managed HMAC key
- The first bootstrap user is created as `admin`
- Additional users are created and managed through backend-owned admin flows
- This is the only auth mode that supports user management in the MVP
- Every managed user requires a unique `username` and a unique `email`
- Passwords must be at least 12 characters and include at least 3 of these 4 categories: lowercase letters, uppercase letters, digits, and symbols
- Administrators may edit themselves, but may not delete themselves or demote themselves from `admin`

Example stored user shape:

```json
{
  "id": "user-123",
  "username": "alice",
  "email": "alice@example.com",
  "display_name": "Alice",
  "role": "user"
}
```

## Auth Mode Discovery

`GET /api/auth/mode` remains unauthenticated.

Optional additive fields may include:

- `server_id`
- `server_name`
- `api_version`

## Notes

- Permission enforcement still belongs in `notes-server-axum`, not the auth store and not `FsStorage`
- `SERVER_NAME` is used to derive the active server slug for filesystem layout, but user role membership lives with the auth/user record
- `local` mode remains single-user and out of scope for user management
- `oidc` mode remains future work for user-management support; Nyx Notes does not store passwords for `oidc`
