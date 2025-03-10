// | Key                              | Type                                      | Description |
// |----------------------------------|-------------------------------------------|-------------|
// | `alerts`                         | Array                                    | List of alert rules. |
// | `alerts[].metric_id`             | Enum                                   | The metric to monitor. |
// | `alerts[].logic`                 | `"eq" \| "gt" \| "lt" \| "gte" \| "lte"`    | Logical comparison operator for the alert condition. |
// | `alerts[].value`                 | Number                                   | Threshold value for triggering an alert. |
// | `alerts[].request`               | Object                                   | HTTP request details for triggered alerts. |

use super::metrics::MetricType;
use super::request::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Alert {
    pub metric_id: MetricType,
    pub logic: Logic,
    pub value: String,
    pub request: Request,
}

impl Alert {
    pub fn new(metric_id: MetricType, logic: Logic, value: String, request: Request) -> Alert {
        Alert {
            metric_id,
            logic,
            value,
            request,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Logic {
    Eq,
    Gt,
    Lt,
    Gte,
    Lte,
}
