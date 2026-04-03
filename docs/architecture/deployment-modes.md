# Deployment Modes

Nyx Notes supports multiple deployment shapes while keeping the same filesystem-first storage model.

## Local

Everything runs on one machine.

```text
NOTES_ROOT=/path/to/notes
SERVER_NAME="Main Server"
NOTES_USER_ID=local
AUTH_MODE=local
PORT=8080
```

- The active server slug is derived from `SERVER_NAME`
- Personal vaults resolve under `<server-slug>/homes/<NOTES_USER_ID>/`
- The local user is treated as `admin`

## Self-hosted

Run the Rust server on a NAS, home server, or VPS.

```text
NOTES_ROOT=/data/notes
SERVER_NAME="Home Server"
AUTH_MODE=secret_key
PORT=8080
```

- Shared server vaults resolve under `<server-slug>/vaults/`
- Users authenticate with username/password in `secret_key` mode
- Server-vault administration is role-gated (`admin` vs `user`)

## Native App

The Tauri app embeds the Axum server and uses the same HTTP contract.

- `AUTH_MODE=local`
- `local/` remains reserved in the filesystem contract, but active runtime support for local-only synced vaults is deferred in the MVP

## Configuration Reference

| Env var | Default | Purpose |
|---|---|---|
| `NOTES_ROOT` | `~/notes` | Root directory for storage |
| `SERVER_NAME` | `Main Server` | Human-facing name used to derive `<server-slug>` |
| `NOTES_USER_ID` | `local` | User ID and home lookup key in the MVP |
| `AUTH_MODE` | `local` | Auth implementation |
| `PORT` | `8080` | Server port |
