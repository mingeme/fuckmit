use clap::Parser;

#[derive(Parser)]
#[command(name = "fuckmit")]
#[command(about = "AI-powered git commit message generator")]
#[command(version)]
pub struct Cli {
    /// Show the generated message without committing
    #[arg(short, long)]
    pub dry_run: bool,

    /// Specify which AI model to use (openai, azure, deepseek, qwen) or provider/model format (e.g., openai/gpt-4)
    #[arg(short, long)]
    pub model: Option<String>,

    /// Additional rules for commit message generation
    #[arg(short, long)]
    pub rules: Option<String>,

    /// Additional context for the changes
    #[arg(short, long)]
    pub context: Option<String>,

    /// Maximum number of tokens for the generated message
    #[arg(long, default_value = "8192")]
    pub max_tokens: u32,

    /// Temperature for AI generation (0.0 to 2.0)
    #[arg(long, default_value = "0.7")]
    pub temperature: f32,
}

impl Cli {
    pub async fn execute(&self) -> anyhow::Result<()> {
        // Import the generate module
        use super::generate;

        // Execute the generate command with the provided options
        generate::generate_commit(self).await
    }
}
