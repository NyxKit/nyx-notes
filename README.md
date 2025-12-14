# Nyx Notes (working title)

A self-hosted, Markdown-first notes app built in Rust.

* 🗂 Notes are stored as plain `.md` files on disk (filesystem is the **source of truth**).
* 🧱 Backend is a portable Rust server (Axum), designed to run on:

  * any Linux server (incl. NAS),
  * your local machine,
  * behind any static/frontend host (Netlify/Firebase/etc.).
* 🔐 Future: optional **end-to-end encryption (E2EE)** and multi-user auth.
* 🤖 Optional (opt-in) AI assistant later – *never required*.

---

## Goals

* **Filesystem-native**
  All content is plain Markdown, editable by any text editor, git-friendly, and easy to back up.

* **Portable**
  Single binary / container that runs on Linux servers, NAS devices, and desktops.

* **Layered design**
  Core logic lives in a reusable crate; storage/auth implementations are pluggable.

* **Privacy-first**
  Design keeps the door open for true end-to-end encryption. AI features are explicitly opt-in and may bypass E2EE only with user consent.

---

## Project Structure

Planned layout as a Cargo workspace + separate frontend:

```text
nyx-notes/
  Cargo.toml          # workspace manifest
  crates/
    notes-core/       # domain types, traits, no IO/frameworks
    notes-storage-fs/ # filesystem implementation of StorageBackend
    notes-server-axum/# HTTP API server (Axum)
    notes-cli/        # optional CLI (later)
  frontend/           # Vue/Nuxt/other SPA/PWA
```

### Backend crates

#### `notes-core`

Pure domain logic:

* Core types (`NoteMeta`, `Note`)
* Storage and auth traits
* Frontmatter parsing helpers (later)
* Simple search helpers (later)

**Conceptual definitions (Rust-like pseudocode):**

```rust
pub struct NoteMeta {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub category: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Indicates whether the note body is encrypted (for future E2EE).
    pub is_encrypted: bool,
}

pub struct Note {
    pub meta: NoteMeta,
    /// Raw note body:
    /// - Non-E2EE mode: plaintext Markdown.
    /// - E2EE mode: ciphertext (opaque to the server).
    pub content: String,
}

/// Abstract storage over notes. The backend should not assume that
/// `content` is plaintext; it just stores and retrieves bytes/strings.
pub trait StorageBackend: Send + Sync {
    fn list_notes(&self, user_id: &str) -> Result<Vec<NoteMeta>, StorageError>;
    fn load_note(&self, id: &str, user_id: &str) -> Result<Note, StorageError>;
    fn save_note(&self, note: &Note, user_id: &str) -> Result<(), StorageError>;
    fn delete_note(&self, id: &str, user_id: &str) -> Result<(), StorageError>;
}
```

Later, an `AuthStore` trait will handle users and sessions:

```rust
pub struct User {
    pub id: String,
    pub email: String,
    pub display_name: String,
    // Passwords are never stored in plaintext.
}

pub trait AuthStore: Send + Sync {
    fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AuthError>;
    fn verify_password(&self, user: &User, password: &str) -> Result<bool, AuthError>;
    fn create_session(&self, user_id: &str) -> Result<String, AuthError>; // returns session/token id
}
```

#### `notes-storage-fs`

Implements `StorageBackend` using the local filesystem:

* Root directory: `NOTES_ROOT` (configurable via env or config file)
* Each note is a `.md` file with frontmatter + body.
* No HTTP or DB logic; only:

  * `std::fs` / `tokio::fs`
  * `serde` + YAML/TOML for frontmatter (later)

**Conceptual implementation sketch:**

```rust
pub struct FsStorage {
    root: std::path::PathBuf,
}

impl FsStorage {
    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

impl StorageBackend for FsStorage {
    fn list_notes(&self, user_id: &str) -> Result<Vec<NoteMeta>, StorageError> {
        // Walk `self.root`, find .md files, parse frontmatter -> NoteMeta
        // For now, `user_id` can be ignored in single-user mode.
        todo!()
    }

    fn load_note(&self, id: &str, user_id: &str) -> Result<Note, StorageError> {
        // Open `<id>.md`, split frontmatter/body -> Note
        todo!()
    }

    fn save_note(&self, note: &Note, user_id: &str) -> Result<(), StorageError> {
        // Write frontmatter + note.content into `<id>.md`
        todo!()
    }

    fn delete_note(&self, id: &str, user_id: &str) -> Result<(), StorageError> {
        // Remove `<id>.md`
        todo!()
    }
}
```

#### `notes-server-axum`

Axum HTTP API that wraps a `StorageBackend` (e.g. `FsStorage`).

Responsibilities:

* HTTP routes
* JSON (de)serialization
* Auth middleware (later)
* Logging / DB access (later)

**Minimal example (very early stage):**

```rust
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use notes_core::{Note, NoteMeta, StorageBackend};

#[derive(Clone)]
struct AppState<B: StorageBackend> {
    storage: B,
}

#[tokio::main]
async fn main() {
    let root = std::env::var("NOTES_ROOT").unwrap_or_else(|_| "notes".to_string());
    let storage = notes_storage_fs::FsStorage::new(root);
    let state = AppState { storage };

    let app = Router::new()
        .route("/api/notes", get(list_notes).post(save_note))
        // later: .route("/api/notes/:id", get(get_note).put(update_note).delete(delete_note))
        .with_state(state);

    let addr = ([0, 0, 0, 0], 8080).into();
    println!("Listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn list_notes<B: StorageBackend>(
    State(state): State<AppState<B>>,
) -> Json<Vec<NoteMeta>> {
    // For now, single-user: "default"
    let notes = state.storage.list_notes("default").unwrap_or_default();
    Json(notes)
}

#[derive(serde::Deserialize)]
struct SaveNotePayload {
    id: String,
    title: String,
    content: String,
}

async fn save_note<B: StorageBackend>(
    State(state): State<AppState<B>>,
    Json(payload): Json<SaveNotePayload>,
) {
    // In a real implementation, add error handling and avoid overwriting
    // `created_at` when updating existing notes.
    let meta = NoteMeta {
        id: payload.id,
        title: payload.title,
        tags: vec![],
        category: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        is_encrypted: false,
    };

    let note = Note {
        meta,
        content: payload.content,
    };

    let _ = state.storage.save_note(&note, "default");
}
```

> **Note:** Static frontend files (built SPA) can be served by Axum at `/` later using `tower-http`'s `ServeDir`.

---

## Frontend (PWA / Web UI)

* Lives in `frontend/` (e.g. Nuxt/Vue, React, Svelte – up to you).
* Talks to the backend via `/api/*`:

  * `GET /api/notes` → list notes
  * `GET /api/notes/:id` → open a note
  * `POST /api/notes` → create/update
* Provides:

  * Markdown source editor
  * Preview / split view
  * Tags / categories UX

The frontend can be:

* Served by the Rust server (static files), **or**
* Deployed to a static host (Netlify, Firebase, etc.) and configured to use your Rust backend as the API.

---

## Database & Auth (future)

Notes content remains on disk as `.md` files.

A database is introduced later for **app-level data**:

* users & auth
* sessions/tokens
* logs/audit trail
* search index metadata (optional)

Initial choice: **SQLite** (portable and ideal for self-host/NAS).

Later, add **PostgreSQL** support for larger/multi-user deployments.

Conceptual auth types in `notes-core`:

```rust
pub struct User {
    pub id: String,
    pub email: String,
    pub display_name: String,
    // Passwords are never stored in plaintext.
}

pub trait AuthStore: Send + Sync {
    fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AuthError>;
    fn verify_password(&self, user: &User, password: &str) -> Result<bool, AuthError>;
    fn create_session(&self, user_id: &str) -> Result<String, AuthError>; // returns session/token id
}
```

Axum middleware will later use `AuthStore` to protect routes.

---

## End-to-End Encryption (E2EE) – Future Direction

Design choices that keep E2EE possible:

* `Note.content` is treated as **opaque data** by the server.
* Server only understands and stores **metadata** (`NoteMeta`) and never needs to parse `content`.
* E2EE mode:

  * Client encrypts `content` before sending it to the server.
  * Server stores ciphertext as-is (in files or DB).
  * Decryption keys live only on user devices (or are derived from a passphrase).
* AI features (if added) are **explicitly opt-in**:

  * Either run client-side,
  * Or clearly marked operations that send plaintext to a server-side assistant (non-E2EE by definition).

This README describes the **intended architecture**, not a fully implemented app (yet). The plan is to evolve from:

1. Single-user, filesystem-only, simple Axum API.
2. Add DB-backed auth/logging.
3. Add multi-user support + optional E2EE.
4. Add optional AI/integrations on top.
