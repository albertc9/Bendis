use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = ".bendis.toml";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BendisConfig {
    /// Silent mode: suppress bender output during cache preparation
    /// 1 = on (default), 0 = off
    #[serde(default = "default_silent_mode")]
    pub silent_mode: u8,

    /// Storage saving mode: clean up .bendis/.bender/ after update
    /// 0 = off (default, keep cache), 1 = on (delete entire .bendis/.bender/)
    #[serde(default = "default_storage_saving_mode")]
    pub storage_saving_mode: u8,
}

fn default_silent_mode() -> u8 {
    1
}

fn default_storage_saving_mode() -> u8 {
    0
}

impl Default for BendisConfig {
    fn default() -> Self {
        Self {
            silent_mode: default_silent_mode(),
            storage_saving_mode: default_storage_saving_mode(),
        }
    }
}

impl BendisConfig {
    /// Load configuration from .bendis.toml file
    /// If file doesn't exist, create it with default values
    pub fn load() -> Result<Self> {
        let config_path = get_root_dir().join(CONFIG_FILE_NAME);

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .context("Failed to read .bendis.toml")?;
            let config: BendisConfig = toml::from_str(&content)
                .context("Failed to parse .bendis.toml")?;
            Ok(config)
        } else {
            // Create default config file
            let default_config = BendisConfig::default();
            default_config.save()?;
            Ok(default_config)
        }
    }

    /// Save configuration to .bendis.toml file
    pub fn save(&self) -> Result<()> {
        let config_path = get_root_dir().join(CONFIG_FILE_NAME);

        // Add comments to the generated TOML
        let content_with_comments = format!(
"# Bendis Configuration File
#
# This file controls the behavior of bendis (Bender Integration System)

# Silent mode: suppress bender output during cache preparation
# 1 = on (default), 0 = off
silent_mode = {}

# Storage saving mode: clean up .bendis/.bender/ after update
# 0 = off (default, keep cache), 1 = on (delete entire .bendis/.bender/)
storage_saving_mode = {}
",
            self.silent_mode,
            self.storage_saving_mode
        );

        fs::write(&config_path, content_with_comments)
            .context("Failed to write .bendis.toml")?;
        Ok(())
    }
}

/// Get the .bendis directory path
pub fn get_bendis_dir() -> PathBuf {
    PathBuf::from(".bendis")
}

/// Get the project root directory path
pub fn get_root_dir() -> PathBuf {
    PathBuf::from(".")
}
