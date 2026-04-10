use std::{path::PathBuf, sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}};

use axum::{body::Body, http::{header, Request, StatusCode}};
use http_body_util::BodyExt;
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

    let response = app.clone()
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
    let mut body = response.into_body();
    let frame = tokio::time::timeout(Duration::from_secs(2), body.frame()).await.unwrap().unwrap().unwrap();
    let text = String::from_utf8(frame.into_data().unwrap().to_vec()).unwrap();
    assert!(text.contains("\"type\":\"snapshot\""));
    assert!(text.contains("\"slug\":\"writing\""));

    let _ = std::fs::remove_dir_all(root);
}

#[tokio::test]
async fn live_note_scope_returns_sse_snapshot() {
    let (app, _admin_token, alice_token, root) = seeded_app().await;

    let response = app.clone()
        .oneshot(
            Request::get("/api/live?collection=note&scope_kind=document&server_slug=main-server&vault_id=writing&note_id=note-1")
                .header("authorization", format!("Bearer {alice_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let mut body = response.into_body();
    let frame = tokio::time::timeout(Duration::from_secs(2), body.frame()).await.unwrap().unwrap().unwrap();
    let text = String::from_utf8(frame.into_data().unwrap().to_vec()).unwrap();
    assert!(text.contains("\"title\":\"Draft\""));
    assert!(text.contains("\"content\":\"Hello live\""));

    let _ = std::fs::remove_dir_all(root);
}

#[tokio::test]
async fn live_note_scope_requires_note_id_for_document_queries() {
    let (app, _admin_token, alice_token, root) = seeded_app().await;

    let response = app.clone()
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

#[tokio::test]
async fn live_note_scope_pushes_updates_to_connected_clients() {
    let (app, _admin_token, alice_token, root) = seeded_app().await;

    let response = app.clone()
        .oneshot(
            Request::get("/api/live?collection=note_list&scope_kind=collection&server_slug=main-server&vault_id=writing")
                .header("authorization", format!("Bearer {alice_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let mut body = response.into_body();
    let first = tokio::time::timeout(Duration::from_secs(2), body.frame()).await.unwrap().unwrap().unwrap();
    let first_text = String::from_utf8(first.into_data().unwrap().to_vec()).unwrap();
    assert!(first_text.contains("Draft"));

    let create_response = app.clone()
        .oneshot(
            Request::post("/api/vaults/writing/notes")
                .header("authorization", format!("Bearer {alice_token}"))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::json!({
                    "title": "Second",
                    "content": "live update",
                    "tags": [],
                    "category": null,
                    "images": [],
                    "permission": "edit"
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::CREATED);

    let second = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            if let Some(frame) = body.frame().await {
                let frame = frame.unwrap();
                if let Ok(data) = frame.into_data() {
                    let text = String::from_utf8(data.to_vec()).unwrap();
                    if text.contains("Second") {
                        return text;
                    }
                }
            }
        }
    }).await.unwrap();

    assert!(second.contains("Second"));
    let _ = std::fs::remove_dir_all(root);
}

#[tokio::test]
async fn live_broker_fanout_multiple_listeners() {
    use notes_server_axum::live::broker::LiveBroker;

    let broker = LiveBroker::default();
    let key = "test-scope";

    let count1 = broker.attach(key);
    assert_eq!(count1, 1);

    let count2 = broker.attach(key);
    assert_eq!(count2, 2);

    let count3 = broker.listeners(key);
    assert_eq!(count3, 2);

    let remaining = broker.release(key);
    assert_eq!(remaining, 1);

    let final_count = broker.release(key);
    assert_eq!(final_count, 0);
}

#[tokio::test]
async fn live_broker_version_increments_on_publish() {
    use notes_server_axum::live::broker::LiveBroker;

    let broker = LiveBroker::default();
    let key = "test-scope";

    let v1 = broker.publish(key);
    assert_eq!(v1, 1);

    let v2 = broker.version(key);
    assert_eq!(v2, 1);

    let v3 = broker.publish(key);
    assert_eq!(v3, 2);

    let v4 = broker.version(key);
    assert_eq!(v4, 2);
}
