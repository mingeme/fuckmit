use anyhow::Result;
use git2::{Repository, StatusOptions};
use std::fs;
use tempfile::TempDir;

// Import the git utilities from our crate
use fuckmit::utils::git;

#[test]
fn test_add_all_files() -> Result<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    // Initialize a test git repository
    let repo = Repository::init(repo_path)?;

    // Set git config for the test
    let mut config = repo.config()?;
    config.set_str("user.name", "Test User")?;
    config.set_str("user.email", "test@example.com")?;

    // Create multiple test files but don't stage them
    let test_file1_path = repo_path.join("test1.txt");
    fs::write(&test_file1_path, "Test content 1")?;
    
    let test_file2_path = repo_path.join("test2.txt");
    fs::write(&test_file2_path, "Test content 2")?;
    
    // Create a subdirectory with a file
    fs::create_dir_all(repo_path.join("subdir"))?;
    let test_file3_path = repo_path.join("subdir/test3.txt");
    fs::write(&test_file3_path, "Test content 3")?;

    // Verify that no files are staged initially
    let statuses = repo.statuses(Some(StatusOptions::new().include_untracked(true)))?;
    let has_staged = statuses.iter().any(|s| s.status().is_index_new() || 
                                       s.status().is_index_modified() || 
                                       s.status().is_index_deleted() || 
                                       s.status().is_index_renamed() || 
                                       s.status().is_index_typechange());
    assert!(!has_staged, "Expected no staged files initially");
    
    // Use the add_all_files function
    git::add_all_files(&repo)?;
    
    // Verify that all files are now staged
    let statuses_after = repo.statuses(Some(StatusOptions::new().include_untracked(true)))?;
    
    // Check that all files are now in the index
    let all_staged = statuses_after.iter().all(|s| s.status().is_index_new() || 
                                             s.status().is_index_modified() || 
                                             s.status().is_index_deleted() || 
                                             s.status().is_index_renamed() || 
                                             s.status().is_index_typechange());
    assert!(all_staged, "Expected all files to be staged after add_all_files");
    
    // Count the number of staged files to ensure we have all 3
    let staged_count = statuses_after.iter().filter(|s| s.status().is_index_new()).count();
    assert_eq!(staged_count, 3, "Expected 3 staged files");

    Ok(())
}
