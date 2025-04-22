use clap::Parser;
use fuckmit::commands::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    cli.execute().await?;

    Ok(())
}
