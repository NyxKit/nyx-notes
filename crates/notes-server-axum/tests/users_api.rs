use std::{
    path::PathBuf,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use notes_auth::{LocalAuthStore, SecretKeyAuthStore};
use notes_core::{AuthStore, CreateUserInput, ServerRole};
use notes_server_axum::{routes, storage_adapter::AsyncStorageAdapter, types::AuthConfig, AppState};
use notes_storage_fs::FsStorage;
use tower::ServiceExt;

fn temp_root() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("nyx-notes-users-test-{nanos}"))
}

fn test_app() -> (axum::Router, String, String) {
    let root = temp_root();
    let _ = std::fs::create_dir_all(&root);
    let storage = FsStorage::new(&root);
    let store = SecretKeyAuthStore::new(&root, &[9; 32], "main-server").unwrap();
    store.setup_initial_user(CreateUserInput {
        username: "admin".into(),
        email: "admin@example.com".into(),
        display_name: "Admin".into(),
        role: Some(ServerRole::Admin),
        password: "Correct-password1".into(),
    }).unwrap();
    let admin = store.login("admin", "Correct-password1").unwrap();
    let admin_user = store.verify_token(&admin.token).unwrap();
    store
        .create_user(
            &admin_user,
            CreateUserInput {
                username: "alice".into(),
                email: "alice@example.com".into(),
                display_name: "Alice".into(),
                role: Some(ServerRole::User),
                password: "Correct-password1".into(),
            },
        )
        .unwrap();
    let alice = store.login("alice", "Correct-password1").unwrap();

    let app = routes::router().with_state(AppState {
        storage: AsyncStorageAdapter::new(Arc::new(storage)),
        auth: Arc::new(store),
        auth_config: AuthConfig::SecretKey,
        root_path: root.clone(),
    });

    (app, admin.token, alice.token)
}

#[tokio::test]
async fn users_routes_return_422_in_local_mode() {
    let root = temp_root();
    let storage = FsStorage::new(&root);
    let app = routes::router().with_state(AppState {
        storage: AsyncStorageAdapter::new(Arc::new(storage)),
        auth: Arc::new(LocalAuthStore::new("local".into(), "Local User".into())),
        auth_config: AuthConfig::Local,
        root_path: root.clone(),
    });

    let response = app
        .oneshot(Request::get("/api/users").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn list_users_is_admin_only() {
    let (app, _admin_token, alice_token) = test_app();
    let response = app
        .oneshot(
            Request::get("/api/users")
                .header("authorization", format!("Bearer {alice_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn admin_can_list_and_create_users_with_duplicate_email_rejected() {
    let (app, admin_token, _alice_token) = test_app();

    let list_response = app
        .clone()
        .oneshot(
            Request::get("/api/users")
                .header("authorization", format!("Bearer {admin_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = to_bytes(list_response.into_body(), usize::MAX).await.unwrap();
    let users = serde_json::from_slice::<serde_json::Value>(&list_body).unwrap();
    assert_eq!(users.as_array().unwrap().len(), 2);

    let duplicate_response = app
        .clone()
        .oneshot(
            Request::post("/api/users")
                .header("authorization", format!("Bearer {admin_token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"username":"bob","email":"alice@example.com","display_name":"Bob","role":"user","password":"Correct-password1"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);

    let invalid_response = app
        .oneshot(
            Request::post("/api/users")
                .header("authorization", format!("Bearer {admin_token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"username":"bob","email":"","display_name":"Bob","role":"user","password":"weak"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(invalid_response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn blocks_self_delete_and_last_admin_demotion() {
    let (app, admin_token, _alice_token) = test_app();

    let list_response = app
        .clone()
        .oneshot(
            Request::get("/api/users")
                .header("authorization", format!("Bearer {admin_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let list_body = to_bytes(list_response.into_body(), usize::MAX).await.unwrap();
    let users = serde_json::from_slice::<serde_json::Value>(&list_body).unwrap();
    let admin_id = users
        .as_array()
        .unwrap()
        .iter()
        .find(|user| user["username"] == "admin")
        .unwrap()["id"]
        .as_str()
        .unwrap()
        .to_string();

    let self_delete = app
        .clone()
        .oneshot(
            Request::delete(format!("/api/users/{admin_id}"))
                .header("authorization", format!("Bearer {admin_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(self_delete.status(), StatusCode::CONFLICT);

    let self_demote = app
        .oneshot(
            Request::patch(format!("/api/users/{admin_id}"))
                .header("authorization", format!("Bearer {admin_token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"email":"admin@localhost","display_name":"Admin","role":"user"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(self_demote.status(), StatusCode::CONFLICT);
}
