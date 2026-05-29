pub mod browsing_sessions;
pub mod devices;
pub mod html_chunks;
pub mod me;
pub mod tasks;
pub mod userscripts;
pub mod openapi;

use crate::state::AppState;
use axum::{
    routing::{delete, get, post, put},
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
        .route("/me/devices", get(devices::list_device_requests))
        .route("/me/devices/:id/approve", post(devices::approve_device_request))
        .route("/me/devices/:id/reject", post(devices::reject_device_request))
        .route("/me/sessions", get(me::list_device_sessions))
        .route("/me/sessions/:id/revoke", post(me::revoke_device_session))
        .route("/me/scripts", get(userscripts::list_userscripts))
        .route("/me/scripts", post(userscripts::create_userscript))
        .route("/me/scripts/:id", get(userscripts::get_userscript))
        .route("/me/scripts/:id", put(userscripts::update_userscript))
        .route("/me/scripts/:id", delete(userscripts::delete_userscript))
        .route("/me/scripts/:id/devices", get(userscripts::get_script_devices))
        .route("/me/scripts/:id/assign-devices", post(userscripts::assign_devices_to_script))
        .route("/devices/register", post(devices::register_device))
        .route("/devices/status/:id", get(devices::get_device_status))
        .route("/devices/emojis", get(devices::list_emojis))
        // Browsing sessions
        .route("/me/browsing-sessions", get(browsing_sessions::list_browsing_sessions))
        .route("/me/browsing-sessions", post(browsing_sessions::create_browsing_session))
        .route("/me/browsing-sessions/:id", delete(browsing_sessions::delete_browsing_session))
        .route("/me/browsing-sessions/:id/pages", post(browsing_sessions::add_session_page))
        .route("/me/browsing-sessions/:id/pages/:page_id", put(browsing_sessions::update_session_page))
        .route("/me/browsing-sessions/:id/pages/:page_id", delete(browsing_sessions::remove_session_page))
        // Error reporting (API token auth — called from extension)
        .route("/sessions/:id/errors", post(browsing_sessions::report_session_error))
        // Error listing (session auth — dashboard)
        .route("/me/errors", get(browsing_sessions::list_all_errors))
        .route("/me/errors/:id", get(browsing_sessions::get_session_error_html))
        .route("/me/errors/:id", delete(browsing_sessions::delete_session_error))
}
