use crate::models::{alert::Alert, metrics};
use std::collections::HashMap;

use super::{
    config::{self, EnvConfig},
    db,
    log::Logger,
};

pub struct Watchtower {
    pool: sqlx::SqlitePool,
    env: EnvConfig,
    logger: Logger,
}

impl Watchtower {
    pub fn new(pool: sqlx::SqlitePool, env: EnvConfig, logger: Logger) -> Self {
        Watchtower { pool, env, logger }
    }

    async fn get_alerts(&self) -> Vec<Alert> {
        let mut alerts = config::AlertConfig::new(&self.env);
        alerts.read_config().await;
        alerts.get_alerts().await.clone()
    }

    pub async fn watch(&mut self) {
        loop {
            let alerts = self.get_alerts().await;
            for alert in alerts {
                match metrics::get_metric_type_from_str(&alert.metric_id) {
                    Some(metric_type) => {
                        let metric = db::get_metric(&self.pool, metric_type).await;
                        if let Ok(value) = alert.value.parse::<f32>() {
                            if metric.check(value, alert.metric_id.clone(), alert.logic.clone()) {
                                exec_alert(&alert, metric).await;
                                if let Err(e) = self.logger.log(alert) {
                                    eprintln!("Failed to log alert: {:?}", e);
                                }
                            }
                        } else {
                            eprintln!("Failed to parse value: {} as f32", alert.value);
                        }
                    }
                    None => eprintln!("Invalid metric id: {}", alert.metric_id),
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }
}

async fn exec_alert(alert: &Alert, metric: metrics::MetricType) {
    match alert.request.request_type {
        crate::models::request::RequestType::Get => {
            let _ = reqwest::get(
                alert.request.url.as_str().replace(
                    "{metric}",
                    metric
                        .get_value(alert.metric_id.clone())
                        .to_string()
                        .as_str(),
                ),
            )
            .await;
        }
        crate::models::request::RequestType::Post => match alert.request.body.format {
            crate::models::request::BodyFormat::Json => {
                let _ = reqwest::Client::new()
                    .post(
                        alert.request.url.as_str().replace(
                            "{metric}",
                            metric
                                .get_value(alert.metric_id.clone())
                                .to_string()
                                .as_str(),
                        ),
                    )
                    .json(
                        &alert.request.body.payload.replace(
                            "{metric}",
                            metric
                                .get_value(alert.metric_id.clone())
                                .to_string()
                                .as_str(),
                        ),
                    )
                    .send()
                    .await
                    .map_err(|e| {
                        eprintln!("Failed to send request: {:?}", e);
                    });
            }
            crate::models::request::BodyFormat::XWwwFormUrlEncoded => {
                let form = alert
                    .request
                    .body
                    .payload
                    .replace(
                        "{metric}",
                        metric
                            .get_value(alert.metric_id.clone())
                            .to_string()
                            .as_str(),
                    )
                    .split('&')
                    .map(|kv| {
                        let mut split = kv.split('=');
                        (
                            split.next().unwrap_or("").to_string(),
                            split.next().unwrap_or("").to_string(),
                        )
                    })
                    .collect::<HashMap<String, String>>();
                let _ = reqwest::Client::new()
                    .post(
                        alert.request.url.as_str().replace(
                            "{metric}",
                            metric
                                .get_value(alert.metric_id.clone())
                                .to_string()
                                .as_str(),
                        ),
                    )
                    .form(&form)
                    .send()
                    .await
                    .map_err(|e| {
                        eprintln!("Failed to send request: {:?}", e);
                    });
            }
        },
    }
}
