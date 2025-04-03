use axum::{Router, response::Html, routing::get};

async fn index() -> Html<String> {
    let html = include_str!("../../templates/index.html");
    Html(html.to_string())
}

pub fn get_routes() -> Router {
    Router::new().route("/", get(index))
}
