pub mod pages;
pub mod templates;

use crate::state::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(pages::serve_index))
        .route("/dashboard", get(pages::serve_dashboard))
        .route("/settings", get(pages::serve_settings))
}
