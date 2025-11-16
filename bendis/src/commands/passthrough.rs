use anyhow::{Context, Result};
use colored::Colorize;
use std::process::Command;

pub fn run(args: &[String]) -> Result<()> {
    println!(
        "{} {}",
        "Passing through to bender:".dimmed(),
        args.join(" ").cyan()
    );

    let status = Command::new("bender")
        .args(args)
        .status()
        .context("Failed to execute bender command. Is bender installed?")?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}
