use std::{path::PathBuf, sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use axum::{body::{to_bytes, Body}, http::{Request, StatusCode}};
use notes_auth::LocalAuthStore;
use notes_core::{
    Comment, CommentAnchor, CommentAttachment, CommentVisibility, Note, NoteMeta,
    NotePermission, StorageBackend, Vault, VaultOwner,
};
use notes_server_axum::{routes, storage_adapter::AsyncStorageAdapter, types::AuthConfig, AppState};
use notes_storage_fs::FsStorage;
use tower::ServiceExt;

fn temp_root() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("nyx-notes-server-test-{nanos}"))
}

fn seed_note(storage: &FsStorage, vault_id: &str, note_id: &str) {
    let now = chrono::Utc::now();
    let vault = Vault {
        id: vault_id.into(),
        slug: "home".into(),
        name: "Home".into(),
        description: None,
        owner: VaultOwner::Home {
            server_slug: "main-server".into(),
            home_slug: "local".into(),
        },
        permission: NotePermission::Restricted,
        icon: None,
    };
    storage.create_vault(&vault).unwrap();
    storage.save_note(&Note {
        meta: NoteMeta {
            id: note_id.into(),
            vault_id: vault_id.into(),
            title: "Note".into(),
            description: None,
            author_id: "local".into(),
            tags: Vec::new(),
            category: None,
            created_at: now,
            updated_at: now,
            is_encrypted: false,
            permission: NotePermission::Comment,
        },
        content: "A selected line of note text".into(),
    }).unwrap();
}

#[tokio::test]
async fn create_comment_accepts_structured_anchor_payloads() {
    let root = temp_root();
    let storage = FsStorage::new(&root);
    seed_note(&storage, "vault-1", "note-1");

    let app = routes::router().with_state(AppState {
        storage: AsyncStorageAdapter::new(Arc::new(storage)),
        auth: Arc::new(LocalAuthStore::new("local".into(), "Local User".into())),
        auth_config: AuthConfig::Local,
    });

    let response = app
        .oneshot(
            Request::post("/api/vaults/vault-1/notes/note-1/comments")
                .header("content-type", "application/json")
                .body(Body::from(r#"{
                    "body": "Please expand this thought.",
                    "anchor": {
                        "text": "selected line",
                        "prefix": "before ",
                        "suffix": " after",
                        "range_from": 4,
                        "range_to": 17,
                        "line_preview": "A selected line of note text"
                    }
                }"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json = serde_json::from_slice::<serde_json::Value>(&body).unwrap();
    assert_eq!(json["body"], "Please expand this thought.");
    assert_eq!(json["anchor"]["text"], "selected line");
    assert_eq!(json["anchor"]["line_preview"], "A selected line of note text");
    assert_eq!(json["visibility"], "visible");

    let _ = std::fs::remove_dir_all(root);
}

#[tokio::test]
async fn list_comments_omits_hidden_legacy_threads() {
    let root = temp_root();
    let storage = FsStorage::new(&root);
    seed_note(&storage, "vault-1", "note-1");

    let now = chrono::Utc::now();
    storage.save_comments(
        "vault-1",
        "note-1",
        &[
            Comment {
                id: "visible".into(),
                note_id: "note-1".into(),
                author_id: "local".into(),
                author_name: "Local User".into(),
                body: "Visible comment".into(),
                anchor: CommentAnchor {
                    text: "selected line".into(),
                    prefix: "before ".into(),
                    suffix: " after".into(),
                    range_from: 4,
                    range_to: 17,
                    attachment: CommentAttachment::Attached,
                    line_preview: "A selected line of note text".into(),
                    last_matched_at: Some(now),
                },
                resolved: false,
                visibility: CommentVisibility::Visible,
                created_at: now,
                updated_at: now,
                replies: Vec::new(),
            },
            Comment {
                id: "hidden".into(),
                note_id: "note-1".into(),
                author_id: "local".into(),
                author_name: "Local User".into(),
                body: "Hidden comment".into(),
                anchor: CommentAnchor {
                    text: "legacy quote".into(),
                    prefix: String::new(),
                    suffix: String::new(),
                    range_from: 0,
                    range_to: 0,
                    attachment: CommentAttachment::Detached,
                    line_preview: "legacy quote".into(),
                    last_matched_at: None,
                },
                resolved: false,
                visibility: CommentVisibility::HiddenLegacy,
                created_at: now,
                updated_at: now,
                replies: Vec::new(),
            },
        ],
    ).unwrap();

    let app = routes::router().with_state(AppState {
        storage: AsyncStorageAdapter::new(Arc::new(storage)),
        auth: Arc::new(LocalAuthStore::new("local".into(), "Local User".into())),
        auth_config: AuthConfig::Local,
    });

    let response = app
        .oneshot(
            Request::get("/api/vaults/vault-1/notes/note-1/comments")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json = serde_json::from_slice::<serde_json::Value>(&body).unwrap();
    assert_eq!(json.as_array().unwrap().len(), 1);
    assert_eq!(json[0]["id"], "visible");

    let _ = std::fs::remove_dir_all(root);
}
