use askama::Template;
use axum::{extract::Request, response::Html, routing::get, Router};

use crate::utils::config;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    url: String,
}

async fn index(request: Request) -> Html<String> {
    let mut app_config = config::EnvConfig::new();
    
    app_config.read_config();

    let mut host = String::from("localhost");

    if request.headers().contains_key("host") {
        host = String::from(request.headers().get("host").unwrap().to_str().unwrap());
    }

    let template = IndexTemplate {
        url: host,
    };
    Html(template.render().unwrap())
}

pub fn get_routes() -> Router {
    Router::new().route("/", get(index))
}