pub mod auth_extractor;
pub mod error;
pub mod live;
pub mod routes;
pub mod storage_adapter;
pub mod types;

use std::sync::Arc;
use std::path::PathBuf;

use live::broker::LiveBroker;
use notes_core::AuthStore;
use storage_adapter::AsyncStorageAdapter;
use types::AuthConfig;

/// Shared state injected into every Axum handler via `State<AppState>`.
#[derive(Clone)]
pub struct AppState {
    pub storage: AsyncStorageAdapter,
    pub live_broker: LiveBroker,
    pub auth: Arc<dyn AuthStore>,
    pub auth_config: AuthConfig,
    pub root_path: PathBuf,
}
