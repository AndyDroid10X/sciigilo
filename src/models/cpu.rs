use std::fmt::Display;


// **CPU Metrics**
//   - Usage percentage per core
//   - Load average (1min, 5min, 15min)
pub struct CpuMetrics {
    pub usage_percentage: f32,
    pub load_average: [f32; 3]
}


impl CpuMetrics {
    pub fn new(usage_percentage: f32, load_average: [f32; 3]) -> CpuMetrics {
        CpuMetrics {
            usage_percentage,
            load_average
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