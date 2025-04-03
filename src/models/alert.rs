// | Key                              | Type                                      | Description |
// |----------------------------------|-------------------------------------------|-------------|
// | `alerts`                         | Array                                    | List of alert rules. |
// | `alerts[].metric_id`             | Enum                                   | The metric to monitor. |
// | `alerts[].logic`                 | `"eq" \| "gt" \| "lt" \| "gte" \| "lte"`    | Logical comparison operator for the alert condition. |
// | `alerts[].value`                 | Number                                   | Threshold value for triggering an alert. |
// | `alerts[].request`               | Object                                   | HTTP request details for triggered alerts. |

use std::fmt::Display;

use super::request::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub struct Alert {
    #[serde(default = "uuid::Uuid::new_v4")]
    pub id: Uuid,
    pub metric_id: String,
    pub logic: Logic,
    pub value: String,
    pub request: Request,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub enum Logic {
    #[default]
    Eq,
    Gt,
    Lt,
    Gte,
    Lte,
}

impl Display for Logic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Logic::Eq => write!(f, "=="),
            Logic::Gt => write!(f, ">"),
            Logic::Lt => write!(f, "<"),
            Logic::Gte => write!(f, ">="),
            Logic::Lte => write!(f, "<="),
        }
    }
}

impl Display for Alert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Alert metric_id: {} {} {}, {}",
            self.metric_id, self.logic, self.value, self.request
        )
    }
}

impl Logic {
    pub fn check<T>(&self, value: T, threshold: T) -> bool
    where
        T: PartialOrd,
    {
        match self {
            Logic::Eq => value == threshold,
            Logic::Gt => value > threshold,
            Logic::Lt => value < threshold,
            Logic::Gte => value >= threshold,
            Logic::Lte => value <= threshold,
        }
    }
}
