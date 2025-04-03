use axum::{
    Json,
    extract::{Query, State},
};

use crate::utils::log::Logger;

#[derive(serde::Deserialize)]
struct LogsQuery {
    n: Option<usize>,
}

async fn get_logs(State(logger): State<Logger>, query: Query<LogsQuery>) -> Json<Vec<String>> {
    let n = query.n.unwrap_or(0);
    let logs = logger.get(n).unwrap();
    logs.join("\n");
    Json(logs)
}

pub fn get_routes() -> axum::Router<Logger> {
    axum::Router::new().route("/get", axum::routing::get(get_logs))
}
