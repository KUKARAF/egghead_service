use anyhow::{anyhow, Context, Result};

async fn fetch_kv_secret(
    http: &reqwest::Client,
    kv_url: &str,
    kv_api_key: &str,
    key_name: &str,
) -> Result<String> {
    let url = format!("{kv_url}/kv/{key_name}");

    let resp = http
        .get(&url)
        .header("X-Api-Key", kv_api_key)
        .send()
        .await
        .context(format!("failed to fetch {key_name} from KV"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(anyhow!("KV fetch failed for {key_name}: {status} {text}"));
    }

    let value = resp
        .text()
        .await
        .context(format!("failed to read KV response for {key_name}"))?
        .trim()
        .to_string();

    if value.is_empty() {
        return Err(anyhow!("{key_name} is empty in KV"));
    }

    Ok(value)
}

pub async fn fetch_openrouter_key(
    http: &reqwest::Client,
    kv_url: &str,
    kv_api_key: &str,
) -> Result<String> {
    fetch_kv_secret(http, kv_url, kv_api_key, "EXTENSION_OPENROUTER_API").await
}

pub async fn fetch_github_token(
    http: &reqwest::Client,
    kv_url: &str,
    kv_api_key: &str,
) -> Result<String> {
    fetch_kv_secret(http, kv_url, kv_api_key, "GITHUB_TOKEN").await
}

pub async fn fetch_github_repo(
    http: &reqwest::Client,
    kv_url: &str,
    kv_api_key: &str,
) -> Result<String> {
    fetch_kv_secret(http, kv_url, kv_api_key, "GITHUB_REPO").await
}
