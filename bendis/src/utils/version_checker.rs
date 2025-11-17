use anyhow::{Context, Result};
use colored::Colorize;
use serde::Deserialize;
use std::io::{self, Write};
use std::process::Command;

#[derive(Debug, Deserialize)]
struct CrateInfo {
    #[serde(rename = "crate")]
    crate_data: CrateData,
}

#[derive(Debug, Deserialize)]
struct CrateData {
    max_version: String,
}

/// Check the latest version of a crate from crates.io
fn get_latest_version(crate_name: &str) -> Result<String> {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let response = ureq::get(&url)
        .timeout(std::time::Duration::from_secs(5))
        .call()
        .context("Failed to connect to crates.io")?;

    let body = response
        .into_string()
        .context("Failed to read response body")?;

    let crate_info: CrateInfo = serde_json::from_str(&body)
        .context("Failed to parse crates.io response")?;

    Ok(crate_info.crate_data.max_version)
}

/// Get the currently installed bender version
fn get_bender_version() -> Option<String> {
    let output = Command::new("bender")
        .arg("--version")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let version_str = String::from_utf8_lossy(&output.stdout);
    // Parse "bender 0.29.0" -> "0.29.0"
    version_str
        .split_whitespace()
        .nth(1)
        .map(|v| v.trim().to_string())
}

/// Compare two semantic versions
fn is_newer_version(latest: &str, current: &str) -> bool {
    // Simple string comparison works for semantic versioning in most cases
    // For production, consider using the `semver` crate
    latest != current && latest > current
}

/// Prompt user for update confirmation
fn prompt_user(message: &str) -> bool {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();
    let input = input.trim().to_lowercase();

    input == "y" || input == "yes"
}

/// Install or update a crate using cargo install
fn install_crate(crate_name: &str) -> Result<()> {
    println!("Installing {}...", crate_name.bold());

    let status = Command::new("cargo")
        .arg("install")
        .arg(crate_name)
        .status()
        .context("Failed to run cargo install")?;

    if status.success() {
        println!("{} {} updated successfully!", "✓".green(), crate_name);
        Ok(())
    } else {
        anyhow::bail!("Failed to install {}", crate_name)
    }
}

pub struct VersionUpdate {
    pub crate_name: String,
    pub current: String,
    pub latest: String,
}

/// Check for updates to bendis and bender
pub fn check_for_updates(current_bendis_version: &str) -> Result<Vec<VersionUpdate>> {
    let mut updates = Vec::new();

    // Check bendis version
    match get_latest_version("bendis") {
        Ok(latest) => {
            if is_newer_version(&latest, current_bendis_version) {
                updates.push(VersionUpdate {
                    crate_name: "bendis".to_string(),
                    current: current_bendis_version.to_string(),
                    latest,
                });
            }
        }
        Err(_) => {
            // Silently ignore, will be reported as network error by caller
        }
    }

    // Check bender version
    if let Some(current_bender) = get_bender_version() {
        match get_latest_version("bender") {
            Ok(latest) => {
                if is_newer_version(&latest, &current_bender) {
                    updates.push(VersionUpdate {
                        crate_name: "bender".to_string(),
                        current: current_bender,
                        latest,
                    });
                }
            }
            Err(_) => {
                // Silently ignore
            }
        }
    }

    Ok(updates)
}

/// Display update notification and handle user interaction
pub fn handle_updates(updates: Vec<VersionUpdate>) -> Result<()> {
    if updates.is_empty() {
        return Ok(());
    }

    println!();
    if updates.len() == 1 {
        let update = &updates[0];
        println!(
            "{} New version available: {} v{} (current: v{})",
            "🔔".yellow(),
            update.crate_name.bold(),
            update.latest.green(),
            update.current.dimmed()
        );
    } else {
        println!("{} New versions available:", "🔔".yellow());
        for update in &updates {
            println!(
                "  - {} v{} (current: v{})",
                update.crate_name.bold(),
                update.latest.green(),
                update.current.dimmed()
            );
        }
    }

    if prompt_user("Do you want to update? (y/N): ") {
        println!();
        for update in &updates {
            install_crate(&update.crate_name)?;
        }
        println!();
        println!("{} Updates completed!", "✓".green().bold());
    }

    Ok(())
}
