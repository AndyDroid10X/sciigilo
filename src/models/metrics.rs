use serde::{Deserialize, Serialize};
// use super::{mem, cpu, disk};

#[derive(Serialize, Deserialize)]
pub enum MetricType {
    Cpu,
    Memory,
    Disk,
}
