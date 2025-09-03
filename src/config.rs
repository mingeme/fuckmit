//! Configuration management for the LLM Gateway

use crate::error::{GatewayError, Result};
use crate::providers::ProviderType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

/// Configuration for a specific provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// API key for the provider
    pub api_key: String,
    /// Base URL for the provider's API
    pub base_url: Option<String>,
    /// Default model to use
    pub default_model: Option<String>,
    /// Additional provider-specific configuration
    pub extra: HashMap<String, String>,
}

impl ProviderConfig {
    /// Create a new provider configuration
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: None,
            default_model: None,
            extra: HashMap::new(),
        }
    }

    /// Set the base URL
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Set the default model
    pub fn with_default_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = Some(model.into());
        self
    }

    /// Add extra configuration
    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}

/// Main gateway configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// Default provider to use
    pub default_provider: ProviderType,
    /// Configuration for each provider
    pub providers: HashMap<ProviderType, ProviderConfig>,
    /// Global timeout in seconds
    pub timeout_seconds: Option<u64>,
    /// Maximum number of retries
    pub max_retries: Option<u32>,
}

impl GatewayConfig {
    /// Create a new gateway configuration
    pub fn new(default_provider: ProviderType) -> Self {
        Self {
            default_provider,
            providers: HashMap::new(),
            timeout_seconds: None,
            max_retries: None,
        }
    }

    /// Add a provider configuration
    pub fn with_provider(mut self, provider_type: ProviderType, config: ProviderConfig) -> Self {
        self.providers.insert(provider_type, config);
        self
    }

    /// Set global timeout
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = Some(timeout_seconds);
        self
    }

    /// Set maximum retries
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let mut config = Self::new(ProviderType::OpenAI); // Default to OpenAI

        // Support provider/model format for unified model specification
        if let Ok(provider_model) = env::var("LLM_MODEL") {
            let parts: Vec<&str> = provider_model.split('/').collect();
            if parts.len() == 2 {
                let provider_str = parts[0];
                let _model_name = parts[1];

                // Parse provider type
                let provider_type = provider_str.parse::<ProviderType>().map_err(|_| {
                    GatewayError::Config(format!("Invalid provider in LLM_MODEL: {}", provider_str))
                })?;

                // Set as default provider
                config.default_provider = provider_type;

                // We'll update the provider config with the specified model after loading all configs
            } else {
                return Err(GatewayError::Config(
                    "LLM_MODEL must be in format 'provider/model' (e.g., 'openai/gpt-4')"
                        .to_string(),
                ));
            }
        }

        // Load OpenAI configuration
        if let Ok(api_key) = env::var("OPENAI_API_KEY") {
            let mut provider_config = ProviderConfig::new(api_key);

            if let Ok(base_url) = env::var("OPENAI_BASE_URL") {
                provider_config = provider_config.with_base_url(base_url);
            }

            if let Ok(model) = env::var("OPENAI_MODEL") {
                provider_config = provider_config.with_default_model(model);
            }

            config
                .providers
                .insert(ProviderType::OpenAI, provider_config);
        }

        // Load Azure configuration
        if let Ok(api_key) = env::var("AZURE_OPENAI_API_KEY") {
            let mut provider_config = ProviderConfig::new(api_key);

            if let Ok(endpoint) = env::var("AZURE_OPENAI_ENDPOINT") {
                provider_config = provider_config.with_base_url(endpoint);
            }

            if let Ok(deployment) = env::var("AZURE_OPENAI_DEPLOYMENT") {
                provider_config = provider_config.with_default_model(deployment);
            }

            if let Ok(api_version) = env::var("AZURE_OPENAI_API_VERSION") {
                provider_config = provider_config.with_extra("api_version", api_version);
            }

            config
                .providers
                .insert(ProviderType::Azure, provider_config);
        }

        // Load DeepSeek configuration
        if let Ok(api_key) = env::var("DEEPSEEK_API_KEY") {
            let mut provider_config = ProviderConfig::new(api_key);

            if let Ok(base_url) = env::var("DEEPSEEK_BASE_URL") {
                provider_config = provider_config.with_base_url(base_url);
            }

            if let Ok(model) = env::var("DEEPSEEK_MODEL") {
                provider_config = provider_config.with_default_model(model);
            }

            config
                .providers
                .insert(ProviderType::DeepSeek, provider_config);
        }

        // Load Qwen configuration
        if let Ok(api_key) = env::var("QWEN_API_KEY") {
            let mut provider_config = ProviderConfig::new(api_key);

            if let Ok(base_url) = env::var("QWEN_BASE_URL") {
                provider_config = provider_config.with_base_url(base_url);
            }

            if let Ok(model) = env::var("QWEN_MODEL") {
                provider_config = provider_config.with_default_model(model);
            }

            config.providers.insert(ProviderType::Qwen, provider_config);
        }

        // Apply LLM_MODEL override if specified
        if let Ok(provider_model) = env::var("LLM_MODEL") {
            let parts: Vec<&str> = provider_model.split('/').collect();
            if parts.len() == 2 {
                let provider_str = parts[0];
                let model_name = parts[1];

                if let Ok(provider_type) = provider_str.parse::<ProviderType>() {
                    if let Some(provider_config) = config.providers.get_mut(&provider_type) {
                        provider_config.default_model = Some(model_name.to_string());
                    }
                }
            }
        }

        // Load global settings
        if let Ok(timeout) = env::var("LLM_TIMEOUT_SECONDS") {
            config.timeout_seconds = Some(
                timeout
                    .parse()
                    .map_err(|_| GatewayError::Config("Invalid timeout value".to_string()))?,
            );
        }

        if let Ok(retries) = env::var("LLM_MAX_RETRIES") {
            config.max_retries = Some(
                retries
                    .parse()
                    .map_err(|_| GatewayError::Config("Invalid max retries value".to_string()))?,
            );
        }

        // Validate that we have at least one provider configured
        if config.providers.is_empty() {
            return Err(GatewayError::Config(
                "No providers configured. Please set at least one provider's API key.".to_string(),
            ));
        }

        // Ensure the default provider is configured
        if !config.providers.contains_key(&config.default_provider) {
            return Err(GatewayError::Config(format!(
                "Default provider {:?} is not configured",
                config.default_provider
            )));
        }

        Ok(config)
    }

    /// Get configuration for a specific provider
    pub fn get_provider_config(&self, provider_type: &ProviderType) -> Option<&ProviderConfig> {
        self.providers.get(provider_type)
    }

    /// Get the default provider configuration
    pub fn get_default_provider_config(&self) -> Option<&ProviderConfig> {
        self.get_provider_config(&self.default_provider)
    }
}
