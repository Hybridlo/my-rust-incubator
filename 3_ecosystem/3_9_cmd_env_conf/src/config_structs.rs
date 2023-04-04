use std::{time::Duration, net::IpAddr};

use config::{ConfigError, Source};
use serde::{Serialize, Deserialize};
use url::Url;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WatchdogConfig {
    #[serde(with="humantime_serde")]
    period: Duration,
    limit: u32,
    #[serde(with="humantime_serde")]
    lock_timeout: Duration
}

impl Default for WatchdogConfig {
    fn default() -> Self {
        Self {
            period: Duration::from_secs(5),
            limit: 10,
            lock_timeout: Duration::from_secs(4)
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct BackgroundConfig {
    watchdog: WatchdogConfig
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct LogAppConfig {
    level: LogLevel
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct LogConfig {
    app: LogAppConfig
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConnectionsConfig {
    max_idle: u32,
    max_open: u32
}

impl Default for ConnectionsConfig {
    fn default() -> Self {
        Self {
            max_idle: 30,
            max_open: 30
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MySQLConfig {
    host: IpAddr,
    port: u16,
    dating: String,
    user: String,
    pass: String,
    connections: ConnectionsConfig
}

impl Default for MySQLConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".parse().unwrap(),
            port: 3306,
            dating: "default".to_string(),
            user: "root".to_string(),
            pass: "".to_string(),
            connections: Default::default()
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct DbConfig {
    mysql: MySQLConfig
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    external_url: Url,
    http_port: u16,
    grpc_port: u16,
    healthz_port: u16,
    metrics_port: u16
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            external_url: "http://127.0.0.1".parse().unwrap(),
            http_port: 8081,
            grpc_port: 8082,
            healthz_port: 10025,
            metrics_port: 9199
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ModeConfig {
    debug: bool
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct MyConfig {
    mode: ModeConfig,
    server: ServerConfig,
    db: DbConfig,
    log: LogConfig,
    background: BackgroundConfig
}

impl Source for MyConfig {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new(self.clone())
    }

    fn collect(&self) -> Result<config::Map<String, config::Value>, ConfigError> {
        let json_repr = serde_json::to_string(self).expect("JSON serialization to succeed");

        Ok(serde_json::from_str(&json_repr).expect("JSON deserialization to succeed"))
    }
}