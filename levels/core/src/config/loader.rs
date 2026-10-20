use std::io::ErrorKind;
use std::path::Path;

use log::{info, warn};

use crate::config::LevelsConfig;
use crate::errors::ConfigError;

pub struct ConfigLoader;

impl ConfigLoader {
    // always returns a usable config: a missing, unreadable or broken file gives the defaults
    pub fn load(path: &Path) -> LevelsConfig {
        match Self::read(path) {
            Ok(config) => config.sanitized(),
            Err(ConfigError::Read(error)) if error.kind() == ErrorKind::NotFound => {
                info!("config {} not found; using defaults", path.display());
                LevelsConfig::default()
            }
            Err(error) => {
                warn!("config {}: {error}; using defaults", path.display());
                LevelsConfig::default()
            }
        }
    }

    fn read(path: &Path) -> Result<LevelsConfig, ConfigError> {
        let text = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&text)?)
    }
}
