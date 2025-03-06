use serde::{Deserialize, Serialize};
use std::fmt::Display;

// **Memory Metrics**
//   - Total RAM
//   - Available RAM
//   - Used RAM
//   - Swap usage

#[derive(Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub total: u64,
    pub free: u64,
    pub used: u64,
    pub usage_percentage: f32,
    pub swap_total: u64,
    pub swap_free: u64,
    pub swap_used: u64,
    pub swap_usage_percentage: f32,
}

impl MemoryMetrics {
    pub fn new(total: u64, used: u64, swap_total: u64, swap_used: u64) -> MemoryMetrics {
        let free = total - used;
        let usage_percentage = used as f32 / total as f32;
        let swap_free = swap_total - swap_used;
        let swap_usage_percentage = swap_used as f32 / swap_total as f32;
        MemoryMetrics {
            total,
            free,
            used,
            usage_percentage,
            swap_total,
            swap_free,
            swap_used,
            swap_usage_percentage,
        }
    }
}

impl Display for MemoryMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Memory: {} MB ({}%) used, {} MB free | Swap: {} MB ({}%) used, {} MB free",
            self.used / 1024 / 1024,
            self.usage_percentage,
            self.free / 1024 / 1024,
            self.swap_used / 1024 / 1024,
            self.swap_usage_percentage,
            self.swap_free / 1024 / 1024
        )
    }
}
