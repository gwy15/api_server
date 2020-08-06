use config::{Config as CConfig, ConfigError, File};
use std::env;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub jwt_secret: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = CConfig::new();
        // default
        s.merge(File::with_name("config/default.toml"))?;
        s.merge(File::with_name("config/local.toml").required(false))?;
        // Am I in CI?
        let workflow_name = env::var("GITHUB_WORKFLOW").unwrap_or("".into());
        if workflow_name != "" {
            log::info!("CI environment detected: {}", workflow_name);
            s.merge(File::with_name("config/ci.default.toml"))?;
            s.merge(File::with_name("config/ci.local.toml").required(false))?;
        }

        s.try_into().and_then(|c: Self| {
            log::info!("Config loaded.");
            Ok(c)
        })
    }
}
