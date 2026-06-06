use crate::{
    auth::token::{generate_session_token, hash_key},
    error::AppError,
};
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use uuid::Uuid;

// SHA-256 of each superuser email address (never store plaintext).
const SUPERUSER_EMAIL_HASHES: &[&str] = &[
    "ac96178ea219f8f2a4488f467b52f3b10410ada21a83625edd75e9de6410a968", // rafal.kuka94@gmail.com
    "804d16db39c3c195d71c2a52b554d22266c88795560a182a4facd1bbcf2a611e", // hi@osmosis.page
];

pub struct SessionClaims {
    pub id: String,
    pub oidc_subject: String,
    pub email: String,
}

impl SessionClaims {
    pub fn is_superuser(&self) -> bool {
        let hash = hex::encode(Sha256::digest(self.email.as_bytes()));
        SUPERUSER_EMAIL_HASHES.contains(&hash.as_str())
    }
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
