// | Key                              | Type                                      | Description |
// |----------------------------------|-------------------------------------------|-------------|
// | `alerts`                         | Array                                    | List of alert rules. |
// | `alerts[].metric_id`             | Enum                                   | The metric to monitor. |
// | `alerts[].logic`                 | `"eq" \| "gt" \| "lt" \| "gte" \| "lte"`    | Logical comparison operator for the alert condition. |
// | `alerts[].value`                 | Number                                   | Threshold value for triggering an alert. |
// | `alerts[].request`               | Object                                   | HTTP request details for triggered alerts. |

use super::request::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Alert {
    #[serde(default = "uuid::Uuid::new_v4")]
    pub id: Uuid,
    pub metric_id: String,
    pub logic: Logic,
    pub value: String,
    pub request: Request,
}

impl Alert {
    pub fn new(
        id: Uuid,
        metric_id: String,
        logic: Logic,
        value: String,
        request: Request,
    ) -> Alert {
        Alert {
            id,
            metric_id,
            logic,
            value,
            request,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Logic {
    Eq,
    Gt,
    Lt,
    Gte,
    Lte,
}
