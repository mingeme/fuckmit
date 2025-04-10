use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_add_option_in_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("fuckmit")?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--add"))
        .stdout(predicate::str::contains("Add all untracked and modified files before generating commit"));

    Ok(())
}

#[test]
fn test_capital_a_for_amend_option() -> Result<()> {
    let mut cmd = Command::cargo_bin("fuckmit")?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("-A"))
        .stdout(predicate::str::contains("--amend"))
        .stdout(predicate::str::contains("Amend the last commit with a new message"));

    // Make sure the short form is -A, not -a
    let help_output = cmd.output()?.stdout;
    let help_text = String::from_utf8(help_output)?;
    
    // Check that -a is for add, not for amend
    assert!(help_text.contains("-a, --add"), "Expected -a to be for the add option");
    assert!(help_text.contains("-A, --amend"), "Expected -A to be for the amend option");

    Ok(())
}

#[test]
fn test_add_option_error_without_git_repo() -> Result<()> {
    // Create a temporary directory that is not a git repo
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    let mut cmd = Command::cargo_bin("fuckmit")?;

    cmd.current_dir(repo_path).arg("--add").arg("--dry-run");

    cmd.assert().failure().stderr(predicate::str::contains(
        "Failed to open git repository. Make sure you're in a git repository.",
    ));

    Ok(())
}
