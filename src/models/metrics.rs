use super::{cpu, disk, mem};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MetricType {
    Cpu(cpu::CpuMetrics),
    Memory(mem::MemoryMetrics),
    Disk(disk::DiskMetrics),
}
