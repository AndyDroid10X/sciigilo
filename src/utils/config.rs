use crate::models::alert::Alert;
use std::env;
use tokio::{
    fs::{self, File},
    io::AsyncReadExt,
};

pub fn load_env() {
    match dotenvy::dotenv() {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to load .env file: {}", e),
    }
}

pub struct EnvConfig {
    pub db_file_path: String,
    pub port: u16,
}

impl EnvConfig {
    pub fn new() -> EnvConfig {
        EnvConfig {
            db_file_path: String::new(),
            port: 3000,
        }
    }

    pub fn read_config(&mut self) {
        load_env();

        let db_path = env::var("DATABASE_URL").unwrap_or_else(|_| "metrics.db".to_string());
        let port = env::var("PORT")
            .ok()
            .and_then(|val| val.parse().ok())
            .unwrap_or(3000);

        self.db_file_path = db_path;
        self.port = port;
    }
}

#[derive(Clone)]
pub struct AlertConfig {
    alerts: Vec<Alert>,
}

impl AlertConfig {
    const ALERT_CONFIG_FILE: &str = "alerts.json";

    pub fn new() -> AlertConfig {
        AlertConfig { alerts: vec![] }
    }

    pub async fn read_config(&mut self) {
        let mut file = match File::open(Self::ALERT_CONFIG_FILE).await {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open config file: {}", e);
                return;
            }
        };

        let mut content = String::new();
        if let Err(e) = file.read_to_string(&mut content).await {
            eprintln!("Failed to read config file: {}", e);
            return;
        }

        self.alerts = serde_json::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Failed to parse alerts config: {}", e);
            vec![]
        });
    }

    pub async fn save(&self) -> tokio::io::Result<()> {
        let content = serde_json::to_string_pretty(&self.alerts).expect("Failed to save Json file");
        fs::write(Self::ALERT_CONFIG_FILE, content).await
    }

    pub async fn get_alerts(&mut self) -> &Vec<Alert> {
        self.read_config().await;
        &self.alerts
    }

    pub async fn add_alert(&mut self, alert: Alert) {
        self.read_config().await;
        self.alerts.push(alert);
        self.save().await.unwrap_or_else(|e| {
            eprintln!("Failed to save alerts: {}", e);
        });
    }

    pub async fn remove_alert(&mut self, uuid: &str) {
        self.read_config().await;
        if self
            .alerts
            .iter()
            .map(|a| a.id.to_string())
            .any(|id| id == uuid)
        {
            eprintln!("Alert with id {} not found", uuid);
            return;
        }
        self.alerts.retain(|a| a.id.to_string() != uuid);

        self.save().await.unwrap_or_else(|e| {
            eprintln!("Failed to save alerts: {}", e);
        });
    }

    pub async fn update_alert(&mut self, alert: Alert) {
        self.read_config().await;
        self.alerts.retain(|a| a.id != alert.id);
        self.alerts.push(alert);
        self.save().await.unwrap_or_else(|e| {
            eprintln!("Failed to save alerts: {}", e);
        });
    }
}
