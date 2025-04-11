use clap::{Args, Subcommand};
use anyhow::Result;

use crate::config::AuthConfig;

#[derive(Args)]
pub struct AuthCommand {
    #[command(subcommand)]
    command: AuthSubcommand,
}

#[derive(Subcommand)]
enum AuthSubcommand {
    /// Add a new provider configuration
    Add {
        /// Provider name (e.g., openai, anthropic)
        provider: String,
        
        /// API key for the provider
        api_key: String,
    },
    
    /// Set the current provider
    Use {
        /// Provider name to use
        provider: String,
    },
    
    /// Set a specific provider property
    Set {
        /// Property path in format provider.property (e.g., openai.model)
        property_path: String,
        
        /// Value to set
        value: String,
    },
    
    /// List all configured providers
    List,
}

impl AuthCommand {
    pub async fn execute(&self) -> Result<()> {
        match &self.command {
            AuthSubcommand::Add { provider, api_key } => {
                println!("Adding provider: {}", provider);
                let mut config = AuthConfig::load()?;
                config.add_provider(provider, api_key)?;
                config.save()?;
                println!("Provider {} added successfully", provider);
            }
            AuthSubcommand::Use { provider } => {
                let mut config = AuthConfig::load()?;
                if config.set_active_provider(provider)? {
                    config.save()?;
                    println!("Active provider set to {}", provider);
                } else {
                    return Err(anyhow::anyhow!("Provider '{}' not found", provider));
                }
            }
            AuthSubcommand::Set { property_path, value } => {
                let parts: Vec<&str> = property_path.split('.').collect();
                if parts.len() != 2 {
                    return Err(anyhow::anyhow!("Invalid property path format. Use provider.property"));
                }
                
                let provider_name = parts[0];
                let property = parts[1];
                
                println!("Setting {}.{} to {}", provider_name, property, value);
                let mut config = AuthConfig::load()?;
                config.set_provider_property(provider_name, property, value)?;
                config.save()?;
                println!("Property updated successfully");
            }
            AuthSubcommand::List => {
                let config = AuthConfig::load()?;
                let active_provider = config.get_active_provider_name();
                
                if !config.has_providers() {
                    println!("No providers configured. Use 'fuckmit auth add <provider> <apiKey>' to add a provider.");
                    return Ok(());
                }
                
                println!("Configured providers:\n");
                println!("{:<2} {:<15} {:<25} ENDPOINT", "", "PROVIDER", "MODEL");
                println!("{}", "-".repeat(80));
                
                for (name, provider_config) in config.get_providers() {
                    let active_marker = if Some(name.as_str()) == active_provider {
                        "*"
                    } else {
                        " "
                    };
                    
                    let model = provider_config.model
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "<not set>".to_string());
                    
                    let endpoint = provider_config.endpoint
                        .as_ref()
                        .map(|e| e.to_string())
                        .unwrap_or_else(|| "<default>".to_string());
                    
                    println!("{:<2} {:<15} {:<25} {}", active_marker, name, model, endpoint);
                }
                
                if active_provider.is_none() {
                    eprintln!("\nNo active provider set. Use 'fuckmit auth use <provider>' to set one.");
                }
            }
        }
        
        Ok(())
    }
}
