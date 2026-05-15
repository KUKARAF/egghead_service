use anyhow::{anyhow, Context, Result};

/// Fetches the OpenRouter API key from kv.osmosis.page using a KV API key.
pub async fn fetch_openrouter_key(
    http: &reqwest::Client,
    kv_url: &str,
    kv_api_key: &str,
) -> Result<String> {
    let url = format!("{kv_url}/kv/EXTENSION_OPENROUTER_API");

    tracing::info!("Fetching OpenRouter key from: {}", url);

    let resp = http
        .get(&url)
        .header("X-Api-Key", kv_api_key)
        .send()
        .await
        .context("failed to fetch OpenRouter key from KV")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        tracing::error!("KV fetch failed: {} {}", status, text);
        return Err(anyhow!(
            "KV fetch failed: {} {}",
            status,
            text
        ));
    }

    let key = resp
        .text()
        .await
        .context("failed to read KV response body")?
        .trim()
        .to_string();

    if key.is_empty() {
        tracing::error!("EXTENSION_OPENROUTER_API key is empty in KV");
        return Err(anyhow!("EXTENSION_OPENROUTER_API key is empty in KV"));
    }

    tracing::info!("Successfully fetched OpenRouter key (length: {})", key.len());
    Ok(key)
}
