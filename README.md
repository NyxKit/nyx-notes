# Nyx Notes (working title)

A self-hosted, Markdown-first notes app.

- Notes are stored as plain `.md` files on disk — the filesystem is the **source of truth**
- Backend: portable Rust server (Axum) — runs on any Linux server, NAS, or local machine
- Frontend: Vue 3 SPA using [nyx-kit](https://github.com/nyxkit/nyx-kit) and TipTap, with one local workspace profile plus multiple remote server profiles per client, global search/favorites across reachable profiles, and note cards that show source server and vault
- Review: line-based discussion threads anchored to exact selected text, with comment history stored in `.comments.json` sidecars
- CLI: terminal-first workflows, operates directly on the filesystem with no server required
- Auth: pluggable on the server — `local` (no auth), `secret_key` (self-hosted JWT plus backend-managed users), or `oidc` (any OIDC provider); the multi-profile client flow in this feature supports local profiles and remote `secret_key` username/password sign-in
- Native app: Tauri desktop app for macOS, Windows, and Linux
- Future: optional end-to-end encryption and optional AI assistant (never required)

---

## Goals

- **Filesystem-native** — all content is plain Markdown: editable by any text editor, git-friendly, easy to back up
- **Server-first storage layout** — content lives under one server namespace with per-user homes and shared server vaults; `local/` stays reserved for later sync/local work
- **Portable** — single binary that runs on Linux servers, NAS devices, and desktops
- **Layered** — core logic in a reusable crate; storage and auth are pluggable via traits
- **Privacy-first** — design keeps the door open for E2EE; AI features are explicitly opt-in

---

## Project Structure

```
nyx-notes/
  Cargo.toml              # workspace manifest
  crates/
    notes-core/           # domain types and traits — no IO or frameworks
    notes-storage-fs/     # filesystem implementation of StorageBackend
    notes-auth/     # LocalAuthStore + SecretKeyAuthStore (no external deps)
    notes-auth-oidc/      # OidcAuthStore
    notes-server-axum/    # Axum HTTP API server
    notes-cli/            # CLI — direct filesystem access, no server required
  app/               # Vue 3 SPA (nyx-kit + NyxEditor)
    native/               # Tauri native app shell (embeds the Axum server)
  docs/                   # architecture, interface, conventions, and testing specs
```

---

## Documentation

| Path | What it covers |
|---|---|
| [docs/architecture/](docs/architecture/README.md) | System design, backend layers, domain model, API, auth |
| [docs/interface/](docs/interface/README.md) | Frontend (Vue SPA) and CLI |
| [docs/conventions/](docs/conventions/README.md) | Coding standards, naming, file conventions |
| [docs/testing/](docs/testing/README.md) | Test strategy per layer |

---

## Getting Started

### Prerequisites

- Rust (stable) — [rustup.rs](https://rustup.rs)

### Environment

Copy `.env.example` to `.env` and adjust to your machine, then source it before running anything:

```sh
cp .env.example .env
# edit .env — set NOTES_ROOT, NOTES_USER_ID, and SERVER_NAME

# bash / zsh
source .env && export $(cut -d= -f1 .env)

# fish
export (grep -v '^#' .env | xargs -L1)
```

> **Note:** In the MVP, `NOTES_USER_ID` is both the authenticated user ID and the home lookup key used by the CLI and server. The active server slug is derived from `SERVER_NAME` at startup.

### Build

```sh
cargo build
```

### CLI

```sh
# (after sourcing .env)

# The server namespace is derived from SERVER_NAME.
# Personal vaults live under <server-slug>/homes/<NOTES_USER_ID>/.

# Create a personal vault in your home namespace
cargo run -p notes-cli -- vault new --slug journal --name Journal

# Create a note
cargo run -p notes-cli -- new --title "Hello world" --tags "test"

# List notes
cargo run -p notes-cli -- list

# Show a note (use the ID from the list output)
cargo run -p notes-cli -- show <id>

# Search
cargo run -p notes-cli -- search "hello"
cargo run -p notes-cli -- search "hello" --body   # include note body

# Tags
cargo run -p notes-cli -- tags

# Vault management
cargo run -p notes-cli -- vault list
cargo run -p notes-cli -- vault new --slug journal --name Journal
```

### Server

```sh
# (after sourcing .env)
cargo run -p notes-server-axum
# Listening on http://localhost:8080
```

```sh
# Auth mode discovery
curl http://localhost:${PORT:-8080}/api/auth/mode

# Create a personal vault in the caller's home namespace
curl -s -X POST http://localhost:${PORT:-8080}/api/vaults \
  -H 'Content-Type: application/json' \
  -d '{"slug":"journal","name":"Journal"}'

# Create a note (use the vault id from above)
curl -s -X POST "http://localhost:${PORT:-8080}/api/vaults/<vault-id>/notes" \
  -H 'Content-Type: application/json' \
  -d '{"title":"Hello","content":"My first note","tags":["test"]}'

# List notes
curl "http://localhost:${PORT:-8080}/api/vaults/<vault-id>/notes"
```

### Configuration

| Setting | Env var | Default | Config key |
|---|---|---|---|
| Notes root | `NOTES_ROOT` | `~/notes` | `notes_root` |
| User ID | `NOTES_USER_ID` | `"local"` | `user_id` |
| Server name | `SERVER_NAME` | `"Main Server"` | `server_name` |
| Active vault | `NOTES_VAULT` | `"home"` | `vault` |
| Editor | `EDITOR` | `vi` | — |
| Server port | `PORT` | `8080` | — |

For local frontend development, Vite proxies `/api` to `http://localhost:$PORT` by default. Override that with `VITE_API_PROXY_TARGET` when your backend runs elsewhere.

> **Note:** `NOTES_USER_ID` must match between the CLI and the server. In the MVP, both resolve the caller's personal home under `<server-slug>/homes/<NOTES_USER_ID>/`, where `<server-slug>` is derived from `SERVER_NAME`.

CLI config file: `~/.config/nyx-notes/config.toml`

```toml
notes_root = "/mnt/data/notes"
user_id = "alice"
server_name = "Main Server"
vault = "home"
```

---

## Roadmap

1. Single-user, local-mode Axum API + Vue frontend + CLI
2. Tauri native app (embedded server, local auth)
3. Multi-profile client: local workspace plus multiple remote self-hosted server connections
4. Multi-user vault namespacing + `secret_key` auth (self-hosted)
5. `oidc` auth mode (cloud/managed, any OIDC provider)
6. Optional E2EE
7. Optional AI/integrations (never required)
