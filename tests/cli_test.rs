use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_help_command() -> Result<()> {
    let mut cmd = Command::cargo_bin("fuckmit")?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "AI-powered git commit message generator",
        ))
        .stdout(predicate::str::contains("--dry-run"));

    Ok(())
}

#[test]
fn test_prompt_init_command() -> Result<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    let mut cmd = Command::cargo_bin("fuckmit")?;

    cmd.current_dir(repo_path).arg("prompt").arg("init");

    cmd.assert().success();

    // Verify the config file was created
    let config_path = repo_path.join(".fuckmit.yml");
    assert!(Path::new(&config_path).exists());

    // Verify the content of the config file
    let config_content = fs::read_to_string(&config_path)?;
    assert!(config_content.contains("system:"));
    assert!(config_content.contains("user:"));
    assert!(config_content.contains("{{diff}}"));

    Ok(())
}

#[test]
fn test_amend_option_in_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("fuckmit")?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--amend"))
        .stdout(predicate::str::contains("Amend the last commit with a new message"));

    Ok(())
}

#[test]
fn test_dry_run_without_git_repo() -> Result<()> {
    // Create a temporary directory that is not a git repo
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    let mut cmd = Command::cargo_bin("fuckmit")?;

    cmd.current_dir(repo_path).arg("--dry-run");

    cmd.assert().failure().stderr(predicate::str::contains(
        "Failed to open git repository. Make sure you're in a git repository.",
    ));

    Ok(())
}

#[test]
fn test_amend_option_error_without_commits() -> Result<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    // Initialize a git repository but don't create any commits
    git2::Repository::init(repo_path)?;

    let mut cmd = Command::cargo_bin("fuckmit")?;

    // Test the --amend option without any commits
    cmd.current_dir(repo_path).arg("--amend").arg("--dry-run");

    cmd.assert().failure().stderr(predicate::str::contains(
        "No commits found to amend message for",
    ));

    Ok(())
}
