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

- User records live in `$NOTES_ROOT/.users.json`
- Passwords are hashed with Argon2
- JWTs are signed with a server-managed HMAC key
- The first bootstrap user is created as `admin`
- Additional users default to role `user` unless the user store is edited or extended by later admin flows

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
