use crate::{ai::estimate::call_estimate, state::AppState};
use std::sync::Arc;
use tokio::time::{interval, Duration};

pub async fn run(state: Arc<AppState>) {
    let poll_secs = state.config.worker_poll_interval_secs;
    let mut ticker = interval(Duration::from_secs(poll_secs));
    loop {
        ticker.tick().await;
        if let Err(e) = poll_pending(&state).await {
            tracing::error!("estimator poll error: {e:#}");
        }
    }
}

async fn poll_pending(state: &Arc<AppState>) -> anyhow::Result<()> {
    #[derive(sqlx::FromRow)]
    struct TaskRow {
        id: String,
    }

    let rows = sqlx::query_as::<_, TaskRow>(
        "SELECT id FROM tasks WHERE status = 'pending' LIMIT 5"
    )
    .fetch_all(&state.pool)
    .await?;

    for row in rows {
        let claimed = sqlx::query(
            "UPDATE tasks
             SET status = 'estimating', worker_started_at = datetime('now'), updated_at = datetime('now')
             WHERE id = ? AND status = 'pending'"
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
            if let Err(e) = run_estimation(&state, &task_id).await {
                tracing::error!(task_id = %task_id, "estimation failed: {e:#}");
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

async fn run_estimation(state: &Arc<AppState>, task_id: &str) -> anyhow::Result<()> {
    #[derive(sqlx::FromRow)]
    struct TaskData {
        tab_url: String,
        prompt: String,
        page_html: String,
    }

    let task = sqlx::query_as::<_, TaskData>(
        "SELECT tab_url, prompt, page_html FROM tasks WHERE id = ?"
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

    let resp = call_estimate(&client, &task.tab_url, &task.prompt, &task.page_html).await?;

    sqlx::query(
        "UPDATE tasks
         SET status = 'awaiting_approval',
             estimated_price_cents = ?,
             price_rationale = ?,
             updated_at = datetime('now')
         WHERE id = ?"
    )
    .bind(resp.price_cents)
    .bind(&resp.rationale)
    .bind(task_id)
    .execute(&state.pool)
    .await?;

    tracing::info!(task_id = %task_id, price_cents = resp.price_cents, "estimation complete");
    Ok(())
}
