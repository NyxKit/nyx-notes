use std::{path::PathBuf, sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use axum::{body::{to_bytes, Body}, http::Request};
use notes_auth::SecretKeyAuthStore;
use notes_core::{AuthStore, Note, NoteMeta, NotePermission, Vault, VaultOwner};
use notes_server_axum::{live::broker::LiveBroker, routes, storage_adapter::AsyncStorageAdapter, types::AuthConfig, AppState};
use notes_storage_fs::FsStorage;
use tower::ServiceExt;

fn temp_root() -> PathBuf {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    std::env::temp_dir().join(format!("nyx-notes-sync-test-{nanos}"))
}

#[tokio::test]
async fn admin_sync_homes_rewrites_note_author_ids_to_canonical_user_ids() {
    let root = temp_root();
    let storage = FsStorage::new(&root);
    let storage_adapter = AsyncStorageAdapter::new(Arc::new(storage));
    let storage_for_check = storage_adapter.clone();
    let auth_store = SecretKeyAuthStore::new(&root, &[9; 32], "main-server").unwrap();
    auth_store.setup_initial_user(notes_core::CreateUserInput {
        username: "admin".into(),
        email: "admin@example.com".into(),
        display_name: "Admin".into(),
        role: Some(notes_core::ServerRole::Admin),
        password: "Correct-password1".into(),
    }).unwrap();
    let admin = auth_store.login("admin", "Correct-password1").unwrap();
    let admin_user = auth_store.verify_token(&admin.token).unwrap();
    auth_store.create_user(
        &admin_user,
        notes_core::CreateUserInput {
            username: "alice".into(),
            email: "alice@example.com".into(),
            display_name: "Alice".into(),
            role: Some(notes_core::ServerRole::User),
            password: "Correct-password1".into(),
        },
    ).unwrap();
    let alice = auth_store.login("alice", "Correct-password1").unwrap();
    let alice_user = auth_store.verify_token(&alice.token).unwrap();

    let vault = Vault {
        id: "vault-1".into(),
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
    storage_adapter.create_vault(vault.clone()).await.unwrap();
    storage_adapter.save_note(vault.owner.clone(), Note {
        meta: NoteMeta {
            id: "note-1".into(),
            vault_id: vault.slug.clone(),
            title: "Draft".into(),
            description: None,
            author_id: alice_user.username.clone(),
            images: Vec::new(),
            tags: Vec::new(),
            category: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            is_encrypted: false,
            permission: NotePermission::Edit,
            feedback_type: None,
            app_location: None,
            storage_path: None,
            console_output: None,
            interaction_trail: None,
        },
        content: "Hello".into(),
    }).await.unwrap();

    let app = routes::router().with_state(AppState {
        storage: storage_adapter,
        live_broker: LiveBroker::default(),
        auth: Arc::new(auth_store),
        auth_config: AuthConfig::SecretKey,
        root_path: root.clone(),
    });

    let response = app
        .oneshot(
            Request::post("/api/admin/sync-homes")
                .header("authorization", format!("Bearer {}", admin.token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(response.status().is_success());

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json = serde_json::from_slice::<serde_json::Value>(&body).unwrap();
    assert_eq!(json["homes_scanned"], 1);
    assert_eq!(json["notes_fixed"], 1);

    let loaded = storage_for_check.load_note(vault.owner.clone(), vault.id.clone(), "note-1".to_string()).await.unwrap();
    assert_eq!(loaded.meta.author_id, alice_user.id);

    let _ = std::fs::remove_dir_all(root);
}
