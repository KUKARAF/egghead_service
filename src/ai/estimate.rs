use crate::ai::client::OpenRouterClient;
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

const SYSTEM_PROMPT: &str = r#"You are a pricing assistant for userscript automation.

Return a JSON object with exactly these keys:
- human_hours (number >= 0): developer hours to implement this manually.
  Set to 0 if the task is simple enough for an AI agent to handle autonomously without human review.
- token_cost_eur (number > 0): estimated cost in EUR for the AI generation call.
  Assume ~4000 output tokens at €0.00005/token, then multiply by 2 for markup.
  Typical values: simple 0.02, moderate 0.05, complex 0.15.
- rationale (string): one or two sentences explaining your estimates.

Complexity guide for human_hours:
- Simple (CSS, hide/show, read-only DOM): 0.5–1 h
- Moderate (form fill, clicks, MutationObserver): 1–3 h
- Complex (API calls, state machines, WebSocket): 3–8 h

Set human_hours to 0 only for truly trivial tasks where AI generation is fully reliable."#;

#[derive(Debug, Deserialize)]
pub struct EstimateResponse {
    pub human_hours: f64,
    pub token_cost_eur: f64,
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

    let resp: EstimateResponse = serde_json::from_str(json_str)
        .context(format!("failed to parse estimate JSON from AI response: '{}'", json_str))?;

    if resp.human_hours < 0.0 {
        return Err(anyhow!("human_hours must be >= 0, got {}", resp.human_hours));
    }

    if resp.token_cost_eur <= 0.0 || resp.token_cost_eur >= 100.0 {
        return Err(anyhow!(
            "token_cost_eur out of acceptable range (0, 100): {}",
            resp.token_cost_eur
        ));
    }

    Ok(resp)
}
