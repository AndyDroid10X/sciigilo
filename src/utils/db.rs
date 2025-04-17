use crate::models::cpu::CpuMetrics;
use crate::models::disk::DiskMetrics;
use crate::models::mem::MemoryMetrics;
use crate::models::metrics::MetricType;
use sqlx::{Row, SqlitePool};
use std::{path::Path, sync::Arc};
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
    let db_path = Path::new(path);
    let db_dir = db_path.parent().unwrap_or(Path::new("."));
    match tokio::fs::create_dir_all(db_dir).await {
        Ok(_) => {
            if !db_path.exists() {
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(db_path)
                    .await
                    .map_err(|e| {
                        eprintln!("Failed to create database file: {:?}", e);
                        e
                    })?;
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to create directory for database: {:?}", e);
            Err(sqlx::Error::Io(e))
        }
    }
}

pub async fn init_db(pool: &Arc<SqlitePool>, retention_period: u32) {
    let query = get_init_query();
    if let Err(e) = sqlx::query(query).execute(&**pool).await {
        eprintln!("Error initializing database: {:?}", e);
    } else {
        println!("Database initialized successfully.");
    }

    let pool = pool.clone();

    tokio::spawn(async move {
        loop {
            cleanup_metrics(&pool, retention_period).await;
            tokio::time::sleep(tokio::time::Duration::from_secs(86400)).await;
        }
    });
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

pub async fn get_historical_cpu_metrics(
    pool: &SqlitePool,
    start_time: i64,
    mut end_time: i64,
) -> Result<Vec<(String, f32)>, sqlx::Error> {
    if end_time == 0 {
        let now = chrono::Utc::now().timestamp();
        end_time = now;
    }

    let rows = sqlx::query(
        r#"
        SELECT datetime(timestamp) as formatted_time, usage_percentage
        FROM CpuMetrics
        WHERE timestamp >= datetime(?, 'unixepoch') AND timestamp <= datetime(?, 'unixepoch')
        ORDER BY timestamp ASC
        "#,
    )
    .bind(start_time)
    .bind(end_time)
    .fetch_all(pool)
    .await?;
    println!(
        "Query for historical CPU metrics: Start={}, End={}",
        start_time, end_time
    );

    let mut metrics = Vec::with_capacity(rows.len());
    for row in rows {
        let timestamp: String = row.get("formatted_time");
        let usage_percentage: f32 = row.get("usage_percentage");
        metrics.push((timestamp, usage_percentage));
    }
    Ok(metrics)
}

pub async fn get_historical_memory_metrics(
    pool: &SqlitePool,
    start_time: i64,
    mut end_time: i64,
) -> Result<Vec<(String, MemoryMetrics)>, sqlx::Error> {
    if end_time == 0 {
        let now = chrono::Utc::now().timestamp();
        end_time = now;
    }
    let rows = sqlx::query(
        r#"
        SELECT datetime(timestamp) as formatted_time, total_memory, used_memory, total_swap, used_swap
        FROM MemoryMetrics
        WHERE timestamp >= datetime(?, 'unixepoch') AND timestamp <= datetime(?, 'unixepoch')
        ORDER BY timestamp ASC
        "#,
    )
    .bind(start_time)
    .bind(end_time)
    .fetch_all(pool)
    .await?;

    let mut metrics = Vec::with_capacity(rows.len());
    for row in rows {
        let timestamp: String = row.get("formatted_time");
        let total_memory: u32 = row.get("total_memory");
        let used_memory: u32 = row.get("used_memory");
        let total_swap: u32 = row.get("total_swap");
        let used_swap: u32 = row.get("used_swap");
        metrics.push((
            timestamp,
            MemoryMetrics::new(total_memory, used_memory, total_swap, used_swap),
        ));
    }
    Ok(metrics)
}

pub async fn cleanup_metrics(pool: &SqlitePool, retention_period: u32) {
    let query = format!(
        r#"
    DELETE FROM CpuMetrics WHERE strftime('%s', timestamp) < strftime('%s', 'now', '-{retention_period} day');
    DELETE FROM MemoryMetrics WHERE strftime('%s', timestamp) < strftime('%s', 'now', '-{retention_period} day');
    DELETE FROM DiskMetrics WHERE strftime('%s', timestamp) < strftime('%s', 'now', '-{retention_period} day');
    "#
    );
    if let Err(e) = sqlx::query(query.as_str()).execute(pool).await {
        eprintln!("Error cleaning up old metrics: {:?}", e);
    }
}
