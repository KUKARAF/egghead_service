use crate::{
    ai::generate::with_userscript_header,
    auth::extractors::ApiTokenAuth,
    error::AppError,
    models::task::Task,
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

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub tab_url: String,
    pub prompt: String,
    pub page_html: String,
    pub action_recording: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateTaskResponse {
    pub id: String,
    pub status: String,
}

pub async fn create_task(
    State(state): State<Arc<AppState>>,
    ApiTokenAuth { user_id, .. }: ApiTokenAuth,
    Json(req): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<CreateTaskResponse>), AppError> {
    if req.page_html.len() > state.config.max_html_bytes {
        return Err(AppError::BadRequest(
            format!(
                "page_html exceeds {} bytes",
                state.config.max_html_bytes
            )
        ));
    }

    let task_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO tasks (id, user_id, tab_url, prompt, page_html, action_recording, status, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, 'pending', datetime('now'), datetime('now'))"
    )
    .bind(&task_id)
    .bind(&user_id)
    .bind(&req.tab_url)
    .bind(&req.prompt)
    .bind(&req.page_html)
    .bind(&req.action_recording)
    .execute(&state.pool)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateTaskResponse {
            id: task_id,
            status: "pending".to_string(),
        }),
    ))
}

#[derive(Debug, Serialize)]
pub struct GetTaskResponse {
    pub id: String,
    pub tab_url: String,
    pub prompt: String,
    pub status: String,
    pub estimated_price_cents: Option<i64>,
    pub price_rationale: Option<String>,
    pub script_name: Option<String>,
    pub script_code: Option<String>,
    pub match_pattern: Option<String>,
    pub error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn get_task(
    State(state): State<Arc<AppState>>,
    ApiTokenAuth { user_id, .. }: ApiTokenAuth,
    Path(task_id): Path<String>,
) -> Result<Json<GetTaskResponse>, AppError> {
    let task = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE id = ? AND user_id = ?",
    )
    .bind(&task_id)
    .bind(&user_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let script_code = if task.status == "done" {
        task.script_code.as_ref().map(|code| {
            with_userscript_header(
                &task.script_name.clone().unwrap_or_default(),
                &task.match_pattern.clone().unwrap_or_default(),
                code,
            )
        })
    } else {
        None
    };

    Ok(Json(GetTaskResponse {
        id: task.id,
        tab_url: task.tab_url,
        prompt: task.prompt,
        status: task.status,
        estimated_price_cents: task.estimated_price_cents,
        price_rationale: task.price_rationale,
        script_name: task.script_name,
        script_code,
        match_pattern: task.match_pattern,
        error_message: task.error_message,
        created_at: task.created_at,
        updated_at: task.updated_at,
    }))
}
