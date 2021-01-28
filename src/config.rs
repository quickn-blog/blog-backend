use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Deserialize, Debug, Default)]
pub struct Config {
    pub server: ServerConfig,
    pub blog: BlogConfig,
    pub secret: SecretConfig,
}

#[derive(Clone, Deserialize, Debug, Default)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

#[derive(Clone, Deserialize, Debug, Default)]
pub struct BlogConfig {
    pub name: String,
    pub url: String,
}

#[derive(Clone, Deserialize, Debug, Default)]
pub struct SecretConfig {
    pub secret: String,
}

pub fn load_config(path: &str) -> std::io::Result<Config> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(toml::from_str(&buf).unwrap())
}
