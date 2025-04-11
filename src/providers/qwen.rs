use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::Provider;
use crate::config::get_commit_config;

pub struct QwenProvider {
    api_key: String,
    client: Client,
    model: String,
}

#[derive(Serialize)]
struct QwenRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum QwenResponse {
    Success {
        choices: Vec<Choice>,
    },
    Error {
        error: ErrorDetails,
        request_id: String,
    },
}

#[derive(Deserialize, Debug)]
struct ErrorDetails {
    message: String,
    #[serde(rename = "type")]
    code: String,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize, Debug)]
struct ResponseMessage {
    content: String,
}

impl QwenProvider {
    pub fn new(api_key: &str, model: Option<String>) -> Self {
        QwenProvider {
            api_key: api_key.to_string(),
            client: Client::new(),
            model: model.unwrap_or_else(|| "qwen-max".to_string()),
        }
    }
}

#[async_trait]
impl Provider for QwenProvider {
    async fn generate_commit_message(&self, diff: &str) -> Result<String> {
        let commit_config = get_commit_config()?;

        let system_prompt = commit_config.prompt.system.replace("{{diff}}", diff);
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

        let request = QwenRequest {
            model: self.model.clone(),
            messages,
            temperature: 0.7,
            max_tokens: 8192,
        };

        let endpoint = "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions";

        // Send request to Qwen API
        let response = self
            .client
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&request)
            .send()
            .await?
            .json::<QwenResponse>()
            .await?;

        match response {
            QwenResponse::Success { choices } => {
                if let Some(choice) = choices.first() {
                    Ok(choice.message.content.trim().to_string())
                } else {
                    Err(anyhow::anyhow!(
                        "No response choices returned from Qwen API"
                    ))
                }
            }
            QwenResponse::Error { error, request_id } => Err(anyhow::anyhow!(
                "Qwen API error (request_id: {}): {} ({})",
                request_id,
                error.message,
                error.code
            )),
        }
    }
}
