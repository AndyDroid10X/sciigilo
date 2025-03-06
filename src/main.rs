use axum::{Router, response::Html, routing::get, serve};
use sysinfo::System;
use tokio::net::TcpListener;
mod models;
mod utils;
use utils::config;

#[tokio::main]
async fn main() {
    let mut app_config = config::Config::new();
    config::read_config(&mut app_config);

    let route_healthcheck = Router::new().route(
        "/check",
        get(|| async {
            Html(format!(
                "{} is OK",
                System::name().unwrap_or_else(|| "System".into())
            ))
        }),
    );

    let listener = TcpListener::bind(("127.0.0.1", *app_config.get_port()))
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    serve(listener, route_healthcheck).await.unwrap();
}
