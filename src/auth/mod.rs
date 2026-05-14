pub mod extractors;
pub mod oidc;
pub mod session;
pub mod token;

use axum::{routing::{get, post}, Router};
use std::sync::Arc;
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", get(oidc::login))
        .route("/callback", get(oidc::callback))
        .route("/logout", post(oidc::logout))
}
