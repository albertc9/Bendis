mod commands;
mod converter;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

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
