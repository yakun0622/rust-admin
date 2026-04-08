use anyhow::Context;
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    app::{routes, state::AppState},
    core::{config::AppConfig, logging},
};

pub async fn run() -> anyhow::Result<()> {
    let cfg = AppConfig::load()?;
    logging::init(&cfg.app.log_level)?;
    let state = AppState::new(cfg.clone()).await?;

    let addr = format!("{}:{}", cfg.server.host, cfg.server.port);
    let listener = TcpListener::bind(&addr)
        .await
        .with_context(|| format!("failed to bind address: {addr}"))?;

    let app = routes::build_router(state);
    info!(
        app = %cfg.app.name,
        env = %cfg.app.env,
        %addr,
        database_driver = %cfg.database.driver.as_str(),
        database_max_connections = cfg.database.max_connections,
        redis_pool_size = cfg.redis.pool_size,
        "admin-api started"
    );

    axum::serve(listener, app)
        .await
        .context("admin-api exited with server error")
}
