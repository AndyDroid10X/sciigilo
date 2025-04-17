use std::sync::Arc;

use crate::models::{mem::MemoryMetrics, metrics::MetricType};
use crate::utils::db;
use axum::extract::Query;
use axum::{Router, extract::State, response::Json, routing::get};
use sqlx::SqlitePool;

#[derive(serde::Deserialize)]
struct MetricHistory {
    start_time: Option<i64>,
    end_time: Option<i64>,
}

pub fn get_routes() -> Router<Arc<SqlitePool>> {
    Router::new()
        .route("/cpu", get(get_last_cpu_metrics))
        .route("/cpu/history", get(cpu_history))
        .route("/memory", get(get_last_memory_metrics))
        .route("/memory/history", get(memory_history))
        .route("/disk", get(get_last_disk_metrics))
}

async fn get_last_cpu_metrics(State(pool): State<Arc<SqlitePool>>) -> Json<MetricType> {
    let metric = db::get_metric(&pool, MetricType::Cpu(Default::default())).await;
    Json(metric)
}

async fn get_last_memory_metrics(State(pool): State<Arc<SqlitePool>>) -> Json<MetricType> {
    let metric = db::get_metric(&pool, MetricType::Memory(Default::default())).await;
    Json(metric)
}

async fn get_last_disk_metrics(State(pool): State<Arc<SqlitePool>>) -> Json<MetricType> {
    let metric = db::get_metric(&pool, MetricType::Disk(Default::default())).await;
    Json(metric)
}

async fn cpu_history(
    State(pool): State<Arc<SqlitePool>>,
    params: Query<MetricHistory>,
) -> Json<Vec<(String, f32)>> {
    let start_time = params.start_time.unwrap_or(0);
    let end_time = params.end_time.unwrap_or(0);
    let rows = db::get_historical_cpu_metrics(&pool, start_time, end_time)
        .await
        .unwrap_or(vec![]);
    Json(rows)
}

async fn memory_history(
    State(pool): State<Arc<SqlitePool>>,
    params: Query<MetricHistory>,
) -> Json<Vec<(String, MemoryMetrics)>> {
    let start_time = params.start_time.unwrap_or(0);
    let end_time = params.end_time.unwrap_or(0);
    let rows = db::get_historical_memory_metrics(&pool, start_time, end_time)
        .await
        .unwrap_or(vec![]);
    Json(rows)
}
