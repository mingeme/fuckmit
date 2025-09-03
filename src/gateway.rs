//! Main gateway implementation

use crate::config::GatewayConfig;
use crate::error::{GatewayError, Result};
use crate::providers::{create_provider, Provider, ProviderType};
use crate::types::{ChatMessage, ChatRequest, ChatResponse};
use std::collections::HashMap;
use std::sync::Arc;

/// Main LLM Gateway struct
pub struct LLMGateway {
    config: GatewayConfig,
    providers: HashMap<ProviderType, Arc<dyn Provider>>,
}

impl LLMGateway {
    /// Create a new gateway with the given configuration
    pub fn new(config: GatewayConfig) -> Result<Self> {
        let mut providers = HashMap::new();

        // Initialize all configured providers
        for (provider_type, provider_config) in &config.providers {
            let provider = create_provider(*provider_type, provider_config)?;
            provider.validate_config()?;
            providers.insert(*provider_type, Arc::from(provider));
        }

        Ok(Self { config, providers })
    }

    /// Create a gateway from environment variables
    pub async fn from_env() -> Result<Self> {
        let config = GatewayConfig::from_env()?;
        Self::new(config)
    }

    /// Send a chat completion request using the default provider
    pub async fn chat_completion(&self, messages: Vec<ChatMessage>) -> Result<ChatResponse> {
        self.chat_completion_with_provider(messages, None).await
    }

    /// Send a chat completion request with a specific provider
    pub async fn chat_completion_with_provider(
        &self,
        messages: Vec<ChatMessage>,
        provider_type: Option<ProviderType>,
    ) -> Result<ChatResponse> {
        self.chat_completion_with_provider_and_model(messages, provider_type, None)
            .await
    }

    /// Send a chat completion request with a specific provider and model
    pub async fn chat_completion_with_provider_and_model(
        &self,
        messages: Vec<ChatMessage>,
        provider_type: Option<ProviderType>,
        model: Option<String>,
    ) -> Result<ChatResponse> {
        let provider_type = provider_type.unwrap_or(self.config.default_provider);
        let provider = self.get_provider(&provider_type)?;

        // Use specified model or provider's default model
        let model_name = model.unwrap_or_else(|| {
            provider
                .default_model()
                .unwrap_or("gpt-3.5-turbo")
                .to_string()
        });

        let request = ChatRequest::new(messages, model_name);
        provider.chat_completion(request).await
    }

    /// Send a custom chat request
    pub async fn chat_request(&self, request: ChatRequest) -> Result<ChatResponse> {
        self.chat_request_with_provider(request, None).await
    }

    /// Send a custom chat request with a specific provider
    pub async fn chat_request_with_provider(
        &self,
        request: ChatRequest,
        provider_type: Option<ProviderType>,
    ) -> Result<ChatResponse> {
        let provider_type = provider_type.unwrap_or(self.config.default_provider);
        let provider = self.get_provider(&provider_type)?;
        provider.chat_completion(request).await
    }

    /// Send a chat completion request with custom parameters
    pub async fn chat_with_options(
        &self,
        messages: Vec<ChatMessage>,
        provider_type: Option<ProviderType>,
        model: Option<String>,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> Result<ChatResponse> {
        let provider_type = provider_type.unwrap_or(self.config.default_provider);
        let provider = self.get_provider(&provider_type)?;

        // Use specified model or provider's default model
        let model_name = model.unwrap_or_else(|| {
            provider
                .default_model()
                .unwrap_or("gpt-3.5-turbo")
                .to_string()
        });

        let mut request = ChatRequest::new(messages, model_name);

        if let Some(tokens) = max_tokens {
            request = request.with_max_tokens(tokens);
        }

        if let Some(temp) = temperature {
            request = request.with_temperature(temp);
        }

        provider.chat_completion(request).await
    }

    /// Get a provider instance
    pub fn get_provider(&self, provider_type: &ProviderType) -> Result<Arc<dyn Provider>> {
        self.providers.get(provider_type).cloned().ok_or_else(|| {
            GatewayError::Config(format!("Provider {:?} is not configured", provider_type))
        })
    }

    /// Get the list of available providers
    pub fn available_providers(&self) -> Vec<ProviderType> {
        self.providers.keys().copied().collect()
    }

    /// Get the default provider type
    pub fn default_provider(&self) -> ProviderType {
        self.config.default_provider
    }

    /// Check if a provider is available
    pub fn has_provider(&self, provider_type: &ProviderType) -> bool {
        self.providers.contains_key(provider_type)
    }

    /// Get the gateway configuration
    pub fn config(&self) -> &GatewayConfig {
        &self.config
    }
}

impl LLMGateway {
    /// Convenience method to send a simple text message
    pub async fn chat(&self, message: impl Into<String>) -> Result<String> {
        let messages = vec![ChatMessage::user(message)];
        let response = self.chat_completion(messages).await?;
        Ok(response.content().unwrap_or("").to_string())
    }

    /// Convenience method to send a text message with a specific provider
    pub async fn chat_with_provider(
        &self,
        message: impl Into<String>,
        provider_type: ProviderType,
    ) -> Result<String> {
        let messages = vec![ChatMessage::user(message)];
        let response = self
            .chat_completion_with_provider(messages, Some(provider_type))
            .await?;
        Ok(response.content().unwrap_or("").to_string())
    }
}
