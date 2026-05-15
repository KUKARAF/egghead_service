use crate::ai::client::OpenRouterClient;
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

const SYSTEM_PROMPT: &str = r#"You are a pricing assistant for userscript development. Return ONLY valid JSON.

Estimate based on human developer hours at $50/hour (~833 cents/hour).

Complexity levels:
- Simple (hide/show, CSS tweaks): 0.5-1 hour = $25-50 = 2500-5000 cents
- Moderate (form fill, clicks, observer): 1-3 hours = $50-150 = 5000-15000 cents
- Complex (API calls, state, WebSocket): 3-8 hours = $150-400 = 15000-40000 cents

Return ONLY this JSON, no other text:
{
  "min_hours": NUMBER,
  "max_hours": NUMBER,
  "total_price_cents": NUMBER,
  "rationale": "brief explanation"
}

Example: {"min_hours": 1, "max_hours": 2, "total_price_cents": 8333, "rationale": "Requires DOM manipulation"}"#;

#[derive(Debug, Deserialize)]
pub struct EstimateResponse {
    pub min_hours: f64,
    pub max_hours: f64,
    pub total_price_cents: i64,
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

    let text = client.complete(SYSTEM_PROMPT, &user_message, 256).await?;

    tracing::warn!("Estimate response from OpenRouter: {}", text);

    // Strip markdown code fences if present (for models that wrap JSON)
    let json_str = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let resp: EstimateResponse =
        serde_json::from_str(json_str).context(format!("failed to parse estimate JSON from AI response: '{}'", json_str))?;

    if resp.total_price_cents < 0 || resp.total_price_cents > 100_000 {
        return Err(anyhow!(
            "total_price_cents out of acceptable range: {}",
            resp.total_price_cents
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
