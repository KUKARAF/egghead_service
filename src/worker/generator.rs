use crate::{
    ai::generate::call_generate,
    state::AppState,
    worker::git,
};
use std::sync::Arc;
use tokio::time::{interval, Duration};

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
         LIMIT 5"
    )
    .fetch_all(&state.pool)
    .await?;

    for row in rows {
        let claimed = sqlx::query(
            "UPDATE tasks
             SET status = 'processing', worker_started_at = datetime('now'), updated_at = datetime('now')
             WHERE id = ? AND status = 'awaiting_approval' AND approved_at IS NOT NULL"
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
                    "UPDATE tasks SET status = 'failed', error_message = ?, updated_at = datetime('now') WHERE id = ?"
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
        tab_url: String,
        prompt: String,
        page_html: String,
        action_recording: Option<String>,
        files_json: Option<String>,
    }

    let task = sqlx::query_as::<_, TaskData>(
        "SELECT user_id, tab_url, prompt, page_html, action_recording, files_json FROM tasks WHERE id = ?"
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

    let scripts_dir = "/app/scripts";
    git::init_repo_if_needed(scripts_dir)?;

    let user_scripts_dir = format!("{}/{}", scripts_dir, task.user_id);
    std::fs::create_dir_all(&user_scripts_dir)?;

    let file_path = format!("{}/{}.user.js", user_scripts_dir, resp.name.replace(" ", "_"));
    let full_script = crate::ai::generate::with_userscript_header(&resp.name, &resp.match_pattern, &resp.script_code);

    let git_sha = git::commit_script(
        scripts_dir,
        &file_path,
        &full_script,
        &format!("generate {} for task {}", resp.name, task_id),
    )?;

    sqlx::query(
        "UPDATE tasks
         SET status = 'done',
             script_name = ?,
             script_code = ?,
             match_pattern = ?,
             git_sha = ?,
             updated_at = datetime('now')
         WHERE id = ?"
    )
    .bind(&resp.name)
    .bind(&resp.script_code)
    .bind(&resp.match_pattern)
    .bind(&git_sha)
    .bind(task_id)
    .execute(&state.pool)
    .await?;

    tracing::info!(task_id = %task_id, script_name = %resp.name, git_sha = %git_sha, "generation complete");
    Ok(())
}
