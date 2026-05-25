use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::Deserialize;

#[derive(Deserialize)]
struct GithubCommit {
    sha: String,
}

#[derive(Deserialize)]
struct GithubContentResponse {
    sha: String,
}

#[derive(Deserialize)]
struct GithubPushResponse {
    commit: GithubCommit,
}

fn slugify(s: &str) -> String {
    let raw: String = s
        .chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
        .collect();
    raw.split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn extract_site(url: &str) -> String {
    ::url::Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(|h| h.to_string()))
        .unwrap_or_else(|| "unknown".to_string())
}

async fn get_file_sha(
    http: &reqwest::Client,
    token: &str,
    repo: &str,
    path: &str,
) -> Result<Option<String>> {
    let url = format!("https://api.github.com/repos/{repo}/contents/{path}");

    let resp = http
        .get(&url)
        .header("Authorization", format!("Bearer {token}"))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "egghead-service")
        .send()
        .await
        .context("GitHub API request failed")?;

    if resp.status() == 404 {
        return Ok(None);
    }

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(anyhow!("GitHub API error {status}: {text}"));
    }

    let content: GithubContentResponse = resp
        .json()
        .await
        .context("failed to parse GitHub API response")?;

    Ok(Some(content.sha))
}

fn build_toml(task_id: &str, tab_url: &str, prompt: &str, status: &str) -> String {
    let escaped_prompt = prompt.replace('\\', "\\\\").replace('"', "\\\"");
    format!(
        "[task]\nid = \"{task_id}\"\nstatus = \"{status}\"\ntab_url = \"{tab_url}\"\nprompt = \"\"\"\n{escaped_prompt}\n\"\"\"\n"
    )
}

async fn put_file(
    http: &reqwest::Client,
    token: &str,
    repo: &str,
    path: &str,
    message: &str,
    content: &str,
) -> Result<String> {
    let url = format!("https://api.github.com/repos/{repo}/contents/{path}");
    let encoded = STANDARD.encode(content.as_bytes());

    let sha = get_file_sha(http, token, repo, path).await?;

    let mut body = serde_json::json!({
        "message": message,
        "content": encoded,
    });

    if let Some(sha) = sha {
        body["sha"] = serde_json::json!(sha);
    }

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

pub async fn push_to_github(
    http: &reqwest::Client,
    token: &str,
    repo: &str,
    task_id: &str,
    user_slug: &str,
    tab_url: &str,
    script_name: &str,
    prompt: &str,
    js_content: &str,
) -> Result<String> {
    let site = slugify(&extract_site(tab_url));
    let script_slug = slugify(script_name);
    let base = format!("{user_slug}/{site}/{script_slug}");

    let js_path = format!("{base}.user.js");
    let toml_path = format!("{base}.toml");

    let sha = put_file(http, token, repo, &js_path, &format!("add: {script_name}"), js_content).await?;

    let toml = build_toml(task_id, tab_url, prompt, "done");
    if let Err(e) = put_file(http, token, repo, &toml_path, &format!("meta: {script_name}"), &toml).await {
        tracing::warn!(task_id = %task_id, "TOML push failed (non-fatal): {e:#}");
    }

    Ok(sha)
}
