use crate::models::cpu::CpuMetrics;
use crate::models::disk::DiskMetrics;
use crate::models::mem::MemoryMetrics;
use crate::models::metrics::MetricType;
use sqlx::{Row, SqlitePool};
use std::path::Path;
use tokio::fs::OpenOptions;

pub async fn connect(path: &str) -> Result<SqlitePool, sqlx::Error> {
    create_db_file_if_not_exists(path).await?;
    let sqlite_url = format!("sqlite://{}", path);
    println!("Connecting to database: {}", sqlite_url);
    SqlitePool::connect(sqlite_url.as_str()).await.map_err(|e| {
        eprintln!("Failed to connect to database: {:?}", e);
        e
    })
}

async fn create_db_file_if_not_exists(path: &str) -> Result<(), sqlx::Error> {
    let system_path = Path::new(path);
    if !system_path.exists() {
        OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(path)
            .await
            .map_err(|e| {
                eprintln!("Failed to create database file: {:?}", e);
                sqlx::Error::Io(e)
            })?;
    }
    Ok(())
}

pub async fn init_db(pool: &SqlitePool) {
    let query = get_init_query();
    if let Err(e) = sqlx::query(query).execute(pool).await {
        eprintln!("Error initializing database: {:?}", e);
    } else {
        println!("Database initialized successfully.");
    }
}

fn get_init_query() -> &'static str {
    r#"
    CREATE TABLE IF NOT EXISTS CpuMetrics (
        timestamp DATETIME PRIMARY KEY DEFAULT CURRENT_TIMESTAMP,
        usage_percentage REAL NOT NULL,
        load_average_1m REAL NOT NULL,
        load_average_5m REAL NOT NULL,
        load_average_15m REAL NOT NULL
    );

    CREATE TABLE IF NOT EXISTS MemoryMetrics (
        timestamp DATETIME PRIMARY KEY DEFAULT CURRENT_TIMESTAMP,
        total_memory INTEGER NOT NULL,
        used_memory INTEGER NOT NULL,
        total_swap INTEGER NOT NULL,
        used_swap INTEGER NOT NULL
    );

    CREATE TABLE IF NOT EXISTS DiskMetrics (
        timestamp DATETIME PRIMARY KEY DEFAULT CURRENT_TIMESTAMP,
        available_space INTEGER NOT NULL,
        total_space INTEGER NOT NULL
    );
    "#
}

pub async fn insert_metrics(pool: &SqlitePool, metric: MetricType) -> Result<(), sqlx::Error> {
    match metric {
        MetricType::Cpu(cpu_metrics) => insert_cpu_metrics(pool, cpu_metrics).await,
        MetricType::Memory(memory_metrics) => insert_memory_metrics(pool, memory_metrics).await,
        MetricType::Disk(disk_metrics) => insert_disk_metrics(pool, disk_metrics).await,
    }
}

async fn insert_cpu_metrics(pool: &SqlitePool, cpu_metrics: CpuMetrics) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO CpuMetrics (usage_percentage, load_average_1m, load_average_5m, load_average_15m)
        VALUES (?, ?, ?, ?)
        "#
    )
    .bind(cpu_metrics.usage_percentage)
    .bind(cpu_metrics.load_average[0])
    .bind(cpu_metrics.load_average[1])
    .bind(cpu_metrics.load_average[2])
    .execute(pool)
    .await
    .map(|_| ())
}

async fn insert_memory_metrics(
    pool: &SqlitePool,
    memory_metrics: MemoryMetrics,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO MemoryMetrics (total_memory, used_memory, total_swap, used_swap)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(memory_metrics.total as i64)
    .bind(memory_metrics.used as i64)
    .bind(memory_metrics.swap_total as i64)
    .bind(memory_metrics.swap_used as i64)
    .execute(pool)
    .await
    .map(|_| ())
}

async fn insert_disk_metrics(
    pool: &SqlitePool,
    disk_metrics: DiskMetrics,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO DiskMetrics (total_space, available_space)
        VALUES (?, ?)
        "#,
    )
    .bind(disk_metrics.total)
    .bind(disk_metrics.free)
    .execute(pool)
    .await
    .map(|_| ())
}

pub async fn get_metric(pool: &SqlitePool, metric_type: MetricType) -> MetricType {
    match metric_type {
        MetricType::Cpu(_) => get_cpu_metric(pool).await.unwrap_or(metric_type),
        MetricType::Memory(_) => get_memory_metric(pool).await.unwrap_or(metric_type),
        MetricType::Disk(_) => get_disk_metric(pool).await.unwrap_or(metric_type),
    }
}

async fn get_cpu_metric(pool: &SqlitePool) -> Result<MetricType, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT usage_percentage, load_average_1m, load_average_5m, load_average_15m
        FROM CpuMetrics
        ORDER BY timestamp DESC
        LIMIT 1
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(MetricType::Cpu(CpuMetrics {
        usage_percentage: row.get("usage_percentage"),
        load_average: [
            row.get("load_average_1m"),
            row.get("load_average_5m"),
            row.get("load_average_15m"),
        ],
    }))
}

async fn get_memory_metric(pool: &SqlitePool) -> Result<MetricType, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT total_memory, used_memory, total_swap, used_swap
        FROM MemoryMetrics
        ORDER BY timestamp DESC
        LIMIT 1
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(MetricType::Memory(MemoryMetrics::new(
        row.get::<i64, _>("total_memory") as u32,
        row.get::<i64, _>("used_memory") as u32,
        row.get::<i64, _>("total_swap") as u32,
        row.get::<i64, _>("used_swap") as u32,
    )))
}

async fn get_disk_metric(pool: &SqlitePool) -> Result<MetricType, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT available_space, total_space
        FROM DiskMetrics
        ORDER BY timestamp DESC
        LIMIT 1
        "#,
    )
    .fetch_one(pool)
    .await?;
    Ok(MetricType::Disk(DiskMetrics::new(
        row.get::<i64, _>("total_space") as u32,
        row.get::<i64, _>("available_space") as u32,
    )))
}

pub async fn get_cpu_average_since(pool: &SqlitePool, timestamp: i64) -> Result<f32, sqlx::Error> {
    let row = sqlx::query(
        "SELECT AVG(usage_percentage) as avg_usage FROM CpuMetrics WHERE strftime('%s', timestamp) >= ?"
    )
    .bind(timestamp)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(r.get::<Option<f32>, _>("avg_usage").unwrap_or(0.0)),
        None => Ok(0.0),
    }
}
