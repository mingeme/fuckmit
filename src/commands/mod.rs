pub mod auth;
pub mod generate;
pub mod prompt;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Authentication commands
    Auth(auth::AuthCommand),
    
    /// Prompt configuration commands
    Prompt(prompt::PromptCommand),
}

impl Commands {
    pub async fn execute(&self) -> anyhow::Result<()> {
        match self {
            Commands::Auth(cmd) => cmd.execute().await,
            Commands::Prompt(cmd) => cmd.execute().await,
        }
    }
}
