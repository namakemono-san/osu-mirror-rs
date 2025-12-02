use super::OsuClient;
use super::client::start_rate_limiter;
use super::sync::{load_cursor, save_cursor, sync_beatmapsets_page};
use anyhow::Result;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;

pub async fn start_scheduler(pool: PgPool, client: Arc<OsuClient>, interval_seconds: u64) {
    tracing::info!(
        "Starting sync scheduler (base interval: {}s)",
        interval_seconds
    );

    start_rate_limiter().await;

    let pool = Arc::new(pool);

    spawn_worker(
        pool.clone(),
        client.clone(),
        "ranked_sync",
        "status=ranked",
        interval_seconds,
    );

    spawn_worker(
        pool.clone(),
        client.clone(),
        "loved_sync",
        "status=loved",
        interval_seconds * 2,
    );

    spawn_worker(
        pool.clone(),
        client.clone(),
        "qualified_sync",
        "status=qualified",
        interval_seconds,
    );

    spawn_worker(
        pool.clone(),
        client.clone(),
        "pending_sync",
        "status=pending",
        interval_seconds * 2,
    );

    spawn_worker(
        pool.clone(),
        client.clone(),
        "graveyard_sync",
        "status=graveyard&sort=updated_asc",
        interval_seconds * 3,
    );

    spawn_worker(
        pool.clone(),
        client.clone(),
        "any_updated_desc_sync",
        "sort=updated_desc",
        30,
    );

    spawn_worker(
        pool.clone(),
        client.clone(),
        "any_updated_asc_sync",
        "sort=updated_asc",
        interval_seconds * 3,
    );

    futures::future::pending::<()>().await;
}

fn spawn_worker(
    pool: Arc<PgPool>,
    client: Arc<OsuClient>,
    cursor_id: &'static str,
    query: &'static str,
    interval_seconds: u64,
) {
    tracing::info!(
        "Spawning worker: id={} query={} interval={}s",
        cursor_id,
        query,
        interval_seconds
    );

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(interval_seconds));

        interval.tick().await;
        interval.tick().await;

        loop {
            interval.tick().await;

            if let Err(e) = run_sync_cycle(&pool, &client, cursor_id, query).await {
                tracing::error!(
                    "Sync cycle failed: id={} query={} error={}",
                    cursor_id,
                    query,
                    e
                );
            } else {
                tracing::info!("Sync cycle completed: id={} query={}", cursor_id, query);
            }
        }
    });
}

async fn run_sync_cycle(
    pool: &PgPool,
    client: &OsuClient,
    cursor_id: &str,
    query: &str,
) -> Result<()> {
    let cursor = load_cursor(pool, cursor_id).await?;
    let new_cursor = sync_beatmapsets_page(pool, client, query, cursor).await?;
    save_cursor(pool, cursor_id, new_cursor).await?;
    Ok(())
}
