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
    files_json: Option<&str>,
) -> Result<GenerateResponse> {
    let recording_section = action_recording
        .map(|r| format!("Action recording:\n---\n{r}\n---"))
        .unwrap_or_else(|| "Action recording:\n---\nNone\n---".to_string());

    let mut user_message = format!(
        "Page URL: {tab_url}\nUser request: {prompt}\nPage HTML:\n---\n{page_html}\n---\n{recording_section}"
    );

    if let Some(files) = files_json {
        user_message.push_str("\n\nSource files:\n");
        user_message.push_str(files);
    }

    let text = client.complete("google/gemma-4-31b-it", SYSTEM_PROMPT, &user_message, 4096).await?;

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

/// Iterative generation: takes existing conversation history (role, content pairs) and a new prompt.
/// The first user message includes page HTML; subsequent ones are plain follow-ups.
pub async fn call_generate_iterative(
    client: &OpenRouterClient<'_>,
    tab_url: &str,
    new_prompt: &str,
    page_html: &str,
    history: &[(String, String)], // (role, content) from script_messages, oldest first
) -> Result<GenerateResponse> {
    let mut messages: Vec<(String, String)> = Vec::new();

    if history.is_empty() {
        // No history: treat like a fresh generation
        let user_msg = format!("Page URL: {tab_url}\nUser request: {new_prompt}\nPage HTML:\n---\n{page_html}\n---");
        messages.push(("user".to_string(), user_msg));
    } else {
        // First message gets the fresh HTML; history follows; new prompt last
        let first_user = format!("Page URL: {tab_url}\nUser request: {}\nPage HTML:\n---\n{page_html}\n---",
            history.iter().find(|(r, _)| r == "user").map(|(_, c)| c.as_str()).unwrap_or(new_prompt)
        );
        messages.push(("user".to_string(), first_user));
        // Append remaining history after the first user message
        let mut skip_first_user = true;
        for (role, content) in history {
            if skip_first_user && role == "user" {
                skip_first_user = false;
                continue;
            }
            messages.push((role.clone(), content.clone()));
        }
        messages.push(("user".to_string(), new_prompt.to_string()));
    }

    let text = client.complete_with_messages("google/gemma-4-31b-it", SYSTEM_PROMPT, messages, 4096).await?;

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
pub fn with_userscript_header(name: &str, tab_url: &str, match_pattern: &str, code: &str) -> String {
    format!(
        "// ==UserScript==\n// @name         {name}\n// @namespace    https://egghead.osmosis.page\n// @version      1.0\n// @url          {tab_url}\n// @match        {match_pattern}\n// @run-at       document-end\n// @grant        none\n// ==/UserScript==\n\n{code}"
    )
}
