//! Error types for the LLM Gateway library

use std::fmt;

/// Result type alias for the gateway
pub type Result<T> = std::result::Result<T, GatewayError>;

/// Main error type for the LLM Gateway
#[derive(Debug)]
pub enum GatewayError {
    /// Configuration error
    Config(String),
    /// HTTP request error
    Http(reqwest::Error),
    /// JSON serialization/deserialization error
    Json(serde_json::Error),
    /// Provider-specific error
    Provider(String),
    /// Authentication error
    Auth(String),
    /// Rate limit error
    RateLimit(String),
    /// Invalid request error
    InvalidRequest(String),
    /// Network timeout error
    Timeout(String),
    /// Generic error
    Other(String),
}

impl fmt::Display for GatewayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GatewayError::Config(msg) => write!(f, "Configuration error: {}", msg),
            GatewayError::Http(err) => write!(f, "HTTP error: {}", err),
            GatewayError::Json(err) => write!(f, "JSON error: {}", err),
            GatewayError::Provider(msg) => write!(f, "Provider error: {}", msg),
            GatewayError::Auth(msg) => write!(f, "Authentication error: {}", msg),
            GatewayError::RateLimit(msg) => write!(f, "Rate limit error: {}", msg),
            GatewayError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            GatewayError::Timeout(msg) => write!(f, "Timeout error: {}", msg),
            GatewayError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for GatewayError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GatewayError::Http(err) => Some(err),
            GatewayError::Json(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for GatewayError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            GatewayError::Timeout(err.to_string())
        } else if err.is_status() {
            match err.status() {
                Some(status) if status == 401 => GatewayError::Auth(err.to_string()),
                Some(status) if status == 429 => GatewayError::RateLimit(err.to_string()),
                Some(status) if status.is_client_error() => {
                    GatewayError::InvalidRequest(err.to_string())
                }
                _ => GatewayError::Http(err),
            }
        } else {
            GatewayError::Http(err)
        }
    }
}

impl From<serde_json::Error> for GatewayError {
    fn from(err: serde_json::Error) -> Self {
        GatewayError::Json(err)
    }
}

impl From<std::env::VarError> for GatewayError {
    fn from(err: std::env::VarError) -> Self {
        GatewayError::Config(format!("Environment variable error: {}", err))
    }
}
