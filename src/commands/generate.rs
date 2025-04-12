use anyhow::{Context, Result, anyhow};
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;

use crate::config::AuthConfig;
use crate::providers::get_provider;
use crate::utils::git;

pub async fn generate_commit(dry_run: bool, amend: bool, add_all: bool) -> Result<()> {
    // Get the current directory as the repo path
    let repo_path = std::env::current_dir()
        .context("Failed to get current directory")?;
    
    // Check if we're in a git repository
    let git_dir = repo_path.join(".git");
    if !git_dir.exists() {
        return Err(anyhow!("Failed to open git repository. Make sure you're in a git repository."));
    }

    // If add_all is true, add all untracked and modified files to the staging area
    if add_all {
        let output = Command::new("git")
            .current_dir(&repo_path)
            .args(["add", "."])
            .output()
            .context("Failed to execute git add command")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Failed to add files: {}", error));
        }
    }

    // If amend is true, we want to amend the last commit with a new message
    // Otherwise, check if there are staged changes
    if amend {
        // Check if there's a HEAD commit to amend
        let output = Command::new("git")
            .current_dir(&repo_path)
            .args(["rev-parse", "HEAD"])
            .output()
            .context("Failed to check for HEAD commit")?;

        if !output.status.success() {
            return Err(anyhow!("No commits found to amend message for"));
        }
    } else {
        // Check if there are staged changes
        let output = Command::new("git")
            .current_dir(&repo_path)
            .args(["diff", "--cached", "--quiet"])
            .status()
            .context("Failed to check for staged changes")?;

        // Exit code 1 means there are changes, 0 means no changes
        if output.success() {
            return Err(anyhow!(
                "No staged changes found. Stage your changes with 'git add' first."
            ));
        }
    }

    // Get the diff - for amend, include both last commit diff and any staged changes
    // For normal commit, just get staged changes
    let diff = if amend {
        // Get the diff from the last commit
        let last_commit_diff = git::get_last_commit_diff(&repo_path)?;

        // Also check if there are any staged changes to include
        let staged_diff = git::get_staged_diff(&repo_path)?;

        // Combine both diffs if there are staged changes
        if !staged_diff.is_empty() {
            format!("{last_commit_diff}\n\n--- STAGED CHANGES ---\n\n{staged_diff}")
        } else {
            last_commit_diff
        }
    } else {
        let diff = git::get_staged_diff(&repo_path)?;
        if diff.is_empty() {
            return Err(anyhow!("No changes to commit"));
        }
        diff
    };

    // Load config
    let config = AuthConfig::load()?;
    let active_provider = config.get_active_provider()?;

    // Get the provider
    let provider = get_provider(&active_provider)?;

    // Generate commit message with a dynamic loading indicator
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner} Generating commit message...")
            .unwrap(),
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let commit_message = provider.generate_commit_message(&diff).await?;

    // Clear the spinner when done
    spinner.finish_and_clear();

    // Print the generated message
    println!("{}\n", commit_message);

    // Create or amend commit if not in dry-run mode
    if !dry_run {
        if amend {
            git::amend_commit(&repo_path, &commit_message)?;
            println!("Commit amended successfully");
        } else {
            git::create_commit(&repo_path, &commit_message)?;
            println!("Commit created successfully");
        }
    } else if amend {
        println!("Dry run mode - no commit amended");
    } else {
        println!("Dry run mode - no commit created");
    }

    Ok(())
}
