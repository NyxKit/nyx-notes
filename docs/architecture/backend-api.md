# Backend API (`notes-server-axum`)

## Purpose

An Axum HTTP server that exposes note CRUD operations over a REST API. It wraps a `StorageBackend` (concretely `FsStorage`) and delegates authentication to whichever `AuthStore` implementation is configured by `AUTH_MODE`.

## Responsibilities

- HTTP routing and JSON serialization/deserialization
- Token verification via the active `AuthStore` (mode-agnostic)
- Map `StorageError` / `AuthError` to appropriate HTTP status codes
- Expose auth mode discovery endpoint so the frontend knows how to authenticate
- Optionally serve the built frontend SPA as static files

## API Routes

All routes under `/api/` (except `/api/auth/*`) require an `Authorization: Bearer <token>` header. What constitutes a valid token depends on `AUTH_MODE` — see [authentication.md](./authentication.md).

For the full vault and team model including permission matrices, see [vaults-and-teams.md](./vaults-and-teams.md).

### Auth Routes (unauthenticated)

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/auth/mode` | Returns the server's auth mode — called by the frontend on startup |
| `POST` | `/api/auth/login` | `secret_key` mode only — accepts credentials, returns a signed JWT |

#### `GET /api/auth/mode`

```json
{ "mode": "local" }
{ "mode": "secret_key" }
{ "mode": "firebase", "project_id": "my-project" }
{ "mode": "oidc", "issuer": "https://auth.example.com", "client_id": "nyx-notes" }
```

The frontend calls this on startup to decide which login UI to render (or to skip login entirely in `local` mode).

### Note Routes (vault-scoped)

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/vaults/:vault_id/notes` | List notes in a vault |
| `GET` | `/api/vaults/:vault_id/notes/:id` | Fetch a single note |
| `POST` | `/api/vaults/:vault_id/notes` | Create a note |
| `PUT` | `/api/vaults/:vault_id/notes/:id` | Update a note |
| `DELETE` | `/api/vaults/:vault_id/notes/:id` | Delete a note (note author or vault owner only) |
| `PATCH` | `/api/vaults/:vault_id/notes/:id/permission` | Change note-level permission (note author only) |

### Vault Routes

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/vaults` | List all vaults accessible to the user |
| `POST` | `/api/vaults` | Create a personal vault |
| `DELETE` | `/api/vaults/:vault_id` | Delete a personal vault (must be empty) |
| `PATCH` | `/api/teams/:team_id/vaults/:vault_id/permission` | Change a team vault's permission (team owner or admin) |

### Team Routes

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/teams` | List teams the user belongs to |
| `POST` | `/api/teams` | Create a team |
| `GET` | `/api/teams/:team_id` | Get team details and member list |
| `DELETE` | `/api/teams/:team_id` | Delete a team (team owner only) |
| `POST` | `/api/teams/:team_id/members` | Add a member |
| `PATCH` | `/api/teams/:team_id/members/:user_id` | Change a member's role |
| `DELETE` | `/api/teams/:team_id/members/:user_id` | Remove a member |
| `POST` | `/api/teams/:team_id/vaults` | Create a team vault |
| `DELETE` | `/api/teams/:team_id/vaults/:vault_id` | Delete a team vault (must be empty; owner or admin) |

### `GET /api/vaults/:vault_id/notes`

Returns `Vec<NoteMeta>` for all notes in the vault the user has access to.

Permission check: resolved from the vault's permission and the user's team role (if a team vault). See [vaults-and-teams.md](./vaults-and-teams.md) for the matrix.

Response: `200 OK`.

### `GET /api/vaults/:vault_id/notes/:id`

Response: `200 OK` with full `Note` (meta + content). `404` if not found. `403` if the user lacks view access.

### `POST /api/vaults/:vault_id/notes`

Request body:
```json
{
  "title": "My Note",
  "content": "# Hello\n\nMarkdown body.",
  "tags": ["rust"],
  "category": "work",
  "permission": "restricted"
}
```

`permission` defaults to the vault's permission level if omitted.

Response: `201 Created` with the created `NoteMeta`.

### `PUT /api/vaults/:vault_id/notes/:id`

Request body: same shape as `POST` (excluding `permission`).

Response: `200 OK` with updated `NoteMeta`. `403` if the user cannot edit. `404` if not found.

### `DELETE /api/vaults/:vault_id/notes/:id`

Only the note's `author_id` or the vault/team owner may delete.

Response: `204 No Content`. `403` otherwise. `404` if not found.

### `PATCH /api/vaults/:vault_id/notes/:id/permission`

Only the note's `author_id` may change note-level permission.

Request body: `{ "permission": "comment" }`. Valid values: `"restricted"`, `"comment"`, `"edit"`.

Response: `200 OK` with updated `NoteMeta`.

### `GET /api/vaults`

Returns all vaults accessible to the user:
- All personal vaults owned by the user
- All team vaults for teams the user is a member of

Response: `200 OK` with `Vec<Vault>`.

### `POST /api/vaults`

Creates a personal vault. Request body: `{ "slug": "journal", "name": "Journal" }`.

`slug` must be unique among the user's personal vaults.

Response: `201 Created` with the created `Vault`.

### `POST /api/teams`

Creates a team. The calling user becomes the team owner. A default `home` vault is created automatically.

Request body: `{ "name": "Engineering" }`.

Response: `201 Created` with the created `Team`.

### `POST /api/teams/:team_id/members`

Request body: `{ "user_id": "firebase-uid", "role": "member" }`.

Valid roles: `"admin"`, `"member"`. (Only one owner is allowed; ownership is transferred separately.)

Response: `200 OK` with updated `Team`.

## App State

```rust
#[derive(Clone)]
struct AppState {
    storage: Arc<dyn StorageBackend>,
    auth: Arc<dyn AuthStore>,
}
```

> Use `Arc<dyn Trait>` instead of generics on `AppState` to keep handler signatures clean and avoid monomorphization complexity.

## Permission Enforcement

Permission checks happen in the route handlers **after** authentication, not in the storage layer. The pattern is:

```rust
async fn get_note(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, id)): Path<(String, String)>,
) -> Result<Json<Note>, AppError> {
    let note = state.storage.load_note(&vault_id, &id)?;

    if note.meta.author_id != user.id {
        match note.meta.permission {
            NotePermission::Restricted => return Err(AppError::Forbidden),
            NotePermission::Comment | NotePermission::Edit => {} // allowed
        }
    }

    Ok(Json(note))
}
```

The `StorageBackend` trait has no knowledge of permissions — it is the caller's responsibility to enforce them before calling into storage.

## Auth Middleware

All `/api/*` routes run through an Axum extractor that:

1. Reads the `Authorization: Bearer <token>` header
2. Calls `auth.verify_token(token)` to get a `User`
3. Injects the `User` into the request extensions
4. Returns `401 Unauthorized` if the token is missing or invalid

```rust
pub struct AuthenticatedUser(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // extract Bearer token, verify with auth store
    }
}
```

## Error Handling

| Error | HTTP Status |
|---|---|
| `StorageError::NotFound` | `404 Not Found` |
| `StorageError::PermissionDenied` | `403 Forbidden` |
| `StorageError::IoError` | `500 Internal Server Error` |
| `AuthError::InvalidToken` | `401 Unauthorized` |
| `AuthError::UserNotFound` | `401 Unauthorized` |

## Static File Serving (optional)

The server can serve the compiled frontend SPA:

```rust
Router::new()
    .nest("/api", api_router)
    .fallback_service(ServeDir::new("dist").append_index_html_on_directories(true))
```

This allows a single binary deployment where the Rust server handles both API and frontend.

## Configuration

| Env var | Default | Description |
|---|---|---|
| `NOTES_ROOT` | `./notes` | Passed to `FsStorage` |
| `FIREBASE_PROJECT_ID` | — | Required for token verification |
| `PORT` | `8080` | Listening port |
| `FRONTEND_DIST` | `./dist` | Path to compiled frontend (optional) |

## Dependencies

- `axum` — HTTP framework
- `tokio` — async runtime
- `serde` / `serde_json` — JSON
- `tower-http` — `ServeDir`, CORS, tracing middleware
- `notes-core`, `notes-storage-fs` — storage
- Firebase auth crate (see [authentication.md](./authentication.md))
