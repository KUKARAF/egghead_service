use crate::{
    auth::extractors::SessionAuth,
    error::AppError,
    state::AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use std::sync::Arc;

// ── Error reports ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AdminSessionErrorResponse {
    pub id: String,
    pub session_id: String,
    pub session_name: String,
    pub url: String,
    pub reported_at: String,
    pub owner_email: String,
}

pub async fn list_admin_errors(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
) -> Result<Json<Vec<AdminSessionErrorResponse>>, AppError> {
    if !claims.is_superuser() {
        return Err(AppError::Forbidden("superuser only".into()));
    }

    #[derive(sqlx::FromRow)]
    struct Row {
        id: String,
        session_id: String,
        session_name: String,
        url: String,
        reported_at: String,
        owner_email: String,
    }

    let rows = sqlx::query_as::<_, Row>(
        "SELECT se.id, se.session_id, bs.name AS session_name, se.url, se.reported_at, u.email AS owner_email
         FROM session_errors se
         JOIN browsing_sessions bs ON bs.id = se.session_id
         JOIN users u ON u.id = bs.user_id
         ORDER BY se.reported_at DESC
         LIMIT 500",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(
        rows.into_iter()
            .map(|r| AdminSessionErrorResponse {
                id: r.id,
                session_id: r.session_id,
                session_name: r.session_name,
                url: r.url,
                reported_at: r.reported_at,
                owner_email: r.owner_email,
            })
            .collect(),
    ))
}

pub async fn get_admin_error_html(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
    Path(error_id): Path<String>,
) -> Result<axum::response::Html<String>, AppError> {
    if !claims.is_superuser() {
        return Err(AppError::Forbidden("superuser only".into()));
    }

    #[derive(sqlx::FromRow)]
    struct Row {
        url: String,
        page_html: String,
        reported_at: String,
        owner_email: String,
    }

    let row = sqlx::query_as::<_, Row>(
        "SELECT se.url, se.page_html, se.reported_at, u.email AS owner_email
         FROM session_errors se
         JOIN browsing_sessions bs ON bs.id = se.session_id
         JOIN users u ON u.id = bs.user_id
         WHERE se.id = ?",
    )
    .bind(&error_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let wrapper = format!(
        "<!-- Error report: {} ({}) at {} -->\n{}",
        row.url, row.owner_email, row.reported_at, row.page_html
    );
    Ok(axum::response::Html(wrapper))
}

// ── Browsing sessions ────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AdminPageResponse {
    pub id: String,
    pub url: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct AdminBrowsingSessionResponse {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub owner_email: String,
    pub pages: Vec<AdminPageResponse>,
}

pub async fn list_all_browsing_sessions(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
) -> Result<Json<Vec<AdminBrowsingSessionResponse>>, AppError> {
    if !claims.is_superuser() {
        return Err(AppError::Forbidden("superuser only".into()));
    }

    #[derive(sqlx::FromRow)]
    struct SessionRow {
        id: String,
        name: String,
        created_at: String,
        owner_email: String,
    }

    #[derive(sqlx::FromRow)]
    struct PageRow {
        id: String,
        session_id: String,
        url: String,
        created_at: String,
    }

    let sessions = sqlx::query_as::<_, SessionRow>(
        "SELECT bs.id, bs.name, bs.created_at, u.email AS owner_email
         FROM browsing_sessions bs
         JOIN users u ON u.id = bs.user_id
         ORDER BY bs.created_at DESC",
    )
    .fetch_all(&state.pool)
    .await?;

    let pages = sqlx::query_as::<_, PageRow>(
        "SELECT id, session_id, url, created_at FROM session_pages ORDER BY created_at ASC",
    )
    .fetch_all(&state.pool)
    .await?;

    let result = sessions
        .into_iter()
        .map(|s| {
            let session_pages = pages
                .iter()
                .filter(|p| p.session_id == s.id)
                .map(|p| AdminPageResponse {
                    id: p.id.clone(),
                    url: p.url.clone(),
                    created_at: p.created_at.clone(),
                })
                .collect();
            AdminBrowsingSessionResponse {
                id: s.id,
                name: s.name,
                created_at: s.created_at,
                owner_email: s.owner_email,
                pages: session_pages,
            }
        })
        .collect();

    Ok(Json(result))
}

#[derive(Debug, Serialize)]
pub struct AdminDeviceSessionView {
    pub id: String,
    pub device_name: String,
    pub created_at: String,
    pub expires_at: String,
    pub last_used_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminUserView {
    pub id: String,
    pub email: String,
    pub created_at: String,
    pub device_sessions: Vec<AdminDeviceSessionView>,
}

pub async fn list_all_users(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
) -> Result<Json<Vec<AdminUserView>>, AppError> {
    if !claims.is_superuser() {
        return Err(AppError::Forbidden("superuser only".into()));
    }

    #[derive(sqlx::FromRow)]
    struct UserRow {
        id: String,
        email: String,
        created_at: String,
    }

    #[derive(sqlx::FromRow)]
    struct TokenRow {
        id: String,
        user_id: String,
        device_name: String,
        created_at: String,
        expires_at: String,
        last_used_at: Option<String>,
    }

    let users = sqlx::query_as::<_, UserRow>(
        "SELECT id, email, created_at FROM users ORDER BY created_at DESC",
    )
    .fetch_all(&state.pool)
    .await?;

    let tokens = sqlx::query_as::<_, TokenRow>(
        "SELECT id, user_id, device_name, created_at, expires_at, last_used_at
         FROM device_tokens
         WHERE revoked_at IS NULL AND expires_at > datetime('now')
         ORDER BY created_at DESC",
    )
    .fetch_all(&state.pool)
    .await?;

    let result = users
        .into_iter()
        .map(|u| {
            let sessions = tokens
                .iter()
                .filter(|t| t.user_id == u.id)
                .map(|t| AdminDeviceSessionView {
                    id: t.id.clone(),
                    device_name: t.device_name.clone(),
                    created_at: t.created_at.clone(),
                    expires_at: t.expires_at.clone(),
                    last_used_at: t.last_used_at.clone(),
                })
                .collect();
            AdminUserView {
                id: u.id,
                email: u.email,
                created_at: u.created_at,
                device_sessions: sessions,
            }
        })
        .collect();

    Ok(Json(result))
}

pub async fn revoke_any_session(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
    Path(token_id): Path<String>,
) -> Result<StatusCode, AppError> {
    if !claims.is_superuser() {
        return Err(AppError::Forbidden("superuser only".into()));
    }

    let updated = sqlx::query(
        "UPDATE device_tokens SET revoked_at = datetime('now')
         WHERE id = ? AND revoked_at IS NULL",
    )
    .bind(&token_id)
    .execute(&state.pool)
    .await?
    .rows_affected();

    if updated == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
