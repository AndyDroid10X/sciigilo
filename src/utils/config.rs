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
#[derive(Clone)]
pub struct EnvConfig {
    pub db_file_path: String,
    pub port: u16,
    pub alerts_file_path: String,
}

impl EnvConfig {
    pub fn new() -> EnvConfig {
        EnvConfig {
            db_file_path: String::new(),
            port: 3000,
            alerts_file_path: String::new(),
        }
    }

    pub fn read_config(&mut self) {
        load_env();

        let db_path = env::var("DATABASE_URL").unwrap_or_else(|_| {
            let config_dir = dirs::config_dir().expect("Failed to get config directory");
            format!("{}/sciigilo/metrics.db", config_dir.display())
        });
        let port = env::var("PORT")
            .ok()
            .and_then(|val| val.parse().ok())
            .unwrap_or(3000);
        let alerts_path = env::var("ALERTS_FILE").unwrap_or_else(|_| {
            let config_dir = dirs::config_dir().expect("Failed to get config directory");
            format!("{}/sciigilo/alerts.json", config_dir.display())
        });

        self.db_file_path = db_path;
        self.port = port;
        self.alerts_file_path = alerts_path
    }
}

#[derive(Clone)]
pub struct AlertConfig {
    alerts: Vec<Alert>,
    file_path: String,
}

impl AlertConfig {
    pub fn new(env: &EnvConfig) -> AlertConfig {
        AlertConfig { alerts: vec![], file_path: env.alerts_file_path.clone() }
    }

    pub async fn read_config(&mut self) {
        if !fs::try_exists(&self.file_path).await.unwrap_or(false) {
            if let Err(e) = self.save().await {
                eprintln!("Failed to create initial config file: {}", e);
            }
            return;
        }

        let mut file = match File::open(&self.file_path).await {
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
        if let Some(parent) = std::path::Path::new(&self.file_path).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await?;
            }
        }
        let content = serde_json::to_string_pretty(&self.alerts).expect("Failed to save Json file");
        fs::write(&self.file_path, content).await
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
        if !self
            .alerts
            .iter()
            .map(|a| a.id.to_string())
            .any(|id| id == uuid)
        {
            eprintln!("Alert with id {} not found", uuid);
            println!("{:?}", self.alerts);
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
