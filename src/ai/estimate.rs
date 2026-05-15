use crate::ai::client::OpenRouterClient;
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

const SYSTEM_PROMPT: &str = r#"You are a pricing assistant. Your ONLY job is to return valid JSON.

Guidelines:
- Simple changes (hide/show, CSS): 5-10 cents
- Moderate (form fill, clicks, observer): 10-25 cents
- Complex (API, state, WebSocket): 25-50 cents

Return ONLY this JSON, nothing else:
{"price_cents": NUMBER, "rationale": "explanation"}

Example: {"price_cents": 15, "rationale": "Requires DOM manipulation"}

DO NOT explain, apologize, or add text. ONLY JSON."#;

#[derive(Debug, Deserialize)]
pub struct EstimateResponse {
    pub price_cents: i64,
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

    // Strip markdown code fences if present
    let json_str = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let resp: EstimateResponse =
        serde_json::from_str(json_str).context(format!("failed to parse estimate JSON from AI response: '{}'", json_str))?;

    if resp.price_cents < 0 || resp.price_cents > 10_000 {
        return Err(anyhow!(
            "price_cents out of acceptable range: {}",
            resp.price_cents
        ));
    }

    Ok(resp)
}
