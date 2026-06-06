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
