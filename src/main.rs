use anyhow::Result;
use clap::{Parser, Subcommand};
use std::collections::HashMap;

mod client;
mod config;
mod interactive;
mod output;
mod request;
mod storage;
mod utils;

use utils::*;
use client::TerziClient;
use config::Config;
use interactive::InteractiveMode;
use output::ResponseFormatter;
use request::RequestBuilder;
use storage::Storage;

#[derive(Parser)]
#[command(
    name = "terzi",
    about = "A blazingly fast, deadly efficient CLI API client",
    long_about = "Terzi eliminates API complexity with precision and efficiency. \
                  Perfect for developers who want the power of Postman in their terminal.",
    version,
    author = "Sabir Khan <simplysabir@gmail.com>"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// URL to make request to (for quick requests)
    #[arg(value_name = "URL", conflicts_with = "command")]
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

#[derive(Subcommand)]
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
}

#[derive(Subcommand)]
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
    let cli = Cli::parse();
    
    // Initialize configuration and storage
    let config = Config::load().await?;
    let storage = Storage::new().await?;
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
        
        Some(Commands::Show { name }) => {
            match storage.get_request(&name).await? {
                Some(request) => print_request_details(&request),
                None => eprintln!("❌ Request '{}' not found", name),
            }
        }
        
        Some(Commands::Delete { name }) => {
            if storage.delete_request(&name).await? {
                println!("✅ Request '{}' deleted", name);
            } else {
                eprintln!("❌ Request '{}' not found", name);
            }
        }
        
        Some(Commands::Edit { name }) => {
            match storage.get_request(&name).await? {
                Some(mut request) => {
                    let mut interactive = InteractiveMode::new(client, storage, formatter);
                    interactive.edit_request(&mut request).await?;
                }
                None => eprintln!("❌ Request '{}' not found", name),
            }
        }
        
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
        
        None => {
            // Direct request mode
            if let Some(url) = cli.url {
                let mut request = build_request_from_cli(&cli, &url)?;
                
                if let Some(ref name) = cli.save {
                    request.name = name.clone();
                    storage.save_request(name, &request).await?;
                    println!("💾 Request saved as '{}'", name);
                }
                
                match client.execute_request(&request).await {
                    Ok(response) => {
                        // Save to history
                        storage.add_to_history(&request, &response).await?;
                        
                        // Format and display response
                        if !cli.silent {
                            formatter.display_response(&response, &cli).await?;
                        }
                    }
                    Err(e) => {
                        storage.add_error_to_history(&request, &e.to_string()).await?;
                        eprintln!("❌ Request failed: {}", e);
                        std::process::exit(1);
                    }
                }
            } else if let Some(ref name) = cli.load {
                match storage.get_request(name).await? {
                    Some(request) => {
                        match client.execute_request(&request).await {
                            Ok(response) => {
                                storage.add_to_history(&request, &response).await?;
                                
                                if !cli.silent {
                                    formatter.display_response(&response, &cli).await?;
                                }
                            }
                            Err(e) => {
                                storage.add_error_to_history(&request, &e.to_string()).await?;
                                eprintln!("❌ Request failed: {}", e);
                                std::process::exit(1);
                            }
                        }
                    }
                    None => {
                        eprintln!("❌ Request '{}' not found", name);
                        std::process::exit(1);
                    }
                }
            } else {
                // No URL provided, show help or enter interactive mode
                println!("🎯 Welcome to Terzi! Use --help for options or run 'terzi interactive' for guided mode.");
            }
        }
    }

    Ok(())
}

fn build_request_from_cli(cli: &Cli, url: &str) -> Result<request::SavedRequest> {
    let mut builder = RequestBuilder::new(url, &cli.method)?;
    
    // Add headers
    for header in &cli.headers {
        if let Some((key, value)) = header.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            if key.is_empty() {
                return Err(anyhow::anyhow!("Invalid header format: '{}'. Use 'key:value'", header));
            }
            builder = builder.header(key, value);
        } else {
            return Err(anyhow::anyhow!("Invalid header format: '{}'. Use 'key:value'", header));
        }
    }
    
    // Add auth
    if let Some(ref auth) = cli.auth {
        builder = builder.auth(auth)?;
    }
    
    // Add body (validate only one body type)
    let mut body_count = 0;
    if cli.json.is_some() { body_count += 1; }
    if cli.body.is_some() { body_count += 1; }
    if !cli.form_data.is_empty() { body_count += 1; }
    
    if body_count > 1 {
        return Err(anyhow::anyhow!("Only one body type allowed: --json, --body, or --form"));
    }
    
    if let Some(ref json) = cli.json {
        builder = builder.json_body(json)?;
    } else if let Some(ref body) = cli.body {
        builder = builder.raw_body(body);
    } else if !cli.form_data.is_empty() {
        let mut form = HashMap::new();
        for pair in &cli.form_data {
            if let Some((key, value)) = pair.split_once('=') {
                form.insert(key.to_string(), value.to_string());
            } else {
                return Err(anyhow::anyhow!("Invalid form data format: '{}'. Use 'key=value'", pair));
            }
        }
        builder = builder.form_body(form)?;
    }
    
    builder = builder.timeout(cli.timeout);
    builder = builder.follow_redirects(cli.follow_redirects);
    
    Ok(builder.build())
}

fn print_request_list(requests: &[request::SavedRequest]) {
    if requests.is_empty() {
        println!("📭 No saved requests found");
        return;
    }
    
    use comfy_table::*;
    let mut table = Table::new();
    table
        .set_header(vec!["Name", "Method", "URL", "Created"])
        .load_preset(presets::UTF8_FULL_CONDENSED);
    
    for req in requests {
        table.add_row(vec![
            &req.name,
            &req.method,
            &req.url,
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
        for (key, value) in &request.headers {
            println!("  {}: {}", key, value);
        }
    }
    
    if let Some(ref body) = request.body {
        println!("📝 Body: {}", body);
    }
    
    println!("📅 Created: {}", request.created_at);
}

fn print_history(history: &[storage::HistoryEntry]) {
    if history.is_empty() {
        println!("📭 No request history found");
        return;
    }
    
    use comfy_table::*;
    let mut table = Table::new();
    table
        .set_header(vec!["Time", "Method", "URL", "Status", "Duration"])
        .load_preset(presets::UTF8_FULL_CONDENSED);
    
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
        
        table.add_row(vec![
            &entry.timestamp.format("%H:%M:%S").to_string(),
            &entry.method,
            &entry.url,
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
            println!("✅ Set {} = {}", key, value);
        }
        ConfigAction::Get { key } => {
            if let Some(value) = config.get_value(&key).await {
                println!("📋 {} = {}", key, value);
            } else {
                eprintln!("❌ Configuration key '{}' not found", key);
            }
        }
        ConfigAction::List => {
            println!("📋 Configuration:");
            for key in Config::list_all_keys() {
                if let Some(value) = config.get_value(key).await {
                    println!("  {}: {}", key, value);
                }
            }
        }
        ConfigAction::Reset => {
            let mut config = config.clone();
            config.reset_to_defaults().await?;
            println!("✅ Configuration reset to defaults");
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
            let mut file = tokio::fs::File::create(file_path).await?;
            file.write_all(output_content.as_bytes()).await?;
            file.flush().await?;
            println!("📤 Requests exported to {}", file_path);
        }
        None => {
            println!("{}", output_content);
        }
    }
    
    Ok(())
}