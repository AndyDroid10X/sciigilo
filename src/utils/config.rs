use std::env;

pub fn load_env() {
    match dotenvy::dotenv() {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to load .env file: {}", e),
    }
}

pub struct Config {
    pub db_file_path: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Config {
        Config {
            db_file_path: String::new(),
            port: 3000,
        }
    }
}

pub fn read_config(config: &mut Config) {
    load_env();

    let db_path = env::var("DATABASE_URL").unwrap_or_else(|_| "metrics.db".to_string());
    let port = env::var("PORT")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(3000);

    config.db_file_path = db_path;
    config.port = port;
}
