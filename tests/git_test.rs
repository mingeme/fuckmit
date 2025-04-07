use anyhow::Result;
use git2::{Repository, Signature};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

// Import the git utilities from our crate
use fuckmit::utils::git;

#[test]
fn test_create_commit() -> Result<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    // Initialize a test git repository
    let repo = Repository::init(repo_path)?;

    // Create a test file
    let test_file_path = repo_path.join("test.txt");
    fs::write(&test_file_path, "Test content")?;

    // Stage the file
    let mut index = repo.index()?;
    index.add_path(Path::new("test.txt"))?;
    index.write()?;

    // Set git config for the test
    let mut config = repo.config()?;
    config.set_str("user.name", "Test User")?;
    config.set_str("user.email", "test@example.com")?;

    // Create a commit
    let message = "Test commit message";
    git::create_commit(&repo, message)?;

    // Verify the commit was created
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;

    assert_eq!(commit.message().unwrap_or(""), message);
    assert_eq!(commit.author().name().unwrap_or(""), "Test User");
    assert_eq!(commit.author().email().unwrap_or(""), "test@example.com");

    Ok(())
}

#[test]
fn test_get_last_commit_diff() -> Result<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    // Initialize a test git repository
    let repo = Repository::init(repo_path)?;

    // Set git config for the test
    let mut config = repo.config()?;
    config.set_str("user.name", "Test User")?;
    config.set_str("user.email", "test@example.com")?;

    // Create a test file
    let test_file_path = repo_path.join("test.txt");
    fs::write(&test_file_path, "Test content")?;

    // Stage the file
    let mut index = repo.index()?;
    index.add_path(Path::new("test.txt"))?;
    index.write()?;

    // Create a commit
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let signature = Signature::now("Test User", "test@example.com")?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )?;

    // Get the diff of the last commit
    let diff = git::get_last_commit_diff(&repo)?;

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
    let repo = Repository::init(repo_path)?;

    // Set git config for the test
    let mut config = repo.config()?;
    config.set_str("user.name", "Test User")?;
    config.set_str("user.email", "test@example.com")?;

    // Create a test file
    let test_file_path = repo_path.join("test.txt");
    fs::write(&test_file_path, "Test content")?;

    // Configure git to use the test user globally for this test
    std::process::Command::new("git")
        .current_dir(repo_path)
        .arg("config")
        .arg("--global")
        .arg("user.name")
        .arg("Test User")
        .output()?;

    std::process::Command::new("git")
        .current_dir(repo_path)
        .arg("config")
        .arg("--global")
        .arg("user.email")
        .arg("test@example.com")
        .output()?;

    // Stage the file
    let mut index = repo.index()?;
    index.add_path(Path::new("test.txt"))?;
    index.write()?;

    // Create a commit using our utility function
    git::create_commit(&repo, "Initial commit")?;

    // Get the original commit message
    let head_before = repo.head()?;
    let commit_before = head_before.peel_to_commit()?;
    assert_eq!(commit_before.message().unwrap_or(""), "Initial commit");

    // Amend the commit with a new message
    let new_message = "Amended commit message";
    git::amend_commit(&repo, new_message)?;

    // Verify the commit was amended by checking the new message
    let head_after = repo.head()?;
    let commit_after = head_after.peel_to_commit()?;

    // Git might add a newline at the end, so we'll trim and compare
    let actual_message = commit_after.message().unwrap_or("").trim();
    assert_eq!(actual_message, new_message);

    Ok(())
}

#[test]
fn test_get_staged_diff() -> Result<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    // Initialize a test git repository
    let repo = Repository::init(repo_path)?;

    // Set git config for the test
    let mut config = repo.config()?;
    config.set_str("user.name", "Test User")?;
    config.set_str("user.email", "test@example.com")?;

    // Create and commit an initial file
    let test_file_path = repo_path.join("test.txt");
    fs::write(&test_file_path, "Initial content")?;

    // Stage the file
    let mut index = repo.index()?;
    index.add_path(Path::new("test.txt"))?;
    index.write()?;

    // Create initial commit
    let signature = Signature::now("Test User", "test@example.com")?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )?;

    // Modify the file
    fs::write(&test_file_path, "Modified content")?;

    // Stage the changes
    index.add_path(Path::new("test.txt"))?;
    index.write()?;

    // Get the diff
    let diff = git::get_staged_diff(&repo)?;

    // Verify the diff contains the expected changes
    assert!(diff.contains("+Modified content"));
    assert!(diff.contains("-Initial content"));

    Ok(())
}
