use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct RequestBody {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct ResponseBody {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct ChoiceMessage {
    content: String,
}

pub struct OpenRouterClient<'a> {
    http: &'a reqwest::Client,
    api_key: &'a str,
}

impl<'a> OpenRouterClient<'a> {
    pub fn new(http: &'a reqwest::Client, api_key: &'a str) -> Self {
        OpenRouterClient { http, api_key }
    }

    pub async fn complete(
        &self,
        system: &str,
        user_message: &str,
        max_tokens: u32,
    ) -> Result<String> {
        let body = RequestBody {
            model: "anthropic/claude-3.5-haiku".to_string(),
            max_tokens,
            system: system.to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: user_message.to_string(),
            }],
        };

        let resp = self
            .http
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .context("OpenRouter API request failed")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("OpenRouter API error {status}: {text}"));
        }

        let response: ResponseBody = resp
            .json()
            .await
            .context("failed to parse OpenRouter response")?;

        response
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| anyhow!("no choices in OpenRouter response"))
    }
}
