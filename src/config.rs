use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub output: OutputConfig,
    pub network: NetworkConfig,
    pub auth: AuthConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub default_timeout: u64,
    pub follow_redirects: bool,
    pub save_history: bool,
    pub max_history_entries: usize,
    pub auto_save_requests: bool,
    pub check_updates: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub default_format: String,
    pub pretty_print: bool,
    pub show_headers: bool,
    pub show_timing: bool,
    pub show_size: bool,
    pub syntax_highlighting: bool,
    pub color_scheme: String,
    pub max_body_length: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub user_agent: String,
    pub proxy_url: Option<String>,
    pub verify_ssl: bool,
    pub connection_timeout: u64,
    pub read_timeout: u64,
    pub max_redirects: u8,
    pub keep_alive: bool,
    pub compression: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub default_auth_type: Option<String>,
    pub stored_tokens: HashMap<String, StoredToken>,
    pub auto_refresh_tokens: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredToken {
    pub token_type: String,
    pub value: String,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub refresh_token: Option<String>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub editor: String,
    pub confirm_dangerous_operations: bool,
    pub show_welcome_message: bool,
    pub auto_complete: bool,
    pub fuzzy_search: bool,
    pub table_style: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                default_timeout: 30,
                follow_redirects: true,
                save_history: true,
                max_history_entries: 1000,
                auto_save_requests: false,
                check_updates: true,
            },
            output: OutputConfig {
                default_format: "auto".to_string(),
                pretty_print: true,
                show_headers: false,
                show_timing: true,
                show_size: true,
                syntax_highlighting: true,
                color_scheme: "dark".to_string(),
                max_body_length: Some(10_000),
            },
            network: NetworkConfig {
                user_agent: format!("terzi/{}", env!("CARGO_PKG_VERSION")),
                proxy_url: None,
                verify_ssl: true,
                connection_timeout: 10,
                read_timeout: 30,
                max_redirects: 10,
                keep_alive: true,
                compression: true,
            },
            auth: AuthConfig {
                default_auth_type: None,
                stored_tokens: HashMap::new(),
                auto_refresh_tokens: true,
            },
            ui: UiConfig {
                theme: "default".to_string(),
                editor: std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string()),
                confirm_dangerous_operations: true,
                show_welcome_message: true,
                auto_complete: true,
                fuzzy_search: true,
                table_style: "rounded".to_string(),
            },
        }
    }
}

impl Config {
    pub async fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if config_path.exists() {
            let mut file = fs::File::open(&config_path).await?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;

            let config: Config = toml::from_str(&contents).unwrap_or_else(|_| Config::default());

            Ok(config)
        } else {
            let config = Config::default();
            config.save().await?;
            Ok(config)
        }
    }

    pub async fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await?;
            }
        }

        let contents = toml::to_string_pretty(self)?;

        let mut file = fs::File::create(&config_path).await?;
        file.write_all(contents.as_bytes()).await?;
        file.flush().await?;

        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        if let Some(config_dir) = dirs::config_dir() {
            Ok(config_dir.join("terzi").join("config.toml"))
        } else if let Some(home_dir) = dirs::home_dir() {
            Ok(home_dir.join(".terzi").join("config.toml"))
        } else {
            Ok(PathBuf::from(".terzi/config.toml"))
        }
    }

    pub async fn get_value(&self, key: &str) -> Option<String> {
        match key {
            "general.default_timeout" => Some(self.general.default_timeout.to_string()),
            "general.follow_redirects" => Some(self.general.follow_redirects.to_string()),
            "general.save_history" => Some(self.general.save_history.to_string()),
            "general.max_history_entries" => Some(self.general.max_history_entries.to_string()),
            "general.auto_save_requests" => Some(self.general.auto_save_requests.to_string()),
            "general.check_updates" => Some(self.general.check_updates.to_string()),

            "output.default_format" => Some(self.output.default_format.clone()),
            "output.pretty_print" => Some(self.output.pretty_print.to_string()),
            "output.show_headers" => Some(self.output.show_headers.to_string()),
            "output.show_timing" => Some(self.output.show_timing.to_string()),
            "output.show_size" => Some(self.output.show_size.to_string()),
            "output.syntax_highlighting" => Some(self.output.syntax_highlighting.to_string()),
            "output.color_scheme" => Some(self.output.color_scheme.clone()),
            "output.max_body_length" => self.output.max_body_length.map(|v| v.to_string()),

            "network.user_agent" => Some(self.network.user_agent.clone()),
            "network.proxy_url" => self.network.proxy_url.clone(),
            "network.verify_ssl" => Some(self.network.verify_ssl.to_string()),
            "network.connection_timeout" => Some(self.network.connection_timeout.to_string()),
            "network.read_timeout" => Some(self.network.read_timeout.to_string()),
            "network.max_redirects" => Some(self.network.max_redirects.to_string()),
            "network.keep_alive" => Some(self.network.keep_alive.to_string()),
            "network.compression" => Some(self.network.compression.to_string()),

            "ui.theme" => Some(self.ui.theme.clone()),
            "ui.editor" => Some(self.ui.editor.clone()),
            "ui.confirm_dangerous_operations" => {
                Some(self.ui.confirm_dangerous_operations.to_string())
            }
            "ui.show_welcome_message" => Some(self.ui.show_welcome_message.to_string()),
            "ui.auto_complete" => Some(self.ui.auto_complete.to_string()),
            "ui.fuzzy_search" => Some(self.ui.fuzzy_search.to_string()),
            "ui.table_style" => Some(self.ui.table_style.clone()),

            _ => None,
        }
    }

    pub async fn set_value(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "general.default_timeout" => {
                self.general.default_timeout = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid timeout value"))?;
            }
            "general.follow_redirects" => {
                self.general.follow_redirects = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }
            "general.save_history" => {
                self.general.save_history = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }
            "general.max_history_entries" => {
                self.general.max_history_entries = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid number value"))?;
            }
            "general.auto_save_requests" => {
                self.general.auto_save_requests = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }
            "general.check_updates" => {
                self.general.check_updates = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }

            "output.default_format" => {
                let valid_formats = ["auto", "json", "yaml", "table", "raw"];
                if valid_formats.contains(&value) {
                    self.output.default_format = value.to_string();
                } else {
                    return Err(anyhow::anyhow!(
                        "Invalid format. Valid options: {}",
                        valid_formats.join(", ")
                    ));
                }
            }
            "output.pretty_print" => {
                self.output.pretty_print = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }
            "output.show_headers" => {
                self.output.show_headers = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }
            "output.show_timing" => {
                self.output.show_timing = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }
            "output.show_size" => {
                self.output.show_size = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }
            "output.syntax_highlighting" => {
                self.output.syntax_highlighting = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }
            "output.color_scheme" => {
                let valid_schemes = ["dark", "light", "auto"];
                if valid_schemes.contains(&value) {
                    self.output.color_scheme = value.to_string();
                } else {
                    return Err(anyhow::anyhow!(
                        "Invalid color scheme. Valid options: {}",
                        valid_schemes.join(", ")
                    ));
                }
            }
            "output.max_body_length" => {
                if value == "none" || value.is_empty() {
                    self.output.max_body_length = None;
                } else {
                    self.output.max_body_length = Some(
                        value
                            .parse()
                            .map_err(|_| anyhow::anyhow!("Invalid number value"))?,
                    );
                }
            }

            "network.user_agent" => {
                self.network.user_agent = value.to_string();
            }
            "network.proxy_url" => {
                if value.is_empty() || value == "none" {
                    self.network.proxy_url = None;
                } else {
                    // Validate URL
                    url::Url::parse(value).map_err(|_| anyhow::anyhow!("Invalid proxy URL"))?;
                    self.network.proxy_url = Some(value.to_string());
                }
            }
            "network.verify_ssl" => {
                self.network.verify_ssl = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }
            "network.connection_timeout" => {
                self.network.connection_timeout = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid timeout value"))?;
            }
            "network.read_timeout" => {
                self.network.read_timeout = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid timeout value"))?;
            }
            "network.max_redirects" => {
                self.network.max_redirects = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid number value"))?;
            }
            "network.keep_alive" => {
                self.network.keep_alive = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }
            "network.compression" => {
                self.network.compression = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }

            "ui.theme" => {
                let valid_themes = ["default", "dark", "light", "minimal"];
                if valid_themes.contains(&value) {
                    self.ui.theme = value.to_string();
                } else {
                    return Err(anyhow::anyhow!(
                        "Invalid theme. Valid options: {}",
                        valid_themes.join(", ")
                    ));
                }
            }
            "ui.editor" => {
                self.ui.editor = value.to_string();
            }
            "ui.confirm_dangerous_operations" => {
                self.ui.confirm_dangerous_operations = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }
            "ui.show_welcome_message" => {
                self.ui.show_welcome_message = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }
            "ui.auto_complete" => {
                self.ui.auto_complete = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }
            "ui.fuzzy_search" => {
                self.ui.fuzzy_search = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value"))?;
            }
            "ui.table_style" => {
                let valid_styles = ["ascii", "rounded", "modern", "minimal"];
                if valid_styles.contains(&value) {
                    self.ui.table_style = value.to_string();
                } else {
                    return Err(anyhow::anyhow!(
                        "Invalid table style. Valid options: {}",
                        valid_styles.join(", ")
                    ));
                }
            }

            _ => return Err(anyhow::anyhow!("Unknown configuration key: {}", key)),
        }

        self.save().await?;
        Ok(())
    }

    pub async fn reset_to_defaults(&mut self) -> Result<()> {
        *self = Config::default();
        self.save().await?;
        Ok(())
    }

    pub fn list_all_keys() -> Vec<&'static str> {
        vec![
            "general.default_timeout",
            "general.follow_redirects",
            "general.save_history",
            "general.max_history_entries",
            "general.auto_save_requests",
            "general.check_updates",
            "output.default_format",
            "output.pretty_print",
            "output.show_headers",
            "output.show_timing",
            "output.show_size",
            "output.syntax_highlighting",
            "output.color_scheme",
            "output.max_body_length",
            "network.user_agent",
            "network.proxy_url",
            "network.verify_ssl",
            "network.connection_timeout",
            "network.read_timeout",
            "network.max_redirects",
            "network.keep_alive",
            "network.compression",
            "ui.theme",
            "ui.editor",
            "ui.confirm_dangerous_operations",
            "ui.show_welcome_message",
            "ui.auto_complete",
            "ui.fuzzy_search",
            "ui.table_style",
        ]
    }

    // Token management
    pub async fn save_token(&mut self, name: &str, token: StoredToken) -> Result<()> {
        self.auth.stored_tokens.insert(name.to_string(), token);
        self.save().await?;
        Ok(())
    }

    pub fn get_token(&self, name: &str) -> Option<&StoredToken> {
        self.auth.stored_tokens.get(name)
    }

    pub async fn delete_token(&mut self, name: &str) -> Result<bool> {
        let removed = self.auth.stored_tokens.remove(name).is_some();
        if removed {
            self.save().await?;
        }
        Ok(removed)
    }

    pub fn list_tokens(&self) -> Vec<String> {
        self.auth.stored_tokens.keys().cloned().collect()
    }

    // Validation helpers
    pub fn validate(&self) -> Result<()> {
        // Validate timeouts
        if self.general.default_timeout == 0 || self.general.default_timeout > 3600 {
            return Err(anyhow::anyhow!(
                "Default timeout must be between 1 and 3600 seconds"
            ));
        }

        if self.network.connection_timeout == 0 || self.network.connection_timeout > 300 {
            return Err(anyhow::anyhow!(
                "Connection timeout must be between 1 and 300 seconds"
            ));
        }

        if self.network.read_timeout == 0 || self.network.read_timeout > 3600 {
            return Err(anyhow::anyhow!(
                "Read timeout must be between 1 and 3600 seconds"
            ));
        }

        // Validate max redirects
        if self.network.max_redirects > 50 {
            return Err(anyhow::anyhow!("Max redirects cannot exceed 50"));
        }

        // Validate history entries
        if self.general.max_history_entries == 0 || self.general.max_history_entries > 10000 {
            return Err(anyhow::anyhow!(
                "Max history entries must be between 1 and 10000"
            ));
        }

        // Validate proxy URL if set
        if let Some(ref proxy_url) = self.network.proxy_url {
            url::Url::parse(proxy_url)
                .map_err(|_| anyhow::anyhow!("Invalid proxy URL: {}", proxy_url))?;
        }

        Ok(())
    }

    pub fn get_color_scheme(&self) -> &str {
        &self.output.color_scheme
    }

    pub fn should_use_colors(&self) -> bool {
        match self.output.color_scheme.as_str() {
            "auto" => atty::is(atty::Stream::Stdout),
            "dark" | "light" => true,
            _ => false,
        }
    }
}
