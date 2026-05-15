use crate::state::AppState;
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
        sqlx::query(
            "UPDATE tasks
             SET status = 'ready_for_implementation', updated_at = datetime('now')
             WHERE id = ? AND status = 'awaiting_approval' AND approved_at IS NOT NULL"
        )
        .bind(&row.id)
        .execute(&state.pool)
        .await?;

        tracing::info!(task_id = %row.id, "task ready for human implementation");
    }

    Ok(())
}
