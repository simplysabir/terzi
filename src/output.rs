use anyhow::Result;
use colored::*;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::*;
use serde_json::Value;
use std::collections::HashMap;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};

use crate::Cli;
use crate::client::Response;
use crate::config::Config;

pub struct ResponseFormatter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    config: Config,
}

impl ResponseFormatter {
    pub fn new(config: &Config) -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            config: config.clone(),
        }
    }

    pub async fn display_response(&self, response: &Response, cli: &Cli) -> Result<()> {
        // Print status line
        self.print_status_line(response);

        // Print headers if requested
        if cli.include_headers {
            self.print_headers(&response.headers);
        }

        // Print body based on format
        match cli.output.as_str() {
            "json" => self.print_json_body(&response.body, cli.pretty),
            "yaml" => self.print_yaml_body(&response.body),
            "table" => self.print_table_body(&response.body),
            "raw" => self.print_raw_body(&response.body),
            _ => self.print_auto_body(response, cli.pretty),
        }

        // Print footer with timing info
        if cli.verbose {
            self.print_footer(response);
        }

        Ok(())
    }

    fn print_status_line(&self, response: &Response) {
        let status_color = match response.status {
            200..=299 => "bright_green",
            300..=399 => "bright_yellow",
            400..=499 => "bright_red",
            500..=599 => "red",
            _ => "white",
        };

        println!(
            "{} {} {} {} {}",
            response.status_emoji(),
            response.method.bright_blue().bold(),
            response.url.bright_cyan().underline(),
            response.status.to_string().color(status_color).bold(),
            format!("({})", response.duration_human())
                .bright_black()
                .italic()
        );
    }

    fn print_headers(&self, headers: &HashMap<String, String>) {
        if !headers.is_empty() {
            println!("{}", "Headers:".bright_yellow().bold());
            let mut table = Table::new();
            table.load_preset(UTF8_FULL_CONDENSED);
            table.set_header(vec!["Name", "Value"]);

            for (key, value) in headers {
                table.add_row(vec![key.bright_blue().to_string(), value.to_string()]);
            }

            println!("{}", table);
            println!();
        }
    }

    fn print_json_body(&self, body: &str, pretty: bool) {
        if body.is_empty() {
            println!("{}", "No response body".bright_black());
            return;
        }

        // Validate JSON first, then format appropriately
        if crate::utils::is_valid_json(body) {
            let formatted = if pretty {
                // Use utils prettify_json for better formatting
                match crate::utils::prettify_json(body) {
                    Ok(formatted) => formatted,
                    Err(_) => body.to_string(),
                }
            } else {
                // Use minified JSON for compact display
                match crate::utils::minify_json(body) {
                    Ok(minified) => minified,
                    Err(_) => body.to_string(),
                }
            };
            self.highlight_and_print(&formatted, "json");
        } else {
            println!("{}", "Invalid JSON response:".bright_red());
            self.print_raw_body(body);
        }
    }

    fn pretty_format_json(&self, json_str: &str) -> Result<String> {
        // Simple pretty formatting that preserves field order
        let mut formatted = String::new();
        let mut indent_level = 0;
        let mut in_string = false;
        let mut escape_next = false;
        let chars: Vec<char> = json_str.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            if escape_next {
                formatted.push(ch);
                escape_next = false;
                continue;
            }

            match ch {
                '"' if !escape_next => {
                    in_string = !in_string;
                    formatted.push(ch);
                }
                '\\' if in_string => {
                    escape_next = true;
                    formatted.push(ch);
                }
                '{' | '[' if !in_string => {
                    formatted.push(ch);
                    indent_level += 1;
                    if i + 1 < chars.len() && chars[i + 1] != '}' && chars[i + 1] != ']' {
                        formatted.push('\n');
                        formatted.push_str(&"  ".repeat(indent_level));
                    }
                }
                '}' | ']' if !in_string => {
                    if chars.get(i.saturating_sub(1)) != Some(&'{')
                        && chars.get(i.saturating_sub(1)) != Some(&'[')
                    {
                        formatted.push('\n');
                        indent_level = indent_level.saturating_sub(1);
                        formatted.push_str(&"  ".repeat(indent_level));
                    } else {
                        indent_level = indent_level.saturating_sub(1);
                    }
                    formatted.push(ch);
                }
                ',' if !in_string => {
                    formatted.push(ch);
                    formatted.push('\n');
                    formatted.push_str(&"  ".repeat(indent_level));
                }
                ':' if !in_string => {
                    formatted.push(ch);
                    formatted.push(' ');
                }
                _ if ch.is_whitespace() && !in_string => {
                    // Skip existing whitespace outside strings
                }
                _ => {
                    formatted.push(ch);
                }
            }
        }

        Ok(formatted)
    }

    fn print_yaml_body(&self, body: &str) {
        if body.is_empty() {
            println!("{}", "No response body".bright_black());
            return;
        }

        // Try to parse as JSON first, then convert to YAML
        match serde_json::from_str::<Value>(body) {
            Ok(json) => match serde_yaml::to_string(&json) {
                Ok(yaml) => self.highlight_and_print(&yaml, "yaml"),
                Err(_) => self.print_raw_body(body),
            },
            Err(_) => {
                // Assume it's already YAML
                self.highlight_and_print(body, "yaml");
            }
        }
    }

    fn print_table_body(&self, body: &str) {
        if body.is_empty() {
            println!("{}", "No response body".bright_black());
            return;
        }

        match serde_json::from_str::<Value>(body) {
            Ok(Value::Array(arr)) => {
                if arr.is_empty() {
                    println!("{}", "Empty array".bright_black());
                    return;
                }

                let mut table = Table::new();
                table.load_preset(UTF8_FULL_CONDENSED);

                // Get headers from first object
                if let Some(Value::Object(first_obj)) = arr.first() {
                    let headers: Vec<String> = first_obj.keys().cloned().collect();
                    table.set_header(headers.clone());

                    // Add rows
                    for item in &arr {
                        if let Value::Object(obj) = item {
                            let row: Vec<String> = headers
                                .iter()
                                .map(|h| {
                                    obj.get(h)
                                        .map(|v| self.value_to_string(v))
                                        .unwrap_or_else(|| "".to_string())
                                })
                                .collect();
                            table.add_row(row);
                        }
                    }

                    println!("{}", table);
                } else {
                    println!(
                        "{}",
                        "Cannot create table from non-object array".bright_red()
                    );
                    self.print_json_body(body, true);
                }
            }
            Ok(Value::Object(obj)) => {
                let mut table = Table::new();
                table.load_preset(UTF8_FULL_CONDENSED);
                table.set_header(vec!["Key", "Value"]);

                for (key, value) in &obj {
                    table.add_row(vec![
                        key.bright_blue().to_string(),
                        self.value_to_string(value),
                    ]);
                }

                println!("{}", table);
            }
            _ => {
                println!(
                    "{}",
                    "Cannot create table from this response type".bright_red()
                );
                self.print_json_body(body, true);
            }
        }
    }

    fn print_raw_body(&self, body: &str) {
        if body.is_empty() {
            println!("{}", "No response body".bright_black());
        } else {
            println!("{}", body);
        }
    }

    fn print_auto_body(&self, response: &Response, pretty: bool) {
        if response.is_json() {
            self.print_json_body(&response.body, pretty);
        } else if response.is_xml() {
            self.highlight_and_print(&response.body, "xml");
        } else if response.is_html() {
            self.highlight_and_print(&response.body, "html");
        } else {
            self.print_raw_body(&response.body);
        }
    }

    fn highlight_and_print(&self, content: &str, syntax: &str) {
        let syntax_ref = self
            .syntax_set
            .find_syntax_by_extension(syntax)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["base16-ocean.dark"];
        let mut highlighter = HighlightLines::new(syntax_ref, theme);

        for line in LinesWithEndings::from(content) {
            let ranges: Vec<(Style, &str)> =
                highlighter.highlight_line(line, &self.syntax_set).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            print!("{}", escaped);
        }
        println!();
    }

    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".bright_black().to_string(),
            Value::Array(_) => "[Array]".bright_magenta().to_string(),
            Value::Object(_) => "{Object}".bright_magenta().to_string(),
        }
    }

    fn print_footer(&self, response: &Response) {
        println!();
        println!("{}", "Response Info:".bright_yellow().bold());

        let mut info_table = Table::new();
        info_table.load_preset(UTF8_FULL_CONDENSED);

        info_table.add_row(vec![
            "Duration".bright_blue().to_string(),
            response.duration_human(),
        ]);

        info_table.add_row(vec![
            "Size".bright_blue().to_string(),
            response.size_human(),
        ]);

        if let Some(content_type) = response.content_type() {
            info_table.add_row(vec![
                "Content-Type".bright_blue().to_string(),
                content_type.clone(),
            ]);
        }

        println!("{}", info_table);
    }

    pub fn display_error(&self, error: &str) {
        println!("{} {}", "❌ Error:".bright_red().bold(), error);
    }

    pub fn display_success(&self, message: &str) {
        println!("{} {}", "✅".green(), message);
    }

    pub fn display_info(&self, message: &str) {
        println!("{} {}", "ℹ️".blue(), message);
    }

    pub fn display_warning(&self, message: &str) {
        println!("{} {}", "⚠️".yellow(), message);
    }
}
