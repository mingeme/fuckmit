//! # Fuckmit - AI-powered Git Commit Message Generator
//!
//! A unified gateway library for multiple LLM providers with git commit message generation.

// Core LLM Gateway modules
pub mod config;
pub mod error;
pub mod gateway;
pub mod providers;
pub mod types;

// Application modules
pub mod commands;

// Re-export main types for convenience
pub use config::{GatewayConfig, ProviderConfig};
pub use error::{GatewayError, Result};
pub use gateway::LLMGateway;
pub use types::{ChatMessage, ChatRequest, ChatResponse, MessageRole};

// Re-export provider types
pub use providers::{Provider, ProviderType};
