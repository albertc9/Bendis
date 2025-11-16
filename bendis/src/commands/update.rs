use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::converter::format;
use crate::utils::config;

pub fn run() -> Result<()> {
    println!("{}", "=".repeat(60).bright_blue());
    println!("{}", "Starting Bendis Update Process".bold().green());
    println!("{}", "=".repeat(60).bright_blue());

    let bendis_dir = config::get_bendis_dir();
    let root_dir = config::get_root_dir();

    // Check if .bendis directory exists
    if !bendis_dir.exists() {
        bail!(
            "{}\n{}",
            "Error: .bendis directory not found!".red().bold(),
            format!(
                "Please run {} first to initialize the project.",
                "bendis init".cyan().bold()
            )
        );
    }

    // Step 1: Run bender update in .bendis directory
    println!(
        "\n{} {}",
        "Step 1/5:".bold().yellow(),
        "Running bender update in .bendis/".bold()
    );
    println!("{}", "-".repeat(60).dimmed());

    run_bender_update_in_bendis(&bendis_dir)?;

    // Step 2: Run format converter
    println!(
        "\n{} {}",
        "Step 2/5:".bold().yellow(),
        "Converting URLs and generating files".bold()
    );
    println!("{}", "-".repeat(60).dimmed());

    format::convert(&bendis_dir, &root_dir)?;

    // Step 3: Run bender update in root directory
    println!(
        "\n{} {}",
        "Step 3/5:".bold().yellow(),
        "Running bender update in root directory".bold()
    );
    println!("{}", "-".repeat(60).dimmed());

    run_bender_update_in_root()?;

    // Step 4: Remove .bendis/.bender directory
    println!(
        "\n{} {}",
        "Step 4/5:".bold().yellow(),
        "Cleaning up .bendis/.bender/".bold()
    );
    println!("{}", "-".repeat(60).dimmed());

    cleanup_bendis_bender_dir(&bendis_dir)?;

    // Step 5: Done
    println!(
        "\n{} {}",
        "Step 5/5:".bold().yellow(),
        "Verification".bold()
    );
    println!("{}", "-".repeat(60).dimmed());

    verify_completion(&root_dir)?;

    println!("\n{}", "=".repeat(60).bright_blue());
    println!("{}", "Bendis Update Completed Successfully!".bold().green());
    println!("{}", "=".repeat(60).bright_blue());
    println!(
        "\n{} Your project is now updated with IHEP internal URLs.",
        "✓".green()
    );
    println!(
        "{} Lock file: {}",
        "✓".green(),
        "Bender.lock".cyan()
    );
    println!(
        "{} Dependencies: {}",
        "✓".green(),
        ".bender/".cyan()
    );

    Ok(())
}

fn run_bender_update_in_bendis(bendis_dir: &Path) -> Result<()> {
    println!("  {} Executing: bender -d ./.bendis update", "→".blue());

    let status = Command::new("bender")
        .args(&["-d", "./.bendis", "update"])
        .status()
        .context("Failed to run bender. Is bender installed and in PATH?")?;

    if !status.success() {
        bail!(
            "{}",
            "Bender update in .bendis/ failed!".red().bold()
        );
    }

    // Check if Bender.lock was created
    let lock_file = bendis_dir.join("Bender.lock");
    if !lock_file.exists() {
        bail!(
            "{}",
            "Failed to generate .bendis/Bender.lock".red().bold()
        );
    }

    println!("  {} Bender update in .bendis/ completed", "✓".green());
    println!("  {} Generated .bendis/Bender.lock", "✓".green());

    Ok(())
}

fn run_bender_update_in_root() -> Result<()> {
    println!("  {} Executing: bender update", "→".blue());

    let status = Command::new("bender")
        .arg("update")
        .status()
        .context("Failed to run bender in root directory")?;

    if !status.success() {
        bail!(
            "{}",
            "Bender update in root directory failed!".red().bold()
        );
    }

    println!("  {} Bender update in root directory completed", "✓".green());
    println!("  {} Generated Bender.lock", "✓".green());
    println!("  {} Generated .bender/ with dependencies", "✓".green());

    Ok(())
}

fn cleanup_bendis_bender_dir(bendis_dir: &Path) -> Result<()> {
    let bender_dir = bendis_dir.join(".bender");

    if bender_dir.exists() {
        println!("  {} Removing .bendis/.bender/", "→".blue());
        fs::remove_dir_all(&bender_dir)
            .context("Failed to remove .bendis/.bender/")?;
        println!("  {} Removed .bendis/.bender/", "✓".green());
    } else {
        println!("  {} .bendis/.bender/ not found (already clean)", "ℹ".blue());
    }

    Ok(())
}

fn verify_completion(root_dir: &Path) -> Result<()> {
    let mut success = true;

    // Check Bender.yml
    let bender_yml = root_dir.join("Bender.yml");
    if bender_yml.exists() {
        println!("  {} Bender.yml exists", "✓".green());
    } else {
        println!("  {} Bender.yml missing!", "✗".red());
        success = false;
    }

    // Check .bender.yml
    let dot_bender_yml = root_dir.join(".bender.yml");
    if dot_bender_yml.exists() {
        println!("  {} .bender.yml exists", "✓".green());
    } else {
        println!("  {} .bender.yml missing!", "✗".red());
        success = false;
    }

    // Check Bender.lock
    let lock_file = root_dir.join("Bender.lock");
    if lock_file.exists() {
        println!("  {} Bender.lock exists", "✓".green());
    } else {
        println!("  {} Bender.lock missing!", "✗".red());
        success = false;
    }

    // Check .bender directory
    let bender_dir = root_dir.join(".bender");
    if bender_dir.exists() {
        println!("  {} .bender/ directory exists", "✓".green());
    } else {
        println!("  {} .bender/ directory missing!", "✗".red());
        success = false;
    }

    if !success {
        bail!("Verification failed! Some files are missing.");
    }

    Ok(())
}
