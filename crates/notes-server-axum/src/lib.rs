pub mod auth_extractor;
pub mod error;
pub mod routes;
pub mod storage_adapter;
pub mod types;

use std::sync::Arc;

use notes_core::AuthStore;
use storage_adapter::AsyncStorageAdapter;
use types::AuthConfig;

/// Shared state injected into every Axum handler via `State<AppState>`.
#[derive(Clone)]
pub struct AppState {
    pub storage: AsyncStorageAdapter,
    pub auth: Arc<dyn AuthStore>,
    pub auth_config: AuthConfig,
}
