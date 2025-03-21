use serde::{Deserialize, Serialize};
use std::fmt::Display;

// **CPU Metrics**
//   - Usage percentage per core
//   - Load average (1min, 5min, 15min)

#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub struct CpuMetrics {
    pub usage_percentage: f32,
    pub load_average: [f32; 3],
}

pub fn get_values() -> Vec<String> {
    vec![
        "usage_percentage".to_string(),
        "load_average_1m".to_string(),
        "load_average_5m".to_string(),
        "load_average_15m".to_string(),
    ]
}

impl CpuMetrics {
    pub fn new(usage_percentage: f32, load_average: [f32; 3]) -> CpuMetrics {
        CpuMetrics {
            usage_percentage,
            load_average,
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
