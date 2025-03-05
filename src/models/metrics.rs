use super::{mem, cpu, disk};

pub enum MetricType {
    Cpu,
    Memory,
    Disk,
}