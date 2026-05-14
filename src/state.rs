use crate::config::Config;
use openidconnect::core::CoreClient;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub pool: SqlitePool,
    pub config: Config,
    pub oidc_client: Arc<RwLock<Option<CoreClient>>>,
    pub http_client: reqwest::Client,
}

impl AppState {
    pub fn new(pool: SqlitePool, config: Config, oidc_client: Option<CoreClient>) -> Arc<Self> {
        Arc::new(AppState {
            pool,
            config,
            oidc_client: Arc::new(RwLock::new(oidc_client)),
            http_client: reqwest::Client::new(),
        })
    }
}
