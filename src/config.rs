use anyhow::{Context, Result};
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub listen_addr: String,
    pub dev_mode: bool,

    pub oidc_issuer_url: String,
    pub oidc_client_id: String,
    pub oidc_client_secret: String,
    pub oidc_redirect_uri: String,
    pub session_signing_key: String,

    pub kv_api_key: String,
    pub kv_url: String,

    pub worker_poll_interval_secs: u64,
    pub max_html_bytes: usize,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let dev_mode = env::var("ENV").as_deref() == Ok("DEVELOPMENT");

        Ok(Config {
            database_url: env::var("DATABASE_URL").context("DATABASE_URL is required")?,
            listen_addr: env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
            dev_mode,

            oidc_issuer_url: env::var("OIDC_ISSUER_URL").unwrap_or_default(),
            oidc_client_id: env::var("OIDC_CLIENT_ID").unwrap_or_default(),
            oidc_client_secret: env::var("OIDC_CLIENT_SECRET").unwrap_or_default(),
            oidc_redirect_uri: env::var("OIDC_REDIRECT_URI").unwrap_or_default(),

            session_signing_key: env::var("SESSION_SIGNING_KEY").unwrap_or_else(|_| {
                use rand::RngCore;
                let mut bytes = [0u8; 32];
                rand::thread_rng().fill_bytes(&mut bytes);
                let key = hex::encode(bytes);
                tracing::warn!(
                    "SESSION_SIGNING_KEY not set — generated ephemeral key (sessions will invalidate on restart)"
                );
                key
            }),

            kv_api_key: env::var("KV_API_KEY")
                .context("KV_API_KEY is required to fetch OpenRouter API key from kv.osmosis.page")?,
            kv_url: env::var("KV_URL")
                .unwrap_or_else(|_| "https://kv.osmosis.page".to_string()),

            worker_poll_interval_secs: env::var("WORKER_POLL_INTERVAL_SECS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .context("WORKER_POLL_INTERVAL_SECS must be a number")?,

            max_html_bytes: env::var("MAX_HTML_BYTES")
                .unwrap_or_else(|_| "150000".to_string())
                .parse()
                .context("MAX_HTML_BYTES must be a number")?,
        })
    }
}
