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
    println!("{}", ASCII_ART.cyan());
    println!("{}", format!("Welcome to Bendis v{}", VERSION).bold().green());
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
