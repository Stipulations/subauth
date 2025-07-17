use axum::{Json, Router, routing::get};
use chrono::Utc;
use serde::Serialize;
use sysinfo::System;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(health_check));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct HealthCheck {
    status: String,
    cpu_usage: f32,
    memory_usage: Memory,
    request_time: i64,
    response_time: i64,
}

#[derive(Serialize)]
struct Memory {
    used_mb: u64,
    total_mb: u64,
    usage_percent: f32,
}

async fn health_check() -> Json<HealthCheck> {
    let request_time = Utc::now().timestamp();

    let mut sys = System::new();

    sys.refresh_cpu_all();
    sys.refresh_memory();

    let cpu_usage: f32 =
        sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();

    let memory_usage = Memory {
        used_mb: used_memory / 1024 / 1024,
        total_mb: total_memory / 1024 / 1024,
        usage_percent: (used_memory as f32 / total_memory as f32) * 100.0,
    };

    let response_time = Utc::now().timestamp();

    let health = HealthCheck {
        status: "healthy".to_string(),
        cpu_usage,
        memory_usage,
        request_time,
        response_time,
    };

    Json(health)
}