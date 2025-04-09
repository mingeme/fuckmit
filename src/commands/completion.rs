use anyhow::Result;
use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Shell};
use std::io;

// Recreate the CLI structure for completion generation
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CompletionCli {
    #[command(subcommand)]
    command: Option<CompletionCommands>,

    /// Generate a commit message without creating a commit
    #[arg(short, long)]
    dry_run: bool,

    /// Amend the last commit with a new message
    #[arg(short, long)]
    amend: bool,
}

#[derive(Subcommand)]
enum CompletionCommands {
    /// Authentication commands
    Auth(AuthCommands),
    
    /// Generate shell completions
    Completion(CompletionCommand),
    
    /// Prompt configuration commands
    Prompt(PromptCommands),
}

#[derive(Args)]
struct AuthCommands {
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

#[derive(Args)]
struct PromptCommands {
    #[command(subcommand)]
    command: PromptSubcommand,
}

#[derive(Subcommand)]
enum PromptSubcommand {
    /// Set the system prompt
    System {
        /// New system prompt
        prompt: String,
    },
    
    /// Set the user prompt
    User {
        /// New user prompt
        prompt: String,
    },
    
    /// Show current prompt configuration
    Show,
}

#[derive(Args)]
pub struct CompletionCommand {
    /// Shell to generate completions for
    #[arg(value_enum)]
    shell: CompletionShell,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum CompletionShell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

impl From<CompletionShell> for Shell {
    fn from(shell: CompletionShell) -> Self {
        match shell {
            CompletionShell::Bash => Shell::Bash,
            CompletionShell::Zsh => Shell::Zsh,
            CompletionShell::Fish => Shell::Fish,
            CompletionShell::PowerShell => Shell::PowerShell,
            CompletionShell::Elvish => Shell::Elvish,
        }
    }
}

impl CompletionCommand {
    pub async fn execute(&self) -> Result<()> {
        // Get the shell
        let shell: Shell = self.shell.into();
        
        // Generate completions using our recreated CLI structure
        let mut cmd = CompletionCli::command();
        let bin_name = "fuckmit".to_string();
        
        generate(shell, &mut cmd, bin_name, &mut io::stdout());
        
        Ok(())
    }
}
