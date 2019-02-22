use std::env;
use config::{ConfigError, Config, File, Environment};
use crate::domain::config::Configuration;

pub fn load_config() -> Result<Configuration, ConfigError> {
    let mut s = Config::new();

    s.merge(File::with_name("config/default"))?;
    s.merge(File::with_name("config/secured"))?;
    let env = env::var("ENVIRONMENT").unwrap_or("development".into());
    s.merge(File::with_name(&format!("config/{}", env)).required(false))?;
    s.merge(File::with_name("config/local").required(false))?;
    s.merge(Environment::with_prefix("app"))?;

    s.try_into()
}