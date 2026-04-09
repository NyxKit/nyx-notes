use std::{path::PathBuf, sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use axum::{body::{to_bytes, Body}, http::{header, Request, StatusCode}};
use notes_auth::SecretKeyAuthStore;
use notes_core::{AuthStore, Note, NoteMeta, NotePermission, StorageBackend, Vault, VaultOwner};
use notes_server_axum::{live::broker::LiveBroker, routes, storage_adapter::AsyncStorageAdapter, types::AuthConfig, AppState};
use notes_storage_fs::FsStorage;
use tower::ServiceExt;

fn temp_root() -> PathBuf {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    std::env::temp_dir().join(format!("nyx-notes-live-test-{nanos}"))
}

async fn seeded_app() -> (axum::Router, String, String, String) {
    let root = temp_root();
    let storage = FsStorage::new(&root);
    let auth = SecretKeyAuthStore::new(&root, &[9; 32], "main-server").unwrap();
    auth.setup_initial_user(notes_core::CreateUserInput {
        username: "admin".into(),
        email: "admin@example.com".into(),
        display_name: "Admin".into(),
        role: Some(notes_core::ServerRole::Admin),
        password: "Correct-password1".into(),
    }).unwrap();
    let admin = auth.login("admin", "Correct-password1").unwrap();
    let admin_user = auth.verify_token(&admin.token).unwrap();
    auth.create_user(
        &admin_user,
        notes_core::CreateUserInput {
            username: "alice".into(),
            email: "alice@example.com".into(),
            display_name: "Alice".into(),
            role: Some(notes_core::ServerRole::User),
            password: "Correct-password1".into(),
        },
    ).unwrap();
    let alice = auth.login("alice", "Correct-password1").unwrap();

    let vault = Vault {
        id: "writing".into(),
        slug: "writing".into(),
        name: "Writing".into(),
        description: None,
        owner: VaultOwner::Home {
            server_slug: "main-server".into(),
            home_slug: "alice".into(),
        },
        permission: NotePermission::Edit,
        icon: None,
    };
    storage.create_vault(&vault).unwrap();
    let now = chrono::Utc::now();
    storage.save_note(&vault.owner, &Note {
        meta: NoteMeta {
            id: "note-1".into(),
            vault_id: vault.slug.clone(),
            title: "Draft".into(),
            description: None,
            author_id: admin_user.id.clone(),
            images: Vec::new(),
            tags: Vec::new(),
            category: None,
            created_at: now,
            updated_at: now,
            is_encrypted: false,
            permission: NotePermission::Edit,
            feedback_type: None,
            app_location: None,
            storage_path: None,
            console_output: None,
            interaction_trail: None,
        },
        content: "Hello live".into(),
    }).unwrap();

    let app = routes::router().with_state(AppState {
        storage: AsyncStorageAdapter::new(Arc::new(storage)),
        live_broker: LiveBroker::default(),
        auth: Arc::new(auth),
        auth_config: AuthConfig::SecretKey,
        root_path: root.clone(),
    });

    (app, admin.token, alice.token, root.to_string_lossy().to_string())
}

#[tokio::test]
async fn live_personal_vault_scope_returns_sse_snapshot() {
    let (app, _admin_token, alice_token, root) = seeded_app().await;

    let response = app
        .oneshot(
            Request::get("/api/live?collection=vault_list_personal&scope_kind=collection&server_slug=main-server")
                .header("authorization", format!("Bearer {alice_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get(header::CONTENT_TYPE).unwrap(), "text/event-stream");
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let text = String::from_utf8(body.to_vec()).unwrap();
    assert!(text.contains("event: snapshot"));
    assert!(text.contains("\"type\":\"snapshot\""));
    assert!(text.contains("\"slug\":\"writing\""));

    let _ = std::fs::remove_dir_all(root);
}

#[tokio::test]
async fn live_note_scope_returns_sse_snapshot() {
    let (app, _admin_token, alice_token, root) = seeded_app().await;

    let response = app
        .oneshot(
            Request::get("/api/live?collection=note&scope_kind=document&server_slug=main-server&vault_id=writing&note_id=note-1")
                .header("authorization", format!("Bearer {alice_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let text = String::from_utf8(body.to_vec()).unwrap();
    assert!(text.contains("\"title\":\"Draft\""));
    assert!(text.contains("\"content\":\"Hello live\""));

    let _ = std::fs::remove_dir_all(root);
}

#[tokio::test]
async fn live_note_scope_requires_note_id_for_document_queries() {
    let (app, _admin_token, alice_token, root) = seeded_app().await;

    let response = app
        .oneshot(
            Request::get("/api/live?collection=note&scope_kind=document&server_slug=main-server&vault_id=writing")
                .header("authorization", format!("Bearer {alice_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let _ = std::fs::remove_dir_all(root);
}
