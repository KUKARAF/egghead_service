pub mod html_chunks;
pub mod me;
pub mod tasks;
pub mod openapi;

use crate::state::AppState;
use axum::{
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/html-chunks", post(html_chunks::upload_chunk))
        .route("/tasks", post(tasks::create_task))
        .route("/tasks/:id", get(tasks::get_task))
        .route("/tasks/status/:token", get(tasks::get_task_by_token))
        .route("/me/tasks", get(me::list_tasks))
        .route("/me/tasks/:id", get(me::get_task_detail))
        .route("/me/tasks/:id/approve", post(me::approve_task))
        .route("/me/tasks/:id/reject", post(me::reject_task))
        .route("/me/tasks/:id", delete(me::delete_task))
        .route("/me/token", get(me::get_api_token))
        .route("/me/token/regenerate", post(me::regenerate_api_token))
}
