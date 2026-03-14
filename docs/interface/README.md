# Interface

User-facing interfaces for Nyx Notes. There are two: a web frontend and a CLI.

## Documents

| File | What it covers |
|---|---|
| [frontend.md](./frontend.md) | Vue 3 SPA — components, views, routing, editor, comments, permissions UI |
| [cli.md](./cli.md) | Terminal CLI — commands, vault context, configuration |

## Summary

### Frontend

A Vue 3 SPA using `@nyxkit/nyx-kit` as the component library and TipTap as the Markdown editor. Authenticates via Firebase JS SDK. Three-panel layout: vault/note list · editor · comment sidebar.

Key views: `NoteView`, `VaultSettingsView`, `TeamSettingsView`, `LoginView`.

### CLI (`notes-cli`)

Direct filesystem access — no running server needed. A thin wrapper around `FsStorage` for terminal-first workflows. Supports vault context via `--vault` flag, `NOTES_VAULT` env var, or config file.

Key commands: `notes new`, `notes edit`, `notes list`, `notes search`, `notes vault`, `notes team`.
