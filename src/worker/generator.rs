use crate::{
    ai::{
        generate::{call_generate, with_userscript_header},
        secrets::{fetch_github_repo, fetch_github_token},
    },
    state::AppState,
    worker::git::push_to_github,
};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use uuid::Uuid;

pub async fn run(state: Arc<AppState>) {
    let poll_secs = state.config.worker_poll_interval_secs;
    let mut ticker = interval(Duration::from_secs(poll_secs));
    loop {
        ticker.tick().await;
        if let Err(e) = poll_approved(&state).await {
            tracing::error!("generator poll error: {e:#}");
        }
    }
}

async fn poll_approved(state: &Arc<AppState>) -> anyhow::Result<()> {
    #[derive(sqlx::FromRow)]
    struct TaskRow {
        id: String,
    }

    let rows = sqlx::query_as::<_, TaskRow>(
        "SELECT id FROM tasks
         WHERE status = 'awaiting_approval' AND approved_at IS NOT NULL
         ORDER BY approved_at ASC LIMIT 5",
    )
    .fetch_all(&state.pool)
    .await?;

    for row in rows {
        let claimed = sqlx::query(
            "UPDATE tasks
             SET status = 'processing', worker_started_at = datetime('now'), updated_at = datetime('now')
             WHERE id = ? AND status = 'awaiting_approval' AND approved_at IS NOT NULL",
        )
        .bind(&row.id)
        .execute(&state.pool)
        .await?
        .rows_affected();

        if claimed == 0 {
            continue;
        }

        let state = Arc::clone(state);
        let task_id = row.id.clone();
        tokio::spawn(async move {
            if let Err(e) = run_generation(&state, &task_id).await {
                tracing::error!(task_id = %task_id, "generation failed: {e:#}");
                let msg = format!("{e:#}");
                let _ = sqlx::query(
                    "UPDATE tasks SET status = 'failed', error_message = ?, updated_at = datetime('now') WHERE id = ?",
                )
                .bind(&msg)
                .bind(&task_id)
                .execute(&state.pool)
                .await;
            }
        });
    }

    Ok(())
}

async fn run_generation(state: &Arc<AppState>, task_id: &str) -> anyhow::Result<()> {
    #[derive(sqlx::FromRow)]
    struct TaskData {
        user_id: String,
        device_token_id: Option<String>,
        tab_url: String,
        prompt: String,
        page_html: String,
        action_recording: Option<String>,
        files_json: Option<String>,
        user_email: String,
    }

    let task = sqlx::query_as::<_, TaskData>(
        "SELECT t.user_id, t.device_token_id, t.tab_url, t.prompt, t.page_html, t.action_recording, t.files_json, u.email as user_email
         FROM tasks t
         JOIN users u ON u.id = t.user_id
         WHERE t.id = ?",
    )
    .bind(task_id)
    .fetch_one(&state.pool)
    .await?;

    let openrouter_key = crate::ai::secrets::fetch_openrouter_key(
        &state.http_client,
        &state.config.kv_url,
        &state.config.kv_api_key,
    )
    .await?;

    let client = crate::ai::client::OpenRouterClient::new(&state.http_client, &openrouter_key);

    let resp = call_generate(
        &client,
        &task.tab_url,
        &task.prompt,
        &task.page_html,
        task.action_recording.as_deref(),
        task.files_json.as_deref(),
    )
    .await?;

    let full_script = with_userscript_header(&resp.name, &resp.match_pattern, &resp.script_code);

    sqlx::query(
        "UPDATE tasks
         SET status = 'done',
             script_name = ?,
             script_code = ?,
             match_pattern = ?,
             updated_at = datetime('now')
         WHERE id = ?",
    )
    .bind(&resp.name)
    .bind(&full_script)
    .bind(&resp.match_pattern)
    .bind(task_id)
    .execute(&state.pool)
    .await?;

    // Auto-save the generated script into the user's userscripts library
    let userscript_id = Uuid::new_v4().to_string();
    let saved = sqlx::query(
        "INSERT INTO userscripts (id, user_id, name, prompt, url, script_code, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
    )
    .bind(&userscript_id)
    .bind(&task.user_id)
    .bind(&resp.name)
    .bind(&task.prompt)
    .bind(&task.tab_url)
    .bind(&full_script)
    .execute(&state.pool)
    .await;

    if let Err(e) = saved {
        tracing::warn!(task_id = %task_id, "failed to auto-save userscript: {e:#}");
    } else if let Some(ref device_token_id) = task.device_token_id {
        let assignment_id = Uuid::new_v4().to_string();
        if let Err(e) = sqlx::query(
            "INSERT OR IGNORE INTO script_device_assignments (id, script_id, device_token_id)
             VALUES (?, ?, ?)",
        )
        .bind(&assignment_id)
        .bind(&userscript_id)
        .bind(device_token_id)
        .execute(&state.pool)
        .await
        {
            tracing::warn!(task_id = %task_id, "failed to assign script to device: {e:#}");
        }
    }

    tracing::info!(task_id = %task_id, script_name = %resp.name, "generation complete");

    let user_slug = task.user_email
        .split('@')
        .next()
        .map(|s| slugify(s))
        .unwrap_or_else(|| "user".to_string());

    if let Err(e) = push_script_to_github(
        state,
        task_id,
        &user_slug,
        &task.tab_url,
        &task.prompt,
        &resp.name,
        &full_script,
    ).await {
        tracing::warn!(task_id = %task_id, "GitHub push skipped: {e:#}");
    }

    Ok(())
}

fn slugify(s: &str) -> String {
    let raw: String = s
        .chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
        .collect();
    raw.split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

async fn push_script_to_github(
    state: &Arc<AppState>,
    task_id: &str,
    user_slug: &str,
    tab_url: &str,
    prompt: &str,
    script_name: &str,
    content: &str,
) -> anyhow::Result<()> {
    let token = fetch_github_token(
        &state.http_client,
        &state.config.kv_url,
        &state.config.kv_api_key,
    )
    .await?;

    let repo = fetch_github_repo(
        &state.http_client,
        &state.config.kv_url,
        &state.config.kv_api_key,
    )
    .await?;

    let sha = push_to_github(
        &state.http_client,
        &token,
        &repo,
        task_id,
        user_slug,
        tab_url,
        script_name,
        prompt,
        content,
    ).await?;

    sqlx::query(
        "UPDATE tasks SET git_sha = ?, updated_at = datetime('now') WHERE id = ?",
    )
    .bind(&sha)
    .bind(task_id)
    .execute(&state.pool)
    .await?;

    tracing::info!(task_id = %task_id, git_sha = %sha, "pushed to GitHub");
    Ok(())
}
