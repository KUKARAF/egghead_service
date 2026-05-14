use crate::{
    auth::extractors::SessionAuth,
    error::AppError,
    frontend::templates::{DASHBOARD_HTML, INDEX_HTML, SETTINGS_HTML},
    state::AppState,
};
use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect, Response},
};
use std::sync::Arc;

pub async fn serve_index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

pub async fn serve_dashboard(
    State(_state): State<Arc<AppState>>,
    session: Result<SessionAuth, AppError>,
) -> Response {
    if session.is_err() {
        return Redirect::to("/auth/login").into_response();
    }
    Html(DASHBOARD_HTML).into_response()
}

pub async fn serve_settings(
    State(_state): State<Arc<AppState>>,
    session: Result<SessionAuth, AppError>,
) -> Response {
    if session.is_err() {
        return Redirect::to("/auth/login").into_response();
    }
    Html(SETTINGS_HTML).into_response()
}
