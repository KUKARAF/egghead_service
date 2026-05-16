use crate::{
    ai::generate::with_userscript_header,
    auth::extractors::ApiTokenAuth,
    error::AppError,
    models::task::{Task, FileAttachment},
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

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateTaskRequest {
    pub tab_url: String,
    pub prompt: String,
    /// Inline HTML for small pages (< ~100 KB)
    pub page_html: Option<String>,
    /// Reference to a pre-uploaded chunked HTML (large pages)
    pub html_id: Option<String>,
    pub action_recording: Option<String>,
    pub files: Option<Vec<FileAttachment>>,
}

#[derive(sqlx::FromRow)]
struct HtmlChunkRow {
    chunk_index: i64,
    total_chunks: i64,
    content: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CreateTaskResponse {
    pub id: String,
    pub status: String,
    pub submission_token: String,
}

#[utoipa::path(
    post,
    path = "/api/tasks",
    request_body = CreateTaskRequest,
    responses(
        (status = 201, description = "Task created", body = CreateTaskResponse),
        (status = 400, description = "Bad request", body = crate::error::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
    ),
    security(("ApiToken" = [])),
    tag = "tasks"
)]
pub async fn create_task(
    State(state): State<Arc<AppState>>,
    ApiTokenAuth { user_id, .. }: ApiTokenAuth,
    Json(req): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<CreateTaskResponse>), AppError> {
    // Resolve HTML: either inline or assembled from pre-uploaded chunks
    let page_html = if let Some(html_id) = &req.html_id {
        let chunks: Vec<HtmlChunkRow> = sqlx::query_as(
            "SELECT chunk_index, total_chunks, content FROM html_uploads
             WHERE html_id = ? ORDER BY chunk_index",
        )
        .bind(html_id)
        .fetch_all(&state.pool)
        .await?;

        if chunks.is_empty() {
            return Err(AppError::BadRequest(
                format!("html_id '{}' not found or no chunks uploaded", html_id),
            ));
        }

        let total = chunks[0].total_chunks;
        if chunks.len() as i64 != total {
            return Err(AppError::BadRequest(format!(
                "incomplete upload: got {} of {} chunks",
                chunks.len(),
                total
            )));
        }

        let html = chunks.into_iter().map(|c| c.content).collect::<String>();

        // Clean up chunks now that they're assembled
        sqlx::query("DELETE FROM html_uploads WHERE html_id = ?")
            .bind(html_id)
            .execute(&state.pool)
            .await?;

        html
    } else if let Some(html) = req.page_html {
        html
    } else {
        return Err(AppError::BadRequest(
            "either page_html or html_id is required".into(),
        ));
    };

    let total_size = page_html.len()
        + req.files.as_ref()
            .map(|f| f.iter().map(|fa| fa.content.len()).sum::<usize>())
            .unwrap_or(0);

    if total_size > state.config.max_html_bytes {
        return Err(AppError::BadRequest(format!(
            "total content (html + files) exceeds {} bytes",
            state.config.max_html_bytes
        )));
    }

    let task_id = Uuid::new_v4().to_string();
    let submission_token = format!("task_{}", Uuid::new_v4().to_string());
    let files_json = req.files.as_ref().map(|f| serde_json::to_string(f).unwrap_or_default());

    sqlx::query(
        "INSERT INTO tasks (id, user_id, tab_url, prompt, page_html, action_recording, files_json, submission_token, status, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'pending', datetime('now'), datetime('now'))"
    )
    .bind(&task_id)
    .bind(&user_id)
    .bind(&req.tab_url)
    .bind(&req.prompt)
    .bind(&page_html)
    .bind(&req.action_recording)
    .bind(&files_json)
    .bind(&submission_token)
    .execute(&state.pool)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateTaskResponse {
            id: task_id,
            status: "pending".to_string(),
            submission_token,
        }),
    ))
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct GetTaskResponse {
    pub id: String,
    pub tab_url: String,
    pub prompt: String,
    pub status: String,
    pub estimated_price_pln: Option<i64>,
    pub price_rationale: Option<String>,
    pub script_name: Option<String>,
    pub script_code: Option<String>,
    pub match_pattern: Option<String>,
    pub error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[utoipa::path(
    get,
    path = "/api/tasks/{id}",
    params(("id" = String, Path, description = "Task UUID")),
    responses(
        (status = 200, description = "Task details", body = GetTaskResponse),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 404, description = "Not found", body = crate::error::ErrorResponse),
    ),
    security(("ApiToken" = [])),
    tag = "tasks"
)]
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
        estimated_price_pln: task.estimated_price_pln,
        price_rationale: task.price_rationale,
        script_name: task.script_name,
        script_code,
        match_pattern: task.match_pattern,
        error_message: task.error_message,
        created_at: task.created_at,
        updated_at: task.updated_at,
    }))
}

#[utoipa::path(
    get,
    path = "/api/tasks/status/{token}",
    params(("token" = String, Path, description = "Submission token (read-only access)")),
    responses(
        (status = 200, description = "Task status", body = GetTaskResponse),
        (status = 404, description = "Token not found", body = crate::error::ErrorResponse),
    ),
    tag = "tasks"
)]
pub async fn get_task_by_token(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
) -> Result<Json<GetTaskResponse>, AppError> {
    let task = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE submission_token = ?",
    )
    .bind(&token)
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
        estimated_price_pln: task.estimated_price_pln,
        price_rationale: task.price_rationale,
        script_name: task.script_name,
        script_code,
        match_pattern: task.match_pattern,
        error_message: task.error_message,
        created_at: task.created_at,
        updated_at: task.updated_at,
    }))
}
