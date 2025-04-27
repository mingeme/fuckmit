use clap::{command, Parser, Subcommand};

use super::{auth, completion, config, generate};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Generate a commit message without creating a commit
    #[arg(short, long)]
    dry_run: bool,

    /// Amend the last commit with a new message
    #[arg(short = 'A', long)]
    amend: bool,

    /// Add all untracked and modified files before generating commit
    #[arg(short, long)]
    add_all: bool,

    /// Path to a custom auth.yaml configuration file
    #[arg(short = 'c', long = "auth-config")]
    config: Option<String>,
}

impl Cli {
    pub async fn execute(&self) -> anyhow::Result<()> {
        match &self.command {
            Some(command) => command.execute().await?,
            None => {
                // Default behavior: generate commit message
                let dry_run = self.dry_run;
                let amend = self.amend;
                let add_all = self.add_all;
                let config_path = self.config.as_deref();
                generate::generate_commit(dry_run, amend, add_all, config_path).await?
            }
        }
        
        Ok(())
    }
}

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
