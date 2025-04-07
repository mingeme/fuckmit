use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

// Import the Provider trait from our crate
use fuckmit::providers::Provider;

// Mock provider for testing
struct MockProvider {
    response: String,
    call_count: Arc<Mutex<usize>>,
}

impl MockProvider {
    fn new(response: &str) -> Self {
        Self {
            response: response.to_string(),
            call_count: Arc::new(Mutex::new(0)),
        }
    }
    
    async fn get_call_count(&self) -> usize {
        *self.call_count.lock().await
    }
}

#[async_trait]
impl Provider for MockProvider {
    async fn generate_commit_message(&self, _diff: &str) -> Result<String> {
        let mut count = self.call_count.lock().await;
        *count += 1;
        Ok(self.response.clone())
    }
}

#[tokio::test]
async fn test_provider_generate_message() -> Result<()> {
    // Create a mock provider with a predefined response
    let expected_message = "feat: add new feature";
    let provider = MockProvider::new(expected_message);
    
    // Generate a commit message
    let diff = "+Added new feature\n-Removed old code";
    let message = provider.generate_commit_message(diff).await?;
    
    // Verify the message matches the expected response
    assert_eq!(message, expected_message);
    
    // Verify the provider was called once
    assert_eq!(provider.get_call_count().await, 1);
    
    Ok(())
}
