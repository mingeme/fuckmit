pub mod mappings;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::utils::git_repo::find_git_repo_root;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AuthConfig {
    providers: HashMap<String, ProviderConfig>,
    active_provider: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProviderConfig {
    pub api_key: String,
    pub model: Option<String>,
    pub endpoint: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommitConfig {
    pub prompt: PromptTemplates,
    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PromptTemplates {
    pub system: String,
    pub user: String,
}

impl Default for CommitConfig {
    fn default() -> Self {
        Self {
            prompt: PromptTemplates {
                system:
                    "You are a helpful assistant that generates clear and concise git commit messages.Follow conventional commits format."
                        .to_string(),
                user:
                    "Generate a concise git commit message for the following changes:\n\n{{diff}}"
                        .to_string(),
            },
            exclude: vec![],
        }
    }
}

impl CommitConfig {
    /// Creates a new CommitConfig with the specified exclude patterns
    pub fn new(exclude_patterns: Vec<String>) -> Self {
        Self {
            exclude: exclude_patterns,
            ..Default::default()
        }
    }
}

impl AuthConfig {
    /// Load auth config from the default path
    pub fn load() -> Result<Self> {
        let config_path = Self::get_path()?;
        Self::load_from_path(&config_path)
    }

    /// Load auth config from a custom path
    pub fn load_from_path(config_path: &Path) -> Result<Self> {
        if !config_path.exists() {
            return Err(anyhow!(
                "Config file not found at {}",
                config_path.display()
            ));
        }

        let config_str =
            std::fs::read_to_string(config_path).context("Failed to read config file")?;

        let config: AuthConfig =
            serde_yaml::from_str(&config_str).context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_path()?;

        // Ensure parent directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let config_str = serde_yaml::to_string(self).context("Failed to serialize config")?;

        std::fs::write(&config_path, config_str).context("Failed to write config file")?;

        Ok(())
    }

    pub fn get_path() -> Result<PathBuf> {
        // Check if FUCKMIT_CONFIG_DIR environment variable is set (for testing)
        if let Ok(config_dir) = std::env::var("FUCKMIT_CONFIG_DIR") {
            let mut path = PathBuf::from(config_dir);
            path.push("auth.yml");
            return Ok(path);
        }

        // Default path
        let mut path =
            dirs::config_dir().ok_or_else(|| anyhow!("Could not determine config directory"))?;

        path.push("fuckmit");
        path.push("auth.yml");

        Ok(path)
    }

    pub fn add_provider(&mut self, provider: &str, api_key: &str) -> Result<()> {
        let provider_config = ProviderConfig {
            api_key: api_key.to_string(),
            model: None,
            endpoint: None,
        };

        self.providers.insert(provider.to_string(), provider_config);

        // If this is the first provider, set it as active
        if self.active_provider.is_none() {
            self.active_provider = Some(provider.to_string());
        }

        Ok(())
    }

    pub fn set_active_provider(&mut self, provider: &str) -> Result<bool> {
        if self.providers.contains_key(provider) {
            self.active_provider = Some(provider.to_string());
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_active_provider(&self) -> Result<String> {
        self.active_provider.clone()
            .ok_or_else(|| anyhow!("No active provider set. Use 'fuckmit auth add <provider> <apiKey>' to add a provider."))
    }

    pub fn set_provider_property(
        &mut self,
        provider: &str,
        property: &str,
        value: &str,
    ) -> Result<()> {
        let provider_config = self
            .providers
            .get_mut(provider)
            .ok_or_else(|| anyhow!("Provider not found"))?;

        match property {
            "api_key" => provider_config.api_key = value.to_string(),
            "model" => provider_config.model = Some(value.to_string()),
            "endpoint" => provider_config.endpoint = Some(value.to_string()),
            _ => return Err(anyhow!("Invalid property: {}", property)),
        }

        Ok(())
    }

    pub fn get_provider_config(&self, provider: &str) -> Result<&ProviderConfig> {
        self.providers
            .get(provider)
            .ok_or_else(|| anyhow!("Provider not found: {}", provider))
    }

    /// Get all configured providers
    pub fn get_providers(&self) -> &HashMap<String, ProviderConfig> {
        &self.providers
    }

    /// Check if any providers are configured
    pub fn has_providers(&self) -> bool {
        !self.providers.is_empty()
    }

    /// Get the name of the active provider, if set
    pub fn get_active_provider_name(&self) -> Option<&str> {
        self.active_provider.as_deref()
    }
}

/// Get the config directory path
pub fn get_config_dir() -> Result<PathBuf> {
    // Check if FUCKMIT_CONFIG_DIR environment variable is set (for testing)
    if let Ok(config_dir) = std::env::var("FUCKMIT_CONFIG_DIR") {
        return Ok(PathBuf::from(config_dir));
    }

    // Default path
    let mut path =
        dirs::config_dir().ok_or_else(|| anyhow!("Could not determine config directory"))?;

    path.push("fuckmit");
    Ok(path)
}

pub fn get_commit_config() -> Result<CommitConfig> {
    // Try to load from local .fuckmit.yml file first
    if let Ok(local_config) = load_local_commit_config() {
        return Ok(local_config);
    }

    // Try to load from mappings.toml based on current directory
    if let Ok(mappings) = mappings::load() {
        // Only try to use mappings if there are any defined
        if !mappings.mappings.is_empty() {
            // Try to find the current directory in mappings
            if let Ok(current_dir) = find_git_repo_root(None) {
                // Convert to canonical path to match how paths are stored in mappings
                if let Ok(canonical_path) = std::fs::canonicalize(&current_dir) {
                    let path_str = canonical_path.to_string_lossy().to_string();

                    // Check if this path is in our mappings
                    if let Some(config_name) = mappings.mappings.get(&path_str) {
                        // Found a mapping, try to load the specified config
                        if let Ok(config_dir) = get_config_dir() {
                            let config_path =
                                config_dir.join(format!("{}.fuckmit.yml", config_name));

                            // Try to load the mapped configuration
                            if config_path.exists() {
                                return load_config_from_file(
                                    &config_path,
                                    &format!("mapped configuration '{}'", config_name),
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    // If no local config, check for global config
    if let Ok(config_dir) = get_config_dir() {
        // First check the symlink to the active configuration
        let symlink_path = config_dir.join(".fuckmit.yml");
        if symlink_path.exists() {
            return load_config_from_file(&symlink_path, "active commit config");
        }

        // If no symlink, check for default configuration
        let default_path = config_dir.join("default.fuckmit.yml");
        if default_path.exists() {
            return load_config_from_file(&default_path, "default commit config");
        }
    }

    // Fall back to default commit config
    Ok(CommitConfig::default())
}

/// Load a CommitConfig from a file with unified error handling
fn load_config_from_file(path: &Path, config_type: &str) -> Result<CommitConfig> {
    let config_str = std::fs::read_to_string(path).context(format!(
        "Failed to read {}: {}",
        config_type,
        path.display()
    ))?;

    let config: CommitConfig = serde_yaml::from_str(&config_str).context(format!(
        "Failed to parse {}: {}",
        config_type,
        path.display()
    ))?;

    Ok(config)
}

fn load_local_commit_config() -> Result<CommitConfig> {
    // First check for local config files in current directory
    let local_paths = [".fuckmit.yml", ".fuckmit.yaml"];

    for path in local_paths.iter() {
        let config_path = std::path::Path::new(path);
        if config_path.exists() {
            return load_config_from_file(config_path, "local commit config");
        }
    }

    Err(anyhow::anyhow!("No local commit config found"))
}
