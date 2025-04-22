use anyhow::Result;
use clap::{Args, CommandFactory, ValueEnum};
use clap_complete::{generate, Shell};
use std::io;

use super::cli::Cli;

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
        let mut cmd = Cli::command();
        let bin_name = "fuckmit".to_string();

        generate(shell, &mut cmd, bin_name, &mut io::stdout());

        Ok(())
    }
}
