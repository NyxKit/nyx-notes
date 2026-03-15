# Nyx Notes (working title)

A self-hosted, Markdown-first notes app.

- Notes are stored as plain `.md` files on disk — the filesystem is the **source of truth**
- Backend: portable Rust server (Axum) — runs on any Linux server, NAS, or local machine
- Frontend: Vue 3 SPA using [nyx-kit](https://github.com/nyxkit/nyx-kit) and TipTap
- CLI: terminal-first workflows, operates directly on the filesystem with no server required
- Auth: pluggable — `local` (no auth), `secret_key` (self-hosted JWT), or `oidc` (any OIDC provider); no external dependency required for local or self-hosted deployments
- Native app: Tauri desktop app for macOS, Windows, and Linux
- Future: optional end-to-end encryption and optional AI assistant (never required)

---

## Goals

- **Filesystem-native** — all content is plain Markdown: editable by any text editor, git-friendly, easy to back up
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
    notes-auth-local/     # LocalAuthStore + SecretKeyAuthStore (no external deps)
    notes-auth-oidc/      # OidcAuthStore
    notes-server-axum/    # Axum HTTP API server
    notes-cli/            # CLI — direct filesystem access, no server required
  frontend/               # Vue 3 SPA (nyx-kit + TipTap)
  src-tauri/              # Tauri native app (embeds the Axum server)
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

### Build

```sh
cargo build
```

### CLI

```sh
export NOTES_ROOT=/tmp/nyx-test
export NOTES_USER_ID=alice

# Create your home vault
cargo run -p notes-cli -- vault new --slug home --name Home

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
export NOTES_ROOT=/tmp/nyx-test
cargo run -p notes-server-axum
# Listening on http://localhost:8080
```

```sh
# Auth mode discovery
curl http://localhost:8080/api/auth/mode

# Create a vault
curl -s -X POST http://localhost:8080/api/vaults \
  -H 'Content-Type: application/json' \
  -d '{"slug":"home","name":"Home"}'

# Create a note (use the vault id from above)
curl -s -X POST "http://localhost:8080/api/vaults/<vault-id>/notes" \
  -H 'Content-Type: application/json' \
  -d '{"title":"Hello","content":"My first note","tags":["test"]}'

# List notes
curl "http://localhost:8080/api/vaults/<vault-id>/notes"
```

### Configuration

| Setting | Env var | Default | Config key |
|---|---|---|---|
| Notes root | `NOTES_ROOT` | `~/notes` | `notes_root` |
| User ID | `NOTES_USER_ID` | `"default"` | `user_id` |
| Active vault | `NOTES_VAULT` | `"home"` | `vault` |
| Editor | `EDITOR` | `vi` | — |
| Server port | `PORT` | `8080` | — |

CLI config file: `~/.config/nyx-notes/config.toml`

```toml
notes_root = "/mnt/data/notes"
user_id = "alice"
vault = "home"
```

---

## Roadmap

1. Single-user, local-mode Axum API + Vue frontend + CLI
2. Tauri native app (embedded server, local auth)
3. Multi-user vault namespacing + `secret_key` auth (self-hosted)
4. `oidc` auth mode (cloud/managed, any OIDC provider)
5. Optional E2EE
6. Optional AI/integrations (never required)
