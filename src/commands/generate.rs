use crate::commands::cli::Cli;
use crate::{gateway::LLMGateway, providers::ProviderType, types::ChatMessage};
use anyhow::{Context, Result};
use std::process::Command;
use std::str::FromStr;

/// Generate a commit message using AI
pub async fn generate_commit(cli: &Cli) -> Result<()> {
    // Get git diff
    let diff = get_git_diff()?;
    if diff.trim().is_empty() {
        println!("No changes detected. Please stage some changes first.");
        return Ok(());
    }

    // Initialize the LLM gateway
    let gateway = LLMGateway::from_env()
        .await
        .context("Failed to initialize LLM gateway. Please check your environment variables.")?;

    // Determine which provider to use and model
    let (provider_type, _model_override) = if let Some(provider_str) = &cli.model {
        // Check if it's in provider/model format
        let parts: Vec<&str> = provider_str.split('/').collect();
        if parts.len() == 2 {
            let provider = ProviderType::from_str(parts[0])
                .map_err(|_| anyhow::anyhow!("Invalid provider: {}", parts[0]))?;
            let model = parts[1].to_string();
            (provider, Some(model))
        } else {
            let provider = ProviderType::from_str(provider_str)
                .map_err(|_| anyhow::anyhow!("Invalid provider: {}", provider_str))?;
            (provider, None)
        }
    } else {
        (gateway.default_provider(), None)
    };

    // Check if the specified provider is available
    if !gateway.has_provider(&provider_type) {
        return Err(anyhow::anyhow!(
            "Provider {:?} is not configured. Available providers: {:?}",
            provider_type,
            gateway.available_providers()
        ));
    }

    // Create the prompt for commit message generation
    let system_prompt = create_system_prompt(cli.rules.as_deref());
    let user_prompt = create_user_prompt(&diff, cli.context.as_deref());

    let messages = vec![
        ChatMessage::system(system_prompt),
        ChatMessage::user(user_prompt),
    ];

    println!("Generating commit message using {:?}...", provider_type);

    // Generate the commit message using gateway's unified method
    let response = gateway
        .chat_with_options(
            messages,
            Some(provider_type),
            _model_override,
            Some(cli.max_tokens),
            Some(cli.temperature),
        )
        .await
        .context("Failed to generate commit message")?;

    let commit_message = response
        .content()
        .ok_or_else(|| anyhow::anyhow!("No content in response"))?
        .trim()
        .to_string();

    if cli.dry_run {
        println!("Generated commit message (dry run):");
        println!("---");
        println!("{}", commit_message);
        println!("---");
    } else {
        // Create the actual commit
        create_commit(&commit_message)?;
        println!("Commit created successfully:");
        println!("{}", commit_message);
    }

    Ok(())
}

/// Get the git diff for staged changes
fn get_git_diff() -> Result<String> {
    let output = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .context("Failed to execute git diff command")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Git diff failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let diff = String::from_utf8(output.stdout).context("Git diff output is not valid UTF-8")?;

    // If no staged changes, try to get unstaged changes
    if diff.trim().is_empty() {
        let output = Command::new("git")
            .args(["diff"])
            .output()
            .context("Failed to execute git diff command for unstaged changes")?;

        if output.status.success() {
            let unstaged_diff =
                String::from_utf8(output.stdout).context("Git diff output is not valid UTF-8")?;

            if !unstaged_diff.trim().is_empty() {
                println!("No staged changes found. Showing unstaged changes:");
                println!("Tip: Use 'git add' to stage changes before generating commit message.");
                return Ok(unstaged_diff);
            }
        }
    }

    Ok(diff)
}

/// Create the system prompt for commit message generation
fn create_system_prompt(additional_rules: Option<&str>) -> String {
    let mut prompt = r#"You are an expert at writing clear, concise git commit messages following conventional commit format.

Rules for commit messages:
1. Use conventional commit format: type(scope): description
2. Types: feat, fix, docs, style, refactor, test, chore, perf, ci, build
3. Keep the first line under 50 characters
4. Use imperative mood (e.g., "add" not "added" or "adds")
5. Don't end the subject line with a period
6. If needed, add a blank line and more detailed explanation
7. Don't output in markdown format

Examples:
- feat: add user authentication system
- fix: resolve memory leak in data processing
- docs: update API documentation for v2.0
- refactor: simplify error handling logic"#.to_string();

    if let Some(rules) = additional_rules {
        prompt.push_str(&format!("\n\nAdditional rules:\n{}", rules));
    }

    prompt.push_str("\n\nGenerate a commit message based on the provided git diff.");
    prompt
}

/// Create the user prompt with git diff and optional context
fn create_user_prompt(diff: &str, additional_context: Option<&str>) -> String {
    let mut prompt = String::from("Please generate a commit message for the following changes:");

    if let Some(context) = additional_context {
        prompt.push_str(&format!("\n\nAdditional context about these changes:\n{}", context));
    }

    prompt.push_str(&format!("\n\n```diff\n{}\n```", diff));
    prompt
}

/// Create a git commit with the generated message
fn create_commit(message: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()
        .context("Failed to execute git commit command")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Git commit failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}
