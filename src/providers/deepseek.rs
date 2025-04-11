use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::Provider;
use crate::config::get_commit_config;

pub struct DeepSeekProvider {
    api_key: String,
    model: String,
    endpoint: Option<String>,
    client: Client,
}

#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: Option<u32>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

impl DeepSeekProvider {
    pub fn new(api_key: &str, model: Option<String>, endpoint: Option<String>) -> Self {
        Self {
            api_key: api_key.to_string(),
            model: model.unwrap_or_else(|| "deepseek-coder".to_string()),
            endpoint,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Provider for DeepSeekProvider {
    async fn generate_commit_message(&self, diff: &str) -> Result<String> {
        let commit_config = get_commit_config()?;

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

        let request = DeepSeekRequest {
            model: self.model.clone(),
            messages,
            temperature: 0.7,
            max_tokens: Some(1024),
        };

        let endpoint = self
            .endpoint
            .clone()
            .unwrap_or_else(|| "https://api.deepseek.com/v1/chat/completions".to_string());

        // Send request to DeepSeek API
        let response = self
            .client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?
            .json::<DeepSeekResponse>()
            .await?;

        if response.choices.is_empty() {
            return Err(anyhow::anyhow!("No response from DeepSeek API"));
        }

        Ok(response.choices[0].message.content.trim().to_string())
    }
}
