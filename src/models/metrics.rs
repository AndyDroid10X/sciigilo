use super::{cpu, disk, mem};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MetricType {
    Cpu(cpu::CpuMetrics),
    Memory(mem::MemoryMetrics),
    Disk(disk::DiskMetrics),
}

pub trait Field {
    fn from_str(s: &str) -> Option<Self>
    where
        Self: Sized;
    fn get_values() -> Vec<String>;
    fn to_str(&self) -> &str;
}

pub trait Metric {
    fn check<T: Field, U: PartialOrd + Into<f32>>(
        &self,
        threshold: U,
        field: T,
        logic: super::alert::Logic,
    ) -> bool;

    fn get_value(&self, field: String) -> f32;
}

impl MetricType {
    pub fn check<U: PartialOrd + Into<f32>>(
        &self,
        threshold: U,
        field: String,
        logic: super::alert::Logic,
    ) -> bool {
        match self {
            MetricType::Cpu(cpu) => cpu.check(
                threshold,
                cpu::Fields::from_str(&field.replace("cpu_", "")).unwrap(),
                logic,
            ),
            MetricType::Memory(mem) => mem.check(
                threshold,
                mem::Fields::from_str(&field.replace("mem_", "")).unwrap(),
                logic,
            ),
            MetricType::Disk(disk) => disk.check(
                threshold,
                disk::Fields::from_str(&field.replace("disk_", "")).unwrap(),
                logic,
            ),
        }
    }

    pub fn get_value(&self, field: String) -> f32 {
        match self {
            MetricType::Cpu(cpu) => cpu.get_value(field),
            MetricType::Memory(mem) => mem.get_value(field),
            MetricType::Disk(disk) => disk.get_value(field),
        }
    }
}

pub fn get_metrics_fields() -> Vec<String> {
    let mut fields = Vec::new();
    fields.extend(
        cpu::Fields::get_values()
            .iter()
            .map(|field| format!("cpu_{}", field)),
    );
    fields.extend(
        mem::Fields::get_values()
            .iter()
            .map(|field| format!("mem_{}", field)),
    );
    fields.extend(
        disk::Fields::get_values()
            .iter()
            .map(|field| format!("disk_{}", field)),
    );
    fields
}

pub fn get_metric_type_from_str(metric_id: &str) -> Option<MetricType> {
    let fields = get_metrics_fields();
    if fields.contains(&metric_id.to_string()) {
        match metric_id {
            metric_id if metric_id.starts_with("cpu") => {
                let field = metric_id.trim_start_matches("cpu_");
                cpu::Fields::from_str(field).map(|_| MetricType::Cpu(cpu::CpuMetrics::default()))
            }
            metric_id if metric_id.starts_with("mem") => {
                let field = metric_id.trim_start_matches("mem_");
                mem::Fields::from_str(field).map(|_| MetricType::Memory(mem::MemoryMetrics::default()))
            }
            metric_id if metric_id.starts_with("disk") => {
                let field = metric_id.trim_start_matches("disk_");
                disk::Fields::from_str(field).map(|_| MetricType::Disk(disk::DiskMetrics::default()))
            }
            _ => None,
        }
    } else {
        None
    }
}
