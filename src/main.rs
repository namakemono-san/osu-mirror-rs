mod api;
mod config;
mod crawler;
mod db;
mod error;
mod middleware;
mod storage;

use anyhow::Result;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};

#[derive(Clone)]
pub struct AppState {
    pub config: config::Config,
    pub db: PgPool,
    pub storage: storage::BeatmapStorage,
    pub osu_client: Arc<crawler::OsuClient>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn".into()),
        )
        .init();

    tracing::info!("Starting osu-mirror-rs...");

    let config = config::Config::load()?;
    let db = db::pool::create_pool(&config.database.url, config.database.max_connections).await?;
    db::pool::run_migrations(&db).await?;

    let storage = match &config.storage.backend {
        config::StorageBackend::Local => {
            let local_config = config
                .storage
                .local
                .as_ref()
                .expect("Local storage config required");
            storage::BeatmapStorage::Local(storage::LocalStorage::new(local_config.path.clone()))
        }
        config::StorageBackend::S3 => {
            let s3_config = config
                .storage
                .s3
                .as_ref()
                .expect("S3 storage config required");
            storage::BeatmapStorage::S3(
                storage::S3Storage::new(
                    &s3_config.endpoint,
                    s3_config.bucket.clone(),
                    &s3_config.region,
                    s3_config.prefix.clone(),
                )
                .await,
            )
        }
    };

    tracing::info!("Storage backend: {:?}", config.storage.backend);

    let osu_client = Arc::new(crawler::OsuClient::new(
        config.osu.client_id.clone(),
        config.osu.client_secret.clone(),
    ));

    let state = AppState {
        config: config.clone(),
        db: db.clone(),
        storage,
        osu_client: osu_client.clone(),
    };

    if config.crawler.enabled {
        let db_clone = db.clone();
        let client_clone = osu_client.clone();
        let interval = config.crawler.sync_interval_seconds;
        tokio::spawn(async move {
            crawler::start_scheduler(db_clone, client_clone, interval).await;
        });
    }

    let app = api::routes::create_router(state)
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server listening on http://{}", addr);
    tracing::info!("Health check: http://{}/health", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
