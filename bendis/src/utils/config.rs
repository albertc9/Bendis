use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.toml";

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

    /// GitIgnore check: auto-manage .bendis/.gitignore file
    /// 1 = on (default), 0 = off
    #[serde(default = "default_gitignore_check")]
    pub gitignore_check: u8,

    /// First run flag: whether this is the first time running bendis
    /// 0 = already shown welcome (default), 1 = need to show welcome
    #[serde(default = "default_first_run")]
    pub first_run: u8,

    /// Last run version: track the version to detect updates
    #[serde(default = "default_version")]
    pub version: String,
}

fn default_silent_mode() -> u8 {
    1
}

fn default_storage_saving_mode() -> u8 {
    0
}

fn default_gitignore_check() -> u8 {
    1
}

fn default_first_run() -> u8 {
    1
}

fn default_version() -> String {
    String::new()
}

impl Default for BendisConfig {
    fn default() -> Self {
        Self {
            silent_mode: default_silent_mode(),
            storage_saving_mode: default_storage_saving_mode(),
            gitignore_check: default_gitignore_check(),
            first_run: default_first_run(),
            version: default_version(),
        }
    }
}

impl BendisConfig {
    /// Load configuration from global config file
    /// If file doesn't exist, create it with default values
    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .context("Failed to read bendis config.toml")?;
            let config: BendisConfig = toml::from_str(&content)
                .context("Failed to parse bendis config.toml")?;
            Ok(config)
        } else {
            // Create default config file
            let default_config = BendisConfig::default();
            default_config.save()?;
            Ok(default_config)
        }
    }

    /// Save configuration to global config file
    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;

        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }

        // Add comments to the generated TOML
        let content_with_comments = format!(
"# Bendis Global Configuration File
#
# This file controls the behavior of bendis (Bender Integration System)
# Location: {}

# Silent mode: suppress bender output during cache preparation
# 1 = on (default), 0 = off
silent_mode = {}

# Storage saving mode: clean up .bendis/.bender/ after update
# 0 = off (default, keep cache), 1 = on (delete entire .bendis/.bender/)
storage_saving_mode = {}

# GitIgnore check: auto-manage .bendis/.gitignore file
# 1 = on (default), 0 = off
gitignore_check = {}

# First run flag: whether this is the first time running bendis
# 0 = already shown welcome, 1 = need to show welcome (default)
first_run = {}

# Last run version: used to detect version updates
version = \"{}\"
",
            config_path.display(),
            self.silent_mode,
            self.storage_saving_mode,
            self.gitignore_check,
            self.first_run,
            self.version
        );

        fs::write(&config_path, content_with_comments)
            .context("Failed to write bendis config.toml")?;
        Ok(())
    }
}

/// Get the global config file path
/// Returns: ~/.config/bendis/config.toml on Linux/macOS
///          %APPDATA%\bendis\config.toml on Windows
pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Failed to get config directory")?;
    Ok(config_dir.join("bendis").join(CONFIG_FILE_NAME))
}

/// Get the .bendis directory path in current project
pub fn get_bendis_dir() -> PathBuf {
    PathBuf::from(".bendis")
}

/// Get the project root directory path
pub fn get_root_dir() -> PathBuf {
    PathBuf::from(".")
}

/// Required entries for .bendis/.gitignore
const REQUIRED_GITIGNORE_ENTRIES: &[&str] = &[
    ".bender/",
];

/// Default .gitignore content with header comment
const DEFAULT_GITIGNORE_CONTENT: &str = "# Auto-managed by bendis
# WARNING: Do not remove the entries below, they are required for proper git tracking
# You can add your own entries after this section

.bender/

";

/// Check if .bendis/.gitignore contains all required entries
/// Returns Ok(()) if all required entries exist, or Err with missing entries
pub fn check_bendis_gitignore() -> Result<()> {
    let gitignore_path = get_bendis_dir().join(".gitignore");

    // If .gitignore doesn't exist, create it with default content
    if !gitignore_path.exists() {
        fs::write(&gitignore_path, DEFAULT_GITIGNORE_CONTENT)
            .context("Failed to create .bendis/.gitignore")?;
        return Ok(());
    }

    // Read existing content
    let existing_content = fs::read_to_string(&gitignore_path)
        .context("Failed to read .bendis/.gitignore")?;

    // Check if all required entries are present
    let missing_entries: Vec<&str> = REQUIRED_GITIGNORE_ENTRIES
        .iter()
        .filter(|&&entry| !existing_content.lines().any(|line| line.trim() == entry))
        .copied()
        .collect();

    if !missing_entries.is_empty() {
        anyhow::bail!(
            "Missing required entries in .bendis/.gitignore: {}",
            missing_entries.join(", ")
        );
    }

    Ok(())
}

/// Create .bendis/.gitignore with default content (used by init command)
pub fn create_bendis_gitignore() -> Result<()> {
    let gitignore_path = get_bendis_dir().join(".gitignore");
    fs::write(&gitignore_path, DEFAULT_GITIGNORE_CONTENT)
        .context("Failed to create .bendis/.gitignore")?;
    Ok(())
}
