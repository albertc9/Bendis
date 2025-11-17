mod commands;
mod converter;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use utils::config::BendisConfig;
use utils::welcome;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
    // Check if this is the first run BEFORE parsing cli (clap will handle --version and exit)
    let mut config = BendisConfig::load()?;
    let is_first_run = config.first_run == 1;
    let is_version_command = std::env::args().any(|arg| arg == "--version" || arg == "-V");

    // Handle first run with --version specially
    if is_first_run && is_version_command {
        welcome::show_welcome();
        config.first_run = 0;
        config.save()?;
        return Ok(());
    }

    // Show welcome message for first run (non-version commands)
    if is_first_run {
        welcome::show_welcome();
        config.first_run = 0;
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
