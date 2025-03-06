use std::env;

pub fn load_env() {
    match dotenvy::dotenv() {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to load .env file: {}", e),
    }
}

pub struct Config {
    db_file_path: String,
    port: u16,
}

impl Config {
    pub fn new() -> Config {
        Config {
            db_file_path: String::new(),
            port: 3000,
        }
    }

    pub fn get_db_file_path(&self) -> &String {
        &self.db_file_path
    }

    pub fn get_port(&self) -> &u16 {
        &self.port
    }
}

pub fn read_config(config: &mut Config) {
    load_env();

    let db_path = env::var("SC_DB_PATH").unwrap_or_else(|_| "metrics.db".to_string());
    let port = env::var("SC_PORT")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(3000);

    config.db_file_path = db_path;
    config.port = port;
}
