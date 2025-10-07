use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Logging {
    pub levels: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct Postgres {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AppConfig {
    pub server: Server,
    pub logging: Logging,
    pub postgres: Postgres,
}

impl AppConfig {
    pub fn parse(config_file_path: String) -> Result<Self, ConfigError> {
        let current_dir = std::env::current_dir().expect("Could not determine CWD");
        let config_file_path = format!("{config_file_path}");
        
        println!("[AppConfig] CWD: '{current_dir:?}'");
        println!("[AppConfig] Loading config relative to CWD from: '{config_file_path}'");

        let app_config = Config::builder()
            .add_source(File::with_name(config_file_path.as_str()).required(true))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        app_config.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::AppConfig;
    use std::path::PathBuf;

    #[test]
    fn parse_config() {
        let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let mut path_no_ext = base_path.clone();
        path_no_ext.push("../../config/default");
        AppConfig::parse(path_no_ext.to_str().unwrap().to_string()).expect("should parse config without file extension");

        let mut path_with_ext = base_path.clone();
        path_with_ext.push("../../config/default.yml");
        AppConfig::parse(path_with_ext.to_str().unwrap().to_string()).expect("should parse config with file extension");
    }
}