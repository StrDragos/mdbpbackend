use config::Config;
use serde::Deserialize;
use tracing::{debug, info};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppEnv {
    Local,
    Production,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server_port: u16,
    pub environment: AppEnv,
    pub db_config: DbConfig,
    pub storage_config: StorageConfig,
}

impl AppConfig {
    pub fn build() -> Result<Self, config::ConfigError> {
        info!("Loading configuration from environment variables with prefix APP");
        let env = config::Environment::with_prefix("APP")
            .ignore_empty(true)
            .separator("__");
        debug!("Using environment: {:?}", env);
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "local".to_string());
        debug!("Running on mode: {}", run_mode.clone());

        let filename = match run_mode.as_ref() {
            "local" => "local.yaml",
            _ => "production.yaml",
        };

        Config::builder()
            .add_source(env)
            .add_source(config::File::with_name(&format!("config/{}", filename)))
            .build()
            .map_err(|e| e.into())
            .and_then(|config| config.try_deserialize::<Self>())
            .map_err(|e| e.into())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DbConfig {
    username: String,
    password: String,
    name: String,
    host: String,
    port: u32
}

impl DbConfig {
    pub fn connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }
}

impl DbConfig {
    pub fn get_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.username, self.password, self.host, self.name
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub bucket_url: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub bucket_name: String,
    pub region: String,
}
