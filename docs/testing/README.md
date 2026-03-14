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
- `.team.json` and `.vault.json` read/write correctness
- `created_at` is not modified on update

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
- `PUT` does not change `created_at`
- `PATCH /permission` is owner-only

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
- `useAuth`: token is attached to requests
- `useComments`: anchor resolution finds `quoted_text` in document; orphaned handling

**E2E tests** (Playwright): full browser against a running dev server.

What to test:
- Login → note list loads → open note → edit → save persists
- Permission selector visible for owner, hidden for non-owner
- Comment: select text → add comment → thread appears aligned to text
- Vault switcher: switching vault loads correct note list

### CLI — Integration Tests (Temp Filesystem)

Invoke the binary as a subprocess against a temp `NOTES_ROOT`.

What to test:
- `notes new --title "foo"` creates `<slug>.md` with correct frontmatter
- `notes list` output includes the created note
- `notes delete <id> --force` removes the file
- `notes search "foo"` returns the note
- `--vault` flag correctly scopes to the target vault

## What Not to Test

- Don't test the Firebase JWKS endpoint (mock the HTTP client in auth tests)
- Don't test nyx-kit component internals
- Don't test that `serde_yaml` parses YAML correctly — trust the dependency
