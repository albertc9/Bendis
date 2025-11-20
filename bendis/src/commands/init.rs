use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::utils::config;

pub fn run() -> Result<()> {
    // Check for legacy structure and offer migration
    if config::check_and_migrate_if_needed()? {
        // Migration was performed or user declined
        // If migration was successful, we're done
        // If user declined, they can still initialize manually later
        return Ok(());
    }

    println!("{}", "Initializing Bendis project...".bold().green());

    let bendis_dir = Path::new("bendis_workspace");
    let bender_yml = bendis_dir.join("Bender.yml");
    let dot_bender_yml = bendis_dir.join(".bender.yml");

    // Check if bendis_workspace/Bender.yml or bendis_workspace/.bender.yml exist and have content
    if bender_yml.exists() {
        let content = fs::read_to_string(&bender_yml)
            .context("Failed to read bendis_workspace/Bender.yml")?;
        if !content.trim().is_empty() {
            bail!(
                "{}\n{}",
                "Initialization failed!".red().bold(),
                format!(
                    "{} already exists with content. Please backup or delete it first.",
                    "bendis_workspace/Bender.yml".yellow()
                )
            );
        }
    }

    if dot_bender_yml.exists() {
        let content = fs::read_to_string(&dot_bender_yml)
            .context("Failed to read bendis_workspace/.bender.yml")?;
        if !content.trim().is_empty() {
            bail!(
                "{}\n{}",
                "Initialization failed!".red().bold(),
                format!(
                    "{} already exists with content. Please backup or delete it first.",
                    "bendis_workspace/.bender.yml".yellow()
                )
            );
        }
    }

    // Create bendis_workspace directory if it doesn't exist
    if !bendis_dir.exists() {
        fs::create_dir(bendis_dir)
            .context("Failed to create bendis_workspace directory")?;
        println!("  {} Created bendis_workspace/ directory", "✓".green());
    } else {
        println!("  {} bendis_workspace/ directory already exists", "ℹ".blue());
    }

    // Create blank Bender.yml if it doesn't exist or is empty
    if !bender_yml.exists() || fs::read_to_string(&bender_yml)?.trim().is_empty() {
        fs::write(&bender_yml, "")
            .context("Failed to create bendis_workspace/Bender.yml")?;
        println!("  {} Created blank bendis_workspace/Bender.yml", "✓".green());
    }

    // Create blank .bender.yml if it doesn't exist or is empty
    if !dot_bender_yml.exists() || fs::read_to_string(&dot_bender_yml)?.trim().is_empty() {
        fs::write(&dot_bender_yml, "")
            .context("Failed to create bendis_workspace/.bender.yml")?;
        println!("  {} Created blank bendis_workspace/.bender.yml", "✓".green());
    }

    // Create .gitignore in bendis_workspace directory
    config::create_bendis_gitignore()?;
    println!("  {} Created bendis_workspace/.gitignore", "✓".green());

    println!("\n{}", "Bendis initialized successfully!".bold().green());
    println!(
        "\nNext steps:\n  1. Edit {} with your dependencies\n  2. Run {} to update",
        "bendis_workspace/Bender.yml".yellow(),
        "bendis update".cyan().bold()
    );

    Ok(())
}
