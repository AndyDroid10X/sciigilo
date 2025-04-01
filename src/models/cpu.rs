use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::{
    alert::Logic,
    metrics::{Field, Metric},
};

// **CPU Metrics**
//   - Usage percentage per core
//   - Load average (1min, 5min, 15min)

#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub struct CpuMetrics {
    pub usage_percentage: f32,
    pub load_average: [f32; 3],
}

pub enum Fields {
    UsagePercentage,
    LoadAverage1m,
    LoadAverage5m,
    LoadAverage15m,
}

impl Field for Fields {
    fn from_str(s: &str) -> Option<Fields> {
        match s {
            "usage_percentage" => Some(Fields::UsagePercentage),
            "load_average_1m" => Some(Fields::LoadAverage1m),
            "load_average_5m" => Some(Fields::LoadAverage5m),
            "load_average_15m" => Some(Fields::LoadAverage15m),
            _ => None,
        }
    }

    fn get_values() -> Vec<String> {
        vec![
            "usage_percentage".to_string(),
            "load_average_1m".to_string(),
            "load_average_5m".to_string(),
            "load_average_15m".to_string(),
        ]
    }

    fn to_str(&self) -> &str {
        match self {
            Fields::UsagePercentage => "usage_percentage",
            Fields::LoadAverage1m => "load_average_1m",
            Fields::LoadAverage5m => "load_average_5m",
            Fields::LoadAverage15m => "load_average_15m",
        }
    }
}

impl CpuMetrics {
    pub fn new(usage_percentage: f32, load_average: [f32; 3]) -> CpuMetrics {
        CpuMetrics {
            usage_percentage,
            load_average,
        }
    }
}

impl Metric for CpuMetrics {
    fn check<T: Field, U: PartialOrd + Into<f32>>(
        &self,
        threshold: U,
        field: T,
        logic: Logic,
    ) -> bool {
        let threshold: f32 = threshold.into();
        match field.to_str() {
            "usage_percentage" => logic.check(self.usage_percentage, threshold),
            "load_average_1m" => logic.check(self.load_average[0], threshold),
            "load_average_5m" => logic.check(self.load_average[1], threshold),
            "load_average_15m" => logic.check(self.load_average[2], threshold),
            _ => false,
        }
    }

    fn get_value(&self, field: String) -> f32 {
        match field.as_str() {
            "usage_percentage" => self.usage_percentage,
            "load_average_1m" => self.load_average[0],
            "load_average_5m" => self.load_average[1],
            "load_average_15m" => self.load_average[2],
            _ => 0.0,
        }
    }
}

impl Display for CpuMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CPU Usage: {:.2}%\nLoad Average: {:.2}, {:.2}, {:.2}",
            self.usage_percentage, self.load_average[0], self.load_average[1], self.load_average[2]
        )
    }
}
