use serde::{Deserialize, Serialize};
use std::fmt::Display;

// **Disk Metrics**
//   - Overall disk (or partition?) usage
//   - Available space

#[derive(Serialize, Deserialize)]
pub struct DiskMetrics {
    pub total: u64,
    pub used: u64,
    pub free: u64,
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
