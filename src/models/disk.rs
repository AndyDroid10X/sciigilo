use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::metrics::{Field, Metric};

// **Disk Metrics**
//   - Overall disk (or partition?) usage
//   - Available space

#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub struct DiskMetrics {
    pub total: u32,
    pub used: u32,
    pub free: u32,
    pub usage_percentage: f32,
}

pub enum Fields {
    Total,
    Used,
    Free,
    UsagePercentage,
}

impl Field for Fields {
    fn from_str(s: &str) -> Option<Fields> {
        match s {
            "total" => Some(Fields::Total),
            "used" => Some(Fields::Used),
            "free" => Some(Fields::Free),
            "usage_percentage" => Some(Fields::UsagePercentage),
            _ => None,
        }
    }

    fn get_values() -> Vec<String> {
        vec![
            "total".to_string(),
            "used".to_string(),
            "free".to_string(),
            "usage_percentage".to_string(),
        ]
    }

    fn to_str(&self) -> &str {
        match self {
            Fields::Total => "total",
            Fields::Used => "used",
            Fields::Free => "free",
            Fields::UsagePercentage => "usage_percentage",
        }
    }
}

impl Metric for DiskMetrics {
    fn check<T: super::metrics::Field, U: PartialOrd + Into<f32>>(
        &self,
        threshold: U,
        field: T,
        logic: super::alert::Logic,
    ) -> bool {
        match field.to_str() {
            "total" => logic.check(self.total, threshold.into() as u32),
            "used" => logic.check(self.used, threshold.into() as u32),
            "free" => logic.check(self.free, threshold.into() as u32),
            "usage_percentage" => logic.check(self.usage_percentage, threshold.into()),
            _ => false,
        }
    }
}

impl DiskMetrics {
    pub fn new(total: u32, free: u32) -> DiskMetrics {
        let used = total.saturating_sub(free);
        let usage_percentage = if total > 0 {
            used as f32 / total as f32 * 100.0
        } else {
            0.0
        };
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
