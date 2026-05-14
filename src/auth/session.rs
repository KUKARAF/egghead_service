use crate::{
    auth::token::{generate_session_token, hash_key},
    error::AppError,
};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SessionClaims {
    pub id: String,
    pub oidc_subject: String,
    pub email: String,
}

/// Creates a new 10-hour browser session, returns plaintext token once.
pub async fn create_session(
    pool: &SqlitePool,
    oidc_subject: &str,
    email: &str,
) -> Result<String, AppError> {
    let (plaintext, token_hash) = generate_session_token();
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO session_tokens (id, token_hash, oidc_subject, email, expires_at)
         VALUES (?, ?, ?, ?, datetime('now', '+10 hours'))"
    )
    .bind(&id)
    .bind(&token_hash)
    .bind(oidc_subject)
    .bind(email)
    .execute(pool)
    .await?;

    Ok(plaintext)
}

/// Validates a session token; returns claims if valid and unexpired.
pub async fn validate_session(
    pool: &SqlitePool,
    plaintext: &str,
) -> Result<SessionClaims, AppError> {
    let token_hash = hash_key(plaintext);

    #[derive(sqlx::FromRow)]
    struct SessionRow {
        id: String,
        oidc_subject: String,
        email: String,
    }

    let row = sqlx::query_as::<_, SessionRow>(
        "SELECT id, oidc_subject, email
         FROM session_tokens
         WHERE token_hash = ? AND expires_at > datetime('now')"
    )
    .bind(&token_hash)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    Ok(SessionClaims {
        id: row.id,
        oidc_subject: row.oidc_subject,
        email: row.email,
    })
}

/// Revokes a session token by deleting it.
pub async fn revoke_session(pool: &SqlitePool, plaintext: &str) -> Result<(), AppError> {
    let token_hash = hash_key(plaintext);
    sqlx::query(
        "DELETE FROM session_tokens WHERE token_hash = ?"
    )
    .bind(&token_hash)
    .execute(pool)
    .await?;
    Ok(())
}
