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

/// Auth via `Authorization: Bearer egghead_<token>` for the Chrome extension.
pub struct ApiTokenAuth {
    pub user_id: String,
    pub email: String,
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
                email: "dev@localhost".to_string(),
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
            email: String,
            token_id: String,
        }

        let row = sqlx::query_as::<_, ApiTokenRow>(
            "SELECT at.user_id, u.email, at.id as token_id
             FROM api_tokens at
             JOIN users u ON u.id = at.user_id
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
            email: row.email,
        })
    }
}
