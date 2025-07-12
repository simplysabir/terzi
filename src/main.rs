use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::collections::HashMap;

mod cli;
mod client;
mod config;
mod interactive;
mod output;
mod request;
mod storage;
mod utils;

use client::TerziClient;
use config::Config;
use interactive::InteractiveMode;
use output::ResponseFormatter;
use request::RequestBuilder;
use storage::Storage;
use utils::*;

#[derive(Parser, Clone)]
#[command(
    name = "terzi",
    about = "🚀 Modern CLI API client with precision and simplicity",
    long_about = "Terzi: A modern CLI API client designed for developers who value precision and simplicity. \
                  Build, test, and manage your API requests with ease.",
    version,
    author = "Sabir Khan <simplysabir@gmail.com>"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// URL to make request to (for quick requests)
    #[arg(value_name = "URL")]
    url: Option<String>,

    /// HTTP method (GET, POST, PUT, DELETE, etc.)
    #[arg(short, long, default_value = "GET")]
    method: String,

    /// Request headers (key:value format)
    #[arg(short = 'H', long = "header", value_name = "HEADER")]
    headers: Vec<String>,

    /// Request body (JSON, form data, or raw)
    #[arg(short, long)]
    body: Option<String>,

    /// JSON data (shorthand for -H "Content-Type: application/json" -b "JSON")
    #[arg(short, long)]
    json: Option<String>,

    /// Form data (key=value pairs)
    #[arg(short, long = "form")]
    form_data: Vec<String>,

    /// Authorization header
    #[arg(short = 'A', long)]
    auth: Option<String>,

    /// Follow redirects
    #[arg(short = 'L', long)]
    follow_redirects: bool,

    /// Timeout in seconds
    #[arg(short, long, default_value = "30")]
    timeout: u64,

    /// Save request with a name
    #[arg(long)]
    save: Option<String>,

    /// Load saved request
    #[arg(long)]
    load: Option<String>,

    /// Output format (auto, json, yaml, table)
    #[arg(short, long, default_value = "auto")]
    output: String,

    /// Include response headers in output
    #[arg(short = 'i', long)]
    include_headers: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Silent mode (no output formatting)
    #[arg(short = 'S', long)]
    silent: bool,

    /// Pretty print JSON responses
    #[arg(short, long, default_value = "true")]
    pretty: bool,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Make an interactive request with guided prompts
    Interactive,

    /// List saved requests
    List {
        /// Filter by pattern
        #[arg(short, long)]
        filter: Option<String>,
    },

    /// Show details of a saved request
    Show {
        /// Name of the saved request
        name: String,
    },

    /// Delete a saved request
    Delete {
        /// Name of the saved request to delete
        name: String,
    },

    /// Edit a saved request
    Edit {
        /// Name of the saved request to edit
        name: String,
    },

    /// Show request history
    History {
        /// Number of recent requests to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Configure terzi settings
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Export saved requests
    Export {
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        /// Format (json, yaml)
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Show version information
    Version,
}

#[derive(Subcommand, Clone)]
enum ConfigAction {
    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },
    /// List all configuration
    List,
    /// Reset configuration to defaults
    Reset,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            // Handle help and version commands
            let error_msg = e.to_string();
            if error_msg.contains("Print help")
                || error_msg.contains("Print version")
                || error_msg.starts_with("terzi ")
                || error_msg.contains("Modern CLI API client")
            {
                print!("{}", e);
                std::process::exit(0);
            }

            // Check if it's a subcommand error and provide suggestions
            let error_msg = e.to_string();
            if error_msg.contains("unrecognized subcommand") || error_msg.contains("invalid value")
            {
                // Extract the invalid command from the error message
                if let Some(captures) = error_msg.split('\'').nth(1) {
                    cli::print_command_suggestions(captures);
                }
            }
            return Err(e.into());
        }
    };

    // Initialize configuration and storage
    let config = Config::load().await?;
    let mut storage = Storage::new().await?;
    let client = TerziClient::new(&config)?;
    let formatter = ResponseFormatter::new(&config);

    match cli.command {
        Some(Commands::Interactive) => {
            let mut interactive = InteractiveMode::new(client, storage, formatter);
            interactive.run().await?;
        }

        Some(Commands::List { filter }) => {
            let requests = storage.list_requests(filter.as_deref()).await?;
            print_request_list(&requests);
        }

        Some(Commands::Show { name }) => match storage.get_request(&name).await? {
            Some(request) => print_request_details(&request),
            None => cli::print_error(&format!("Request '{}' not found", name)),
        },

        Some(Commands::Delete { name }) => {
            if cli::confirm_action_with_config(
                &format!("Are you sure you want to delete request '{}'?", name),
                &config,
            ) {
                if storage.delete_request(&name).await? {
                    cli::print_success(&format!("Request '{}' deleted", name));
                } else {
                    cli::print_error(&format!("Request '{}' not found", name));
                }
            } else {
                cli::print_info("Delete operation cancelled");
            }
        }

        Some(Commands::Edit { name }) => match storage.get_request(&name).await? {
            Some(mut request) => {
                let mut interactive = InteractiveMode::new(client, storage, formatter);
                interactive.edit_request(&mut request).await?;
            }
            None => cli::print_error(&format!("Request '{}' not found", name)),
        },

        Some(Commands::History { limit }) => {
            let history = storage.get_history(limit).await?;
            print_history(&history);
        }

        Some(Commands::Config { action }) => {
            handle_config_action(action, &config).await?;
        }

        Some(Commands::Export { output, format }) => {
            export_requests(&storage, output.as_deref(), &format).await?;
        }

        Some(Commands::Version) => {
            cli::print_version();
        }

        None => {
            // Direct request mode
            if let Some(ref url) = cli.url {
                let mut request = build_request_from_cli(&cli, url, &config)?;

                if let Some(ref name) = cli.save {
                    request.name = name.clone();
                    storage.save_request(name, &request).await?;
                    cli::print_success(&format!("Request saved as '{}'", name));
                }

                match client.execute_request(&request).await {
                    Ok(response) => {
                        // Save to history
                        storage.add_to_history(&request, &response).await?;

                        // Format and display response
                        if !cli.silent {
                            let merged_cli = merge_cli_with_config(&cli, &config);
                            formatter.display_response(&response, &merged_cli).await?;
                        }
                    }
                    Err(e) => {
                        let error_chain = utils::format_error_chain(&e);
                        storage.add_error_to_history(&request, &error_chain).await?;
                        cli::print_error(&format!("Request failed: {}", error_chain));
                        std::process::exit(1);
                    }
                }
            } else if let Some(ref name) = cli.load {
                match storage.get_request(name).await? {
                    Some(request) => match client.execute_request(&request).await {
                        Ok(response) => {
                            storage.add_to_history(&request, &response).await?;

                            if !cli.silent {
                                let merged_cli = merge_cli_with_config(&cli, &config);
                                formatter.display_response(&response, &merged_cli).await?;
                            }
                        }
                        Err(e) => {
                            let error_chain = utils::format_error_chain(&e);
                            storage.add_error_to_history(&request, &error_chain).await?;
                            cli::print_error(&format!("Request failed: {}", error_chain));
                            std::process::exit(1);
                        }
                    },
                    None => {
                        cli::print_error(&format!("Request '{}' not found", name));
                        std::process::exit(1);
                    }
                }
            } else {
                // No URL provided, show welcome message
                cli::print_logo();
                println!();
                cli::print_welcome_with_config(&config);

                // Show helpful sections
                let common_commands = vec![
                    ("interactive", "Start guided request building"),
                    ("list", "Show all saved requests"),
                    ("history", "View request history"),
                    ("config", "Manage configuration"),
                    ("version", "Show version information"),
                ];

                let quick_examples = vec![
                    ("GET request", "terzi https://api.example.com/users"),
                    (
                        "POST JSON",
                        "terzi -X POST -j '{\"name\":\"John\"}' https://api.example.com/users",
                    ),
                    (
                        "With headers",
                        "terzi -H 'Authorization: Bearer token' https://api.example.com/data",
                    ),
                    (
                        "Save request",
                        "terzi --save myrequest https://api.example.com/endpoint",
                    ),
                ];

                print!(
                    "{}",
                    cli::format_help_section("Common Commands", &common_commands)
                );
                print!(
                    "{}",
                    cli::format_help_section("Quick Examples", &quick_examples)
                );
            }
        }
    }

    Ok(())
}

fn merge_cli_with_config(cli: &Cli, config: &Config) -> Cli {
    let mut merged = cli.clone();

    // Use config defaults for output settings if CLI uses defaults
    if merged.output == "auto" {
        merged.output = config.output.default_format.clone();
    }

    if merged.pretty == true && cli.pretty == true {
        merged.pretty = config.output.pretty_print;
    }

    if !merged.include_headers {
        merged.include_headers = config.output.show_headers;
    }

    merged
}

fn build_request_from_cli(cli: &Cli, url: &str, config: &Config) -> Result<request::SavedRequest> {
    // Validate URL first
    if !utils::is_valid_url(url) {
        return Err(anyhow::anyhow!(
            "Invalid URL: {}. Please provide a valid URL starting with http:// or https://",
            url
        ));
    }

    // Validate method
    utils::validate_method(&cli.method)?;

    let mut builder = RequestBuilder::new(url, &cli.method)?;

    // Add headers
    for header in &cli.headers {
        if let Some((key, value)) = header.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            if key.is_empty() {
                return Err(anyhow::anyhow!(
                    "Invalid header format: '{}'. Use 'key:value'",
                    header
                ));
            }

            // Validate header name and value
            if !utils::is_valid_header_name(key) {
                return Err(anyhow::anyhow!(
                    "Invalid header name: '{}'. Header names must be ASCII and cannot contain ':' or newlines",
                    key
                ));
            }
            if !utils::is_valid_header_value(value) {
                return Err(anyhow::anyhow!(
                    "Invalid header value: '{}'. Header values cannot contain newlines",
                    value
                ));
            }

            builder = builder.header(key, value);
        } else {
            return Err(anyhow::anyhow!(
                "Invalid header format: '{}'. Use 'key:value'",
                header
            ));
        }
    }

    // Add auth
    if let Some(ref auth) = cli.auth {
        builder = builder.auth(auth)?;
    }

    // Add body (validate only one body type)
    let mut body_count = 0;
    if cli.json.is_some() {
        body_count += 1;
    }
    if cli.body.is_some() {
        body_count += 1;
    }
    if !cli.form_data.is_empty() {
        body_count += 1;
    }

    if body_count > 1 {
        return Err(anyhow::anyhow!(
            "Only one body type allowed: --json, --body, or --form"
        ));
    }

    if let Some(ref json) = cli.json {
        // Validate JSON before adding
        if !utils::is_valid_json(json) {
            return Err(anyhow::anyhow!(
                "Invalid JSON provided: {}. Please check your JSON syntax",
                json
            ));
        }
        builder = builder.json_body(json)?;
    } else if let Some(ref body) = cli.body {
        builder = builder.raw_body(body);
    } else if !cli.form_data.is_empty() {
        let mut form = HashMap::new();
        for pair in &cli.form_data {
            if let Some((key, value)) = pair.split_once('=') {
                form.insert(key.to_string(), value.to_string());
            } else {
                return Err(anyhow::anyhow!(
                    "Invalid form data format: '{}'. Use 'key=value'",
                    pair
                ));
            }
        }
        builder = builder.form_body(form)?;
    }

    // Use CLI timeout if not default (30), otherwise use config default
    let timeout = if cli.timeout == 30 {
        config.general.default_timeout
    } else {
        cli.timeout
    };

    // Validate timeout
    utils::validate_timeout(timeout)?;

    builder = builder.timeout(timeout);

    // Use CLI follow_redirects if explicitly set, otherwise use config default
    let follow_redirects = if cli.follow_redirects {
        cli.follow_redirects
    } else {
        config.general.follow_redirects
    };
    builder = builder.follow_redirects(follow_redirects);

    Ok(builder.build())
}

fn print_request_list(requests: &[request::SavedRequest]) {
    if requests.is_empty() {
        cli::print_info(
            "No saved requests found. Create one with 'terzi --save <name> <url>' or 'terzi interactive'",
        );
        return;
    }

    use comfy_table::*;
    let mut table = Table::new();
    table
        .set_header(vec!["Name", "Method", "URL", "Created"])
        .load_preset(presets::UTF8_FULL_CONDENSED);

    // Calculate responsive URL width based on terminal size
    let terminal_width = utils::get_terminal_width();
    let fixed_columns_width = 45; // Space for Name, Method, Created + padding
    let url_max_width = if terminal_width > fixed_columns_width + 10 {
        terminal_width - fixed_columns_width
    } else {
        30 // Minimum URL width
    };

    for req in requests {
        let truncated_url = utils::truncate_string(&req.url, url_max_width);
        table.add_row(vec![
            &req.name,
            &req.method,
            &truncated_url,
            &req.created_at.format("%Y-%m-%d %H:%M").to_string(),
        ]);
    }

    println!("{}", table);
}

fn print_request_details(request: &request::SavedRequest) {
    println!("📋 Request Details: {}", request.name);
    println!("🔗 URL: {}", request.url);
    println!("🔧 Method: {}", request.method);

    if !request.headers.is_empty() {
        println!("📤 Headers:");

        // Define sensitive header patterns
        let sensitive_patterns = &[
            r"(?i)authorization",
            r"(?i)api-key",
            r"(?i)x-api-key",
            r"(?i)access[_-]?token",
            r"(?i)bearer",
            r"(?i)session",
            r"(?i)cookie",
            r"(?i)password",
            r"(?i)secret",
        ];

        for (key, value) in &request.headers {
            // Check if this header contains sensitive data
            let is_sensitive = sensitive_patterns.iter().any(|pattern| {
                regex::Regex::new(pattern)
                    .map(|re| re.is_match(key))
                    .unwrap_or(false)
            });

            if is_sensitive {
                let masked_value = utils::mask_sensitive_data(value, &[r".*"]);
                println!("  {}: {}", key, masked_value);
            } else {
                println!("  {}: {}", key, value);
            }
        }
    }

    if let Some(ref body) = request.body {
        // Mask sensitive data in body (tokens, passwords, etc.)
        let sensitive_body_patterns = &[
            r#""password"\s*:\s*"[^"]*""#,
            r#""token"\s*:\s*"[^"]*""#,
            r#""secret"\s*:\s*"[^"]*""#,
            r#""api_key"\s*:\s*"[^"]*""#,
            r#""access_token"\s*:\s*"[^"]*""#,
        ];

        let masked_body = utils::mask_sensitive_data(body, sensitive_body_patterns);
        println!("📝 Body: {}", masked_body);
    }

    println!("📅 Created: {}", request.created_at);
}

fn print_history(history: &[storage::HistoryEntry]) {
    if history.is_empty() {
        cli::print_info("No request history found. Make some requests first!");
        return;
    }

    use comfy_table::*;
    let mut table = Table::new();
    table
        .set_header(vec!["Time", "Method", "URL", "Status", "Duration"])
        .load_preset(presets::UTF8_FULL_CONDENSED);

    // Calculate responsive URL width based on terminal size
    let terminal_width = utils::get_terminal_width();
    let fixed_columns_width = 35; // Space for Time, Method, Status, Duration + padding
    let url_max_width = if terminal_width > fixed_columns_width + 10 {
        terminal_width - fixed_columns_width
    } else {
        25 // Minimum URL width
    };

    for entry in history {
        let status_display = match entry.response_status {
            Some(status) => {
                let color = if status >= 200 && status < 300 {
                    "🟢"
                } else if status >= 400 {
                    "🔴"
                } else {
                    "🟡"
                };
                format!("{} {}", color, status)
            }
            None => "❌ Error".to_string(),
        };

        let truncated_url = utils::truncate_string(&entry.url, url_max_width);

        table.add_row(vec![
            &entry.timestamp.format("%H:%M:%S").to_string(),
            &entry.method,
            &truncated_url,
            &status_display,
            &format!("{}ms", entry.duration_ms.unwrap_or(0)),
        ]);
    }

    println!("{}", table);
}

async fn handle_config_action(action: ConfigAction, config: &Config) -> Result<()> {
    match action {
        ConfigAction::Set { key, value } => {
            let mut config = config.clone();
            config.set_value(&key, &value).await?;
            cli::print_success(&format!("Set {} = {}", key, value));
        }
        ConfigAction::Get { key } => {
            if let Some(value) = config.get_value(&key).await {
                cli::print_info(&format!("{} = {}", key, value));
            } else {
                cli::print_error(&format!("Configuration key '{}' not found", key));
            }
        }
        ConfigAction::List => {
            println!("{}", "🚀 Configuration".bright_cyan().bold());
            println!();

            let mut found_any = false;
            for key in Config::list_all_keys() {
                if let Some(value) = config.get_value(key).await {
                    println!("  {}: {}", key.bright_cyan(), value);
                    found_any = true;
                }
            }

            if !found_any {
                cli::print_info("No configuration found. Using defaults.");
            }
        }
        ConfigAction::Reset => {
            if cli::confirm_action_with_config(
                "Are you sure you want to reset all configuration to defaults?",
                &config,
            ) {
                let mut config = config.clone();
                config.reset_to_defaults().await?;
                cli::print_success("Configuration reset to defaults");
            } else {
                cli::print_info("Reset operation cancelled");
            }
        }
    }
    Ok(())
}

async fn export_requests(storage: &Storage, output: Option<&str>, format: &str) -> Result<()> {
    let data = storage.export_data(false).await?.to_string();

    let output_content = match format {
        "yaml" => {
            let json_value: serde_json::Value = serde_json::from_str(&data)?;
            serde_yaml::to_string(&json_value)?
        }
        "json" => data.clone(),
        _ => data,
    };

    match output {
        Some(file_path) => {
            use tokio::io::AsyncWriteExt;

            // Add appropriate extension if not present
            let final_path = if !file_path.ends_with(&format!(".{}", format)) {
                format!("{}.{}", file_path, format)
            } else {
                file_path.to_string()
            };

            let mut file = tokio::fs::File::create(&final_path).await?;
            file.write_all(output_content.as_bytes()).await?;
            file.flush().await?;
            cli::print_success(&format!("Requests exported to {}", final_path));
        }
        None => {
            println!("{}", output_content);
        }
    }

    Ok(())
}
