use std::{path::Path, sync::Arc};

use notes_auth::{load_or_generate_key, LocalAuthStore, SecretKeyAuthStore};
use notes_core::AuthStore;
use notes_server_axum::{
    routes,
    storage_adapter::AsyncStorageAdapter,
    types::AuthConfig,
    AppState,
};
use notes_storage_fs::FsStorage;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let server_name = std::env::var("SERVER_NAME").unwrap_or_else(|_| "Nyx Server".into());
    let server_slug = notes_core::slugify(&server_name);
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let auth_mode = std::env::var("AUTH_MODE").unwrap_or_else(|_| "local".into());

    // Expand a leading `~/` so that .env files can use tilde paths portably.
    let notes_root_raw = std::env::var("NYX_ROOT").unwrap_or_else(|_| "./nyx-server".into());
    let notes_root = if let Some(rest) = notes_root_raw.strip_prefix("~/") {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        format!("{home}/{rest}")
    } else {
        notes_root_raw
    };

    let notes_root_path = Path::new(&notes_root);
    std::fs::create_dir_all(notes_root_path).expect("failed to create NYX_ROOT");

    let (auth, auth_config): (Arc<dyn AuthStore>, AuthConfig) = match auth_mode.as_str() {
        "local" => {
            let user_id = std::env::var("LOCAL_USER_ID").unwrap_or_else(|_| "local".into());
            let name =
                std::env::var("NOTES_LOCAL_USER_NAME").unwrap_or_else(|_| "Local User".into());
            (Arc::new(LocalAuthStore::new(user_id, name)), AuthConfig::Local)
        }
        "secret_key" => {
            let key = load_or_generate_key();
            let store =
                SecretKeyAuthStore::new(notes_root_path, &key, &server_slug)
                    .expect("failed to initialise secret_key auth store");
            (Arc::new(store), AuthConfig::SecretKey)
        }
        "oidc" => {
            let _issuer = std::env::var("OIDC_ISSUER_URL")
                .expect("OIDC_ISSUER_URL is required for AUTH_MODE=oidc");
            let _client_id = std::env::var("OIDC_CLIENT_ID")
                .expect("OIDC_CLIENT_ID is required for AUTH_MODE=oidc");
            // OidcAuthStore is implemented in notes-auth-oidc (not yet available).
            panic!(
                "AUTH_MODE=oidc is not yet implemented. \
                 Set AUTH_MODE=local or AUTH_MODE=secret_key."
            );
            #[allow(unreachable_code)]
            (
                Arc::new(LocalAuthStore::new("local".into(), "oidc-placeholder".into())),
                AuthConfig::Oidc { issuer: _issuer, client_id: _client_id },
            )
        }
        other => panic!("Unknown AUTH_MODE={other}. Valid values: local, secret_key, oidc"),
    };

    let state = AppState {
        storage: AsyncStorageAdapter::new(Arc::new(FsStorage::new(notes_root_path))),
        auth,
        auth_config,
    };

    let app = routes::router().with_state(state);
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind");

    println!("listening on {addr} (AUTH_MODE={auth_mode})");
    axum::serve(listener, app).await.expect("server error");
}
