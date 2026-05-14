use crate::ai::client::OpenRouterClient;
use anyhow::{Context, Result};
use serde::Deserialize;

const SYSTEM_PROMPT: &str = r#"You are an expert JavaScript developer generating ViolentMonkey-compatible userscripts.

Rules:
- Write vanilla JS only. No jQuery, no CDN dependencies.
- The script must be self-contained and idempotent.
- Do not use document.write or eval.
- Wrap all logic in an IIFE: (function() { 'use strict'; ... })();
- Do NOT include the ==UserScript== header block — it will be added automatically.

Respond with ONLY valid JSON in this exact format:
{"name": "<human-readable script name>", "match_pattern": "<URL match pattern, e.g. *://example.com/*>", "script_code": "<complete JS code>"}

Do not include any other text outside the JSON."#;

#[derive(Debug, Deserialize)]
pub struct GenerateResponse {
    pub name: String,
    pub match_pattern: String,
    pub script_code: String,
}

pub async fn call_generate(
    client: &OpenRouterClient<'_>,
    tab_url: &str,
    prompt: &str,
    page_html: &str,
    action_recording: Option<&str>,
) -> Result<GenerateResponse> {
    let recording_section = action_recording
        .map(|r| format!("Action recording:\n---\n{r}\n---"))
        .unwrap_or_else(|| "Action recording:\n---\nNone\n---".to_string());

    let user_message = format!(
        "Page URL: {tab_url}\nUser request: {prompt}\nPage HTML:\n---\n{page_html}\n---\n{recording_section}"
    );

    let text = client.complete(SYSTEM_PROMPT, &user_message, 4096).await?;

    let json_str = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let resp: GenerateResponse =
        serde_json::from_str(json_str).context("failed to parse generate JSON from AI response")?;

    Ok(resp)
}

/// Prepends the ViolentMonkey ==UserScript== header to script code.
pub fn with_userscript_header(name: &str, match_pattern: &str, code: &str) -> String {
    format!(
        "// ==UserScript==\n// @name         {name}\n// @namespace    https://egghead.osmosis.page\n// @version      1.0\n// @match        {match_pattern}\n// @grant        none\n// ==/UserScript==\n\n{code}"
    )
}
