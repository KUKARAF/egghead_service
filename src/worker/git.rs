use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::Deserialize;

#[derive(Deserialize)]
struct GithubCommit {
    sha: String,
}

#[derive(Deserialize)]
struct GithubPushResponse {
    commit: GithubCommit,
}

pub async fn push_to_github(
    http: &reqwest::Client,
    token: &str,
    repo: &str,
    task_id: &str,
    script_name: &str,
    content: &str,
) -> Result<String> {
    let path = format!("scripts/{task_id}.user.js");
    let url = format!("https://api.github.com/repos/{repo}/contents/{path}");
    let encoded = STANDARD.encode(content.as_bytes());

    let body = serde_json::json!({
        "message": format!("add: {script_name}"),
        "content": encoded,
    });

    let resp = http
        .put(&url)
        .header("Authorization", format!("Bearer {token}"))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "egghead-service")
        .json(&body)
        .send()
        .await
        .context("GitHub API request failed")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(anyhow!("GitHub API error {status}: {text}"));
    }

    let push_resp: GithubPushResponse = resp
        .json()
        .await
        .context("failed to parse GitHub API response")?;

    Ok(push_resp.commit.sha)
}
