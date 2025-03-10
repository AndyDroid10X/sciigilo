use serde::{Deserialize, Serialize};
use std::fmt::Display;

// **Memory Metrics**
//   - Total RAM
//   - Available RAM
//   - Used RAM
//   - Swap usage

#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct MemoryMetrics {
    pub total: u32,
    pub free: u32,
    pub used: u32,
    pub usage_percentage: f32,
    pub swap_total: u32,
    pub swap_free: u32,
    pub swap_used: u32,
    pub swap_usage_percentage: f32,
}

impl MemoryMetrics {
    pub fn new(total: u32, used: u32, swap_total: u32, swap_used: u32) -> MemoryMetrics {
        let free = total - used;
        let usage_percentage = if total > 0 {used as f32 / total as f32} else {0.0};
        let swap_free = swap_total - swap_used;
        let swap_usage_percentage = if swap_total > 0 {swap_used as f32 / swap_total as f32} else {0.0};
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
