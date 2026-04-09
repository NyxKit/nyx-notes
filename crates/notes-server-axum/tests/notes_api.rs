use std::{path::PathBuf, sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use axum::{body::{to_bytes, Body}, http::{Request, StatusCode}};
use notes_auth::SecretKeyAuthStore;
use notes_core::{AuthStore, Note, NoteMeta, NotePermission, StorageBackend, Vault, VaultOwner};
use notes_server_axum::{routes, storage_adapter::AsyncStorageAdapter, types::AuthConfig, AppState};
use notes_storage_fs::FsStorage;
use tower::ServiceExt;

fn temp_root() -> PathBuf {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    std::env::temp_dir().join(format!("nyx-notes-notes-test-{nanos}"))
}

#[tokio::test]
async fn regular_users_can_list_server_vault_notes() {
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
        id: "shared-notes".into(),
        slug: "shared-notes".into(),
        name: "Shared Notes".into(),
        description: None,
        owner: VaultOwner::Server { server_slug: "main-server".into() },
        permission: NotePermission::Edit,
        icon: None,
    };
    storage.create_vault(&vault).unwrap();
    let now = chrono::Utc::now();
    storage.save_note(&vault.owner, &Note {
        meta: NoteMeta {
            id: "note-1".into(),
            vault_id: "shared-notes".into(),
            title: "Shared note".into(),
            description: None,
            author_id: admin_user.id.clone(),
            images: Vec::new(),
            tags: Vec::new(),
            category: None,
            created_at: now,
            updated_at: now,
            is_encrypted: false,
            permission: NotePermission::Restricted,
            feedback_type: None,
            app_location: None,
            storage_path: None,
            console_output: None,
            interaction_trail: None,
        },
        content: "Shared body".into(),
    }).unwrap();

    let app = routes::router().with_state(AppState {
        storage: AsyncStorageAdapter::new(Arc::new(storage)),
        auth: Arc::new(auth),
        auth_config: AuthConfig::SecretKey,
        root_path: root.clone(),
    });

    let response = app
        .oneshot(
            Request::get("/api/vaults/shared-notes/notes")
                .header("authorization", format!("Bearer {alice_token}", alice_token = alice.token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json = serde_json::from_slice::<serde_json::Value>(&body).unwrap();
    assert_eq!(json.as_array().unwrap().len(), 1);
    assert_eq!(json[0]["id"], "note-1");

    let _ = std::fs::remove_dir_all(root);
}
