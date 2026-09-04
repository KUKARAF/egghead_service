mod api;
mod auth;
mod config;
mod db;
mod error;
mod frontend;
mod middleware;
mod models;
mod state;
mod worker;
mod ai;

use anyhow::Result;
use axum::{
    body::Body,
    http::{Response, StatusCode},
    middleware as axum_middleware,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<()> {
    let use_json = std::env::var("LOG_FORMAT").as_deref() == Ok("json");
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "egghead_service=info".into());

    if use_json {
        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    let config = config::Config::from_env()?;
    let pool = db::create_pool(&config.database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("migrations applied");

    let oidc_client = auth::oidc::init_client(
        &config.oidc_issuer_url,
        &config.oidc_client_id,
        &config.oidc_client_secret,
        &config.oidc_redirect_uri,
    )
    .await
    .map_err(|e| {
        tracing::warn!("OIDC init failed (login disabled): {e:#}");
    })
    .ok();

    let oidc_failed = oidc_client.is_none();
    let state = state::AppState::new(pool, config.clone(), oidc_client);

    // Spawn background workers
    worker::spawn_all(Arc::clone(&state));
    if oidc_failed {
        // oidc_retry spawned within worker::spawn_all
    }

    let app = Router::new()
        .route("/health", get(health))
        .route("/healthz", get(healthz))
        .route("/version", get(version))
        .nest("/auth", auth::router())
        .nest("/api", api::router())
        .nest("/", frontend::router())
        .layer(axum_middleware::from_fn(middleware::security_headers::layer))
        .with_state(Arc::clone(&state))
        .merge(
            SwaggerUi::new("/api/docs")
                .url("/api/openapi.json", api::openapi::ApiDoc::openapi())
        );

    let listener = TcpListener::bind(&config.listen_addr).await?;
    tracing::info!("listening on {}", config.listen_addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}

async fn health() -> &'static str {
    "ok"
}

async fn version() -> &'static str {
    env!("APP_VERSION")
}

async fn healthz(
    axum::extract::State(state): axum::extract::State<Arc<state::AppState>>,
) -> Response<Body> {
    match sqlx::query_scalar::<_, i64>("SELECT 1").fetch_one(&state.pool).await {
        Ok(_) => Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(Body::from(r#"{"status":"ok"}"#))
            .unwrap_or_else(|_| Response::new(Body::from(r#"{"status":"ok"}"#))),
        Err(e) => {
            tracing::error!("health check DB error: {e}");
            Response::builder()
                .status(StatusCode::SERVICE_UNAVAILABLE)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"status":"degraded","reason":"db unreachable"}"#))
                .unwrap_or_else(|_| {
                    let mut resp = Response::new(Body::from(
                        r#"{"status":"degraded","reason":"db unreachable"}"#,
                    ));
                    *resp.status_mut() = StatusCode::SERVICE_UNAVAILABLE;
                    resp
                })
        }
    }
}
