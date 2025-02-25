use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub app_name: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("config.yaml")?;
        let config: AppConfig = serde_yaml::from_str(&config_str)?;
        Ok(config)
    }
}
