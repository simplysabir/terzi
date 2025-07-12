use anyhow::Result;
use clap::Parser;
use colored::*;
use dialoguer::{Confirm, FuzzySelect, Input, MultiSelect, Select, theme::ColorfulTheme};
use std::collections::HashMap;

use crate::client::TerziClient;
use crate::output::ResponseFormatter;
use crate::request::{RequestBuilder, SavedRequest};
use crate::storage::Storage;

pub struct InteractiveMode {
    client: TerziClient,
    storage: Storage,
    formatter: ResponseFormatter,
}

impl InteractiveMode {
    pub fn new(client: TerziClient, storage: Storage, formatter: ResponseFormatter) -> Self {
        Self {
            client,
            storage,
            formatter,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!(
            "{}",
            "🚀 Welcome to Terzi Interactive Mode!".bright_cyan().bold()
        );
        println!(
            "{}",
            "Let's build and test your API requests step by step.".bright_white()
        );
        println!();

        loop {
            match self.main_menu().await {
                Ok(true) => continue,
                Ok(false) => break,
                Err(e) => {
                    self.formatter.display_error(&format!("Error: {}", e));
                    continue;
                }
            }
        }

        println!("{}", "👋 Goodbye! Happy API hunting!".bright_green());
        Ok(())
    }

    async fn main_menu(&mut self) -> Result<bool> {
        let options = vec![
            "🚀 Create New Request",
            "📋 Load Saved Request",
            "📚 Browse Request Collection",
            "🔍 Search History",
            "⚙️  Settings",
            "🚪 Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()?;

        match selection {
            0 => self.create_new_request().await?,
            1 => self.load_saved_request().await?,
            2 => self.browse_collection().await?,
            3 => self.search_history().await?,
            4 => self.settings_menu().await?,
            5 => return Ok(false),
            _ => unreachable!(),
        }

        Ok(true)
    }

    async fn create_new_request(&mut self) -> Result<()> {
        println!("{}", "\n🛠️  Creating New Request".bright_cyan().bold());

        // Get URL
        let url: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter the URL")
            .with_initial_text("https://")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.starts_with("http://") || input.starts_with("https://") {
                    Ok(())
                } else {
                    Err("Please enter a valid URL starting with http:// or https://")
                }
            })
            .interact_text()?;

        // Get HTTP method
        let methods = vec!["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];
        let method_index = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select HTTP method")
            .items(&methods)
            .default(0)
            .interact()?;
        let method = methods[method_index];

        // Create request builder
        let mut builder = RequestBuilder::new(&url, method)?;

        // Add headers
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Add custom headers?")
            .default(false)
            .interact()?
        {
            builder = self.add_headers(builder).await?;
        }

        // Add authentication
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Add authentication?")
            .default(false)
            .interact()?
        {
            builder = self.add_authentication(builder).await?;
        }

        // Add body for non-GET requests
        if method != "GET" && method != "HEAD" {
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Add request body?")
                .default(false)
                .interact()?
            {
                builder = self.add_body(builder).await?;
            }
        }

        // Build the request
        let request = builder.build();

        // Preview request
        self.preview_request(&request);

        // Confirm execution
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Execute this request?")
            .default(true)
            .interact()?
        {
            // Execute request
            println!("{}", "🚀 Executing request...".bright_blue());

            match self.client.execute_request(&request).await {
                Ok(response) => {
                    // Add to history
                    self.storage.add_to_history(&request, &response).await?;

                    // Display response
                    println!();
                    let cli = crate::Cli::parse(); // Default CLI for formatting
                    self.formatter.display_response(&response, &cli).await?;

                    // Ask to save request
                    if Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Save this request for future use?")
                        .default(false)
                        .interact()?
                    {
                        self.save_request_interactive(request).await?;
                    }
                }
                Err(e) => {
                    self.formatter
                        .display_error(&format!("Request failed: {}", e));
                }
            }
        }

        Ok(())
    }

    async fn add_headers(&mut self, mut builder: RequestBuilder) -> Result<RequestBuilder> {
        loop {
            let header_name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Header name (or press Enter to finish)")
                .allow_empty(true)
                .interact_text()?;

            if header_name.is_empty() {
                break;
            }

            let header_value: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(&format!("Value for '{}'", header_name))
                .interact_text()?;

            builder = builder.header(&header_name, &header_value);

            if !Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Add another header?")
                .default(false)
                .interact()?
            {
                break;
            }
        }

        Ok(builder)
    }

    async fn add_authentication(&mut self, mut builder: RequestBuilder) -> Result<RequestBuilder> {
        let auth_types = vec![
            "Bearer Token",
            "Basic Auth",
            "API Key (Header)",
            "API Key (Query)",
        ];

        let auth_type = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select authentication type")
            .items(&auth_types)
            .interact()?;

        match auth_type {
            0 => {
                // Bearer Token
                let token: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter bearer token")
                    .interact_text()?;
                builder = builder.auth(&format!("bearer:{}", token))?;
            }
            1 => {
                // Basic Auth
                let username: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Username")
                    .interact_text()?;
                let password: String = dialoguer::Password::with_theme(&ColorfulTheme::default())
                    .with_prompt("Password")
                    .interact()?;
                builder = builder.auth(&format!("basic:{}:{}", username, password))?;
            }
            2 => {
                // API Key Header
                let key_name: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Header name")
                    .with_initial_text("X-API-Key")
                    .interact_text()?;
                let key_value: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("API key value")
                    .interact_text()?;
                builder = builder.header(&key_name, &key_value);
            }
            3 => {
                // API Key Query
                let key_name: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Query parameter name")
                    .with_initial_text("api_key")
                    .interact_text()?;
                let key_value: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("API key value")
                    .interact_text()?;
                // Note: This would need to be implemented in RequestBuilder
                println!(
                    "{}",
                    "Query parameter auth will be added to URL".bright_blue()
                );
            }
            _ => unreachable!(),
        }

        Ok(builder)
    }

    async fn add_body(&mut self, mut builder: RequestBuilder) -> Result<RequestBuilder> {
        let body_types = vec!["JSON", "Form Data", "Raw Text", "File Upload"];

        let body_type = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select body type")
            .items(&body_types)
            .interact()?;

        match body_type {
            0 => {
                // JSON
                println!(
                    "{}",
                    "Enter JSON data (press Ctrl+D when finished):".bright_blue()
                );
                let json_body: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("JSON")
                    .with_initial_text("{}")
                    .interact_text()?;

                // Validate JSON
                match serde_json::from_str::<serde_json::Value>(&json_body) {
                    Ok(_) => {
                        builder = builder.json_body(&json_body)?;
                        self.formatter
                            .display_success("Valid JSON added to request");
                    }
                    Err(e) => {
                        self.formatter
                            .display_warning(&format!("Invalid JSON: {}", e));
                        builder = builder.raw_body(&json_body);
                    }
                }
            }
            1 => {
                // Form Data
                let mut form_data = HashMap::new();

                loop {
                    let key: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Form field name (or press Enter to finish)")
                        .allow_empty(true)
                        .interact_text()?;

                    if key.is_empty() {
                        break;
                    }

                    let value: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt(&format!("Value for '{}'", key))
                        .interact_text()?;

                    form_data.insert(key, value);

                    if !Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Add another field?")
                        .default(false)
                        .interact()?
                    {
                        break;
                    }
                }

                builder = builder.form_body(form_data)?;
            }
            2 => {
                // Raw Text
                let raw_body: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter raw body content")
                    .interact_text()?;
                builder = builder.raw_body(&raw_body);
            }
            3 => {
                // File Upload
                let file_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter file path")
                    .interact_text()?;

                // This would need file upload implementation in RequestBuilder
                self.formatter
                    .display_info("File upload feature coming soon!");
            }
            _ => unreachable!(),
        }

        Ok(builder)
    }

    fn preview_request(&self, request: &SavedRequest) {
        println!("{}", "\n📋 Request Preview".bright_cyan().bold());
        println!("{}  {}", "URL:".bright_blue().bold(), request.url);
        println!("{}  {}", "Method:".bright_blue().bold(), request.method);

        if !request.headers.is_empty() {
            println!("{}:", "Headers".bright_blue().bold());
            for (key, value) in &request.headers {
                println!("  {}: {}", key.bright_green(), value);
            }
        }

        if let Some(ref body) = request.body {
            println!("{}:", "Body".bright_blue().bold());
            println!("  {}", body);
        }
        println!();
    }

    async fn save_request_interactive(&mut self, mut request: SavedRequest) -> Result<()> {
        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter a name for this request")
            .interact_text()?;

        request.name = name.clone();

        self.storage.save_request(&name, &request).await?;
        self.formatter
            .display_success(&format!("Request saved as '{}'", name));

        Ok(())
    }

    async fn load_saved_request(&mut self) -> Result<()> {
        let requests = self.storage.list_requests(None).await?;

        if requests.is_empty() {
            self.formatter
                .display_info("No saved requests found. Create one first!");
            return Ok(());
        }

        let request_names: Vec<String> = requests.iter().map(|r| r.name.clone()).collect();

        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a request to load")
            .items(&request_names)
            .interact()?;

        let selected_request = &requests[selection];

        // Preview the request
        self.preview_request(selected_request);

        // Confirm execution
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Execute this request?")
            .default(true)
            .interact()?
        {
            println!("{}", "🚀 Executing request...".bright_blue());

            match self.client.execute_request(selected_request).await {
                Ok(response) => {
                    self.storage
                        .add_to_history(selected_request, &response)
                        .await?;

                    println!();
                    let cli = crate::Cli::parse();
                    self.formatter.display_response(&response, &cli).await?;
                }
                Err(e) => {
                    self.formatter
                        .display_error(&format!("Request failed: {}", e));
                }
            }
        }

        Ok(())
    }

    async fn browse_collection(&mut self) -> Result<()> {
        let requests = self.storage.list_requests(None).await?;

        if requests.is_empty() {
            self.formatter.display_info("No saved requests found.");
            return Ok(());
        }

        // Display requests in a nice table format
        let headers = vec!["#", "Name", "Method", "URL", "Created"];
        let rows: Vec<Vec<String>> = requests
            .iter()
            .enumerate()
            .map(|(i, req)| {
                vec![
                    (i + 1).to_string(),
                    req.name.clone(),
                    req.method.clone(),
                    req.url.clone(),
                    req.created_at.format("%Y-%m-%d %H:%M").to_string(),
                ]
            })
            .collect();

        let table = crate::utils::create_url_priority_table(headers, rows, 3); // URL is column index 3
        println!("{}", table);

        // Allow user to select and view details
        let actions = vec![
            "View Request Details",
            "Execute Request",
            "Edit Request",
            "Delete Request",
            "Back to Main Menu",
        ];

        let action = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .items(&actions)
            .interact()?;

        match action {
            0..=3 => {
                let request_names: Vec<String> = requests.iter().map(|r| r.name.clone()).collect();
                let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a request")
                    .items(&request_names)
                    .interact()?;

                match action {
                    0 => self.preview_request(&requests[selection]),
                    1 => match self.client.execute_request(&requests[selection]).await {
                        Ok(response) => {
                            let cli = crate::Cli::parse();
                            self.formatter.display_response(&response, &cli).await?;
                        }
                        Err(e) => self
                            .formatter
                            .display_error(&format!("Request failed: {}", e)),
                    },
                    2 => {
                        let mut request = requests[selection].clone();
                        self.edit_request(&mut request).await?;
                    }
                    3 => {
                        if Confirm::with_theme(&ColorfulTheme::default())
                            .with_prompt(&format!("Delete request '{}'?", requests[selection].name))
                            .default(false)
                            .interact()?
                        {
                            self.storage
                                .delete_request(&requests[selection].name)
                                .await?;
                            self.formatter.display_success("Request deleted");
                        }
                    }
                    _ => unreachable!(),
                }
            }
            4 => return Ok(()),
            _ => unreachable!(),
        }

        Ok(())
    }

    pub async fn edit_request(&mut self, request: &mut SavedRequest) -> Result<()> {
        println!(
            "{}",
            format!("\n✏️  Editing Request: {}", request.name)
                .bright_cyan()
                .bold()
        );

        // Show current request
        self.preview_request(request);

        let edit_options = vec![
            "Change URL",
            "Change Method",
            "Edit Headers",
            "Edit Body",
            "Save Changes",
            "Cancel",
        ];

        loop {
            let action = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What would you like to edit?")
                .items(&edit_options)
                .interact()?;

            match action {
                0 => {
                    let new_url: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter new URL")
                        .with_initial_text(&request.url)
                        .interact_text()?;
                    request.url = new_url;
                }
                1 => {
                    let methods = vec!["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];
                    let method_index = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select HTTP method")
                        .items(&methods)
                        .interact()?;
                    request.method = methods[method_index].to_string();
                }
                2 => {
                    // Edit headers - simplified for now
                    self.formatter
                        .display_info("Header editing will be implemented in next version");
                }
                3 => {
                    let new_body: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter new body (or leave empty to remove)")
                        .with_initial_text(request.body.as_deref().unwrap_or(""))
                        .allow_empty(true)
                        .interact_text()?;

                    request.body = if new_body.is_empty() {
                        None
                    } else {
                        Some(new_body)
                    };
                }
                4 => {
                    self.storage.save_request(&request.name, request).await?;
                    self.formatter
                        .display_success("Request updated successfully");
                    break;
                }
                5 => break,
                _ => unreachable!(),
            }

            // Show updated request
            self.preview_request(request);
        }

        Ok(())
    }

    async fn search_history(&mut self) -> Result<()> {
        let history = self.storage.get_history(50).await?;

        if history.is_empty() {
            self.formatter.display_info("No request history found.");
            return Ok(());
        }

        // Display history
        let headers = vec!["#", "Time", "Method", "URL", "Status"];
        let rows: Vec<Vec<String>> = history
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let status_display = match entry.response_status {
                    Some(status) => format!(
                        "{} {}",
                        if status >= 200 && status < 300 {
                            "🟢"
                        } else if status >= 400 {
                            "🔴"
                        } else {
                            "🟡"
                        },
                        status
                    ),
                    None => "❌ Error".to_string(),
                };

                vec![
                    (i + 1).to_string(),
                    entry.timestamp.format("%H:%M:%S").to_string(),
                    entry.method.clone(),
                    entry.url.clone(),
                    status_display,
                ]
            })
            .collect();

        let table = crate::utils::create_url_priority_table(headers, rows, 3); // URL is column index 3
        println!("{}", table);

        Ok(())
    }

    async fn settings_menu(&mut self) -> Result<()> {
        let settings_options = vec![
            "View Current Settings",
            "Change Default Timeout",
            "Change Output Format",
            "Reset to Defaults",
            "Back to Main Menu",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Settings")
            .items(&settings_options)
            .interact()?;

        match selection {
            0 => self
                .formatter
                .display_info("Settings display not yet implemented"),
            1 => self
                .formatter
                .display_info("Timeout setting not yet implemented"),
            2 => self
                .formatter
                .display_info("Output format setting not yet implemented"),
            3 => self
                .formatter
                .display_info("Reset settings not yet implemented"),
            4 => return Ok(()),
            _ => unreachable!(),
        }

        Ok(())
    }
}
