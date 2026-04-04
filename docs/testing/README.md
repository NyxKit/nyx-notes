# Testing

Test strategy for Nyx Notes, organized by layer.

## Layers

### `notes-core` — Pure Unit Tests

No IO, no filesystem, no network. Every test is a plain Rust unit test.

What to test:
- Permission logic (the matrix in `core-domain.md`)
- Frontmatter serialization/deserialization round-trips
- Domain type constructors and validation

```rust
#[test]
fn restricted_note_denies_non_owner_view() { ... }
```

### `notes-storage-fs` — Integration Tests (Temp Filesystem)

Use a temporary directory per test. Spin up a real `FsStorage` instance against it.

What to test:
- `save_note` → `load_note` round-trip preserves all fields
- `list_notes` returns correct metadata, sorted by `updated_at`
- `delete_note` removes the file; returns `NotFound` on second call
- `create_vault` / `delete_vault` (including non-empty vault rejection)
- `.server.json`, `.home.json`, `.vault.json`, and `.local.json` read/write correctness
- personal vaults resolve under `<server>/homes/<home>/<vault>`
- server vaults resolve under `<server>/vaults/<vault>`
- `created_at` is not modified on update
- `.comments.json` round-trips structured anchors, visibility state, and replies
- legacy quote-only sidecars remain readable and are retained as hidden legacy records when no reliable anchor can be restored

```rust
#[tokio::test]
async fn save_and_load_note_round_trip() {
    let dir = tempdir().unwrap();
    let storage = FsStorage::new(dir.path());
    // ...
}
```

### `notes-server-axum` — HTTP Integration Tests

Use `axum::test` (or `reqwest` against a bound port) with a real `FsStorage` on a temp directory and a mock/stub `AuthStore`.

What to test:
- All CRUD routes return correct status codes
- Permission enforcement: `403` returned when access is denied
- Auth middleware: `401` on missing or invalid token
- `GET /api/auth/mode` returns the documented response shape, including additive metadata fields when present
- `POST /api/auth/login` returns `422` when password login is unavailable for the configured auth mode
- `/api/users` routes enforce `admin`-only access, required unique email/username validation, self-delete blocking, and last-admin protections
- vault-ID routes resolve the correct personal or server vault in the HTTP contract
- server-vault administration is `admin` only and personal vaults remain owner-scoped
- `PUT` does not change `created_at`
- `PATCH /permission` is owner-only
- comment creation accepts structured anchors and returns `201 Created`
- comment list omits hidden legacy records from the default response
- reply, resolve, reopen, and delete flows enforce the documented permissions

```rust
#[tokio::test]
async fn non_owner_cannot_edit_restricted_note() {
    // spin up router with stub auth returning user_b
    // attempt PUT on a note owned by user_a with permission: restricted
    // expect 403
}
```

### Frontend — Unit + E2E

**Unit tests** (Vitest): composables in isolation, with mocked `fetch`.

What to test:
- `useNotes`: CRUD methods call correct endpoints with correct payloads
- shared frontend types reflect server/home/server-vault ownership and ID-based route parameters
- `useAuth`: active-profile bootstrap discovers auth mode correctly, tokens are attached only for the active remote profile, and logging out one profile does not clear another saved profile session
- `useUsers`: loads managed users only in `secret_key` admin contexts, exposes create/edit/delete mutations, and surfaces validation failures without stale state
- `useWorkspaceProfiles`: duplicate `server_url + username` entries are rejected while same-server different-username profiles are allowed
- `useGlobalNoteBrowsing`: loads notes per reachable/authenticated profile without mutating active profile context, aggregates results across vaults, and reports excluded-profile counts
- `useComments`: maps comment records to `NyxAnnotation[]`, tracks active/focused annotations, and orders attached threads before detached ones
- `useCommentAnnotations`: preserves exact selected-text anchors, exposes containing-line context, and omits hidden legacy comments
- `useNoteBrowsingStore`: derives live search results from sidebar query updates, defaults search to recent ordering, exposes search sort modes (`best match`, `recent`, `grouped`), and limits favorites sorting to `recent` and `grouped`
- `useNoteBrowsingStore`: aggregates the sidebar recent-notes feed across reachable/authenticated profiles and vaults, preserving source origin data per entry
- `GlobalNoteBrowseView`: renders shared empty/loading states, excluded-profile notice, shared sort controls, and note-card grid consistently for search and favorites

**E2E tests** (Playwright): full browser against a running dev server.

What to test:
- First-run setup: choose local mode and enter the app without a login screen
- First-run setup: connect to an existing remote `secret_key` server and enter the app with a saved remote profile
- Profile switcher: switching between local and remote profiles loads the correct workspace and clears stale content from the previous profile
- Secret-key admin flow: open `Users`, create a user, edit it, reject duplicate email/username, block self-delete, and delete an eligible user
- Login → note list loads → open note → edit → save persists
- Sidebar search: typing in `NoteSearch` routes to global search and updates results live without explicit submit
- Sidebar recent notes: the list shows the most recently updated notes across reachable/authenticated profiles and vaults, with source server/vault labels
- Global search: the same query returns the same eligible cross-profile/cross-vault result set regardless of current vault page
- Global favorites: favorites are shown from all reachable/authenticated profiles and vaults using the same browse layout as search
- Shared browse layout: search and favorites both render server/vault origin labels, excluded-profile notice when applicable, and the correct sort controls for the current surface
- Permission selector visible for owner, hidden for non-owner
- no active team-settings route remains in the MVP flow
- Comment: select text → add comment → thread appears with containing-line context while the selected text is annotated in the editor
- Detached visible comments remain understandable after note edits
- Legacy quote-only comments remain stored but do not appear in the default line-discussion sidebar
- Focusing an annotation opens the sidebar if it is closed and highlights the matching thread
- Resolved annotations appear in the editor only while the Resolved tab is active
- Vault switcher: switching vault loads correct note list

### CLI — Integration Tests (Temp Filesystem)

Invoke the binary as a subprocess against a temp `NOTES_ROOT`.

What to test:
- `notes new --title "foo"` creates `<slug>.md` with correct frontmatter
- `notes list` output includes the created note
- `notes delete <id> --force` removes the file
- `notes search "foo"` returns the note
- `--vault` flag correctly scopes to a personal or server vault in the new namespace layout

## What Not to Test

- Don't test the OIDC provider's JWKS endpoint (mock the HTTP client in auth tests)
- Don't test nyx-kit component internals
- Don't test that `serde_yaml` parses YAML correctly — trust the dependency
