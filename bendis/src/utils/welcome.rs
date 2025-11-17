use colored::Colorize;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const ASCII_ART: &str = r#"

 ▄▄▄▄▄                    █    ▀
 █    █  ▄▄▄   ▄ ▄▄    ▄▄▄█  ▄▄▄     ▄▄▄
 █▄▄▄▄▀ █▀  █  █▀  █  █▀ ▀█    █    █   ▀
 █    █ █▀▀▀▀  █   █  █   █    █     ▀▀▀▄
 █▄▄▄▄▀ ▀█▄▄▀  █   █  ▀█▄██  ▄▄█▄▄  ▀▄▄▄▀

"#;

/// Display the welcome message for first-time users
pub fn show_welcome() {
    show_welcome_with_version(None);
}

/// Display the welcome message with optional previous version info
pub fn show_welcome_with_version(previous_version: Option<&str>) {
    println!("{}", ASCII_ART.cyan());

    if let Some(prev_ver) = previous_version {
        println!("{}", format!("Welcome to Bendis v{} (updated from v{})", VERSION, prev_ver).bold().green());
    } else {
        println!("{}", format!("Welcome to Bendis v{}", VERSION).bold().green());
    }

    println!();
    println!("A patch tool for Bender to work better in HERIS project");
    println!();
    println!("Authors: Heris Team, albert.cheung@cern.ch");
    println!("License: MIT");
    println!("Repository: https://github.com/albertc9/Bendis");
    println!();
    println!("Get started with: bendis --help");
    println!();
}

/// Display separator line between welcome message and command output
pub fn show_separator() {
    println!("{}", "--------".dimmed());
}
