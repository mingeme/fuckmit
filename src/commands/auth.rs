use clap::{Args, Subcommand};
use anyhow::Result;

use crate::config::Config;

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
}

impl AuthCommand {
    pub async fn execute(&self) -> Result<()> {
        match &self.command {
            AuthSubcommand::Add { provider, api_key } => {
                println!("Adding provider: {}", provider);
                let mut config = Config::load()?;
                config.add_provider(provider, api_key)?;
                config.save()?;
                println!("Provider {} added successfully", provider);
            }
            AuthSubcommand::Use { provider } => {
                println!("Setting active provider to: {}", provider);
                let mut config = Config::load()?;
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
                let mut config = Config::load()?;
                config.set_provider_property(provider_name, property, value)?;
                config.save()?;
                println!("Property updated successfully");
            }
        }
        
        Ok(())
    }
}
