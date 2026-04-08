# Docker Deployment

Nyx Notes ships as a single container. The Rust server binary serves both the REST API and the built Vue frontend — no nginx sidecar required.

## How it works

```
┌─────────────────────────────────────────────┐
│          notes-server (port 8080)            │
│                                              │
│  /api/*   → Axum route handlers             │
│  /*       → ServeDir from /app/dist/         │
│             (falls back to index.html        │
│              for SPA client-side routing)    │
└─────────────────────────────────────────────┘
```

The `STATIC_DIR` env var controls where the server looks for static files (default: `./dist`; set to `/app/dist` in the Docker image).

## CI/CD

Every push to `main` triggers `.github/workflows/docker.yml`, which builds the image and pushes it to GitHub Container Registry:

```
ghcr.io/nyxkit/nyx-notes:latest
ghcr.io/nyxkit/nyx-notes:<short-sha>
```

Pull requests build the image but do not push. Layer caching via GitHub Actions cache keeps subsequent builds fast.

## Build

The `Dockerfile` at the repo root uses a three-stage build:

| Stage | Base image | Output |
|---|---|---|
| `rust-builder` | `rust:1-slim-bookworm` | `target/release/notes-server` binary |
| `node-builder` | `node:22-slim` | `app/dist/` built Vue SPA |
| Runtime | `debian:bookworm-slim` | Binary + dist, minimal image |

```sh
docker build -t nyx-notes .
```

## docker-compose.yaml

```yaml
services:
  nyx-notes:
    image: ghcr.io/nyxkit/nyx-notes:latest
    ports:
      - "8080:8080"
    volumes:
      - /your/nas/path/notes:/data
    environment:
      NYX_ROOT: /data
      SERVER_NAME: "My Server"
      AUTH_MODE: local
      LOCAL_USER_ID: local
      PORT: 8080
    restart: unless-stopped
```

Adjust `/your/nas/path/notes` to wherever you want notes stored on the host. The container writes nothing outside of `NYX_ROOT`.

## Environment variables

| Variable | Default | Notes |
|---|---|---|
| `NYX_ROOT` | `./nyx-server` | Host-mounted path where notes are stored |
| `SERVER_NAME` | `Nyx Server` | Derives the server slug used in the filesystem layout |
| `AUTH_MODE` | `local` | `local` (no auth), `secret_key` (JWT + user management) |
| `LOCAL_USER_ID` | `local` | User ID for `AUTH_MODE=local` |
| `PORT` | `8080` | Port the server listens on inside the container |
| `STATIC_DIR` | `./dist` | Path to the built frontend; set to `/app/dist` in the image |

## Auth modes

**`local`** — no authentication. A single user (`LOCAL_USER_ID`) owns all vaults. Suitable for personal NAS deployments behind a trusted network or VPN.

**`secret_key`** — JWT-based auth with server-managed users. Requires `NXY_DB_PATH` pointing to a writable path inside `NYX_ROOT`. Suitable when the server is exposed beyond a trusted network.

```yaml
environment:
  AUTH_MODE: secret_key
  NYX_DB_PATH: /data/.nyx.db
```

## Volume

Mount a host directory to `/data` (or whatever `NYX_ROOT` is set to). Notes are plain `.md` files — back them up with any file-based tool (rsync, rclone, restic, etc.).

```
/data/
  <server-slug>/
    homes/
      <user-id>/
        <vault-slug>/
          <note-id>.md
    vaults/
      <vault-slug>/
```

See [`file-system.md`](./file-system.md) for the full filesystem layout.

## Testing locally

```sh
# Build the image
docker build -t nyx-notes .

# Run with a temporary data directory
docker run --rm -p 8080:8080 -v /tmp/nyx-test:/data nyx-notes

# Open the app
open http://localhost:8080
```
