use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

/// Find the root directory of a Git repository
///
/// This function starts from the given directory and traverses up the directory tree
/// until it finds a .git directory, which indicates the root of a Git repository.
///
/// # Arguments
/// * `start_dir` - The directory to start searching from. If None, uses the current directory.
///
/// # Returns
/// * `Ok(PathBuf)` - The absolute path to the Git repository root
/// * `Err` - If no Git repository is found or if there's an error accessing the filesystem
pub fn find_git_repo_root(start_dir: Option<&Path>) -> Result<PathBuf> {
    let start = match start_dir {
        Some(path) => path.to_path_buf(),
        None => std::env::current_dir()?,
    };

    let mut current = start.clone();

    // Traverse up the directory tree until we find a .git directory
    loop {
        let git_dir = current.join(".git");
        if git_dir.exists() && git_dir.is_dir() {
            return Ok(current);
        }

        // Try to go up one directory
        if !current.pop() {
            // We've reached the root of the filesystem without finding a .git directory
            return Err(anyhow!(
                "No Git repository found in '{}' or any parent directory",
                start.display()
            ));
        }
    }
}
