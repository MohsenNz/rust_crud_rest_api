use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    server_addr: String,
    database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        ::config::Config::builder()
            .add_source(::config::Environment::default())
            .build()?
            .try_deserialize()
    }

    pub fn from_file_name(name: &str) -> Result<Self, ConfigError> {
        ::config::Config::builder()
            .add_source(::config::File::with_name(name))
            .build()?
            .try_deserialize()
    }

    pub fn server_addr(&self) -> &str {
        &self.server_addr
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}
