# Deployment Modes

Nyx Notes is designed to run in multiple configurations without code changes — only environment variables differ.

---

## Mode Overview

| Mode | Who runs the server | Auth | Notes storage | Frontend |
|---|---|---|---|---|
| [Local](#local) | Your machine (background process) | `local` | Local disk | Native app (Tauri) or `localhost` |
| [Self-hosted](#self-hosted-nas--home-server) | Your NAS or home server | `secret_key` or `oidc` | NAS disk | Browser or native app |
| [Cloud](#cloud) | VPS / container / managed hosting | `firebase` or `oidc` | Mounted volume | Browser or native app |
| [Native app](#native-app-tauri) | Embedded in the app itself | `local` | Local disk | Tauri webview |

---

## Local

The simplest mode. Everything runs on your machine. No account, no internet, no configuration.

```
$NOTES_ROOT = ~/notes       # or wherever you want
AUTH_MODE   = local
PORT        = 8080          # or any free port
```

**How it runs:** The Tauri native app embeds the Axum server as a background thread and opens a webview pointed at `localhost:{port}`. Alternatively, run the server binary manually and open a browser.

**What you get:**
- No login screen
- Single user
- Notes are plain `.md` files on your disk
- CLI works against the same folder with no extra config

**What you don't get:** Multi-user, teams, sharing, remote access.

---

## Self-hosted (NAS / Home Server)

The NAS runs the server binary (or Docker image). You access it from a browser or native app on any device on your network (or via VPN/Tailscale for remote access).

```
NOTES_ROOT  = /data/notes
AUTH_MODE   = secret_key    # or oidc if you run Authentik/Authelia
PORT        = 8080
```

**`secret_key` auth:** The server generates a signing key on first run and stores it locally. You log in with a username and password; the server issues its own JWTs. No internet dependency. No Google account.

**`oidc` auth:** If you already run an identity provider (Authentik, Keycloak, Authelia), point `OIDC_ISSUER_URL` at it and use it for login. Enables SSO across your self-hosted services.

**Deployment options:**
- Bare binary: `./notes-server`
- Docker: `docker run -v /data/notes:/notes -e AUTH_MODE=secret_key nyx-notes`
- Systemd service on the NAS

**Remote access options:**
- Expose port 8080 directly (behind a reverse proxy with TLS)
- Tailscale / Headscale (no port forwarding, end-to-end encrypted tunnel)
- Cloudflare Tunnel

---

## Cloud

Run on a VPS, container platform, or managed hosting. Multiple users, public internet access.

```
NOTES_ROOT           = /data/notes   # mounted volume
AUTH_MODE            = firebase       # or oidc
FIREBASE_PROJECT_ID  = my-project     # if AUTH_MODE=firebase
PORT                 = 8080
```

**`firebase` auth:** Delegates identity to Google Firebase. Enables email/password and OAuth login providers (Google, GitHub, etc.) without managing user accounts.

**`oidc` auth:** Use your own OIDC provider for full control over identity.

**Frontend deployment options:**
- Served by the Rust server at `/` (single binary, zero extra infra)
- Deployed separately to Firebase Hosting / Netlify / Cloudflare Pages with `/api/*` proxied to the server

---

## Native App (Tauri)

The Tauri app bundles the Axum server and the Vue frontend into a single installable application. No separate server process, no browser required.

```
AUTH_MODE = local   (hardcoded in the Tauri build; not user-configurable)
```

See [docs/interface/native-app.md](../interface/native-app.md) for full details.

**The native app can also connect to a remote server.** If the user provides a server URL in settings, the app talks to that server instead of the embedded one. This enables a "desktop client for your self-hosted instance" mode.

---

## GitHub as a Notes Sync Target

Notes are plain `.md` files. The `$NOTES_ROOT` directory can be a git repository synced to GitHub (or any git host). This requires no code changes and works with any deployment mode.

```bash
cd ~/notes
git init && git remote add origin git@github.com:you/notes.git
git push
```

This gives you: version history, diffs, backup, and access from any device that can run git.

A `GitStorageBackend` (notes stored directly in a git repo, commits on save) is a possible future extension but is not planned for v1.

---

## Configuration Reference

All configuration is via environment variables (or `~/.config/nyx-notes/config.toml` for CLI):

| Env var | Default | Relevant modes |
|---|---|---|
| `NOTES_ROOT` | `~/notes` | All |
| `AUTH_MODE` | `local` | All |
| `PORT` | `8080` | Server modes |
| `FRONTEND_DIST` | `./dist` | Server modes |
| `NOTES_SECRET_KEY_PATH` | `~/.config/nyx-notes/secret.key` | `secret_key` |
| `NOTES_SECRET_KEY` | — | `secret_key` (alternative to file) |
| `FIREBASE_PROJECT_ID` | — | `firebase` |
| `OIDC_ISSUER_URL` | — | `oidc` |
| `OIDC_CLIENT_ID` | — | `oidc` |
