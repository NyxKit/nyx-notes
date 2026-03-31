use std::{path::PathBuf, sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use axum::{body::{to_bytes, Body}, http::{Request, StatusCode}};
use notes_auth_local::{LocalAuthStore, SecretKeyAuthStore};
use notes_core::AuthStore;
use notes_server_axum::{routes, storage_adapter::AsyncStorageAdapter, types::AuthConfig, AppState};
use notes_storage_fs::FsStorage;
use tower::ServiceExt;

fn temp_root() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("nyx-notes-auth-test-{nanos}"))
}

#[tokio::test]
async fn auth_mode_reports_optional_api_version_metadata() {
    let root = temp_root();
    let storage = FsStorage::new(&root);

    let app = routes::router().with_state(AppState {
        storage: AsyncStorageAdapter::new(Arc::new(storage)),
        auth: Arc::new(LocalAuthStore::new("local".into(), "Local User".into())),
        auth_config: AuthConfig::SecretKey,
    });

    let response = app
        .oneshot(Request::get("/api/auth/mode").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json = serde_json::from_slice::<serde_json::Value>(&body).unwrap();
    assert_eq!(json["mode"], "secret_key");
    assert_eq!(json["api_version"], env!("CARGO_PKG_VERSION"));

    let _ = std::fs::remove_dir_all(root);
}

#[tokio::test]
async fn login_returns_422_when_auth_mode_does_not_support_password_login() {
    let root = temp_root();
    let storage = FsStorage::new(&root);

    let app = routes::router().with_state(AppState {
        storage: AsyncStorageAdapter::new(Arc::new(storage)),
        auth: Arc::new(LocalAuthStore::new("local".into(), "Local User".into())),
        auth_config: AuthConfig::Local,
    });

    let response = app
        .oneshot(
            Request::post("/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"username":"alice","password":"bad"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let _ = std::fs::remove_dir_all(root);
}

#[tokio::test]
async fn login_returns_401_for_invalid_secret_key_credentials() {
    let root = temp_root();
    let _ = std::fs::create_dir_all(&root);
    let storage = FsStorage::new(&root);
    let auth: Arc<dyn AuthStore> = Arc::new(
        SecretKeyAuthStore::new(&root, &[7; 32], Some("correct-password")).unwrap(),
    );

    let app = routes::router().with_state(AppState {
        storage: AsyncStorageAdapter::new(Arc::new(storage)),
        auth,
        auth_config: AuthConfig::SecretKey,
    });

    let response = app
        .oneshot(
            Request::post("/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"username":"admin","password":"wrong-password"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let _ = std::fs::remove_dir_all(root);
}
