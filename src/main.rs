use axum::{response::Html, routing::get, Router, serve};
use sysinfo::System;
use tokio::net::TcpListener;
mod models;
use models::{alert, request};

#[tokio::main]
async fn main() {
    let route_healthcheck = Router::new().route("/check", get(|| async { 
        Html(format!("{} is OK", System::name().unwrap_or_else(|| "System".into())))
    }));
    let test = models::cpu::CpuMetrics::new(1.0, [1.0, 1.0, 1.0]);
    let test2 = models::os_info::OsInfo::new(System::host_name().unwrap(), System::uptime(), System::name().unwrap(), System::os_version().unwrap(), System::cpu_arch());
   

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    serve(listener, route_healthcheck).await.unwrap();
}
