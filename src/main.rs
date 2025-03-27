#![forbid(unsafe_code)]

use axum::{Json, Router, routing::get};
use sysinfo::System;
use tokio::net::TcpListener;
mod models;
mod utils;
use utils::{config, db, watchtower};
mod routes;

#[tokio::main]
async fn main() {
    let mut app_config = config::EnvConfig::new();
    let mut alerts_config = config::AlertConfig::new();
    alerts_config.read_config().await;
    app_config.read_config();

    let pool = match db::connect(app_config.db_file_path.as_str()).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to database: {:?}", e);
            return;
        }
    };

    db::init_db(&pool).await;

    let collector_pool = pool.clone();

    tokio::spawn(async move {
        let mut collector = utils::collector::MetricsCollector::new(collector_pool);
        loop {
            collector.collect_metrics().await;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let watchtower_pool = pool.clone();

    tokio::spawn(async move {
        watchtower::watch(watchtower_pool).await;
    });


    let app = Router::new()
        .route(
            "/health",
            get(|| async {
                Json(
                        format!(
                            "{{\"status\": \"ok\", \"version\": \"{}\", \"hostname\": \"{}\", \"uptime\": {}}}",
                            env!("CARGO_PKG_VERSION"),
                            System::host_name().unwrap_or("unknown".to_string()),
                            System::uptime()
                        )
                )
            }),
        )
        .nest("/metrics", routes::metrics::get_routes())
        .with_state(pool)
        .nest("/alerts", routes::alerts::get_routes())
        .with_state(alerts_config)
        .merge(routes::index::get_routes());

    let listener = TcpListener::bind(("0.0.0.0", app_config.port))
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
