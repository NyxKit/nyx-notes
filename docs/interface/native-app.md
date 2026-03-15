# Native App (Tauri)

## Purpose

A native desktop application for macOS, Windows, and Linux. Bundles the Axum backend and the Vue frontend into a single installable binary. No separate server, no browser required.

## Why Tauri

- The backend is already Rust — Tauri is a natural fit
- The frontend is already a web app — no rewrite needed
- Ships the system webview (not Chromium), keeping binary size small
- Single binary install experience

## Architecture

### Embedded server approach (v1)

The Axum server runs as a background thread inside the Tauri process. The Tauri webview connects to `localhost:{port}`.

```
┌─────────────────────────────────────────────────────┐
│  Tauri process                                      │
│                                                     │
│  ┌──────────────────┐    HTTP     ┌──────────────┐  │
│  │  Tauri webview   │ ──────────> │  Axum server │  │
│  │  (Vue SPA)       │ localhost   │  (background │  │
│  └──────────────────┘             │   thread)    │  │
│                                   └──────┬───────┘  │
│                                          │          │
│                                   ┌──────▼───────┐  │
│                                   │  FsStorage   │  │
│                                   │  (local disk)│  │
│                                   └──────────────┘  │
└─────────────────────────────────────────────────────┘
```

This approach reuses all existing HTTP routes, permission logic, and frontend code without modification. The only Tauri-specific code is the startup sequence.

### Future: Tauri IPC (v2 consideration)

Replace Axum routes with Tauri commands for direct Rust↔frontend calls (no HTTP overhead, no port management). This would introduce a `notes-tauri` crate alongside `notes-server-axum`. Not planned for v1 — the embedded server approach is sufficient and avoids duplicating the API contract.

## Auth

`AUTH_MODE=local` is hardcoded for the embedded server. There is no login screen. The app is single-user by default.

**Remote server mode:** The user can configure a remote server URL in app settings. When set, the webview talks to that server instead of the embedded one, and the auth mode of the remote server applies (e.g. `secret_key` for a home NAS, `oidc` for a cloud instance).

## Project Structure

```
src-tauri/
  src/
    main.rs           # Tauri app entry point; spawns Axum in background thread
    server.rs         # starts Axum on a free port, returns the bound address
  tauri.conf.json     # app name, identifier, window config, bundle targets
  Cargo.toml          # depends on notes-server-axum, notes-storage-fs, notes-auth-local
  icons/              # app icons for all platforms
```

### `main.rs` sketch

```rust
fn main() {
    let notes_root = dirs::home_dir().unwrap().join("notes");
    let port = pick_unused_port();

    // Spawn Axum in a background thread
    std::thread::spawn(move || {
        server::start(notes_root, port, AuthMode::Local);
    });

    // Wait for the server to be ready
    wait_for_port(port);

    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            window.eval(&format!("window.__API_BASE__ = 'http://localhost:{port}'"))?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running Tauri app");
}
```

## Notes Storage

Default `$NOTES_ROOT` for the native app:

| Platform | Default path |
|---|---|
| macOS | `~/Library/Application Support/nyx-notes/notes` |
| Windows | `%APPDATA%\nyx-notes\notes` |
| Linux | `~/.local/share/nyx-notes/notes` |

Configurable via app settings (stored in Tauri's app config dir). The chosen path is passed to `FsStorage` at startup.

## Window

- Default size: 1280×800, resizable
- Minimum size: 900×600
- Title bar: native (no custom titlebar in v1)
- Single window; no multi-window support in v1

## Distribution

| Platform | Format |
|---|---|
| macOS | `.dmg` (universal binary: Apple Silicon + Intel) |
| Windows | `.msi` + `.exe` installer |
| Linux | `.AppImage`, `.deb`, `.rpm` |

Built via `tauri build` using GitHub Actions. Binaries are signed (macOS notarization, Windows code signing) for distribution outside app stores.

## CLI and native app coexistence

The native app and the CLI share `$NOTES_ROOT`. The CLI operates directly on the filesystem; the native app runs the server on top of the same folder. They can be used interchangeably — a note created by the CLI appears immediately in the app (on next vault refresh) and vice versa.

## Non-Goals

- No iOS or Android (mobile is a future consideration)
- No Mac App Store / Microsoft Store distribution in v1 (signing complexity)
- No multi-window or split-instance support
- No custom title bar or window chrome (native OS chrome only)
