use anyhow::{anyhow, Result};
use base64::Engine;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedRequest {
    pub id: String,
    pub name: String,
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub timeout: Option<u64>,
    pub follow_redirects: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub description: Option<String>,
}

impl SavedRequest {
    pub fn new(name: String, url: String, method: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            url,
            method,
            headers: HashMap::new(),
            body: None,
            timeout: None,
            follow_redirects: None,
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
            description: None,
        }
    }

    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
        self.updated_at = Utc::now();
    }

    pub fn set_body(&mut self, body: Option<String>) {
        self.body = body;
        self.updated_at = Utc::now();
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone)]
pub struct RequestBuilder {
    request: SavedRequest,
}

impl RequestBuilder {
    pub fn new(url: &str, method: &str) -> Result<Self> {
        // Validate URL
        url::Url::parse(url)
            .map_err(|_| anyhow!("Invalid URL: {}", url))?;

        // Validate method
        let method_upper = method.to_uppercase();
        let valid_methods = ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];
        if !valid_methods.contains(&method_upper.as_str()) {
            return Err(anyhow!("Invalid HTTP method: {}", method));
        }

        Ok(Self {
            request: SavedRequest::new(
                String::new(), // Name will be set later
                url.to_string(),
                method_upper,
            ),
        })
    }

    pub fn name(mut self, name: &str) -> Self {
        self.request.name = name.to_string();
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.request.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.request.headers.extend(headers);
        self
    }

    pub fn auth(mut self, auth: &str) -> Result<Self> {
        if let Some((auth_type, credentials)) = auth.split_once(':') {
            match auth_type.to_lowercase().as_str() {
                "bearer" => {
                    self.request.headers.insert(
                        "Authorization".to_string(),
                        format!("Bearer {}", credentials),
                    );
                }
                "basic" => {
                    if let Some((username, password)) = credentials.split_once(':') {
                        let encoded = base64::prelude::BASE64_STANDARD.encode(format!("{}:{}", username, password));
                        self.request.headers.insert(
                            "Authorization".to_string(),
                            format!("Basic {}", encoded),
                        );
                    } else {
                        return Err(anyhow!("Basic auth requires username:password format"));
                    }
                }
                "api-key" | "apikey" => {
                    if let Some((header_name, key_value)) = credentials.split_once(':') {
                        self.request.headers.insert(header_name.to_string(), key_value.to_string());
                    } else {
                        // Default to X-API-Key
                        self.request.headers.insert("X-API-Key".to_string(), credentials.to_string());
                    }
                }
                _ => {
                    return Err(anyhow!("Unsupported auth type: {}", auth_type));
                }
            }
        } else {
            // Assume it's a bearer token if no type specified
            self.request.headers.insert(
                "Authorization".to_string(),
                format!("Bearer {}", auth),
            );
        }
        Ok(self)
    }

    pub fn json_body(mut self, json: &str) -> Result<Self> {
        // Validate JSON
        serde_json::from_str::<serde_json::Value>(json)
            .map_err(|e| anyhow!("Invalid JSON: {}", e))?;

        self.request.headers.insert(
            "Content-Type".to_string(),
            "application/json".to_string(),
        );
        self.request.body = Some(json.to_string());
        Ok(self)
    }

    pub fn form_body(mut self, form_data: HashMap<String, String>) -> Result<Self> {
        let encoded = form_data
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        self.request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );
        self.request.body = Some(encoded);
        Ok(self)
    }

    pub fn raw_body(mut self, body: &str) -> Self {
        self.request.body = Some(body.to_string());
        self
    }

    pub fn timeout(mut self, seconds: u64) -> Self {
        self.request.timeout = Some(seconds);
        self
    }

    pub fn follow_redirects(mut self, follow: bool) -> Self {
        self.request.follow_redirects = Some(follow);
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.request.description = Some(description.to_string());
        self
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.request.tags.push(tag.to_string());
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.request.tags.extend(tags);
        self
    }

    pub fn build(self) -> SavedRequest {
        self.request
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCollection {
    pub name: String,
    pub description: Option<String>,
    pub requests: Vec<SavedRequest>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl RequestCollection {
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            name,
            description: None,
            requests: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_request(&mut self, request: SavedRequest) {
        self.requests.push(request);
        self.updated_at = Utc::now();
    }

    pub fn remove_request(&mut self, request_id: &str) -> bool {
        let initial_len = self.requests.len();
        self.requests.retain(|r| r.id != request_id);
        let removed = self.requests.len() != initial_len;
        if removed {
            self.updated_at = Utc::now();
        }
        removed
    }

    pub fn find_request(&self, name: &str) -> Option<&SavedRequest> {
        self.requests.iter().find(|r| r.name == name)
    }

    pub fn find_request_mut(&mut self, name: &str) -> Option<&mut SavedRequest> {
        self.requests.iter_mut().find(|r| r.name == name)
    }
}

// Template system for dynamic requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestTemplate {
    pub name: String,
    pub description: Option<String>,
    pub base_request: SavedRequest,
    pub variables: HashMap<String, TemplateVariable>,
    pub environments: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub description: Option<String>,
    pub default_value: Option<String>,
    pub required: bool,
    pub variable_type: VariableType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Url,
    Email,
    Json,
}

impl RequestTemplate {
    pub fn new(name: String, base_request: SavedRequest) -> Self {
        Self {
            name,
            description: None,
            base_request,
            variables: HashMap::new(),
            environments: HashMap::new(),
        }
    }

    pub fn add_variable(&mut self, variable: TemplateVariable) {
        self.variables.insert(variable.name.clone(), variable);
    }

    pub fn add_environment(&mut self, name: String, variables: HashMap<String, String>) {
        self.environments.insert(name, variables);
    }

    pub fn render(&self, environment: Option<&str>, variables: HashMap<String, String>) -> Result<SavedRequest> {
        let mut rendered_request = self.base_request.clone();
        let mut all_variables = HashMap::new();

        // Start with environment variables
        if let Some(env_name) = environment {
            if let Some(env_vars) = self.environments.get(env_name) {
                all_variables.extend(env_vars.clone());
            }
        }

        // Override with provided variables
        all_variables.extend(variables);

        // Check required variables
        for (var_name, var_def) in &self.variables {
            if var_def.required && !all_variables.contains_key(var_name) {
                if var_def.default_value.is_none() {
                    return Err(anyhow!("Required variable '{}' not provided", var_name));
                }
            }
        }

        // Add default values for missing variables
        for (var_name, var_def) in &self.variables {
            if !all_variables.contains_key(var_name) {
                if let Some(ref default) = var_def.default_value {
                    all_variables.insert(var_name.clone(), default.clone());
                }
            }
        }

        // Replace variables in URL
        rendered_request.url = self.replace_variables(&rendered_request.url, &all_variables)?;

        // Replace variables in headers
        for (key, value) in rendered_request.headers.iter_mut() {
            *value = self.replace_variables(value, &all_variables)?;
        }

        // Replace variables in body
        if let Some(ref body) = rendered_request.body {
            rendered_request.body = Some(self.replace_variables(body, &all_variables)?);
        }

        Ok(rendered_request)
    }

    fn replace_variables(&self, text: &str, variables: &HashMap<String, String>) -> Result<String> {
        let mut result = text.to_string();
        
        // Replace {{variable}} patterns
        for (var_name, var_value) in variables {
            let pattern = format!("{{{{{}}}}}", var_name);
            result = result.replace(&pattern, var_value);
        }

        // Check for unresolved variables
        if result.contains("{{") && result.contains("}}") {
            let start = result.find("{{").unwrap();
            let end = result.find("}}").unwrap() + 2;
            let unresolved = &result[start..end];
            return Err(anyhow!("Unresolved variable: {}", unresolved));
        }

        Ok(result)
    }
}

// Request validation
pub fn validate_request(request: &SavedRequest) -> Result<()> {
    // Validate URL
    url::Url::parse(&request.url)
        .map_err(|_| anyhow!("Invalid URL: {}", request.url))?;

    // Validate method
    let valid_methods = ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];
    if !valid_methods.contains(&request.method.as_str()) {
        return Err(anyhow!("Invalid HTTP method: {}", request.method));
    }

    // Validate JSON body if content-type is JSON
    if let Some(ref body) = request.body {
        if let Some(content_type) = request.headers.get("Content-Type") {
            if content_type.contains("application/json") {
                serde_json::from_str::<serde_json::Value>(body)
                    .map_err(|e| anyhow!("Invalid JSON body: {}", e))?;
            }
        }
    }

    // Validate timeout
    if let Some(timeout) = request.timeout {
        if timeout == 0 || timeout > 3600 {
            return Err(anyhow!("Timeout must be between 1 and 3600 seconds"));
        }
    }

    Ok(())
}

// Helper functions for common request patterns
pub fn create_get_request(url: &str) -> Result<SavedRequest> {
    Ok(RequestBuilder::new(url, "GET")?.build())
}

pub fn create_post_json_request(url: &str, json_body: &str) -> Result<SavedRequest> {
    Ok(RequestBuilder::new(url, "POST")?
        .json_body(json_body)?
        .build())
}

pub fn create_authenticated_request(url: &str, method: &str, token: &str) -> Result<SavedRequest> {
    Ok(RequestBuilder::new(url, method)?
        .auth(&format!("bearer:{}", token))?
        .build())
}