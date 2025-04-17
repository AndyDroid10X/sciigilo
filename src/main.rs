#![forbid(unsafe_code)]

use std::sync::Arc;

use axum::{Json, Router, routing::get};
use sysinfo::System;
use tokio::net::TcpListener;
mod models;
mod utils;
use utils::{config, db, log::Logger, watchtower::Watchtower};
mod routes;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let mut app_config = config::EnvConfig::new();
    app_config.read_config();

    let mut alerts_config = config::AlertConfig::new(&app_config);
    alerts_config.read_config().await;

    let logger = Logger::new(&app_config.log_file_path).unwrap();

    let pool = match db::connect(app_config.db_file_path.as_str()).await {
        Ok(pool) => Arc::new(pool),
        Err(e) => {
            eprintln!("Failed to connect to database: {:?}", e);
            return;
        }
    };

    db::init_db(&pool, app_config.retention_period).await;

    let collector_db = pool.clone();

    tokio::spawn(async move {
        let mut collector = utils::collector::MetricsCollector::new(collector_db);
        loop {
            collector.collect_metrics().await;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let wt_env = app_config.clone();

    let mut wt = Watchtower::new(pool.clone(), wt_env, logger.clone());
    tokio::spawn(async move {
        wt.watch().await;
    });

    let cors = if app_config.domain.is_some() {
        CorsLayer::new()
            .allow_origin(app_config.domain.as_ref().unwrap().parse::<axum::http::HeaderValue>().unwrap())
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    };



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
        .with_state(pool.clone())
        .nest("/alerts", routes::alerts::get_routes())
        .with_state(alerts_config)
        .nest("/logs", routes::logs::get_routes())
        .with_state(logger.clone())
        .merge(routes::index::get_routes())
        .layer(cors);

    let listener = TcpListener::bind(("0.0.0.0", app_config.port))
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
