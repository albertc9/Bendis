use anyhow::{Context, Result};
use std::process::Command;

use crate::utils::config::{BendisConfig, get_config_path};

pub fn run() -> Result<()> {
    let config_path = get_config_path()?;

    // Ensure config file exists
    if !config_path.exists() {
        println!("Global configuration file not found, creating default...");
        let default_config = BendisConfig::default();
        default_config.save()?;
        println!("Created global config at: {}", config_path.display());
    } else {
        println!("Opening global config at: {}", config_path.display());
    }

    // Detect available editor
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "nano".to_string());

    println!("Using editor: {}", editor);

    // Open editor
    let status = Command::new(&editor)
        .arg(&config_path)
        .status()
        .with_context(|| format!("Failed to open editor '{}'. Make sure it is installed.", editor))?;

    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status");
    }

    // Validate the config after editing
    match BendisConfig::load() {
        Ok(_) => {
            println!("Configuration validated successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("Warning: Configuration file has syntax errors: {}", e);
            eprintln!("Please fix the errors in {}", config_path.display());
            Ok(())
        }
    }
}
