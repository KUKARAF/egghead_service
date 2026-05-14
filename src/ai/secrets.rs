use anyhow::{anyhow, Context, Result};

/// Fetches the OpenRouter API key from kv.osmosis.page using a KV API key.
pub async fn fetch_openrouter_key(
    http: &reqwest::Client,
    kv_url: &str,
    kv_api_key: &str,
) -> Result<String> {
    let url = format!("{kv_url}/kv/EXTENSION_OPENROUTER_API");

    let resp = http
        .get(&url)
        .header("X-Api-Key", kv_api_key)
        .send()
        .await
        .context("failed to fetch OpenRouter key from KV")?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "KV fetch failed: {} {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        ));
    }

    let key = resp
        .text()
        .await
        .context("failed to read KV response body")?
        .trim()
        .to_string();

    if key.is_empty() {
        return Err(anyhow!("EXTENSION_OPENROUTER_API key is empty in KV"));
    }

    Ok(key)
}
