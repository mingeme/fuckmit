use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::PathBuf;

use crate::config::{CommitConfig, Config};

#[derive(Args)]
pub struct PromptCommand {
    #[command(subcommand)]
    command: PromptSubcommand,
}

#[derive(Subcommand)]
enum PromptSubcommand {
    /// Create a new commit configuration file
    Init {
        /// Create in global config directory instead of current directory
        #[arg(long, short)]
        global: bool,
    },

    /// Show current commit configuration
    Show,
}

impl PromptCommand {
    pub async fn execute(&self) -> Result<()> {
        match &self.command {
            PromptSubcommand::Init { global } => {
                let target_path = if *global {
                    let mut path = dirs::config_dir()
                        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
                    path.push("fuckmit");
                    std::fs::create_dir_all(&path)?;
                    path.push(".fuckmit.yml");
                    path
                } else {
                    PathBuf::from(".fuckmit.yml")
                };

                if target_path.exists() {
                    println!("Commit configuration already exists at {}", target_path.display());
                    return Ok(());
                }

                // Create default commit config
                let commit_config = CommitConfig::default();

                // Save to file
                let yaml = serde_yaml::to_string(&commit_config)?;
                std::fs::write(&target_path, yaml)?;

                println!("Created commit configuration at {}", target_path.display());
            }
            PromptSubcommand::Show => {
                let config = Config::load()?;
                let commit_config = config.get_commit_config()?;

                println!("Current commit configuration:");
                println!("{}", serde_yaml::to_string(&commit_config)?);
            }
        }

        Ok(())
    }
}
