use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::metrics::{Field, Metric};

// **Memory Metrics**
//   - Total RAM
//   - Available RAM
//   - Used RAM
//   - Swap usage

#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
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

pub enum Fields {
    Total,
    Free,
    Used,
    UsagePercentage,
    SwapTotal,
    SwapFree,
    SwapUsed,
    SwapUsagePercentage,
}

impl Field for Fields {
    fn from_str(s: &str) -> Option<Fields> {
        match s {
            "total" => Some(Fields::Total),
            "free" => Some(Fields::Free),
            "used" => Some(Fields::Used),
            "usage_percentage" => Some(Fields::UsagePercentage),
            "swap_total" => Some(Fields::SwapTotal),
            "swap_free" => Some(Fields::SwapFree),
            "swap_used" => Some(Fields::SwapUsed),
            "swap_usage_percentage" => Some(Fields::SwapUsagePercentage),
            _ => None,
        }
    }

    fn get_values() -> Vec<String> {
        vec![
            "total".to_string(),
            "free".to_string(),
            "used".to_string(),
            "usage_percentage".to_string(),
            "swap_total".to_string(),
            "swap_free".to_string(),
            "swap_used".to_string(),
            "swap_usage_percentage".to_string(),
        ]
    }

    fn to_str(&self) -> &str {
        match self {
            Fields::Total => "total",
            Fields::Free => "free",
            Fields::Used => "used",
            Fields::UsagePercentage => "usage_percentage",
            Fields::SwapTotal => "swap_total",
            Fields::SwapFree => "swap_free",
            Fields::SwapUsed => "swap_used",
            Fields::SwapUsagePercentage => "swap_usage_percentage",
        }
    }
}

impl MemoryMetrics {
    pub fn new(total: u32, used: u32, swap_total: u32, swap_used: u32) -> MemoryMetrics {
        let free = total - used;
        let usage_percentage = if total > 0 {
            used as f32 / total as f32 * 100.0
        } else {
            0.0
        };
        let swap_free = swap_total - swap_used;
        let swap_usage_percentage = if swap_total > 0 {
            swap_used as f32 / swap_total as f32 * 100.0
        } else {
            0.0
        };
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

impl Metric for MemoryMetrics {
    fn check<T: Field, U: PartialOrd + Into<f32>>(
        &self,
        threshold: U,
        field: T,
        logic: super::alert::Logic,
    ) -> bool {
        match field.to_str() {
            "total" => logic.check(self.total, threshold.into() as u32),
            "free" => logic.check(self.free, threshold.into() as u32),
            "used" => logic.check(self.used, threshold.into() as u32),
            "usage_percentage" => logic.check(self.usage_percentage, threshold.into()),
            "swap_total" => logic.check(self.swap_total, threshold.into() as u32),
            "swap_free" => logic.check(self.swap_free, threshold.into() as u32),
            "swap_used" => logic.check(self.swap_used, threshold.into() as u32),
            "swap_usage_percentage" => logic.check(self.swap_usage_percentage, threshold.into()),
            _ => false,
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
