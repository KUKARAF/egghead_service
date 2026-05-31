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
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ResponseFormat>,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
    json_schema: JsonSchema,
}

#[derive(Debug, Serialize)]
struct JsonSchema {
    name: String,
    schema: serde_json::Value,
    strict: bool,
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

    async fn send(&self, body: &RequestBody) -> Result<String> {
        let resp = self
            .http
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("content-type", "application/json")
            .json(body)
            .send()
            .await
            .context("OpenRouter API request failed")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("OpenRouter API error {status}: {text}"));
        }

        let text = resp
            .text()
            .await
            .context("failed to read OpenRouter response")?;

        tracing::debug!("OpenRouter response: {}", text);

        let response: ResponseBody = serde_json::from_str(&text)
            .context(format!("failed to parse OpenRouter response: {}", text))?;

        response
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| anyhow!("no choices in OpenRouter response"))
    }

    pub async fn complete(
        &self,
        model: &str,
        system: &str,
        user_message: &str,
        max_tokens: u32,
    ) -> Result<String> {
        let body = RequestBody {
            model: model.to_string(),
            max_tokens,
            messages: vec![
                Message { role: "system".to_string(), content: system.to_string() },
                Message { role: "user".to_string(), content: user_message.to_string() },
            ],
            response_format: None,
        };
        self.send(&body).await
    }

    pub async fn complete_with_messages(
        &self,
        model: &str,
        system: &str,
        messages: Vec<(String, String)>, // (role, content) pairs
        max_tokens: u32,
    ) -> Result<String> {
        let mut msgs = vec![Message { role: "system".to_string(), content: system.to_string() }];
        for (role, content) in messages {
            msgs.push(Message { role, content });
        }
        let body = RequestBody {
            model: model.to_string(),
            max_tokens,
            messages: msgs,
            response_format: None,
        };
        self.send(&body).await
    }

    pub async fn complete_with_schema(
        &self,
        model: &str,
        system: &str,
        user_message: &str,
        max_tokens: u32,
        schema: serde_json::Value,
        schema_name: &str,
    ) -> Result<String> {
        let body = RequestBody {
            model: model.to_string(),
            max_tokens,
            messages: vec![
                Message { role: "system".to_string(), content: system.to_string() },
                Message { role: "user".to_string(), content: user_message.to_string() },
            ],
            response_format: Some(ResponseFormat {
                format_type: "json_schema".to_string(),
                json_schema: JsonSchema {
                    name: schema_name.to_string(),
                    schema,
                    strict: true,
                },
            }),
        };
        self.send(&body).await
    }
}
