use colored::*;
use std::io::{self, Write};

pub fn print_welcome() {
    println!("{}", "🎯 Welcome to Terzi!".bright_cyan().bold());
    println!("{}", "The deadly efficient CLI API client".bright_white());
    println!();
    println!("{}", "Quick start:".bright_yellow().bold());
    println!("  {} terzi https://api.example.com/users", "→".bright_blue());
    println!("  {} terzi interactive", "→".bright_blue());
    println!("  {} terzi --help", "→".bright_blue());
    println!();
}

pub fn print_version() {
    println!("terzi {}", env!("CARGO_PKG_VERSION"));
    println!("A deadly efficient CLI API client");
    println!();
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
    println!("License: {}", env!("CARGO_PKG_LICENSE"));
}

pub fn confirm_action(prompt: &str) -> bool {
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
        "interactive", "list", "show", "delete", "edit", "history", 
        "config", "import", "export", "help", "version"
    ];
    
    commands.into_iter()
        .filter(|cmd| {
            crate::utils::fuzzy_match(input, cmd).is_some() ||
            cmd.starts_with(input) ||
            input.starts_with(cmd)
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
    println!("{}", r#"
    ████████╗███████╗██████╗ ███████╗██╗
    ╚══██╔══╝██╔════╝██╔══██╗╚══███╔╝██║
       ██║   █████╗  ██████╔╝  ███╔╝ ██║
       ██║   ██╔══╝  ██╔══██╗ ███╔╝  ██║
       ██║   ███████╗██║  ██║███████╗██║
       ╚═╝   ╚══════╝╚═╝  ╚═╝╚══════╝╚═╝
    "#.bright_cyan());
    println!("{}", "Deadly efficient CLI API client".bright_white().italic());
    println!();
}