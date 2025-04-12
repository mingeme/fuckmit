use clap::Parser;
use fuckmit::commands::Commands;

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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Process commands
    match cli.command {
        Some(command) => command.execute().await?,
        None => {
            // Default behavior: generate commit message
            let dry_run = cli.dry_run;
            let amend = cli.amend;
            let add_all = cli.add_all;
            fuckmit::commands::generate::generate_commit(dry_run, amend, add_all).await?
        }
    }
    
    Ok(())
}
