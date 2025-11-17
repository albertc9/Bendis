mod commands;
mod converter;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use utils::config::BendisConfig;
use utils::welcome;
use utils::version_checker;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Check for updates and handle user interaction
fn check_and_handle_updates(current_version: &str) -> Result<()> {
    match version_checker::check_for_updates(current_version) {
        Ok(updates) => {
            version_checker::handle_updates(updates)?;
        }
        Err(e) => {
            // Network error or other issues
            eprintln!(
                "\n{} {}",
                "⚠".yellow(),
                format!("Could not check for updates: {}", e).dimmed()
            );
            eprintln!(
                "{}",
                "Please check your internet connection.".dimmed()
            );
        }
    }
    Ok(())
}

#[derive(Parser)]
#[command(name = "bendis")]
#[command(about = "A wrapper and patch tool for Bender to work better in Heris project", long_about = None)]
#[command(version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Pass all other arguments to bender
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize bendis project structure
    Init,
    /// Update dependencies with URL conversion
    Update,
    /// Edit bendis configuration file
    Config,
}

fn main() -> Result<()> {
    // Check if this is the first run or version update BEFORE parsing cli (clap will handle --version and exit)
    let mut config = BendisConfig::load()?;
    let is_first_run = config.first_run == 1;
    let current_version = VERSION;
    let stored_version = config.version.as_str();
    let is_version_changed = !stored_version.is_empty() && stored_version != current_version;
    let is_version_command = std::env::args().any(|arg| arg == "--version" || arg == "-V");

    // Determine if we should show welcome message
    let should_show_welcome = is_first_run || is_version_changed;

    // Handle welcome message display with --version command specially
    if should_show_welcome && is_version_command {
        if is_version_changed {
            welcome::show_welcome_with_version(Some(stored_version));
        } else {
            welcome::show_welcome();
        }
        config.first_run = 0;
        config.version = current_version.to_string();
        config.save()?;

        // Check for updates after showing welcome message
        check_and_handle_updates(current_version)?;
        return Ok(());
    }

    // For --version command without welcome message, check for updates
    if is_version_command {
        // Let clap handle the version display, but check for updates first
        println!("bendis {}", VERSION);
        check_and_handle_updates(current_version)?;
        return Ok(());
    }

    // Show welcome message for first run or version update (non-version commands)
    if should_show_welcome {
        if is_version_changed {
            welcome::show_welcome_with_version(Some(stored_version));
        } else {
            welcome::show_welcome();
        }
        config.first_run = 0;
        config.version = current_version.to_string();
        config.save()?;
        welcome::show_separator();
    }

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => {
            commands::init::run()?;
        }
        Some(Commands::Update) => {
            commands::update::run()?;
        }
        Some(Commands::Config) => {
            commands::config::run()?;
        }
        None => {
            // Pass through to bender
            if !cli.args.is_empty() {
                commands::passthrough::run(&cli.args)?;
            } else {
                // No command, show help
                println!("{}", "Bendis - Bender Pro for Heris project".bold().green());
                println!("\nUsage: bendis <COMMAND>");
                println!("\nCommands:");
                println!("  init     Initialize bendis project structure");
                println!("  update   Update dependencies with URL conversion");
                println!("  config   Edit bendis configuration file");
                println!("  <other>  Pass through to bender");
                println!("\nOptions:");
                println!("  -h, --help     Print help");
                println!("  -v, --version  Print version");
            }
        }
    }

    Ok(())
}
