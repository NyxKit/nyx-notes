# Interface

User-facing interfaces for Nyx Notes. There are two: a web frontend and a CLI.

## Documents

| File | What it covers |
|---|---|
| [frontend.md](./frontend.md) | Vue 3 SPA — components, views, routing, editor, comments, permissions UI |
| [cli.md](./cli.md) | Terminal CLI — commands, vault context, configuration |
| [native-app.md](./native-app.md) | Tauri native app — embedded server, platform targets, distribution |

## Summary

### Frontend

A Vue 3 SPA using `@nyxkit/nyx-kit` as the component library and TipTap as the Markdown editor. Auth UI adapts to the server's `AUTH_MODE` — no login screen in `local` mode, a simple form in `secret_key` mode, redirect flow in `oidc` mode. Three-panel layout: vault/note list · editor · comment sidebar.

Key views: `NoteView`, `VaultSettingsView`, `TeamSettingsView`, `LoginView`.

### CLI (`notes-cli`)

Direct filesystem access — no running server needed. A thin wrapper around `FsStorage` for terminal-first workflows. Supports vault context via `--vault` flag, `NOTES_VAULT` env var, or config file.

Key commands: `notes new`, `notes edit`, `notes list`, `notes search`, `notes vault`, `notes team`.

### Native App (Tauri)

A desktop app for macOS, Windows, and Linux. Bundles the Axum server as a background thread and the Vue SPA as a webview. Runs in `local` auth mode by default — no account, no internet required. Can also connect to a remote server for self-hosted or cloud instances.
