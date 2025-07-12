use anyhow::Result;
use reqwest::{Client, Method, Request, Response as ReqwestResponse, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::timeout;

use crate::config::Config;
use crate::request::SavedRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub duration: Duration,
    pub size: usize,
    pub url: String,
    pub method: String,
}

pub struct TerziClient {
    client: Client,
    config: Config,
}

impl TerziClient {
    pub fn new(config: &Config) -> Result<Self> {
        let mut client_builder = Client::builder()
            .user_agent(&config.network.user_agent)
            .cookie_store(true)
            .timeout(Duration::from_secs(config.network.read_timeout))
            .connect_timeout(Duration::from_secs(config.network.connection_timeout))
            .tcp_keepalive(if config.network.keep_alive {
                Some(Duration::from_secs(60))
            } else {
                None
            })
            // Note: compression is enabled by default in reqwest
            .redirect(reqwest::redirect::Policy::limited(
                config.network.max_redirects as usize,
            ));

        // Set proxy if configured
        if let Some(ref proxy_url) = config.network.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url)?;
            client_builder = client_builder.proxy(proxy);
        }

        // Set SSL verification
        if !config.network.verify_ssl {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }

        let client = client_builder.build()?;

        Ok(Self {
            client,
            config: config.clone(),
        })
    }

    pub async fn execute_request(&self, saved_request: &SavedRequest) -> Result<Response> {
        let start_time = Instant::now();

        // Build the request
        let method = Method::from_bytes(saved_request.method.as_bytes())?;
        let url = reqwest::Url::parse(&saved_request.url)?;

        let mut request_builder = self.client.request(method.clone(), url.clone());

        // Add headers
        for (key, value) in &saved_request.headers {
            request_builder = request_builder.header(key, value);
        }

        // Add body if present
        if let Some(body) = &saved_request.body {
            request_builder = request_builder.body(body.clone());
        }

        // Set timeout - use request timeout if specified, otherwise use config default
        let request_timeout = Duration::from_secs(
            saved_request
                .timeout
                .unwrap_or(self.config.general.default_timeout),
        );

        // Execute request with timeout
        let response = timeout(request_timeout, request_builder.send()).await??;

        let duration = start_time.elapsed();

        // Extract response data
        let status = response.status();
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body = response.text().await?;
        let size = body.len();

        Ok(Response {
            status: status.as_u16(),
            headers,
            body,
            duration,
            size,
            url: url.to_string(),
            method: method.to_string(),
        })
    }

    pub async fn test_connection(&self, url: &str) -> Result<bool> {
        let response = self.client.head(url).send().await;
        Ok(response.is_ok())
    }

    pub async fn get_response_preview(&self, url: &str, limit: usize) -> Result<String> {
        let response = self.client.get(url).send().await?;
        let text = response.text().await?;

        if text.len() > limit {
            Ok(format!("{}...", &text[..limit]))
        } else {
            Ok(text)
        }
    }
}

impl Response {
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    pub fn is_client_error(&self) -> bool {
        self.status >= 400 && self.status < 500
    }

    pub fn is_server_error(&self) -> bool {
        self.status >= 500 && self.status < 600
    }

    pub fn content_type(&self) -> Option<&String> {
        self.headers.get("content-type")
    }

    pub fn is_json(&self) -> bool {
        self.content_type()
            .map(|ct| ct.contains("application/json"))
            .unwrap_or(false)
    }

    pub fn is_xml(&self) -> bool {
        self.content_type()
            .map(|ct| ct.contains("application/xml") || ct.contains("text/xml"))
            .unwrap_or(false)
    }

    pub fn is_html(&self) -> bool {
        self.content_type()
            .map(|ct| ct.contains("text/html"))
            .unwrap_or(false)
    }

    pub fn status_emoji(&self) -> &'static str {
        match self.status {
            200..=299 => "🟢",
            300..=399 => "🟡",
            400..=499 => "🔴",
            500..=599 => "💥",
            _ => "❓",
        }
    }

    pub fn size_human(&self) -> String {
        crate::utils::format_bytes(self.size)
    }

    pub fn duration_human(&self) -> String {
        crate::utils::format_duration(self.duration)
    }
}
