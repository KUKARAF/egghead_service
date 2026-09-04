use crate::{
    auth::token::{generate_device_token, generate_emoji_sequence},
    error::AppError,
    state::AppState,
};
use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::Response,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RegisterDeviceRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterDeviceResponse {
    pub id: String,
    pub emoji: String,
}

#[derive(Debug, Serialize)]
pub struct DeviceStatusResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

static EMOJI_JSON: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/admin/emoji.json"));

pub async fn list_emojis() -> Response {
    Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::CACHE_CONTROL, "public, max-age=86400")
        .body(axum::body::Body::from(EMOJI_JSON))
        .unwrap_or_else(|_| Response::new(axum::body::Body::from(EMOJI_JSON)))
}

pub async fn register_device(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterDeviceRequest>,
) -> Result<(StatusCode, Json<RegisterDeviceResponse>), AppError> {
    let name = req.name.trim().to_string();
    let email = req.email.trim().to_string();

    if name.is_empty() || name.len() > 100 {
        return Err(AppError::BadRequest("name must be 1–100 characters".into()));
    }
    if !email.contains('@') {
        return Err(AppError::BadRequest("invalid email".into()));
    }

    let emoji = generate_emoji_sequence();
    let emoji_hash = bcrypt::hash(&emoji, 6)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("bcrypt error: {e}")))?;

    let id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO device_registrations (id, name, email, emoji_hash, expires_at)
         VALUES (?, ?, ?, ?, datetime('now', '+10 minutes'))",
    )
    .bind(&id)
    .bind(&name)
    .bind(&email)
    .bind(&emoji_hash)
    .execute(&state.pool)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterDeviceResponse { id, emoji }),
    ))
}

pub async fn get_device_status(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<DeviceStatusResponse>, AppError> {
    #[derive(sqlx::FromRow)]
    struct Row {
        status: String,
        token_plain: Option<String>,
        expires_at: String,
    }

    let row = sqlx::query_as::<_, Row>(
        "SELECT status, token_plain, expires_at FROM device_registrations WHERE id = ?",
    )
    .bind(&id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    // Expire stale pending registrations on first access rather than waiting for cleanup worker.
    if row.status == "pending" {
        let expired: bool = sqlx::query_scalar(
            "SELECT expires_at <= datetime('now') FROM device_registrations WHERE id = ?",
        )
        .bind(&id)
        .fetch_one(&state.pool)
        .await?;

        if expired {
            sqlx::query(
                "UPDATE device_registrations SET status = 'expired' WHERE id = ? AND status = 'pending'",
            )
            .bind(&id)
            .execute(&state.pool)
            .await?;
            return Ok(Json(DeviceStatusResponse {
                status: "expired".into(),
                token: None,
            }));
        }
    }

    // Deliver token exactly once: capture and clear in a conditional UPDATE.
    if row.status == "approved" {
        if let Some(token_plain) = row.token_plain {
            sqlx::query(
                "UPDATE device_registrations SET token_plain = NULL
                 WHERE id = ? AND token_plain IS NOT NULL",
            )
            .bind(&id)
            .execute(&state.pool)
            .await?;

            return Ok(Json(DeviceStatusResponse {
                status: "approved".into(),
                token: Some(token_plain),
            }));
        }
    }

    Ok(Json(DeviceStatusResponse {
        status: row.status,
        token: None,
    }))
}

// ---------------------------------------------------------------------------
// Admin handlers (SessionAuth) — device request management
// ---------------------------------------------------------------------------

use crate::auth::extractors::SessionAuth;

#[derive(Debug, Serialize)]
pub struct DeviceRequestView {
    pub id: String,
    pub name: String,
    pub email: String,
    pub status: String,
    pub requested_at: String,
    pub expires_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ApproveDeviceRequest {
    pub confirm: String,
}

pub async fn list_device_requests(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
) -> Result<Json<Vec<DeviceRequestView>>, AppError> {
    if !claims.is_superuser() {
        return Err(AppError::Forbidden("superuser only".into()));
    }
    #[derive(sqlx::FromRow)]
    struct Row {
        id: String,
        name: String,
        email: String,
        status: String,
        requested_at: String,
        expires_at: String,
    }

    let rows = sqlx::query_as::<_, Row>(
        "SELECT id, name, email, status, requested_at, expires_at
         FROM device_registrations
         WHERE status = 'pending' AND expires_at > datetime('now')
         ORDER BY requested_at DESC",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(
        rows.into_iter()
            .map(|r| DeviceRequestView {
                id: r.id,
                name: r.name,
                email: r.email,
                status: r.status,
                requested_at: r.requested_at,
                expires_at: r.expires_at,
            })
            .collect(),
    ))
}

pub async fn approve_device_request(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
    Path(reg_id): Path<String>,
    Json(body): Json<ApproveDeviceRequest>,
) -> Result<StatusCode, AppError> {
    if !claims.is_superuser() {
        return Err(AppError::Forbidden("superuser only".into()));
    }
    #[derive(sqlx::FromRow)]
    struct RegRow {
        email: String,
        name: String,
        emoji_hash: String,
    }

    let reg = sqlx::query_as::<_, RegRow>(
        "SELECT email, name, emoji_hash FROM device_registrations
         WHERE id = ? AND status = 'pending' AND expires_at > datetime('now')",
    )
    .bind(&reg_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let matches = bcrypt::verify(&body.confirm, &reg.emoji_hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("bcrypt verify error: {e}")))?;

    if !matches {
        return Err(AppError::Forbidden("emoji sequence does not match".into()));
    }

    let mut tx = state.pool.begin().await?;

    // Find-or-create user by email. Prefer any existing user (OIDC or device) for this email.
    let user_id: String = match sqlx::query_scalar("SELECT id FROM users WHERE email = ? LIMIT 1")
        .bind(&reg.email)
        .fetch_optional(&mut *tx)
        .await?
    {
        Some(id) => id,
        None => {
            let new_user_id = Uuid::new_v4().to_string();
            sqlx::query("INSERT INTO users (id, oidc_subject, email) VALUES (?, NULL, ?)")
                .bind(&new_user_id)
                .bind(&reg.email)
                .execute(&mut *tx)
                .await?;
            new_user_id
        }
    };

    // Revoke existing device tokens for this user so only one is active at a time.
    sqlx::query(
        "UPDATE device_tokens SET revoked_at = datetime('now')
         WHERE user_id = ? AND revoked_at IS NULL",
    )
    .bind(&user_id)
    .execute(&mut *tx)
    .await?;

    let (plaintext, token_hash) = generate_device_token();
    let token_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO device_tokens (id, token_hash, user_id, device_name, expires_at)
         VALUES (?, ?, ?, ?, datetime('now', '+180 days'))",
    )
    .bind(&token_id)
    .bind(&token_hash)
    .bind(&user_id)
    .bind(&reg.name)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE device_registrations
         SET status = 'approved',
             token_plain = ?,
             user_id = ?,
             device_token_id = ?,
             approved_at = datetime('now')
         WHERE id = ?",
    )
    .bind(&plaintext)
    .bind(&user_id)
    .bind(&token_id)
    .bind(&reg_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn reject_device_request(
    State(state): State<Arc<AppState>>,
    SessionAuth(claims): SessionAuth,
    Path(reg_id): Path<String>,
) -> Result<StatusCode, AppError> {
    if !claims.is_superuser() {
        return Err(AppError::Forbidden("superuser only".into()));
    }
    let updated = sqlx::query(
        "UPDATE device_registrations
         SET status = 'rejected', rejected_at = datetime('now')
         WHERE id = ? AND status = 'pending'",
    )
    .bind(&reg_id)
    .execute(&state.pool)
    .await?
    .rows_affected();

    if updated == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
