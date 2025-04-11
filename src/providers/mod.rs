pub mod anthropic;
pub mod deepseek;
pub mod openai;
pub mod qwen;

use anyhow::Result;
use async_trait::async_trait;

use crate::config::AuthConfig;

#[async_trait]
pub trait Provider {
    async fn generate_commit_message(&self, diff: &str) -> Result<String>;
}

pub fn get_provider(provider_name: &str) -> Result<Box<dyn Provider>> {
    let config = AuthConfig::load()?;
    let provider_config = config.get_provider_config(provider_name)?;

    match provider_name {
        "qwen" => {
            let provider =
                qwen::QwenProvider::new(&provider_config.api_key, provider_config.model.clone());
            Ok(Box::new(provider))
        }
        "openai" => {
            let provider = openai::OpenAIProvider::new(
                &provider_config.api_key,
                provider_config.model.clone(),
                provider_config.endpoint.clone(),
            );
            Ok(Box::new(provider))
        }
        "anthropic" => {
            let provider = anthropic::AnthropicProvider::new(
                &provider_config.api_key,
                provider_config.model.clone(),
                provider_config.endpoint.clone(),
            );
            Ok(Box::new(provider))
        }
        "deepseek" => {
            let provider = deepseek::DeepSeekProvider::new(
                &provider_config.api_key,
                provider_config.model.clone(),
                provider_config.endpoint.clone(),
            );
            Ok(Box::new(provider))
        }
        _ => Err(anyhow::anyhow!("Unsupported provider: {}", provider_name)),
    }
}
