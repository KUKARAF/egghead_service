use crate::{
    auth::extractors::{SessionAuth, UserAuth},
    error::AppError,
    models::userscript::{DeviceAssignment, ScriptMessage},
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
pub struct CreateUserscriptRequest {
    pub name: String,
    pub prompt: String,
    pub url: String,
    pub script_code: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserscriptRequest {
    pub name: Option<String>,
    pub prompt: Option<String>,
    pub url: Option<String>,
    pub script_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AssignDevicesRequest {
    pub device_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct UserscriptResponse {
    pub id: String,
    pub name: String,
    pub prompt: String,
    pub url: String,
    pub script_code: String,
    pub created_at: String,
    pub updated_at: String,
    pub device_ids: Vec<String>,
}

pub async fn list_userscripts(
    State(state): State<Arc<AppState>>,
    UserAuth { user_id, device_token_id }: UserAuth,
) -> Result<Json<Vec<UserscriptResponse>>, AppError> {
    #[derive(sqlx::FromRow)]
    struct ScriptRow {
        id: String,
        name: String,
        prompt: String,
        url: String,
        script_code: String,
        created_at: String,
        updated_at: String,
    }

    let scripts: Vec<ScriptRow> = if let Some(ref dtid) = device_token_id {
        sqlx::query_as(
            "SELECT u.id, u.name, u.prompt, u.url, u.script_code, u.created_at, u.updated_at
             FROM userscripts u
             JOIN script_device_assignments sda ON sda.script_id = u.id
             WHERE sda.device_token_id = ?
             ORDER BY u.created_at DESC",
        )
        .bind(dtid)
        .fetch_all(&state.pool)
        .await?
    } else {
        sqlx::query_as(
            "SELECT id, name, prompt, url, script_code, created_at, updated_at
             FROM userscripts
             WHERE user_id = ?
             ORDER BY created_at DESC",
        )
        .bind(&user_id)
        .fetch_all(&state.pool)
        .await?
    };

    let mut response = Vec::new();

    for script in scripts {
        let device_ids: Vec<String> = sqlx::query_scalar(
            "SELECT dt.id FROM script_device_assignments sda
             JOIN device_tokens dt ON sda.device_token_id = dt.id
             WHERE sda.script_id = ?",
        )
        .bind(&script.id)
        .fetch_all(&state.pool)
        .await?;

        response.push(UserscriptResponse {
            id: script.id,
            name: script.name,
            prompt: script.prompt,
            url: script.url,
            script_code: script.script_code,
            created_at: script.created_at,
            updated_at: script.updated_at,
            device_ids,
        });
    }

    Ok(Json(response))
}

pub async fn create_userscript(
    State(state): State<Arc<AppState>>,
    SessionAuth(_claims): SessionAuth,
    Json(req): Json<CreateUserscriptRequest>,
) -> Result<(StatusCode, Json<UserscriptResponse>), AppError> {
    let name = req.name.trim();
    let prompt = req.prompt.trim();
    let url = req.url.trim();

    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".into()));
    }
    if prompt.is_empty() {
        return Err(AppError::BadRequest("prompt is required".into()));
    }
    if url.is_empty() {
        return Err(AppError::BadRequest("url is required".into()));
    }

    let user_id: Option<String> = sqlx::query_scalar(
        "SELECT id FROM users WHERE oidc_subject IS NOT NULL LIMIT 1",
    )
    .fetch_optional(&state.pool)
    .await?
    .flatten();

    let user_id = user_id.ok_or(AppError::Unauthorized)?;

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO userscripts (id, user_id, name, prompt, url, script_code, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&user_id)
    .bind(name)
    .bind(prompt)
    .bind(url)
    .bind(&req.script_code)
    .bind(&now)
    .bind(&now)
    .execute(&state.pool)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(UserscriptResponse {
            id,
            name: name.to_string(),
            prompt: prompt.to_string(),
            url: url.to_string(),
            script_code: req.script_code,
            created_at: now.clone(),
            updated_at: now,
            device_ids: Vec::new(),
        }),
    ))
}

pub async fn get_userscript(
    State(state): State<Arc<AppState>>,
    SessionAuth(_claims): SessionAuth,
    Path(script_id): Path<String>,
) -> Result<Json<UserscriptResponse>, AppError> {
    #[derive(sqlx::FromRow)]
    struct ScriptRow {
        id: String,
        name: String,
        prompt: String,
        url: String,
        script_code: String,
        created_at: String,
        updated_at: String,
    }

    let script: ScriptRow = sqlx::query_as(
        "SELECT id, name, prompt, url, script_code, created_at, updated_at
         FROM userscripts
         WHERE id = ? AND user_id = (SELECT id FROM users WHERE oidc_subject IS NOT NULL LIMIT 1)",
    )
    .bind(&script_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let device_ids: Vec<String> = sqlx::query_scalar(
        "SELECT dt.id FROM script_device_assignments sda
         JOIN device_tokens dt ON sda.device_token_id = dt.id
         WHERE sda.script_id = ?",
    )
    .bind(&script.id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(UserscriptResponse {
        id: script.id,
        name: script.name,
        prompt: script.prompt,
        url: script.url,
        script_code: script.script_code,
        created_at: script.created_at,
        updated_at: script.updated_at,
        device_ids,
    }))
}

pub async fn update_userscript(
    State(state): State<Arc<AppState>>,
    SessionAuth(_claims): SessionAuth,
    Path(script_id): Path<String>,
    Json(req): Json<UpdateUserscriptRequest>,
) -> Result<Json<UserscriptResponse>, AppError> {
    let script: (String, String, String, String, String, String) = sqlx::query_as(
        "SELECT id, name, prompt, url, script_code, created_at
         FROM userscripts
         WHERE id = ? AND user_id = (SELECT id FROM users WHERE oidc_subject IS NOT NULL LIMIT 1)",
    )
    .bind(&script_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let now = chrono::Utc::now().to_rfc3339();
    let name = req.name.as_deref().unwrap_or(&script.1);
    let prompt = req.prompt.as_deref().unwrap_or(&script.2);
    let url = req.url.as_deref().unwrap_or(&script.3);
    let script_code = req.script_code.as_deref().unwrap_or(&script.4);

    sqlx::query(
        "UPDATE userscripts
         SET name = ?, prompt = ?, url = ?, script_code = ?, updated_at = ?
         WHERE id = ?",
    )
    .bind(name)
    .bind(prompt)
    .bind(url)
    .bind(script_code)
    .bind(&now)
    .bind(&script_id)
    .execute(&state.pool)
    .await?;

    let device_ids: Vec<String> = sqlx::query_scalar(
        "SELECT dt.id FROM script_device_assignments sda
         JOIN device_tokens dt ON sda.device_token_id = dt.id
         WHERE sda.script_id = ?",
    )
    .bind(&script_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(UserscriptResponse {
        id: script_id,
        name: name.to_string(),
        prompt: prompt.to_string(),
        url: url.to_string(),
        script_code: script_code.to_string(),
        created_at: script.5,
        updated_at: now,
        device_ids,
    }))
}

pub async fn delete_userscript(
    State(state): State<Arc<AppState>>,
    SessionAuth(_claims): SessionAuth,
    Path(script_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query(
        "DELETE FROM userscripts
         WHERE id = ? AND user_id = (SELECT id FROM users WHERE oidc_subject IS NOT NULL LIMIT 1)",
    )
    .bind(&script_id)
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn assign_devices_to_script(
    State(state): State<Arc<AppState>>,
    SessionAuth(_claims): SessionAuth,
    Path(script_id): Path<String>,
    Json(req): Json<AssignDevicesRequest>,
) -> Result<StatusCode, AppError> {
    let script_exists: bool = sqlx::query_scalar(
        "SELECT COUNT(*) > 0
         FROM userscripts
         WHERE id = ? AND user_id = (SELECT id FROM users WHERE oidc_subject IS NOT NULL LIMIT 1)",
    )
    .bind(&script_id)
    .fetch_one(&state.pool)
    .await?;

    if !script_exists {
        return Err(AppError::NotFound);
    }

    let mut tx = state.pool.begin().await?;

    sqlx::query("DELETE FROM script_device_assignments WHERE script_id = ?")
        .bind(&script_id)
        .execute(&mut *tx)
        .await?;

    for device_id in &req.device_ids {
        let assignment_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO script_device_assignments (id, script_id, device_token_id)
             VALUES (?, ?, ?)",
        )
        .bind(&assignment_id)
        .bind(&script_id)
        .bind(device_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize)]
pub struct RefineScriptRequest {
    pub prompt: String,
    pub page_html: Option<String>,
    pub html_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RefineScriptResponse {
    pub id: String,
    pub status: String,
    pub submission_token: String,
}

pub async fn refine_script(
    State(state): State<Arc<AppState>>,
    UserAuth { user_id, device_token_id }: UserAuth,
    Path(script_id): Path<String>,
    Json(req): Json<RefineScriptRequest>,
) -> Result<(StatusCode, Json<RefineScriptResponse>), AppError> {
    let tab_url: Option<String> = sqlx::query_scalar(
        "SELECT url FROM userscripts WHERE id = ? AND user_id = ?",
    )
    .bind(&script_id)
    .bind(&user_id)
    .fetch_optional(&state.pool)
    .await?;

    let tab_url = tab_url.ok_or(AppError::NotFound)?;

    let page_html = if let Some(ref html_id) = req.html_id {
        #[derive(sqlx::FromRow)]
        struct HtmlChunkRow {
            chunk_index: i64,
            total_chunks: i64,
            content: String,
        }

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
        sqlx::query("DELETE FROM html_uploads WHERE html_id = ?")
            .bind(html_id)
            .execute(&state.pool)
            .await?;
        html
    } else if let Some(html) = req.page_html {
        html
    } else {
        return Err(AppError::BadRequest("either page_html or html_id is required".into()));
    };

    let task_id = Uuid::new_v4().to_string();
    let submission_token = format!("task_{}", Uuid::new_v4());

    sqlx::query(
        "INSERT INTO tasks (id, user_id, device_token_id, tab_url, prompt, page_html, script_id, submission_token, status, approved_at, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'awaiting_approval', datetime('now'), datetime('now'), datetime('now'))",
    )
    .bind(&task_id)
    .bind(&user_id)
    .bind(&device_token_id)
    .bind(&tab_url)
    .bind(&req.prompt)
    .bind(&page_html)
    .bind(&script_id)
    .bind(&submission_token)
    .execute(&state.pool)
    .await?;

    let msg_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO script_messages (id, script_id, role, content) VALUES (?, ?, 'user', ?)",
    )
    .bind(&msg_id)
    .bind(&script_id)
    .bind(&req.prompt)
    .execute(&state.pool)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(RefineScriptResponse {
            id: task_id,
            status: "awaiting_approval".to_string(),
            submission_token,
        }),
    ))
}

pub async fn list_script_messages(
    State(state): State<Arc<AppState>>,
    UserAuth { user_id, .. }: UserAuth,
    Path(script_id): Path<String>,
) -> Result<Json<Vec<ScriptMessage>>, AppError> {
    let script_exists: bool = sqlx::query_scalar(
        "SELECT COUNT(*) > 0 FROM userscripts WHERE id = ? AND user_id = ?",
    )
    .bind(&script_id)
    .bind(&user_id)
    .fetch_one(&state.pool)
    .await?;

    if !script_exists {
        return Err(AppError::NotFound);
    }

    let messages: Vec<ScriptMessage> = sqlx::query_as(
        "SELECT id, script_id, role, content, created_at FROM script_messages
         WHERE script_id = ? ORDER BY created_at ASC",
    )
    .bind(&script_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(messages))
}

pub async fn get_script_devices(
    State(state): State<Arc<AppState>>,
    SessionAuth(_claims): SessionAuth,
    Path(script_id): Path<String>,
) -> Result<Json<Vec<DeviceAssignment>>, AppError> {
    let script_exists: bool = sqlx::query_scalar(
        "SELECT COUNT(*) > 0
         FROM userscripts
         WHERE id = ? AND user_id = (SELECT id FROM users WHERE oidc_subject IS NOT NULL LIMIT 1)",
    )
    .bind(&script_id)
    .fetch_one(&state.pool)
    .await?;

    if !script_exists {
        return Err(AppError::NotFound);
    }

    #[derive(sqlx::FromRow)]
    struct DeviceRow {
        id: String,
        device_name: String,
    }

    let devices: Vec<DeviceRow> = sqlx::query_as(
        "SELECT dt.id, dt.device_name
         FROM script_device_assignments sda
         JOIN device_tokens dt ON sda.device_token_id = dt.id
         WHERE sda.script_id = ?",
    )
    .bind(&script_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(
        devices
            .into_iter()
            .map(|d| DeviceAssignment {
                device_id: d.id,
                device_name: d.device_name,
            })
            .collect(),
    ))
}
