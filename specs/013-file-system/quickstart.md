# Quickstart: File System Architecture Alignment

## 1. Update Docs First

Treat `docs/architecture/file-system.md` as the source document, then align:

- `docs/architecture/core-domain.md`
- `docs/architecture/filesystem-storage.md`
- `docs/architecture/backend-api.md`
- `docs/architecture/authentication.md`
- `docs/architecture/vaults-and-teams.md` or its replacement
- `docs/architecture/deployment-modes.md`
- `docs/interface/frontend.md`
- `docs/interface/cli.md`
- `docs/testing/README.md`
- `README.md`
- `.env.example`

## 2. Update Core Domain

- Replace team-centric ownership types with server/home/server-vault ownership
- Retire MVP `Team`, `TeamMember`, and `TeamRole`
- Update `StorageBackend` to remove team-only methods and add any namespace metadata operations needed for the new model

## 3. Update Filesystem Storage

- Implement path helpers for:
  - `<server>/homes/<home>/<vault>`
  - `<server>/vaults/<vault>`
  - `local/<vault>`
- Add metadata codecs for `.server.json`, `.home.json`, `.vault.json`, `.local.json`
- Update vault discovery and ID resolution to scan the new namespace layout
- Preserve existing note and comment sidecar formats

## 4. Update Auth And Server Composition

- Keep `NOTES_USER_ID` as the MVP identity/home lookup key
- Introduce `admin` and `user` role resolution in auth-backed user records
- Replace team routes with server-vault administration routes
- Keep permission enforcement in the API layer

## 5. Update CLI And Frontend Contracts

- Remove team-centric CLI commands and addressing
- Update frontend shared types, routes, and vault settings behavior to match the server/home model
- Retire team settings UI from the MVP route contract

## 6. Validate

Run:

```bash
cargo test
```

```bash
npm test -- --runInBand
```

If available for this feature branch:

```bash
npm run e2e
```

## 7. Manual Checks

- creating a personal vault writes into `<server>/homes/<home>/<vault>`
- creating a server vault writes into `<server>/vaults/<vault>`
- local mode still resolves the correct user's home from `NOTES_USER_ID`
- non-admin users cannot create or delete server vaults
- all docs and README examples use the new storage layout
