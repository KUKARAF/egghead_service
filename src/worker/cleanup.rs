use crate::state::AppState;
use std::sync::Arc;
use tokio::time::{interval, Duration};

pub async fn run(state: Arc<AppState>) {
    let mut ticker = interval(Duration::from_secs(300));
    loop {
        ticker.tick().await;
        run_cleanup(&state).await;
    }
}

async fn run_cleanup(state: &Arc<AppState>) {
    // Reset tasks stuck in estimating/processing (worker crashed mid-task)
    match sqlx::query(
        "UPDATE tasks
         SET status = 'pending', worker_started_at = NULL, updated_at = datetime('now')
         WHERE status IN ('estimating', 'processing')
           AND worker_started_at < datetime('now', '-5 minutes')"
    )
    .execute(&state.pool)
    .await
    {
        Ok(r) if r.rows_affected() > 0 => {
            tracing::info!(count = r.rows_affected(), "reset stuck tasks to pending");
        }
        Err(e) => tracing::error!("cleanup: reset stuck tasks error: {e}"),
        _ => {}
    }

    // Delete expired session tokens
    match sqlx::query("DELETE FROM session_tokens WHERE expires_at <= datetime('now')")
        .execute(&state.pool)
        .await
    {
        Err(e) => tracing::error!("cleanup: session expiry error: {e}"),
        _ => {}
    }
}
