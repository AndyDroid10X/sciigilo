use serde::{Deserialize, Serialize};
use std::fmt::Display;

// **Disk Metrics**
//   - Overall disk (or partition?) usage
//   - Available space

#[derive(Serialize, Deserialize, Default)]
pub struct DiskMetrics {
    pub total: u32,
    pub used: u32,
    pub free: u32,
    pub usage_percentage: f32,
}

impl DiskMetrics {
    pub fn new(total: u32, free: u32) -> DiskMetrics {
        let used = if total > free { total - free } else { 0 };
        let usage_percentage = used as f32 / total as f32;
        DiskMetrics {
            total,
            used,
            free,
            usage_percentage,
        }
    }
}

impl Display for DiskMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Disk metrics: total: {}, used: {}, free {}",
            self.total, self.used, self.free
        )
    }
}
