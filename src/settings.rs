use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::{env, net::Ipv4Addr};

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("Failed to load settings");
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ServerSettings {
    pub host: Option<Ipv4Addr>,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct JWTSettings {
    pub secret: String,
    pub expiration_time_in_seconds: usize,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub server: ServerSettings,
    pub jwt: JWTSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            .set_default("jwt.expiration_time_in_seconds", 86400)?
            .build()?;

        s.try_deserialize()
    }
}
