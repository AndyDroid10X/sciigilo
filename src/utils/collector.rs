use std::time;

use sqlx::SqlitePool;

use crate::models::{cpu, disk, mem, metrics::MetricType};

use super::db;

pub struct MetricsCollector {
    pool: SqlitePool,
    sysinfo_instance: sysinfo::System,
    disks_instance: sysinfo::Disks,
}

impl MetricsCollector {
    pub fn new(pool: SqlitePool) -> MetricsCollector {
        MetricsCollector {
            pool,
            sysinfo_instance: sysinfo::System::new(),
            disks_instance: sysinfo::Disks::new_with_refreshed_list(),
        }
    }

    pub async fn collect_metrics(&mut self) {
        let cpu_metrics = self.get_cpu_metrics().await.unwrap_or_default();
        let memory_metrics = self.get_memory_metrics().await.unwrap_or_default();
        let disk_metrics = self.get_disk_metrics().await.unwrap_or_default();

        if let Err(e) = db::insert_metrics(&self.pool, MetricType::Cpu(cpu_metrics)).await {
            eprintln!("Failed to insert CPU metrics: {:?}", e);
        }
        if let Err(e) = db::insert_metrics(&self.pool, MetricType::Memory(memory_metrics)).await {
            eprintln!("Failed to insert memory metrics: {:?}", e);
        };
        if let Err(e) = db::insert_metrics(&self.pool, MetricType::Disk(disk_metrics)).await {
            eprintln!("Failed to insert disk metrics: {:?}", e);
        };
    }

    async fn get_cpu_metrics(&mut self) -> Result<cpu::CpuMetrics, String> {
        let timestamp = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let diff_1m = timestamp - 60;
        let diff_5m = timestamp - 300;
        let diff_15m = timestamp - 900;

        let cpu_metrics_1m = db::get_cpu_since(&self.pool, diff_1m)
            .await
            .map_err(|e| e.to_string())?;
        let cpu_metrics_5m = db::get_cpu_since(&self.pool, diff_5m)
            .await
            .map_err(|e| e.to_string())?;
        let cpu_metrics_15m = db::get_cpu_since(&self.pool, diff_15m)
            .await
            .map_err(|e| e.to_string())?;
        let load_average_1m = if !cpu_metrics_1m.is_empty() {
            cpu_metrics_1m
                .iter()
                .map(|m| m.usage_percentage)
                .sum::<f32>()
                / cpu_metrics_1m.len() as f32
        } else {
            0.0
        };

        let load_average_5m = if !cpu_metrics_5m.is_empty() {
            cpu_metrics_5m
                .iter()
                .map(|m| m.usage_percentage)
                .sum::<f32>()
                / cpu_metrics_5m.len() as f32
        } else {
            0.0
        };

        let load_average_15m = if !cpu_metrics_15m.is_empty() {
            cpu_metrics_15m
                .iter()
                .map(|m| m.usage_percentage)
                .sum::<f32>()
                / cpu_metrics_15m.len() as f32
        } else {
            0.0
        };
        self.sysinfo_instance.refresh_cpu_usage();
        Ok(cpu::CpuMetrics::new(
            self.sysinfo_instance.global_cpu_usage(),
            [load_average_1m, load_average_5m, load_average_15m],
        ))
    }

    async fn get_memory_metrics(&mut self) -> Result<mem::MemoryMetrics, String> {
        self.sysinfo_instance.refresh_memory();
        Ok(mem::MemoryMetrics::new(
            (self.sysinfo_instance.total_memory() / 1048576) as u32,
            (self.sysinfo_instance.used_memory() / 1048576) as u32,
            (self.sysinfo_instance.total_swap() / 1048576) as u32,
            (self.sysinfo_instance.used_swap() / 1048576) as u32,
        ))
    }

    async fn get_disk_metrics(&mut self) -> Result<disk::DiskMetrics, String> {
        let disk = match self.disks_instance.iter_mut().next() {
            Some(disk) => disk,
            None => return Err("No disks found".to_string()),
        };

        disk.refresh();
        Ok(disk::DiskMetrics::new(
            (disk.total_space() / 1048576) as u32,
            (disk.available_space() / 1048576) as u32,
        ))
    }
}
