use anyhow::{Context, Result};
use std::process::Command;

pub fn run(args: &[String]) -> Result<()> {
    // Don't print anything to stdout, as it will interfere with bender's output
    // (e.g., when generating scripts with `bender script`)

    let status = Command::new("bender")
        .args(args)
        .status()
        .context("Failed to execute bender command. Is bender installed?")?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}
