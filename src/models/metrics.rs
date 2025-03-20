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
    fields.extend(
        cpu::get_values()
            .iter()
            .map(|field| format!("cpu_{}", field)),
    );
    fields.extend(
        mem::get_values()
            .iter()
            .map(|field| format!("mem_{}", field)),
    );
    fields.extend(
        disk::get_values()
            .iter()
            .map(|field| format!("disk_{}", field)),
    );
    fields
}
