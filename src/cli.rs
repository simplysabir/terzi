use crate::config::Config;
use colored::*;
use std::io::{self, Write};

pub fn print_welcome_with_config(config: &Config) {
    if !config.ui.show_welcome_message {
        return;
    }

    println!("{}", "🚀 Welcome to Terzi!".bright_yellow().bold());
    println!("{}", "Modern CLI API client".bright_white());
    println!(
        "{}",
        "Designed for developer productivity"
            .bright_black()
            .italic()
    );
    println!();
    println!("{}", "Quick start:".bright_yellow().bold());
    println!(
        "  {} terzi https://api.example.com/users",
        "→".bright_yellow()
    );
    println!("  {} terzi interactive", "→".bright_yellow());
    println!("  {} terzi --help", "→".bright_yellow());
    println!();
    println!(
        "{}",
        "Build, test, and manage your API requests with ease."
            .bright_black()
            .italic()
    );
    println!();
}

pub fn print_version() {
    println!();
    println!(
        "{} {}",
        "🚀 Terzi".bright_yellow().bold(),
        env!("CARGO_PKG_VERSION")
    );
    println!("{}", "Modern CLI API client".bright_white());
    println!(
        "{}",
        "Designed for developer productivity"
            .bright_black()
            .italic()
    );
    println!();
    println!(
        "{}: {}",
        "Repository".bright_yellow(),
        env!("CARGO_PKG_REPOSITORY")
    );
    println!(
        "{}: {}",
        "License".bright_yellow(),
        env!("CARGO_PKG_LICENSE")
    );
    println!(
        "{}: {}",
        "Author".bright_yellow(),
        "Sabir Khan <simplysabir@gmail.com>"
    );
    println!();
}

pub fn confirm_action(prompt: &str) -> bool {
    confirm_action_with_config(prompt, &Config::default())
}

pub fn confirm_action_with_config(prompt: &str, config: &Config) -> bool {
    // Skip confirmation if configured to not confirm dangerous operations
    if !config.ui.confirm_dangerous_operations {
        return true;
    }

    print!("{} [y/N]: ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes"
    } else {
        false
    }
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "❌ Error:".bright_red().bold(), message);
}

pub fn print_success(message: &str) {
    println!("{} {}", "✅".green(), message);
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ️".blue(), message);
}

pub fn print_warning(message: &str) {
    println!("{} {}", "⚠️".yellow(), message);
}

// CLI command completions and suggestions
pub fn suggest_similar_commands(input: &str) -> Vec<&'static str> {
    let commands = vec![
        "interactive",
        "list",
        "show",
        "delete",
        "edit",
        "history",
        "config",
        "import",
        "export",
        "help",
        "version",
    ];

    commands
        .into_iter()
        .filter(|cmd| {
            crate::utils::fuzzy_match(input, cmd).is_some()
                || cmd.starts_with(input)
                || input.starts_with(cmd)
        })
        .collect()
}

pub fn print_command_suggestions(input: &str) {
    let suggestions = suggest_similar_commands(input);
    if !suggestions.is_empty() {
        println!("{}", "Did you mean:".bright_yellow());
        for suggestion in suggestions {
            println!("  {} terzi {}", "→".bright_blue(), suggestion);
        }
    }
}

// Help text formatting
pub fn format_help_section(title: &str, items: &[(&str, &str)]) -> String {
    let mut output = String::new();
    output.push_str(&format!("{}:\n", title.bright_yellow().bold()));

    for (name, description) in items {
        output.push_str(&format!("  {:20} {}\n", name.bright_green(), description));
    }
    output.push('\n');
    output
}

// ASCII art and branding
pub fn print_logo() {
    println!();
    println!(
        "{}",
        r#"
    ████████╗███████╗██████╗ ███████╗██╗
    ╚══██╔══╝██╔════╝██╔══██╗╚══███╔╝██║
       ██║   █████╗  ██████╔╝  ███╔╝ ██║
       ██║   ██╔══╝  ██╔══██╗ ███╔╝  ██║
       ██║   ███████╗██║  ██║███████╗██║
       ╚═╝   ╚══════╝╚═╝  ╚═╝╚══════╝╚═╝
    "#
        .bright_yellow()
    );
    println!(
        "{}",
        "     🚀 Modern CLI API client for developers"
            .bright_white()
            .italic()
    );
    println!(
        "{}",
        "        Build, test, and manage APIs with ease"
            .bright_black()
            .italic()
    );
    println!();
}
