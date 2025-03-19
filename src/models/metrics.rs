use super::{cpu, disk, mem};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MetricType {
    Cpu(cpu::CpuMetrics),
    Memory(mem::MemoryMetrics),
    Disk(disk::DiskMetrics),
}

pub fn get_metrics_fields() -> Vec<String> {
    let mut fields = Vec::new();
    fields.extend(cpu::Fields::get_values());
    fields.extend(mem::Fields::get_values());
    fields.extend(disk::Fields::get_values());
    fields
}
