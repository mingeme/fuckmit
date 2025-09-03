//! AI provider implementations

use crate::error::Result;
use crate::types::{ChatRequest, ChatResponse};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

pub mod azure;
pub mod deepseek;
pub mod openai;
pub mod qwen;

/// Supported AI provider types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProviderType {
    /// OpenAI GPT models
    OpenAI,
    /// Azure OpenAI Service
    Azure,
    /// DeepSeek models
    DeepSeek,
    /// Alibaba Qwen models
    Qwen,
}

impl fmt::Display for ProviderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProviderType::OpenAI => write!(f, "openai"),
            ProviderType::Azure => write!(f, "azure"),
            ProviderType::DeepSeek => write!(f, "deepseek"),
            ProviderType::Qwen => write!(f, "qwen"),
        }
    }
}

impl FromStr for ProviderType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(ProviderType::OpenAI),
            "azure" => Ok(ProviderType::Azure),
            "deepseek" => Ok(ProviderType::DeepSeek),
            "qwen" => Ok(ProviderType::Qwen),
            _ => Err(format!("Unknown provider type: {}", s)),
        }
    }
}

/// Trait that all AI providers must implement
#[async_trait]
pub trait Provider: Send + Sync {
    /// Get the provider type
    fn provider_type(&self) -> ProviderType;

    /// Send a chat completion request
    async fn chat_completion(&self, request: ChatRequest) -> Result<ChatResponse>;

    /// Get the default model for this provider
    fn default_model(&self) -> Option<&str>;

    /// Check if the provider supports streaming
    fn supports_streaming(&self) -> bool {
        false
    }

    /// Validate the configuration for this provider
    fn validate_config(&self) -> Result<()>;
}

/// Factory function to create a provider instance
pub fn create_provider(
    provider_type: ProviderType,
    config: &crate::config::ProviderConfig,
) -> Result<Box<dyn Provider>> {
    match provider_type {
        ProviderType::OpenAI => Ok(Box::new(openai::OpenAIProvider::new(config)?)),
        ProviderType::Azure => Ok(Box::new(azure::AzureProvider::new(config)?)),
        ProviderType::DeepSeek => Ok(Box::new(deepseek::DeepSeekProvider::new(config)?)),
        ProviderType::Qwen => Ok(Box::new(qwen::QwenProvider::new(config)?)),
    }
}
