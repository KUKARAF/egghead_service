use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct FileAttachment {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: String,
    pub user_id: String,
    pub tab_url: String,
    pub prompt: String,
    pub page_html: String,
    pub action_recording: Option<String>,
    pub files_json: Option<String>,
    pub status: String,
    pub estimated_price_pln: Option<i64>,
    pub estimated_human_hours: Option<f64>,
    pub estimated_token_cost_eur: Option<f64>,
    pub price_rationale: Option<String>,
    pub approved_at: Option<String>,
    pub rejected_at: Option<String>,
    pub script_name: Option<String>,
    pub script_code: Option<String>,
    pub match_pattern: Option<String>,
    pub error_message: Option<String>,
    pub worker_started_at: Option<String>,
    pub git_sha: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Lightweight task view for API responses (omits large html/recording/files fields).
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct TaskView {
    pub id: String,
    pub tab_url: String,
    pub prompt: String,
    pub status: String,
    pub estimated_price_pln: Option<i64>,
    pub estimated_human_hours: Option<f64>,
    pub estimated_token_cost_eur: Option<f64>,
    pub price_rationale: Option<String>,
    pub script_name: Option<String>,
    pub script_code: Option<String>,
    pub match_pattern: Option<String>,
    pub error_message: Option<String>,
    pub git_sha: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Task> for TaskView {
    fn from(t: Task) -> Self {
        TaskView {
            id: t.id,
            tab_url: t.tab_url,
            prompt: t.prompt,
            status: t.status,
            estimated_price_pln: t.estimated_price_pln,
            estimated_human_hours: t.estimated_human_hours,
            estimated_token_cost_eur: t.estimated_token_cost_eur,
            price_rationale: t.price_rationale,
            script_name: t.script_name,
            script_code: t.script_code,
            match_pattern: t.match_pattern,
            error_message: t.error_message,
            git_sha: t.git_sha,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}
