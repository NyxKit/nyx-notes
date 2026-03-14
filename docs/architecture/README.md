# Architecture

System design and backend architecture for Nyx Notes.

## Layers

```
┌────────────────────────────────┐
│         notes-server-axum      │  HTTP API, auth middleware, permission enforcement
├────────────────────────────────┤
│         notes-storage-fs       │  Filesystem implementation of StorageBackend
├────────────────────────────────┤
│           notes-core           │  Domain types, traits — no IO or frameworks
└────────────────────────────────┘
```

Each layer depends only on the layer below it. `notes-core` has zero runtime dependencies.

## Documents

| File | What it covers |
|---|---|
| [core-domain.md](./core-domain.md) | Canonical types (`Note`, `Vault`, `Team`), `StorageBackend` and `AuthStore` traits, error types |
| [filesystem-storage.md](./filesystem-storage.md) | Directory layout, note frontmatter format, `FsStorage` implementation |
| [backend-api.md](./backend-api.md) | Axum HTTP routes, permission enforcement, auth middleware, error mapping |
| [authentication.md](./authentication.md) | Auth modes (`local`, `secret_key`, `firebase`, `oidc`) and their implementations |
| [vaults-and-teams.md](./vaults-and-teams.md) | Vault and team model, roles, permission matrices, metadata file formats |
| [deployment-modes.md](./deployment-modes.md) | Deployment configurations: local, self-hosted, cloud, native app |

## Key Design Decisions

- **Filesystem is the source of truth** — notes survive without any database or cloud service
- **`StorageBackend` and `AuthStore` are traits** — implementations are swappable; the server never couples to a concrete storage or auth provider
- **Auth is pluggable** — `AUTH_MODE` selects the implementation at startup; `local` (no auth), `secret_key` (self-signed JWT), `firebase`, or `oidc`. No Google account required.
- **Permission checks happen in the API layer**, not in the storage layer — `FsStorage` is intentionally unaware of permissions
- **`Arc<dyn Trait>` over generics** in `AppState` — avoids monomorphization complexity in Axum handlers
- **Native app embeds the server** — the Tauri app runs Axum as a background thread; the webview connects to `localhost`. Same HTTP API, same frontend, no separate IPC contract.
