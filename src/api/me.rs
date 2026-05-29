use crate::{
    auth::{
        extractors::{SessionAuth, UserAuth},
        token::generate_api_token,
    },
    error::AppError,
    models::task::{Task, TaskView},
    state::AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct DeviceSessionView {
    pub id: String,
    pub device_name: String,
    pub created_at: String,
    pub expires_at: String,
    pub last_used_at: Option<String>,
}

pub async fn list_device_sessions(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
) -> Result<Json<Vec<DeviceSessionView>>, AppError> {
    // Find the user for this OIDC session (may not exist if OIDC user never had a device).
    let user_id: Option<String> = sqlx::query_scalar(
        "SELECT id FROM users WHERE oidc_subject = ?",
    )
    .bind(&claims.oidc_subject)
    .fetch_optional(&state.pool)
    .await?;

    let Some(user_id) = user_id else {
        return Ok(Json(vec![]));
    };

    #[derive(sqlx::FromRow)]
    struct Row {
        id: String,
        device_name: String,
        created_at: String,
        expires_at: String,
        last_used_at: Option<String>,
    }

    let rows = sqlx::query_as::<_, Row>(
        "SELECT id, device_name, created_at, expires_at, last_used_at
         FROM device_tokens
         WHERE user_id = ? AND revoked_at IS NULL AND expires_at > datetime('now')
         ORDER BY created_at DESC",
    )
    .bind(&user_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(
        rows.into_iter()
            .map(|r| DeviceSessionView {
                id: r.id,
                device_name: r.device_name,
                created_at: r.created_at,
                expires_at: r.expires_at,
                last_used_at: r.last_used_at,
            })
            .collect(),
    ))
}

pub async fn revoke_device_session(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
    Path(token_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let user_id: Option<String> = sqlx::query_scalar(
        "SELECT id FROM users WHERE oidc_subject = ?",
    )
    .bind(&claims.oidc_subject)
    .fetch_optional(&state.pool)
    .await?;

    let Some(user_id) = user_id else {
        return Err(AppError::NotFound);
    };

    let updated = sqlx::query(
        "UPDATE device_tokens SET revoked_at = datetime('now')
         WHERE id = ? AND user_id = ? AND revoked_at IS NULL",
    )
    .bind(&token_id)
    .bind(&user_id)
    .execute(&state.pool)
    .await?
    .rows_affected();

    if updated == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ApiTokenResponse {
    pub has_token: bool,
    pub label: Option<String>,
    pub last_used_at: Option<String>,
    pub masked_token: Option<String>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct GenerateTokenResponse {
    pub token: String,
}

#[utoipa::path(
    get,
    path = "/api/me/tasks",
    responses(
        (status = 200, description = "List of tasks", body = Vec<TaskView>),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
    ),
    security(("SessionCookie" = [])),
    tag = "me"
)]
pub async fn list_tasks(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
) -> Result<Json<Vec<TaskView>>, AppError> {
    #[derive(sqlx::FromRow)]
    struct UserRow {
        id: String,
    }

    let user = sqlx::query_as::<_, UserRow>(
        "SELECT id FROM users WHERE oidc_subject = ?"
    )
    .bind(&claims.oidc_subject)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    let tasks = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE user_id = ? ORDER BY created_at DESC LIMIT 100",
    )
    .bind(&user.id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(tasks.into_iter().map(TaskView::from).collect()))
}

#[utoipa::path(
    get,
    path = "/api/me/tasks/{id}",
    params(("id" = String, Path, description = "Task UUID")),
    responses(
        (status = 200, description = "Task details", body = TaskView),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 404, description = "Not found", body = crate::error::ErrorResponse),
    ),
    security(("SessionCookie" = [])),
    tag = "me"
)]
pub async fn get_task_detail(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
    Path(task_id): Path<String>,
) -> Result<Json<TaskView>, AppError> {
    #[derive(sqlx::FromRow)]
    struct UserRow {
        id: String,
    }

    let user = sqlx::query_as::<_, UserRow>(
        "SELECT id FROM users WHERE oidc_subject = ?"
    )
    .bind(&claims.oidc_subject)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    let task = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE id = ? AND user_id = ?",
    )
    .bind(&task_id)
    .bind(&user.id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(TaskView::from(task)))
}

#[utoipa::path(
    post,
    path = "/api/me/tasks/{id}/approve",
    params(("id" = String, Path, description = "Task UUID")),
    responses(
        (status = 204, description = "Task approved"),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 404, description = "Not found", body = crate::error::ErrorResponse),
    ),
    security(("SessionCookie" = [])),
    tag = "me"
)]
pub async fn approve_task(
    State(state): State<Arc<AppState>>,
    UserAuth { user_id, .. }: UserAuth,
    Path(task_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let updated = sqlx::query(
        "UPDATE tasks
         SET approved_at = datetime('now'), updated_at = datetime('now')
         WHERE id = ? AND user_id = ? AND status = 'awaiting_approval'"
    )
    .bind(&task_id)
    .bind(&user_id)
    .execute(&state.pool)
    .await?
    .rows_affected();

    if updated == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/api/me/tasks/{id}/reject",
    params(("id" = String, Path, description = "Task UUID")),
    responses(
        (status = 204, description = "Task rejected"),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 404, description = "Not found", body = crate::error::ErrorResponse),
    ),
    security(("SessionCookie" = [])),
    tag = "me"
)]
pub async fn reject_task(
    State(state): State<Arc<AppState>>,
    UserAuth { user_id, .. }: UserAuth,
    Path(task_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let updated = sqlx::query(
        "UPDATE tasks
         SET rejected_at = datetime('now'), status = 'rejected', updated_at = datetime('now')
         WHERE id = ? AND user_id = ? AND status = 'awaiting_approval'"
    )
    .bind(&task_id)
    .bind(&user_id)
    .execute(&state.pool)
    .await?
    .rows_affected();

    if updated == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    delete,
    path = "/api/me/tasks/{id}",
    params(("id" = String, Path, description = "Task UUID")),
    responses(
        (status = 204, description = "Task deleted"),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 404, description = "Not found", body = crate::error::ErrorResponse),
    ),
    security(("SessionCookie" = [])),
    tag = "me"
)]
pub async fn delete_task(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
    Path(task_id): Path<String>,
) -> Result<StatusCode, AppError> {
    #[derive(sqlx::FromRow)]
    struct UserRow {
        id: String,
    }

    let user = sqlx::query_as::<_, UserRow>(
        "SELECT id FROM users WHERE oidc_subject = ?"
    )
    .bind(&claims.oidc_subject)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    let deleted = sqlx::query(
        "DELETE FROM tasks WHERE id = ? AND user_id = ?"
    )
    .bind(&task_id)
    .bind(&user.id)
    .execute(&state.pool)
    .await?
    .rows_affected();

    if deleted == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/api/me/token",
    responses(
        (status = 200, description = "API token info", body = ApiTokenResponse),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
    ),
    security(("SessionCookie" = [])),
    tag = "me"
)]
pub async fn get_api_token(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
) -> Result<Json<ApiTokenResponse>, AppError> {
    #[derive(sqlx::FromRow)]
    struct UserRow {
        id: String,
    }

    let user = sqlx::query_as::<_, UserRow>(
        "SELECT id FROM users WHERE oidc_subject = ?"
    )
    .bind(&claims.oidc_subject)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    #[derive(sqlx::FromRow)]
    struct TokenRow {
        id: String,
        label: String,
        last_used_at: Option<String>,
    }

    let token = sqlx::query_as::<_, TokenRow>(
        "SELECT id, label, last_used_at FROM api_tokens
         WHERE user_id = ? AND revoked_at IS NULL
         ORDER BY created_at DESC LIMIT 1"
    )
    .bind(&user.id)
    .fetch_optional(&state.pool)
    .await?;

    match token {
        Some(t) => Ok(Json(ApiTokenResponse {
            has_token: true,
            label: Some(t.label),
            last_used_at: t.last_used_at,
            masked_token: None,
        })),
        None => Ok(Json(ApiTokenResponse {
            has_token: false,
            label: None,
            last_used_at: None,
            masked_token: None,
        })),
    }
}

#[utoipa::path(
    post,
    path = "/api/me/token/regenerate",
    responses(
        (status = 200, description = "New token generated", body = GenerateTokenResponse),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
    ),
    security(("SessionCookie" = [])),
    tag = "me"
)]
pub async fn regenerate_api_token(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
) -> Result<Json<GenerateTokenResponse>, AppError> {
    #[derive(sqlx::FromRow)]
    struct UserRow {
        id: String,
    }

    let user = sqlx::query_as::<_, UserRow>(
        "SELECT id FROM users WHERE oidc_subject = ?"
    )
    .bind(&claims.oidc_subject)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    // Revoke old tokens
    sqlx::query(
        "UPDATE api_tokens SET revoked_at = datetime('now')
         WHERE user_id = ? AND revoked_at IS NULL"
    )
    .bind(&user.id)
    .execute(&state.pool)
    .await?;

    // Create new token
    let (plaintext, token_hash) = generate_api_token();
    let token_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO api_tokens (id, token_hash, user_id, label, created_at)
         VALUES (?, ?, ?, 'malpa', datetime('now'))"
    )
    .bind(&token_id)
    .bind(&token_hash)
    .bind(&user.id)
    .execute(&state.pool)
    .await?;

    Ok(Json(GenerateTokenResponse {
        token: plaintext,
    }))
}
