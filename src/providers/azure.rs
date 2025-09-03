//! Azure OpenAI provider implementation

use crate::config::ProviderConfig;
use crate::error::{GatewayError, Result};
use crate::providers::{Provider, ProviderType};
use crate::types::{ChatRequest, ChatResponse};
use async_trait::async_trait;
use reqwest::Client;

/// Azure OpenAI provider
pub struct AzureProvider {
    client: Client,
    api_key: String,
    endpoint: String,
    api_version: String,
    deployment_name: Option<String>,
}

impl AzureProvider {
    /// Create a new Azure provider
    pub fn new(config: &ProviderConfig) -> Result<Self> {
        let endpoint = config
            .base_url
            .clone()
            .ok_or_else(|| GatewayError::Config("Azure endpoint is required".to_string()))?;

        let api_version = config
            .extra
            .get("api_version")
            .cloned()
            .unwrap_or_else(|| "2024-02-15-preview".to_string());

        Ok(Self {
            client: Client::new(),
            api_key: config.api_key.clone(),
            endpoint,
            api_version,
            deployment_name: config.default_model.clone(),
        })
    }
}

#[async_trait]
impl Provider for AzureProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Azure
    }

    async fn chat_completion(&self, mut request: ChatRequest) -> Result<ChatResponse> {
        let deployment = self
            .deployment_name
            .as_ref()
            .ok_or_else(|| GatewayError::Config("Azure deployment name is required".to_string()))?;

        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.endpoint, deployment, self.api_version
        );

        // Azure uses deployment name instead of model
        request.model = deployment.clone();

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(GatewayError::Provider(format!(
                "Azure OpenAI API error {}: {}",
                status, error_text
            )));
        }

        let chat_response: ChatResponse = response.json().await?;
        Ok(chat_response)
    }

    fn default_model(&self) -> Option<&str> {
        self.deployment_name.as_deref()
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn validate_config(&self) -> Result<()> {
        if self.api_key.is_empty() {
            return Err(GatewayError::Config(
                "Azure API key is required".to_string(),
            ));
        }
        if self.endpoint.is_empty() {
            return Err(GatewayError::Config(
                "Azure endpoint is required".to_string(),
            ));
        }
        if self.deployment_name.is_none() {
            return Err(GatewayError::Config(
                "Azure deployment name is required".to_string(),
            ));
        }
        Ok(())
    }
}
