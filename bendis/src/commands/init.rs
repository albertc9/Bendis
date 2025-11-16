use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn run() -> Result<()> {
    println!("{}", "Initializing Bendis project...".bold().green());

    let bendis_dir = Path::new(".bendis");
    let bender_yml = bendis_dir.join("Bender.yml");
    let dot_bender_yml = bendis_dir.join(".bender.yml");

    // Check if .bendis/Bender.yml or .bendis/.bender.yml exist and have content
    if bender_yml.exists() {
        let content = fs::read_to_string(&bender_yml)
            .context("Failed to read .bendis/Bender.yml")?;
        if !content.trim().is_empty() {
            bail!(
                "{}\n{}",
                "Initialization failed!".red().bold(),
                format!(
                    "{} already exists with content. Please backup or delete it first.",
                    ".bendis/Bender.yml".yellow()
                )
            );
        }
    }

    if dot_bender_yml.exists() {
        let content = fs::read_to_string(&dot_bender_yml)
            .context("Failed to read .bendis/.bender.yml")?;
        if !content.trim().is_empty() {
            bail!(
                "{}\n{}",
                "Initialization failed!".red().bold(),
                format!(
                    "{} already exists with content. Please backup or delete it first.",
                    ".bendis/.bender.yml".yellow()
                )
            );
        }
    }

    // Create .bendis directory if it doesn't exist
    if !bendis_dir.exists() {
        fs::create_dir(bendis_dir)
            .context("Failed to create .bendis directory")?;
        println!("  {} Created .bendis/ directory", "✓".green());
    } else {
        println!("  {} .bendis/ directory already exists", "ℹ".blue());
    }

    // Create blank Bender.yml if it doesn't exist or is empty
    if !bender_yml.exists() || fs::read_to_string(&bender_yml)?.trim().is_empty() {
        fs::write(&bender_yml, "")
            .context("Failed to create .bendis/Bender.yml")?;
        println!("  {} Created blank .bendis/Bender.yml", "✓".green());
    }

    // Create blank .bender.yml if it doesn't exist or is empty
    if !dot_bender_yml.exists() || fs::read_to_string(&dot_bender_yml)?.trim().is_empty() {
        fs::write(&dot_bender_yml, "")
            .context("Failed to create .bendis/.bender.yml")?;
        println!("  {} Created blank .bendis/.bender.yml", "✓".green());
    }

    println!("\n{}", "Bendis initialized successfully!".bold().green());
    println!(
        "\nNext steps:\n  1. Edit {} with your dependencies\n  2. Run {} to update",
        ".bendis/Bender.yml".yellow(),
        "bendis update".cyan().bold()
    );

    Ok(())
}
