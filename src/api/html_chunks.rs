use crate::{auth::extractors::AppTokenAuth, error::AppError, state::AppState};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UploadChunkRequest {
    pub html_id: Option<String>,
    pub chunk_index: i64,
    pub total_chunks: i64,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct UploadChunkResponse {
    pub html_id: String,
    pub received: i64,
    pub complete: bool,
}

pub async fn upload_chunk(
    State(state): State<Arc<AppState>>,
    AppTokenAuth { .. }: AppTokenAuth,
    Json(req): Json<UploadChunkRequest>,
) -> Result<Json<UploadChunkResponse>, AppError> {
    if req.chunk_index < 0 || req.total_chunks < 1 || req.chunk_index >= req.total_chunks {
        return Err(AppError::BadRequest("invalid chunk_index or total_chunks".into()));
    }

    let html_id = req.html_id.unwrap_or_else(|| Uuid::new_v4().to_string());

    sqlx::query(
        "INSERT OR REPLACE INTO html_uploads (html_id, chunk_index, total_chunks, content)
         VALUES (?, ?, ?, ?)",
    )
    .bind(&html_id)
    .bind(req.chunk_index)
    .bind(req.total_chunks)
    .bind(&req.content)
    .execute(&state.pool)
    .await?;

    let received: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM html_uploads WHERE html_id = ?")
            .bind(&html_id)
            .fetch_one(&state.pool)
            .await?;

    Ok(Json(UploadChunkResponse {
        html_id,
        received,
        complete: received >= req.total_chunks,
    }))
}
