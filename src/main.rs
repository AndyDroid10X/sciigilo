use axum::{Router, response::Html, routing::get};
use sysinfo::System;
use tokio::net::TcpListener;
mod models;
mod utils;
use utils::{config, db};
mod routes;

#[tokio::main]
async fn main() {
    let mut app_config = config::Config::new();
    config::read_config(&mut app_config);

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
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    let app = Router::new()
        .route(
            "/check",
            get(|| async {
                Html(format!(
                    "{} is OK",
                    System::name().unwrap_or_else(|| "System".into())
                ))
            }),
        )
        .nest("/metrics", routes::metrics::get_routes())
        .with_state(pool);

    let listener = TcpListener::bind(("127.0.0.1", app_config.port))
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
