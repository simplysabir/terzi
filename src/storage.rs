use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::client::Response;
use crate::request::{RequestCollection, SavedRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub url: String,
    pub response_status: Option<u16>,
    pub duration_ms: Option<u64>,
    pub request_size: Option<usize>,
    pub response_size: Option<usize>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StorageData {
    requests: HashMap<String, SavedRequest>,
    collections: HashMap<String, RequestCollection>,
    history: Vec<HistoryEntry>,
    environments: HashMap<String, HashMap<String, String>>,
    settings: HashMap<String, String>,
}

impl Default for StorageData {
    fn default() -> Self {
        Self {
            requests: HashMap::new(),
            collections: HashMap::new(),
            history: Vec::new(),
            environments: HashMap::new(),
            settings: HashMap::new(),
        }
    }
}

pub struct Storage {
    data_dir: PathBuf,
    data: StorageData,
}

impl Storage {
    pub async fn new() -> Result<Self> {
        let data_dir = Self::get_data_directory()?;
        
        // Create data directory if it doesn't exist
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir).await?;
        }

        let mut storage = Self {
            data_dir,
            data: StorageData::default(),
        };

        // Load existing data
        storage.load().await?;
        
        Ok(storage)
    }

    fn get_data_directory() -> Result<PathBuf> {
        if let Some(config_dir) = dirs::config_dir() {
            Ok(config_dir.join("terzi"))
        } else if let Some(home_dir) = dirs::home_dir() {
            Ok(home_dir.join(".terzi"))
        } else {
            Ok(PathBuf::from(".terzi"))
        }
    }

    async fn load(&mut self) -> Result<()> {
        let data_file = self.data_dir.join("data.json");
        
        if data_file.exists() {
            let mut file = fs::File::open(&data_file).await?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;
            
            if !contents.is_empty() {
                self.data = serde_json::from_str(&contents)
                    .unwrap_or_else(|_| StorageData::default());
            }
        }
        
        Ok(())
    }

    async fn save(&self) -> Result<()> {
        let data_file = self.data_dir.join("data.json");
        let contents = serde_json::to_string_pretty(&self.data)?;
        
        let mut file = fs::File::create(&data_file).await?;
        file.write_all(contents.as_bytes()).await?;
        file.flush().await?;
        
        Ok(())
    }

    // Request management
    pub async fn save_request(&mut self, name: &str, request: &SavedRequest) -> Result<()> {
        let mut request = request.clone();
        request.name = name.to_string();
        request.updated_at = Utc::now();
        
        self.data.requests.insert(name.to_string(), request);
        self.save().await?;
        
        Ok(())
    }

    pub async fn get_request(&self, name: &str) -> Result<Option<SavedRequest>> {
        Ok(self.data.requests.get(name).cloned())
    }

    pub async fn list_requests(&self, filter: Option<&str>) -> Result<Vec<SavedRequest>> {
        let mut requests: Vec<SavedRequest> = self.data.requests.values().cloned().collect();
        
        if let Some(filter) = filter {
            let filter_lower = filter.to_lowercase();
            requests.retain(|r| {
                r.name.to_lowercase().contains(&filter_lower) ||
                r.url.to_lowercase().contains(&filter_lower) ||
                r.method.to_lowercase().contains(&filter_lower) ||
                r.tags.iter().any(|tag| tag.to_lowercase().contains(&filter_lower))
            });
        }
        
        // Sort by creation date (newest first)
        requests.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        Ok(requests)
    }

    pub async fn delete_request(&mut self, name: &str) -> Result<bool> {
        let removed = self.data.requests.remove(name).is_some();
        if removed {
            self.save().await?;
        }
        Ok(removed)
    }

    pub async fn search_requests(&self, query: &str) -> Result<Vec<SavedRequest>> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        
        for request in self.data.requests.values() {
            let mut score = 0;
            
            // Exact name match gets highest score
            if request.name.to_lowercase() == query_lower {
                score += 100;
            } else if request.name.to_lowercase().contains(&query_lower) {
                score += 50;
            }
            
            // URL matches
            if request.url.to_lowercase().contains(&query_lower) {
                score += 30;
            }
            
            // Method matches
            if request.method.to_lowercase().contains(&query_lower) {
                score += 20;
            }
            
            // Tag matches
            for tag in &request.tags {
                if tag.to_lowercase().contains(&query_lower) {
                    score += 25;
                }
            }
            
            // Description matches
            if let Some(ref desc) = request.description {
                if desc.to_lowercase().contains(&query_lower) {
                    score += 15;
                }
            }
            
            if score > 0 {
                results.push((score, request.clone()));
            }
        }
        
        // Sort by score (highest first)
        results.sort_by(|a, b| b.0.cmp(&a.0));
        Ok(results.into_iter().map(|(_, req)| req).collect())
    }

    // Collection management
    pub async fn create_collection(&mut self, name: &str, description: Option<String>) -> Result<()> {
        let mut collection = RequestCollection::new(name.to_string());
        collection.description = description;
        
        self.data.collections.insert(name.to_string(), collection);
        self.save().await?;
        
        Ok(())
    }

    pub async fn add_request_to_collection(&mut self, collection_name: &str, request: SavedRequest) -> Result<()> {
        if let Some(collection) = self.data.collections.get_mut(collection_name) {
            collection.add_request(request);
            self.save().await?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Collection '{}' not found", collection_name))
        }
    }

    pub async fn list_collections(&self) -> Result<Vec<RequestCollection>> {
        let mut collections: Vec<RequestCollection> = self.data.collections.values().cloned().collect();
        collections.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(collections)
    }

    pub async fn get_collection(&self, name: &str) -> Result<Option<RequestCollection>> {
        Ok(self.data.collections.get(name).cloned())
    }

    pub async fn delete_collection(&mut self, name: &str) -> Result<bool> {
        let removed = self.data.collections.remove(name).is_some();
        if removed {
            self.save().await?;
        }
        Ok(removed)
    }

    // History management
    pub async fn add_to_history(&mut self, request: &SavedRequest, response: &Response) -> Result<()> {
        let entry = HistoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            method: request.method.clone(),
            url: request.url.clone(),
            response_status: Some(response.status),
            duration_ms: Some(response.duration.as_millis() as u64),
            request_size: request.body.as_ref().map(|b| b.len()),
            response_size: Some(response.size),
            error_message: None,
        };
        
        self.data.history.push(entry);
        
        // Keep only last 1000 entries to prevent infinite growth
        if self.data.history.len() > 1000 {
            self.data.history.remove(0);
        }
        
        self.save().await?;
        Ok(())
    }

    pub async fn add_error_to_history(&mut self, request: &SavedRequest, error: &str) -> Result<()> {
        let entry = HistoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            method: request.method.clone(),
            url: request.url.clone(),
            response_status: None,
            duration_ms: None,
            request_size: request.body.as_ref().map(|b| b.len()),
            response_size: None,
            error_message: Some(error.to_string()),
        };
        
        self.data.history.push(entry);
        
        if self.data.history.len() > 1000 {
            self.data.history.remove(0);
        }
        
        self.save().await?;
        Ok(())
    }

    pub async fn get_history(&self, limit: usize) -> Result<Vec<HistoryEntry>> {
        let mut history = self.data.history.clone();
        history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        if history.len() > limit {
            history.truncate(limit);
        }
        
        Ok(history)
    }

    pub async fn clear_history(&mut self) -> Result<()> {
        self.data.history.clear();
        self.save().await?;
        Ok(())
    }

    pub async fn get_history_stats(&self) -> Result<HistoryStats> {
        let mut stats = HistoryStats::default();
        
        for entry in &self.data.history {
            stats.total_requests += 1;
            
            if let Some(status) = entry.response_status {
                match status {
                    200..=299 => stats.successful_requests += 1,
                    400..=499 => stats.client_errors += 1,
                    500..=599 => stats.server_errors += 1,
                    _ => {}
                }
            } else {
                stats.failed_requests += 1;
            }
            
            if let Some(duration) = entry.duration_ms {
                stats.total_duration_ms += duration;
                stats.min_duration_ms = stats.min_duration_ms.map_or(Some(duration), |min| Some(min.min(duration)));
                stats.max_duration_ms = stats.max_duration_ms.map_or(Some(duration), |max| Some(max.max(duration)));
            }
        }
        
        if stats.total_requests > 0 {
            stats.average_duration_ms = Some(stats.total_duration_ms / stats.total_requests as u64);
        }
        
        Ok(stats)
    }

    // Environment management
    pub async fn save_environment(&mut self, name: &str, variables: HashMap<String, String>) -> Result<()> {
        self.data.environments.insert(name.to_string(), variables);
        self.save().await?;
        Ok(())
    }

    pub async fn get_environment(&self, name: &str) -> Result<Option<HashMap<String, String>>> {
        Ok(self.data.environments.get(name).cloned())
    }

    pub async fn list_environments(&self) -> Result<Vec<String>> {
        let mut env_names: Vec<String> = self.data.environments.keys().cloned().collect();
        env_names.sort();
        Ok(env_names)
    }

    pub async fn delete_environment(&mut self, name: &str) -> Result<bool> {
        let removed = self.data.environments.remove(name).is_some();
        if removed {
            self.save().await?;
        }
        Ok(removed)
    }

    // Settings management
    pub async fn set_setting(&mut self, key: &str, value: &str) -> Result<()> {
        self.data.settings.insert(key.to_string(), value.to_string());
        self.save().await?;
        Ok(())
    }

    pub async fn get_setting(&self, key: &str) -> Result<Option<String>> {
        Ok(self.data.settings.get(key).cloned())
    }

    pub async fn list_settings(&self) -> Result<HashMap<String, String>> {
        Ok(self.data.settings.clone())
    }

    // Export/Import functionality
    pub async fn export_data(&self, include_history: bool) -> Result<String> {
        let mut export_data = self.data.clone();
        
        if !include_history {
            export_data.history.clear();
        }
        
        Ok(serde_json::to_string_pretty(&export_data)?)
    }

    pub async fn import_data(&mut self, data: &str, merge: bool) -> Result<()> {
        let imported_data: StorageData = serde_json::from_str(data)?;
        
        if merge {
            // Merge imported data with existing data
            self.data.requests.extend(imported_data.requests);
            self.data.collections.extend(imported_data.collections);
            self.data.environments.extend(imported_data.environments);
            self.data.settings.extend(imported_data.settings);
            
            // Merge history but maintain chronological order
            self.data.history.extend(imported_data.history);
            self.data.history.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        } else {
            // Replace existing data
            self.data = imported_data;
        }
        
        self.save().await?;
        Ok(())
    }

    // Backup functionality
    pub async fn create_backup(&self) -> Result<PathBuf> {
        let backup_dir = self.data_dir.join("backups");
        if !backup_dir.exists() {
            fs::create_dir_all(&backup_dir).await?;
        }
        
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let backup_file = backup_dir.join(format!("terzi_backup_{}.json", timestamp));
        
        let backup_data = self.export_data(true).await?;
        
        let mut file = fs::File::create(&backup_file).await?;
        file.write_all(backup_data.as_bytes()).await?;
        file.flush().await?;
        
        Ok(backup_file)
    }

    pub async fn restore_backup(&mut self, backup_path: &PathBuf) -> Result<()> {
        let mut file = fs::File::open(backup_path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        
        self.import_data(&contents, false).await?;
        Ok(())
    }

    pub async fn list_backups(&self) -> Result<Vec<PathBuf>> {
        let backup_dir = self.data_dir.join("backups");
        if !backup_dir.exists() {
            return Ok(Vec::new());
        }
        
        let mut backups = Vec::new();
        let mut entries = fs::read_dir(&backup_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                backups.push(path);
            }
        }
        
        // Sort by modification time (newest first)
        backups.sort_by(|a, b| {
            let a_modified = std::fs::metadata(a).and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            let b_modified = std::fs::metadata(b).and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            b_modified.cmp(&a_modified)
        });
        
        Ok(backups)
    }
}

#[derive(Debug, Default)]
pub struct HistoryStats {
    pub total_requests: usize,
    pub successful_requests: usize,
    pub client_errors: usize,
    pub server_errors: usize,
    pub failed_requests: usize,
    pub total_duration_ms: u64,
    pub average_duration_ms: Option<u64>,
    pub min_duration_ms: Option<u64>,
    pub max_duration_ms: Option<u64>,
}