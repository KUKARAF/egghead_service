use crate::{
    auth::{
        session::{validate_session, SessionClaims},
        token::hash_key,
    },
    error::AppError,
    state::AppState,
};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use std::sync::Arc;

/// Auth via browser session cookie or `Authorization: Bearer <session_token>`.
pub struct SessionAuth(pub SessionClaims);

/// Auth via `Authorization: Bearer egghead_<token>` for the Chrome extension (developer use).
pub struct ApiTokenAuth {
    pub user_id: String,
}

/// Auth via `Authorization: Bearer device_<token>` for the Electron app (end-user devices).
pub struct DeviceTokenAuth {
    pub user_id: String,
}

/// Accepts either `ApiTokenAuth` (egghead_*) or `DeviceTokenAuth` (device_*).
/// Used on task endpoints so both the Chrome extension and Electron app can submit tasks.
pub struct AppTokenAuth {
    pub user_id: String,
    pub device_token_id: Option<String>,
}

/// Accepts either a device token (`device_*`) or a browser session (cookie / session bearer).
/// Used on endpoints that must be accessible by both the Electron app and the dashboard.
/// `device_token_id` is `Some` when the caller is an Electron device, `None` for dashboard users.
pub struct UserAuth {
    pub user_id: String,
    pub device_token_id: Option<String>,
}

fn extract_bearer(parts: &Parts) -> Option<String> {
    parts
        .headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|t| t.to_string())
}

fn extract_session_cookie(parts: &Parts) -> Option<String> {
    let cookie_header = parts.headers.get("Cookie")?.to_str().ok()?;
    cookie_header
        .split(';')
        .map(|s| s.trim())
        .find_map(|pair| pair.strip_prefix("session_token="))
        .map(|v| v.to_string())
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for SessionAuth {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        if state.config.dev_mode {
            return Ok(SessionAuth(SessionClaims {
                id: "dev".to_string(),
                oidc_subject: "dev".to_string(),
                email: "dev@localhost".to_string(),
            }));
        }

        let token = extract_bearer(parts)
            .or_else(|| extract_session_cookie(parts))
            .ok_or(AppError::Unauthorized)?;

        let claims = validate_session(&state.pool, &token).await?;
        Ok(SessionAuth(claims))
    }
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for ApiTokenAuth {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        if state.config.dev_mode {
            return Ok(ApiTokenAuth {
                user_id: "dev-user-id".to_string(),
            });
        }

        let token = extract_bearer(parts).ok_or(AppError::Unauthorized)?;

        if !token.starts_with("egghead_") {
            return Err(AppError::Unauthorized);
        }

        let token_hash = hash_key(&token);

        #[derive(sqlx::FromRow)]
        struct ApiTokenRow {
            user_id: String,
            token_id: String,
        }

        let row = sqlx::query_as::<_, ApiTokenRow>(
            "SELECT at.user_id, at.id as token_id
             FROM api_tokens at
             WHERE at.token_hash = ? AND at.revoked_at IS NULL"
        )
        .bind(&token_hash)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(AppError::Unauthorized)?;

        // Fire-and-forget last_used_at update
        let pool = state.pool.clone();
        let token_id = row.token_id.clone();
        tokio::spawn(async move {
            let _ = sqlx::query(
                "UPDATE api_tokens SET last_used_at = datetime('now') WHERE id = ?"
            )
            .bind(&token_id)
            .execute(&pool)
            .await;
        });

        Ok(ApiTokenAuth {
            user_id: row.user_id,
        })
    }
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for DeviceTokenAuth {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        if state.config.dev_mode {
            return Ok(DeviceTokenAuth {
                user_id: "dev-user-id".to_string(),
            });
        }

        let token = extract_bearer(parts).ok_or(AppError::Unauthorized)?;

        if !token.starts_with("device_") {
            return Err(AppError::Unauthorized);
        }

        let token_hash = hash_key(&token);

        let user_id: Option<String> = sqlx::query_scalar(
            "SELECT user_id FROM device_tokens
             WHERE token_hash = ?
               AND revoked_at IS NULL
               AND expires_at > datetime('now')",
        )
        .bind(&token_hash)
        .fetch_optional(&state.pool)
        .await?;

        let user_id = user_id.ok_or(AppError::Unauthorized)?;

        let pool = state.pool.clone();
        let hash = token_hash.clone();
        tokio::spawn(async move {
            let _ = sqlx::query(
                "UPDATE device_tokens SET last_used_at = datetime('now') WHERE token_hash = ?",
            )
            .bind(&hash)
            .execute(&pool)
            .await;
        });

        Ok(DeviceTokenAuth { user_id })
    }
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for AppTokenAuth {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        if state.config.dev_mode {
            return Ok(AppTokenAuth {
                user_id: "dev-user-id".to_string(),
                device_token_id: None,
            });
        }

        let token = extract_bearer(parts).ok_or(AppError::Unauthorized)?;

        if token.starts_with("egghead_") {
            let token_hash = hash_key(&token);

            #[derive(sqlx::FromRow)]
            struct Row {
                user_id: String,
                token_id: String,
            }

            let row = sqlx::query_as::<_, Row>(
                "SELECT user_id, id as token_id FROM api_tokens
                 WHERE token_hash = ? AND revoked_at IS NULL",
            )
            .bind(&token_hash)
            .fetch_optional(&state.pool)
            .await?
            .ok_or(AppError::Unauthorized)?;

            let pool = state.pool.clone();
            let token_id = row.token_id.clone();
            tokio::spawn(async move {
                let _ = sqlx::query(
                    "UPDATE api_tokens SET last_used_at = datetime('now') WHERE id = ?",
                )
                .bind(&token_id)
                .execute(&pool)
                .await;
            });

            return Ok(AppTokenAuth {
                user_id: row.user_id,
                device_token_id: None,
            });
        }

        if token.starts_with("device_") {
            let token_hash = hash_key(&token);

            #[derive(sqlx::FromRow)]
            struct Row {
                user_id: String,
                token_id: String,
            }

            let row = sqlx::query_as::<_, Row>(
                "SELECT user_id, id as token_id FROM device_tokens
                 WHERE token_hash = ?
                   AND revoked_at IS NULL
                   AND expires_at > datetime('now')",
            )
            .bind(&token_hash)
            .fetch_optional(&state.pool)
            .await?
            .ok_or(AppError::Unauthorized)?;

            let pool = state.pool.clone();
            let hash = token_hash.clone();
            tokio::spawn(async move {
                let _ = sqlx::query(
                    "UPDATE device_tokens SET last_used_at = datetime('now') WHERE token_hash = ?",
                )
                .bind(&hash)
                .execute(&pool)
                .await;
            });

            return Ok(AppTokenAuth {
                user_id: row.user_id,
                device_token_id: Some(row.token_id),
            });
        }

        Err(AppError::Unauthorized)
    }
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for UserAuth {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        if state.config.dev_mode {
            return Ok(UserAuth {
                user_id: "dev-user-id".to_string(),
                device_token_id: None,
            });
        }

        let token_opt = extract_bearer(parts);

        if let Some(ref token) = token_opt {
            if token.starts_with("device_") {
                let token_hash = hash_key(token);

                #[derive(sqlx::FromRow)]
                struct Row {
                    user_id: String,
                    token_id: String,
                }

                let row = sqlx::query_as::<_, Row>(
                    "SELECT user_id, id as token_id FROM device_tokens
                     WHERE token_hash = ?
                       AND revoked_at IS NULL
                       AND expires_at > datetime('now')",
                )
                .bind(&token_hash)
                .fetch_optional(&state.pool)
                .await?
                .ok_or(AppError::Unauthorized)?;

                let pool = state.pool.clone();
                let hash = token_hash.clone();
                tokio::spawn(async move {
                    let _ = sqlx::query(
                        "UPDATE device_tokens SET last_used_at = datetime('now') WHERE token_hash = ?",
                    )
                    .bind(&hash)
                    .execute(&pool)
                    .await;
                });

                return Ok(UserAuth {
                    user_id: row.user_id,
                    device_token_id: Some(row.token_id),
                });
            }
        }

        // Fall back to session auth (cookie or non-device bearer)
        let session_token = token_opt
            .or_else(|| extract_session_cookie(parts))
            .ok_or(AppError::Unauthorized)?;

        let claims = validate_session(&state.pool, &session_token).await?;

        let user_id: Option<String> = sqlx::query_scalar(
            "SELECT id FROM users WHERE oidc_subject = ?",
        )
        .bind(&claims.oidc_subject)
        .fetch_optional(&state.pool)
        .await?;

        let user_id = user_id.ok_or(AppError::Unauthorized)?;

        Ok(UserAuth {
            user_id,
            device_token_id: None,
        })
    }
}
