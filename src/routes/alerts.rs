use crate::models::metrics;
use crate::{models::alert::Alert, utils::config::AlertConfig};
use axum::routing::post;
use axum::{Router, extract::State, response::Json, routing::get};

async fn get_alerts(State(mut config): State<AlertConfig>) -> Json<Vec<Alert>> {
    Json(config.get_alerts().await.to_vec())
}

async fn create_alert(
    State(mut config): State<AlertConfig>,
    Json(alert_data): Json<Alert>,
) -> Json<Result<String, String>> {
    config.add_alert(alert_data).await;
    Json(Ok("Success".to_string()))
}

async fn delete_alert(
    State(mut config): State<AlertConfig>,
    axum::extract::Path(uuid): axum::extract::Path<String>,
) -> Json<Result<String, String>> {
    config.remove_alert(&uuid).await;
    Json(Ok("Success".to_string()))
}

async fn update_alert(
    State(mut config): State<AlertConfig>,
    Json(alert_data): Json<Alert>,
) -> Json<Result<String, String>> {
    config.update_alert(alert_data).await;
    Json(Ok("Success".to_string()))
}

pub async fn get_fields(State(_config): State<AlertConfig>) -> Json<Vec<String>> {
    Json(metrics::get_metrics_fields())
}

pub fn get_routes() -> Router<AlertConfig> {
    Router::new()
        .route("/get", get(get_alerts))
        .route("/create", post(create_alert))
        .route("/delete/{uuid}", get(delete_alert))
        .route("/update", post(update_alert))
        .route("/fields", get(get_fields))
}
