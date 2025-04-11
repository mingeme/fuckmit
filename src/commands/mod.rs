pub mod auth;
pub mod completion;
pub mod config;
pub mod generate;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Authentication commands
    Auth(auth::AuthCommand),
    
    /// Generate shell completions
    Completion(completion::CompletionCommand),
    
    /// Configuration commands
    Config(config::ConfigCommand),
}

impl Commands {
    pub async fn execute(&self) -> anyhow::Result<()> {
        match self {
            Commands::Auth(cmd) => cmd.execute().await,
            Commands::Completion(cmd) => cmd.execute().await,
            Commands::Config(cmd) => cmd.execute().await,
        }
    }
}
