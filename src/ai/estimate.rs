use crate::ai::client::OpenRouterClient;
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

const SYSTEM_PROMPT: &str = r#"You are a pricing assistant for userscript development. Estimate based on human developer hours at 200 PLN/hour.

Complexity levels:
- Simple (hide/show, CSS tweaks): 0.5-1 hour = 100-200 PLN
- Moderate (form fill, clicks, observer): 1-3 hours = 200-600 PLN
- Complex (API calls, state, WebSocket): 3-8 hours = 600-1600 PLN

Return JSON with min_hours, max_hours, total_price_pln (whole number), and rationale."#;

#[derive(Debug, Deserialize)]
pub struct EstimateResponse {
    pub min_hours: f64,
    pub max_hours: f64,
    pub total_price_pln: i64,
    pub rationale: String,
}

pub async fn call_estimate(
    client: &OpenRouterClient<'_>,
    tab_url: &str,
    prompt: &str,
    page_html: &str,
    files_json: Option<&str>,
) -> Result<EstimateResponse> {
    let mut user_message = format!(
        "Page URL: {tab_url}\nUser request: {prompt}\nPage HTML:\n---\n{page_html}\n---"
    );

    if let Some(files) = files_json {
        user_message.push_str("\n\nSource files:\n");
        user_message.push_str(files);
    }

    let text = client.complete("google/gemma-4-31b-it", SYSTEM_PROMPT, &user_message, 256).await?;

    tracing::warn!("Estimate response from OpenRouter: {}", text);

    let json_str = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let resp: EstimateResponse =
        serde_json::from_str(json_str).context(format!("failed to parse estimate JSON from AI response: '{}'", json_str))?;

    if resp.total_price_pln < 0 || resp.total_price_pln > 10_000 {
        return Err(anyhow!(
            "total_price_pln out of acceptable range: {}",
            resp.total_price_pln
        ));
    }

    if resp.min_hours <= 0.0 || resp.max_hours <= 0.0 || resp.min_hours > resp.max_hours {
        return Err(anyhow!(
            "invalid hours: min={}, max={}",
            resp.min_hours,
            resp.max_hours
        ));
    }

    Ok(resp)
}
