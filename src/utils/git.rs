use anyhow::{anyhow, Result};
use tempfile::TempDir;
use std::path::Path;
use std::process::Command;

use crate::config::get_commit_config;

use super::file_pattern::filter_excluded_files;

/// Get the list of staged file names
pub fn get_staged_files(repo_path: &Path) -> Result<Vec<String>> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["diff", "--staged", "--name-only"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to get staged files: {}", error));
    }

    let files_output = String::from_utf8(output.stdout)?;
    Ok(files_output.lines().map(|s| s.to_string()).collect())
}

/// Get the diff of staged changes
pub fn get_staged_diff(repo_path: &Path) -> Result<String> {
    let files = get_staged_files(repo_path)?;
    let diff_files = filter_excluded_files(files, get_commit_config()?.exclude);

    // Run git diff --staged command to get staged changes
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["diff", "--staged", "--"])
        .args(diff_files)
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to get staged diff: {}", error));
    }

    let diff_output = String::from_utf8(output.stdout)?;
    Ok(diff_output)
}

/// Create a commit with the given message
pub fn create_commit(repo_path: &Path, message: &str) -> Result<()> {
    // Create a temporary file for the commit message
    let temp_dir = TempDir::with_prefix("create_commit")?;
    let message_file = temp_dir.path().join("commit_message.txt");
    std::fs::write(&message_file, message)?;

    // Run git commit command
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["commit", "-F", message_file.to_str().unwrap()])
        .output()?;

    // Clean up the temporary file
    let _ = std::fs::remove_file(message_file);

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to create commit: {}", error));
    }

    Ok(())
}

/// Get the list of files in the last commit
pub fn get_last_commit_files(repo_path: &Path) -> Result<Vec<String>> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["show", "--format=", "--name-only"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to get last commit files: {}", error));
    }

    let files_output = String::from_utf8(output.stdout)?;
    let files: Vec<String> = files_output.lines().map(|s| s.to_string()).collect();
    Ok(files)
}

/// Get the diff of the last commit
pub fn get_last_commit_diff(repo_path: &Path) -> Result<String> {
    let files = get_last_commit_files(repo_path)?;
    let diff_files = filter_excluded_files(files, get_commit_config()?.exclude);
    if diff_files.is_empty() {
        return Err(anyhow!("No last commit files to diff"));
    }
    // Run git show command to get the diff of the last commit
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["show", "--format=%", "HEAD", "--"])
        .args(diff_files)
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to get last commit diff: {}", error));
    }

    let diff_output = String::from_utf8(output.stdout)?;
    Ok(diff_output)
}

/// Amend the last commit with a new message, including any staged changes
pub fn amend_commit(repo_path: &Path, message: &str) -> Result<()> {
    // Create a temporary file for the commit message
    let temp_dir = TempDir::with_prefix("amend_commit")?;
    let message_file = temp_dir.path().join("commit_message.txt");
    std::fs::write(&message_file, message)?;

    // Use git command line to amend the commit, including any staged changes
    let output = Command::new("git")
        .current_dir(repo_path)
        .args([
            "commit",
            "--amend",
            "--file",
            message_file.to_str().unwrap(),
        ])
        .output()?;

    // Clean up the temporary file
    let _ = std::fs::remove_file(message_file);

    // Check if the command was successful
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to amend commit: {}", error));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_create_commit() -> Result<()> {
        // Create a temporary directory for testing
        let temp_dir = TempDir::with_prefix("test_create_commit")?;
        let repo_path = temp_dir.path();

        // Initialize a test git repository
        Command::new("git")
            .current_dir(repo_path)
            .args(["init"])
            .output()?;

        // Create a test file
        let test_file_path = repo_path.join("test.txt");
        fs::write(&test_file_path, "Test content")?;

        // Stage the file
        Command::new("git")
            .current_dir(repo_path)
            .args(["add", "test.txt"])
            .output()?;

        // Set git config for the test
        Command::new("git")
            .current_dir(repo_path)
            .args(["config", "user.name", "Test User"])
            .output()?;
        Command::new("git")
            .current_dir(repo_path)
            .args(["config", "user.email", "test@example.com"])
            .output()?;

        // Create a commit
        let message = "Test commit message";
        create_commit(repo_path, message)?;

        // Verify the commit was created
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["log", "-1", "--pretty=format:%s|%an|%ae"])
            .output()?;

        let commit_info = String::from_utf8(output.stdout)?;
        let parts: Vec<&str> = commit_info.split('|').collect();

        assert_eq!(parts[0], message);
        assert_eq!(parts[1], "Test User");
        assert_eq!(parts[2], "test@example.com");

        Ok(())
    }

    #[test]
    fn test_get_last_commit_diff() -> Result<()> {
        // Create a temporary directory for testing
        let temp_dir = TempDir::with_prefix("test_get_last_commit_diff")?;
        let repo_path = temp_dir.path();

        // Initialize a test git repository
        Command::new("git")
            .current_dir(repo_path)
            .args(["init"])
            .output()?;

        // Set git config for the test
        Command::new("git")
            .current_dir(repo_path)
            .args(["config", "user.name", "Test User"])
            .output()?;
        Command::new("git")
            .current_dir(repo_path)
            .args(["config", "user.email", "test@example.com"])
            .output()?;

        // Create a test file
        let test_file_path = repo_path.join("test.txt");
        fs::write(&test_file_path, "Test content")?;

        // Stage the file
        Command::new("git")
            .current_dir(repo_path)
            .args(["add", "test.txt"])
            .output()?;

        // Create a commit
        Command::new("git")
            .current_dir(repo_path)
            .args(["commit", "-m", "Initial commit"])
            .output()?;

        // Get the diff of the last commit
        let diff = get_last_commit_diff(repo_path)?;

        // Verify the diff contains the expected content
        assert!(diff.contains("+Test content"));
        assert!(diff.contains("test.txt"));

        Ok(())
    }

    #[test]
    fn test_amend_commit() -> Result<()> {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new()?;
        let repo_path = temp_dir.path();

        // Initialize a test git repository
        Command::new("git")
            .current_dir(repo_path)
            .args(["init"])
            .output()?;

        // Set git config for the test
        Command::new("git")
            .current_dir(repo_path)
            .args(["config", "user.name", "Test User"])
            .output()?;
        Command::new("git")
            .current_dir(repo_path)
            .args(["config", "user.email", "test@example.com"])
            .output()?;

        // Create a test file
        let test_file_path = repo_path.join("test.txt");
        fs::write(&test_file_path, "Test content")?;

        // Stage the file
        Command::new("git")
            .current_dir(repo_path)
            .args(["add", "test.txt"])
            .output()?;

        // Create a commit using our utility function
        create_commit(repo_path, "Initial commit")?;

        // Get the original commit message
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["log", "-1", "--pretty=format:%s"])
            .output()?;
        let original_message = String::from_utf8(output.stdout)?;
        assert_eq!(original_message, "Initial commit");

        // Amend the commit with a new message
        let new_message = "Amended commit message";
        amend_commit(repo_path, new_message)?;

        // Verify the commit was amended by checking the new message
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["log", "-1", "--pretty=format:%s"])
            .output()?;
        let amended_message = String::from_utf8(output.stdout)?;

        // Git might add a newline at the end, so we'll trim and compare
        assert_eq!(amended_message.trim(), new_message);

        Ok(())
    }

    #[test]
    fn test_get_staged_diff() -> Result<()> {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new()?;
        let repo_path = temp_dir.path();

        // Initialize a test git repository
        Command::new("git")
            .current_dir(repo_path)
            .args(["init"])
            .output()?;

        // Set git config for the test
        Command::new("git")
            .current_dir(repo_path)
            .args(["config", "user.name", "Test User"])
            .output()?;
        Command::new("git")
            .current_dir(repo_path)
            .args(["config", "user.email", "test@example.com"])
            .output()?;

        // Create and commit an initial file
        let test_file_path = repo_path.join("test.txt");
        fs::write(&test_file_path, "Initial content")?;

        // Stage the file
        Command::new("git")
            .current_dir(repo_path)
            .args(["add", "test.txt"])
            .output()?;

        // Create initial commit
        Command::new("git")
            .current_dir(repo_path)
            .args(["commit", "-m", "Initial commit"])
            .output()?;

        // Modify the file
        fs::write(&test_file_path, "Modified content")?;

        // Stage the changes
        Command::new("git")
            .current_dir(repo_path)
            .args(["add", "test.txt"])
            .output()?;

        // Get the diff
        let diff = get_staged_diff(repo_path)?;

        // Verify the diff contains the expected changes
        assert!(diff.contains("+Modified content"));
        assert!(diff.contains("-Initial content"));

        Ok(())
    }
}
