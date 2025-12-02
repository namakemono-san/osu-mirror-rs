use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default_server")]
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub storage: StorageConfig,
    pub osu: OsuConfig,
    #[serde(default)]
    pub crawler: CrawlerConfig,
    #[serde(default)]
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_host")]
    pub host: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageConfig {
    #[serde(default = "default_backend")]
    pub backend: StorageBackend,
    #[serde(default)]
    pub local: Option<LocalStorageConfig>,
    #[serde(default)]
    pub s3: Option<S3StorageConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageBackend {
    Local,
    S3,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocalStorageConfig {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct S3StorageConfig {
    pub endpoint: String,
    pub bucket: String,
    pub region: String,
    #[serde(default = "default_prefix")]
    pub prefix: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OsuConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CrawlerConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_sync_interval")]
    pub sync_interval_seconds: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RateLimitConfig {
    #[serde(default = "default_requests_per_minute")]
    pub requests_per_minute: u32,
    #[serde(default = "default_downloads_per_10min")]
    pub downloads_per_10min: u32,
}

fn default_server() -> ServerConfig {
    ServerConfig {
        port: 8080,
        host: "0.0.0.0".to_string(),
    }
}
fn default_port() -> u16 {
    8080
}
fn default_host() -> String {
    "0.0.0.0".to_string()
}
fn default_max_connections() -> u32 {
    20
}
fn default_backend() -> StorageBackend {
    StorageBackend::Local
}
fn default_prefix() -> String {
    "beatmaps".to_string()
}
fn default_true() -> bool {
    true
}
fn default_sync_interval() -> u64 {
    300
}
fn default_requests_per_minute() -> u32 {
    200
}
fn default_downloads_per_10min() -> u32 {
    80
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        let config_path =
            std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());

        if std::path::Path::new(&config_path).exists() {
            let contents = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&contents)?;
            tracing::info!("Loaded config from {}", config_path);
            Ok(config)
        } else {
            let default = Self::default();
            let toml_str = toml::to_string_pretty(&default)?;
            std::fs::write("config.example.toml", &toml_str)?;
            tracing::warn!("{} not found", config_path);
            tracing::warn!("Created config.example.toml");
            anyhow::bail!("Config file not found. Please create config.toml");
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: default_server(),
            database: DatabaseConfig {
                url: "postgres://osu:osu@localhost/osu_mirror".to_string(),
                max_connections: 20,
            },
            storage: StorageConfig {
                backend: StorageBackend::Local,
                local: Some(LocalStorageConfig {
                    path: PathBuf::from("./data/beatmaps"),
                }),
                s3: None,
            },
            osu: OsuConfig {
                client_id: String::new(),
                client_secret: String::new(),
            },
            crawler: CrawlerConfig::default(),
            rate_limit: RateLimitConfig::default(),
        }
    }
}
