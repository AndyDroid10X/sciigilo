use crate::models::metrics::MetricType;
use crate::utils::db;
use axum::{Router, extract::State, response::Json, routing::get};
use sqlx::SqlitePool;

pub fn get_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/cpu", get(get_last_cpu_metrics))
        .route("/memory", get(get_last_memory_metrics))
        .route("/disk", get(get_last_disk_metrics))
}

async fn get_last_cpu_metrics(State(pool): State<SqlitePool>) -> Json<MetricType> {
    let metric = db::get_metric(&pool, MetricType::Cpu(Default::default())).await;
    Json(metric)
}

async fn get_last_memory_metrics(State(pool): State<SqlitePool>) -> Json<MetricType> {
    let metric = db::get_metric(&pool, MetricType::Memory(Default::default())).await;
    Json(metric)
}

async fn get_last_disk_metrics(State(pool): State<SqlitePool>) -> Json<MetricType> {
    let metric = db::get_metric(&pool, MetricType::Disk(Default::default())).await;
    Json(metric)
}
