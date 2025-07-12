use anyhow::Result;
use std::time::{Duration, SystemTime};
use url::Url;

// URL utilities
pub fn normalize_url(url: &str) -> Result<String> {
    let mut parsed = Url::parse(url)?;

    // Remove default ports
    if let Some(port) = parsed.port() {
        let default_port = match parsed.scheme() {
            "http" => Some(80),
            "https" => Some(443),
            _ => None,
        };

        if Some(port) == default_port {
            let _ = parsed.set_port(None);
        }
    }

    // Remove trailing slash for paths
    if parsed.path() == "/" {
        parsed.set_path("");
    }

    Ok(parsed.to_string())
}

pub fn extract_domain(url: &str) -> Result<String> {
    let parsed = Url::parse(url)?;
    Ok(parsed.host_str().unwrap_or("").to_string())
}

pub fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}

// Time utilities
pub fn format_duration(duration: Duration) -> String {
    let total_ms = duration.as_millis();

    if total_ms < 1000 {
        format!("{}ms", total_ms)
    } else if total_ms < 60_000 {
        format!("{:.2}s", duration.as_secs_f64())
    } else {
        let minutes = total_ms / 60_000;
        let seconds = (total_ms % 60_000) / 1000;
        format!("{}m {}s", minutes, seconds)
    }
}

pub fn format_timestamp(timestamp: SystemTime) -> String {
    let datetime: chrono::DateTime<chrono::Utc> = timestamp.into();
    datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

pub fn time_ago(timestamp: SystemTime) -> String {
    let now = SystemTime::now();
    if let Ok(duration) = now.duration_since(timestamp) {
        let seconds = duration.as_secs();

        if seconds < 60 {
            format!("{}s ago", seconds)
        } else if seconds < 3600 {
            format!("{}m ago", seconds / 60)
        } else if seconds < 86400 {
            format!("{}h ago", seconds / 3600)
        } else {
            format!("{}d ago", seconds / 86400)
        }
    } else {
        "in the future".to_string()
    }
}

// String utilities
pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length.saturating_sub(3)])
    }
}

pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c => c,
        })
        .collect()
}

pub fn format_bytes(bytes: usize) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

// JSON utilities
pub fn prettify_json(json: &str) -> Result<String> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    Ok(serde_json::to_string_pretty(&value)?)
}

pub fn minify_json(json: &str) -> Result<String> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    Ok(serde_json::to_string(&value)?)
}

pub fn is_valid_json(json: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(json).is_ok()
}

// HTTP utilities
pub fn parse_content_type(
    content_type: &str,
) -> (String, std::collections::HashMap<String, String>) {
    let mut parts = content_type.split(';');
    let media_type = parts.next().unwrap_or("").trim().to_lowercase();

    let mut parameters = std::collections::HashMap::new();
    for part in parts {
        if let Some((key, value)) = part.split_once('=') {
            let key = key.trim().to_lowercase();
            let value = value.trim().trim_matches('"');
            parameters.insert(key, value.to_string());
        }
    }

    (media_type, parameters)
}

pub fn guess_content_type(body: &str) -> &'static str {
    let trimmed = body.trim();

    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        if is_valid_json(trimmed) {
            return "application/json";
        }
    }

    if trimmed.starts_with('<') {
        return "application/xml";
    }

    if trimmed.contains('=') && !trimmed.contains('\n') {
        return "application/x-www-form-urlencoded";
    }

    "text/plain"
}

// File utilities
pub fn get_file_extension(filename: &str) -> Option<&str> {
    std::path::Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
}

pub fn read_file_to_string(path: &std::path::Path) -> Result<String> {
    std::fs::read_to_string(path).map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))
}

// Environment variable utilities
pub fn get_env_or_default(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

pub fn expand_env_vars(text: &str) -> String {
    let mut result = text.to_string();

    // Simple environment variable expansion for ${VAR} format
    while let Some(start) = result.find("${") {
        if let Some(end) = result[start..].find('}') {
            let var_name = &result[start + 2..start + end];
            let var_value = std::env::var(var_name).unwrap_or_default();
            result.replace_range(start..start + end + 1, &var_value);
        } else {
            break;
        }
    }

    result
}

// Terminal utilities
pub fn get_terminal_width() -> usize {
    if let Some((w, _)) = term_size::dimensions() {
        w
    } else {
        80 // Default width
    }
}

pub fn is_tty() -> bool {
    atty::is(atty::Stream::Stdout)
}

// Validation utilities
pub fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.split('@').count() == 2
}

pub fn is_valid_header_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii() && c != ':' && c != '\n' && c != '\r')
}

pub fn is_valid_header_value(value: &str) -> bool {
    !value.chars().any(|c| c == '\n' || c == '\r')
}

// Security utilities
pub fn mask_sensitive_data(text: &str, patterns: &[&str]) -> String {
    let mut result = text.to_string();

    for pattern in patterns {
        if let Ok(regex) = regex::Regex::new(pattern) {
            result = regex
                .replace_all(&result, |caps: &regex::Captures| {
                    let full_match = caps.get(0).unwrap().as_str();
                    if full_match.len() <= 4 {
                        "*".repeat(full_match.len())
                    } else {
                        format!(
                            "{}****{}",
                            &full_match[..2],
                            &full_match[full_match.len() - 2..]
                        )
                    }
                })
                .to_string();
        }
    }

    result
}

pub fn generate_request_id() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

// Collection utilities
pub fn merge_headers(
    base: &std::collections::HashMap<String, String>,
    override_headers: &std::collections::HashMap<String, String>,
) -> std::collections::HashMap<String, String> {
    let mut result = base.clone();
    result.extend(override_headers.clone());
    result
}

pub fn filter_headers(
    headers: &std::collections::HashMap<String, String>,
    exclude_patterns: &[&str],
) -> std::collections::HashMap<String, String> {
    headers
        .iter()
        .filter(|(key, _)| {
            let key_lower = key.to_lowercase();
            !exclude_patterns
                .iter()
                .any(|pattern| key_lower.contains(&pattern.to_lowercase()))
        })
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

// Template utilities
pub fn extract_template_variables(text: &str) -> Vec<String> {
    let mut variables = Vec::new();
    let mut start = 0;

    while let Some(pos) = text[start..].find("{{") {
        let abs_start = start + pos;
        if let Some(end_pos) = text[abs_start..].find("}}") {
            let var_name = &text[abs_start + 2..abs_start + end_pos];
            if !var_name.trim().is_empty() {
                variables.push(var_name.trim().to_string());
            }
            start = abs_start + end_pos + 2;
        } else {
            break;
        }
    }

    variables.sort();
    variables.dedup();
    variables
}

// Error formatting
pub fn format_error_chain(error: &anyhow::Error) -> String {
    let mut messages = Vec::new();

    messages.push(error.to_string());
    let mut source = error.source();

    while let Some(err) = source {
        messages.push(err.to_string());
        source = err.source();
    }

    messages.join(" → ")
}

// Performance utilities
pub struct Timer {
    start: std::time::Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn elapsed_ms(&self) -> u128 {
        self.elapsed().as_millis()
    }

    pub fn restart(&mut self) -> Duration {
        let elapsed = self.elapsed();
        self.start = std::time::Instant::now();
        elapsed
    }
}

// Diff utilities for comparing responses
pub fn diff_json(old: &str, new: &str) -> Result<String> {
    let old_value: serde_json::Value = serde_json::from_str(old)?;
    let new_value: serde_json::Value = serde_json::from_str(new)?;

    if old_value == new_value {
        Ok("No differences found".to_string())
    } else {
        // Simple diff - in a real implementation you'd use a proper diff library
        Ok(format!(
            "Values differ:\nOld: {}\nNew: {}",
            serde_json::to_string_pretty(&old_value)?,
            serde_json::to_string_pretty(&new_value)?
        ))
    }
}

// Retry utilities
pub struct RetryConfig {
    pub max_attempts: usize,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
        }
    }
}

pub async fn retry_with_backoff<F, Fut, T, E>(config: RetryConfig, mut operation: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut delay = config.initial_delay;

    for attempt in 1..=config.max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if attempt == config.max_attempts {
                    return Err(error);
                }

                tokio::time::sleep(delay).await;
                delay = std::cmp::min(
                    Duration::from_millis(
                        (delay.as_millis() as f64 * config.backoff_multiplier) as u64,
                    ),
                    config.max_delay,
                );
            }
        }
    }

    unreachable!()
}

// Color utilities
pub struct ColorScheme {
    pub success: &'static str,
    pub error: &'static str,
    pub warning: &'static str,
    pub info: &'static str,
    pub highlight: &'static str,
    pub dim: &'static str,
}

impl ColorScheme {
    pub fn dark() -> Self {
        Self {
            success: "bright_green",
            error: "bright_red",
            warning: "bright_yellow",
            info: "bright_blue",
            highlight: "bright_cyan",
            dim: "bright_black",
        }
    }

    pub fn light() -> Self {
        Self {
            success: "green",
            error: "red",
            warning: "yellow",
            info: "blue",
            highlight: "cyan",
            dim: "black",
        }
    }
}

// Progress utilities
pub fn create_progress_bar(len: u64) -> indicatif::ProgressBar {
    let pb = indicatif::ProgressBar::new(len);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );
    pb
}

// Fuzzy matching utilities
pub fn fuzzy_match(pattern: &str, text: &str) -> Option<f64> {
    if pattern.is_empty() {
        return Some(1.0);
    }

    let pattern = pattern.to_lowercase();
    let text = text.to_lowercase();

    if text.contains(&pattern) {
        // Simple scoring: longer matches score higher
        Some(pattern.len() as f64 / text.len() as f64)
    } else {
        None
    }
}

pub fn fuzzy_sort<T>(items: &mut [(f64, T)]) {
    items.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
}

// Input validation utilities
pub fn validate_timeout(timeout: u64) -> Result<()> {
    if timeout == 0 {
        return Err(anyhow::anyhow!("Timeout cannot be zero"));
    }
    if timeout > 3600 {
        return Err(anyhow::anyhow!("Timeout cannot exceed 1 hour"));
    }
    Ok(())
}

pub fn validate_method(method: &str) -> Result<()> {
    let valid_methods = ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];
    if !valid_methods.contains(&method.to_uppercase().as_str()) {
        return Err(anyhow::anyhow!("Invalid HTTP method: {}", method));
    }
    Ok(())
}

// Table utilities for responsive display
pub fn create_responsive_table(
    headers: Vec<&str>,
    rows: Vec<Vec<String>>,
    column_priorities: Option<Vec<usize>>,
) -> comfy_table::Table {
    use comfy_table::*;

    let mut table = Table::new();
    table.load_preset(presets::UTF8_FULL_CONDENSED);

    // Get terminal width and account for table borders and padding
    let terminal_width = get_terminal_width();
    let table_overhead = 3; // Left border + right border + padding
    let column_separator = 3; // Space between columns
    let available_width = terminal_width.saturating_sub(table_overhead);

    // Calculate column widths
    let column_widths =
        calculate_column_widths(&headers, &rows, available_width, column_priorities);

    // Set headers
    table.set_header(headers);

    // Note: Column widths are controlled by truncating content
    // The table will auto-size based on the truncated content

    // Add rows with truncated content
    for row in rows {
        let truncated_row: Vec<String> = row
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                let width = column_widths.get(i).copied().unwrap_or(20);
                truncate_string(cell, width)
            })
            .collect();
        table.add_row(truncated_row);
    }

    table
}

fn calculate_column_widths(
    headers: &[&str],
    rows: &[Vec<String>],
    available_width: usize,
    column_priorities: Option<Vec<usize>>,
) -> Vec<usize> {
    let num_columns = headers.len();

    if num_columns == 0 {
        return vec![];
    }

    // Calculate minimum required width for each column (header + some content)
    let mut min_widths = vec![0; num_columns];
    let mut max_widths = vec![0; num_columns];

    // Consider header widths
    for (i, header) in headers.iter().enumerate() {
        min_widths[i] = header.len().min(8); // Minimum 8 chars for headers
        max_widths[i] = header.len().max(15); // Start with header width
    }

    // Consider content widths
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < num_columns {
                let cell_len = cell.len();
                min_widths[i] = min_widths[i].max(cell_len.min(8)); // Min 8 chars
                max_widths[i] = max_widths[i].max(cell_len.min(40)); // Max 40 chars initially
            }
        }
    }

    // Total separator space needed
    let separator_space = (num_columns - 1) * 3; // 3 chars between columns
    let content_width = available_width.saturating_sub(separator_space);

    // If minimum widths exceed available space, scale down proportionally
    let total_min_width: usize = min_widths.iter().sum();
    if total_min_width > content_width {
        let scale_factor = content_width as f64 / total_min_width as f64;
        for width in min_widths.iter_mut() {
            *width = (*width as f64 * scale_factor).max(5.0) as usize;
        }
        return min_widths;
    }

    // Distribute remaining space intelligently
    let remaining_space = content_width.saturating_sub(total_min_width);
    let mut final_widths = min_widths.clone();

    // Use priorities if provided, otherwise use smart defaults
    let priorities = column_priorities.unwrap_or_else(|| {
        // Default priorities: URL columns get more space
        (0..num_columns)
            .map(|i| {
                let header = headers[i].to_lowercase();
                if header.contains("url") {
                    3 // High priority for URLs
                } else if header.contains("name") || header.contains("method") {
                    2 // Medium priority
                } else {
                    1 // Low priority
                }
            })
            .collect()
    });

    // Distribute remaining space based on priorities
    let total_priority: usize = priorities.iter().sum();
    if total_priority > 0 {
        for (i, &priority) in priorities.iter().enumerate() {
            let additional_space = (remaining_space * priority) / total_priority;
            let max_additional = max_widths[i].saturating_sub(min_widths[i]);
            final_widths[i] += additional_space.min(max_additional);
        }
    }

    final_widths
}

pub fn create_simple_responsive_table(
    headers: Vec<&str>,
    rows: Vec<Vec<String>>,
) -> comfy_table::Table {
    create_responsive_table(headers, rows, None)
}

pub fn create_url_priority_table(
    headers: Vec<&str>,
    rows: Vec<Vec<String>>,
    url_column_index: usize,
) -> comfy_table::Table {
    let mut priorities = vec![1; headers.len()];
    if url_column_index < headers.len() {
        priorities[url_column_index] = 4; // Higher priority for URL column
    }
    create_responsive_table(headers, rows, Some(priorities))
}

// Testing utilities
#[cfg(test)]
pub mod test_utils {
    use super::*;
    use std::collections::HashMap;

    pub fn create_test_request() -> crate::request::SavedRequest {
        crate::request::SavedRequest::new(
            "test".to_string(),
            "https://api.example.com/test".to_string(),
            "GET".to_string(),
        )
    }

    pub fn create_test_response() -> crate::client::Response {
        crate::client::Response {
            status: 200,
            headers: HashMap::new(),
            body: r#"{"message": "Hello, World!"}"#.to_string(),
            duration: Duration::from_millis(100),
            size: 26,
            url: "https://api.example.com/test".to_string(),
            method: "GET".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_url() {
        assert_eq!(
            normalize_url("https://example.com:443/").unwrap(),
            "https://example.com/"
        );
        assert_eq!(
            normalize_url("http://example.com:80/path").unwrap(),
            "http://example.com/path"
        );
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_millis(500)), "500ms");
        assert_eq!(format_duration(Duration::from_secs(2)), "2.00s");
        assert_eq!(format_duration(Duration::from_secs(65)), "1m 5s");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1500), "1.5 KB");
        assert_eq!(format_bytes(1_500_000), "1.4 MB");
    }

    #[test]
    fn test_is_valid_json() {
        assert!(is_valid_json(r#"{"test": true}"#));
        assert!(is_valid_json(r#"[1, 2, 3]"#));
        assert!(!is_valid_json("not json"));
    }

    #[test]
    fn test_guess_content_type() {
        assert_eq!(guess_content_type(r#"{"test": true}"#), "application/json");
        assert_eq!(guess_content_type("<xml></xml>"), "application/xml");
        assert_eq!(
            guess_content_type("name=value"),
            "application/x-www-form-urlencoded"
        );
        assert_eq!(guess_content_type("plain text"), "text/plain");
    }

    #[test]
    fn test_extract_template_variables() {
        let text = "Hello {{name}}, your {{item}} is ready!";
        let vars = extract_template_variables(text);
        assert_eq!(vars, vec!["item", "name"]);
    }

    #[test]
    fn test_fuzzy_match() {
        assert!(fuzzy_match("test", "testing").is_some());
        assert!(fuzzy_match("xyz", "testing").is_none());
        assert_eq!(fuzzy_match("", "anything"), Some(1.0));
    }
}
