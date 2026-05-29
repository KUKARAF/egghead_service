use crate::{
    auth::extractors::{AppTokenAuth, SessionAuth, UserAuth},
    error::AppError,
    state::AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct PageResponse {
    pub id: String,
    pub url: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct BrowsingSessionResponse {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub pages: Vec<PageResponse>,
}

#[derive(Debug, Serialize)]
pub struct SessionErrorResponse {
    pub id: String,
    pub session_id: String,
    pub session_name: String,
    pub url: String,
    pub reported_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct AddPageRequest {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePageRequest {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ReportErrorRequest {
    pub url: String,
    pub page_html: String,
}

async fn resolve_user_id(
    state: &Arc<AppState>,
    oidc_subject: &str,
) -> Result<String, AppError> {
    sqlx::query_scalar::<_, String>(
        "SELECT id FROM users WHERE oidc_subject = ?",
    )
    .bind(oidc_subject)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized)
}

pub async fn list_browsing_sessions(
    State(state): State<Arc<AppState>>,
    UserAuth { user_id, .. }: UserAuth,
) -> Result<Json<Vec<BrowsingSessionResponse>>, AppError> {

    #[derive(sqlx::FromRow)]
    struct SessionRow {
        id: String,
        name: String,
        created_at: String,
    }

    let sessions = sqlx::query_as::<_, SessionRow>(
        "SELECT id, name, created_at FROM browsing_sessions
         WHERE user_id = ? ORDER BY created_at DESC",
    )
    .bind(&user_id)
    .fetch_all(&state.pool)
    .await?;

    #[derive(sqlx::FromRow)]
    struct PageRow {
        id: String,
        session_id: String,
        url: String,
        created_at: String,
    }

    let all_pages = sqlx::query_as::<_, PageRow>(
        "SELECT sp.id, sp.session_id, sp.url, sp.created_at
         FROM session_pages sp
         JOIN browsing_sessions bs ON bs.id = sp.session_id
         WHERE bs.user_id = ?
         ORDER BY sp.created_at ASC",
    )
    .bind(&user_id)
    .fetch_all(&state.pool)
    .await?;

    let result = sessions
        .into_iter()
        .map(|s| {
            let pages = all_pages
                .iter()
                .filter(|p| p.session_id == s.id)
                .map(|p| PageResponse {
                    id: p.id.clone(),
                    url: p.url.clone(),
                    created_at: p.created_at.clone(),
                })
                .collect();
            BrowsingSessionResponse {
                id: s.id,
                name: s.name,
                created_at: s.created_at,
                pages,
            }
        })
        .collect();

    Ok(Json(result))
}

pub async fn create_browsing_session(
    State(state): State<Arc<AppState>>,
    UserAuth { user_id, .. }: UserAuth,
    Json(req): Json<CreateSessionRequest>,
) -> Result<(StatusCode, Json<BrowsingSessionResponse>), AppError> {
    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("name is required".into()));
    }
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO browsing_sessions (id, user_id, name, created_at, updated_at)
         VALUES (?, ?, ?, datetime('now'), datetime('now'))",
    )
    .bind(&id)
    .bind(&user_id)
    .bind(req.name.trim())
    .execute(&state.pool)
    .await?;

    #[derive(sqlx::FromRow)]
    struct Row {
        created_at: String,
    }
    let row = sqlx::query_as::<_, Row>(
        "SELECT created_at FROM browsing_sessions WHERE id = ?",
    )
    .bind(&id)
    .fetch_one(&state.pool)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(BrowsingSessionResponse {
            id,
            name: req.name.trim().to_string(),
            created_at: row.created_at,
            pages: vec![],
        }),
    ))
}

pub async fn delete_browsing_session(
    State(state): State<Arc<AppState>>,
    UserAuth { user_id, .. }: UserAuth,
    Path(session_id): Path<String>,
) -> Result<StatusCode, AppError> {

    let deleted = sqlx::query(
        "DELETE FROM browsing_sessions WHERE id = ? AND user_id = ?",
    )
    .bind(&session_id)
    .bind(&user_id)
    .execute(&state.pool)
    .await?
    .rows_affected();

    if deleted == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_session_page(
    State(state): State<Arc<AppState>>,
    UserAuth { user_id, .. }: UserAuth,
    Path(session_id): Path<String>,
    Json(req): Json<AddPageRequest>,
) -> Result<(StatusCode, Json<PageResponse>), AppError> {
    if req.url.trim().is_empty() {
        return Err(AppError::BadRequest("url is required".into()));
    }

    let owned: bool = sqlx::query_scalar(
        "SELECT COUNT(*) > 0 FROM browsing_sessions WHERE id = ? AND user_id = ?",
    )
    .bind(&session_id)
    .bind(&user_id)
    .fetch_one(&state.pool)
    .await?;

    if !owned {
        return Err(AppError::NotFound);
    }

    let page_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO session_pages (id, session_id, url, created_at)
         VALUES (?, ?, ?, datetime('now'))",
    )
    .bind(&page_id)
    .bind(&session_id)
    .bind(req.url.trim())
    .execute(&state.pool)
    .await?;

    #[derive(sqlx::FromRow)]
    struct Row {
        created_at: String,
    }
    let row = sqlx::query_as::<_, Row>(
        "SELECT created_at FROM session_pages WHERE id = ?",
    )
    .bind(&page_id)
    .fetch_one(&state.pool)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(PageResponse {
            id: page_id,
            url: req.url.trim().to_string(),
            created_at: row.created_at,
        }),
    ))
}

pub async fn update_session_page(
    State(state): State<Arc<AppState>>,
    UserAuth { user_id, .. }: UserAuth,
    Path((session_id, page_id)): Path<(String, String)>,
    Json(req): Json<UpdatePageRequest>,
) -> Result<StatusCode, AppError> {
    if req.url.trim().is_empty() {
        return Err(AppError::BadRequest("url is required".into()));
    }

    let updated = sqlx::query(
        "UPDATE session_pages SET url = ?
         WHERE id = ? AND session_id = (
             SELECT id FROM browsing_sessions WHERE id = ? AND user_id = ?
         )",
    )
    .bind(req.url.trim())
    .bind(&page_id)
    .bind(&session_id)
    .bind(&user_id)
    .execute(&state.pool)
    .await?
    .rows_affected();

    if updated == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn remove_session_page(
    State(state): State<Arc<AppState>>,
    UserAuth { user_id, .. }: UserAuth,
    Path((session_id, page_id)): Path<(String, String)>,
) -> Result<StatusCode, AppError> {

    let deleted = sqlx::query(
        "DELETE FROM session_pages
         WHERE id = ? AND session_id = (
             SELECT id FROM browsing_sessions WHERE id = ? AND user_id = ?
         )",
    )
    .bind(&page_id)
    .bind(&session_id)
    .bind(&user_id)
    .execute(&state.pool)
    .await?
    .rows_affected();

    if deleted == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn report_session_error(
    State(state): State<Arc<AppState>>,
    AppTokenAuth { user_id, .. }: AppTokenAuth,
    Path(session_id): Path<String>,
    Json(req): Json<ReportErrorRequest>,
) -> Result<StatusCode, AppError> {
    let owned: bool = sqlx::query_scalar(
        "SELECT COUNT(*) > 0 FROM browsing_sessions WHERE id = ? AND user_id = ?",
    )
    .bind(&session_id)
    .bind(&user_id)
    .fetch_one(&state.pool)
    .await?;

    if !owned {
        return Err(AppError::NotFound);
    }

    let error_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO session_errors (id, session_id, url, page_html, reported_at)
         VALUES (?, ?, ?, ?, datetime('now'))",
    )
    .bind(&error_id)
    .bind(&session_id)
    .bind(&req.url)
    .bind(&req.page_html)
    .execute(&state.pool)
    .await?;

    Ok(StatusCode::CREATED)
}

pub async fn list_all_errors(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
) -> Result<Json<Vec<SessionErrorResponse>>, AppError> {
    let user_id = resolve_user_id(&state, &claims.oidc_subject).await?;

    #[derive(sqlx::FromRow)]
    struct Row {
        id: String,
        session_id: String,
        session_name: String,
        url: String,
        reported_at: String,
    }

    let rows = sqlx::query_as::<_, Row>(
        "SELECT se.id, se.session_id, bs.name as session_name, se.url, se.reported_at
         FROM session_errors se
         JOIN browsing_sessions bs ON bs.id = se.session_id
         WHERE bs.user_id = ?
         ORDER BY se.reported_at DESC
         LIMIT 200",
    )
    .bind(&user_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(
        rows.into_iter()
            .map(|r| SessionErrorResponse {
                id: r.id,
                session_id: r.session_id,
                session_name: r.session_name,
                url: r.url,
                reported_at: r.reported_at,
            })
            .collect(),
    ))
}

pub async fn get_session_error_html(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
    Path(error_id): Path<String>,
) -> Result<axum::response::Html<String>, AppError> {
    let user_id = resolve_user_id(&state, &claims.oidc_subject).await?;

    #[derive(sqlx::FromRow)]
    struct Row {
        url: String,
        page_html: String,
        reported_at: String,
    }

    let row = sqlx::query_as::<_, Row>(
        "SELECT se.url, se.page_html, se.reported_at
         FROM session_errors se
         JOIN browsing_sessions bs ON bs.id = se.session_id
         WHERE se.id = ? AND bs.user_id = ?",
    )
    .bind(&error_id)
    .bind(&user_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let wrapper = format!(
        "<!-- Error report: {} at {} -->\n{}",
        row.url, row.reported_at, row.page_html
    );
    Ok(axum::response::Html(wrapper))
}

pub async fn delete_session_error(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
    Path(error_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let user_id = resolve_user_id(&state, &claims.oidc_subject).await?;

    let deleted = sqlx::query(
        "DELETE FROM session_errors
         WHERE id = ? AND session_id IN (
             SELECT id FROM browsing_sessions WHERE user_id = ?
         )",
    )
    .bind(&error_id)
    .bind(&user_id)
    .execute(&state.pool)
    .await?
    .rows_affected();

    if deleted == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
