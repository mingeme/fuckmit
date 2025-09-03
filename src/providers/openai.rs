//! OpenAI provider implementation

use crate::config::ProviderConfig;
use crate::error::{GatewayError, Result};
use crate::providers::{Provider, ProviderType};
use crate::types::{ChatRequest, ChatResponse};
use async_trait::async_trait;
use reqwest::Client;

/// OpenAI provider
pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    base_url: String,
    default_model: Option<String>,
}

impl OpenAIProvider {
    /// Create a new OpenAI provider
    pub fn new(config: &ProviderConfig) -> Result<Self> {
        let base_url = config
            .base_url
            .clone()
            .unwrap_or_else(|| "https://api.openai.com/v1".to_string());

        Ok(Self {
            client: Client::new(),
            api_key: config.api_key.clone(),
            base_url,
            default_model: config.default_model.clone(),
        })
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::OpenAI
    }

    async fn chat_completion(&self, request: ChatRequest) -> Result<ChatResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(GatewayError::Provider(format!(
                "OpenAI API error {}: {}",
                status, error_text
            )));
        }

        let chat_response: ChatResponse = response.json().await?;
        Ok(chat_response)
    }

    fn default_model(&self) -> Option<&str> {
        self.default_model.as_deref().or(Some("gpt-3.5-turbo"))
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn validate_config(&self) -> Result<()> {
        if self.api_key.is_empty() {
            return Err(GatewayError::Config(
                "OpenAI API key is required".to_string(),
            ));
        }
        Ok(())
    }
}
