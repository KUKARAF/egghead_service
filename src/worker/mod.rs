pub mod cleanup;
pub mod estimator;
pub mod generator;
pub mod oidc_retry;
pub mod git;

use crate::state::AppState;
use std::sync::Arc;

pub fn spawn_all(state: Arc<AppState>) {
    tokio::spawn(estimator::run(Arc::clone(&state)));
    tokio::spawn(generator::run(Arc::clone(&state)));
    tokio::spawn(cleanup::run(Arc::clone(&state)));

    if state.oidc_client.try_read().map(|g| g.is_none()).unwrap_or(false) {
        tokio::spawn(oidc_retry::run(Arc::clone(&state)));
    }
}
