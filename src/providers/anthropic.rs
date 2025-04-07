use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use super::Provider;

pub struct AnthropicProvider {
    api_key: String,
    model: String,
    endpoint: Option<String>,
    client: Client,
}

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    text: String,
}

impl AnthropicProvider {
    pub fn new(api_key: &str, model: Option<String>, endpoint: Option<String>) -> Self {
        Self {
            api_key: api_key.to_string(),
            model: model.unwrap_or_else(|| "claude-3-opus-20240229".to_string()),
            endpoint,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    async fn generate_commit_message(&self, diff: &str) -> Result<String> {
        let config = Config::load()?;
        let commit_config = config.get_commit_config()?;
        
        let system_prompt = commit_config.prompt.system;
        let user_prompt = commit_config.prompt.user.replace("{{diff}}", diff);
        
        let messages = vec![
            Message {
                role: "system".to_string(),
                content: system_prompt,
            },
            Message {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];
        
        let request = AnthropicRequest {
            model: self.model.clone(),
            messages,
            max_tokens: 1000,
            temperature: 0.7,
        };
        
        let endpoint = self.endpoint.clone().unwrap_or_else(|| 
            "https://api.anthropic.com/v1/messages".to_string()
        );
        
        // Send request to Anthropic API
        let response = self.client
            .post(&endpoint)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?
            .json::<AnthropicResponse>()
            .await?;
        
        if response.content.is_empty() {
            return Err(anyhow::anyhow!("No response from Anthropic API"));
        }
        
        Ok(response.content[0].text.trim().to_string())
    }
}
