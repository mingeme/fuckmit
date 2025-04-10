use anyhow::Result;
use git2::{Repository, Signature, DiffOptions};
use glob::Pattern;



use crate::config::Config;

/// Get the diff of staged changes
pub fn get_staged_diff(repo: &Repository) -> Result<String> {
    let config = Config::load()?;
    let commit_config = config.get_commit_config().unwrap_or_default();
    
    // Prepare diff options
    let mut diff_opts = DiffOptions::new();
    diff_opts.include_untracked(true)
             .recurse_untracked_dirs(true)
             .show_binary(false);
    
    // Get the diff between HEAD and index
    let diff = repo.diff_tree_to_index(
        repo.head()
            .ok()
            .and_then(|h| h.target())
            .and_then(|oid| repo.find_commit(oid).ok())
            .and_then(|c| c.tree().ok())
            .as_ref(),
        None,
        Some(&mut diff_opts),
    )?;
    
    // Convert diff to string
    let mut diff_str = String::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        let content = std::str::from_utf8(line.content()).unwrap_or("");
        
        // Check if this file should be excluded based on patterns
        if let Some(file_path) = _delta.new_file().path() {
            let file_path_str = file_path.to_string_lossy();
            for exclude_pattern in &commit_config.exclude {
                if let Ok(pattern) = Pattern::new(exclude_pattern) {
                    if pattern.matches(&file_path_str) {
                        // Skip excluded files silently
                        return true;
                    }
                } else {
                    eprintln!("Warning: Invalid exclude pattern: {}", exclude_pattern);
                }
            }
        }
        
        match line.origin() {
            '+' | '-' | ' ' => diff_str.push(line.origin()),
            _ => {}
        }
        
        diff_str.push_str(content);
        true
    })?;
    
    Ok(diff_str)
}

/// Create a commit with the given message
pub fn create_commit(repo: &Repository, message: &str) -> Result<()> {
    let signature = get_signature(repo)?;
    
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    
    let parent = match repo.head() {
        Ok(head) => {
            let head_target = head.target()
                .ok_or_else(|| anyhow::anyhow!("Failed to get HEAD target"))?;
            Some(repo.find_commit(head_target)?)
        },
        Err(_) => None,
    };
    
    let parents = match parent {
        Some(ref p) => vec![p],
        None => vec![],
    };
    
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        parents.as_slice(),
    )?;
    
    Ok(())
}

/// Get the diff of the last commit
pub fn get_last_commit_diff(repo: &Repository) -> Result<String> {
    let config = Config::load()?;
    let commit_config = config.get_commit_config().unwrap_or_default();
    
    // Get the last commit
    let head = repo.head()?
        .target()
        .ok_or_else(|| anyhow::anyhow!("Failed to get HEAD target"))?;
    
    let commit = repo.find_commit(head)?
        .clone();
    
    // Get parent commit
    let parent = if commit.parent_count() > 0 {
        Some(commit.parent(0)?)
    } else {
        None
    };
    
    // Prepare diff options
    let mut diff_opts = DiffOptions::new();
    diff_opts.show_binary(false);
    
    // Get the diff between the commit and its parent (or empty tree if no parent)
    let diff = if let Some(parent) = parent {
        let parent_tree = parent.tree()?;
        let commit_tree = commit.tree()?;
        
        repo.diff_tree_to_tree(
            Some(&parent_tree),
            Some(&commit_tree),
            Some(&mut diff_opts),
        )?
    } else {
        // First commit - compare with empty tree
        let commit_tree = commit.tree()?;
        repo.diff_tree_to_tree(
            None,
            Some(&commit_tree),
            Some(&mut diff_opts),
        )?
    };
    
    // Convert diff to string
    let mut diff_str = String::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        let content = std::str::from_utf8(line.content()).unwrap_or("");
        
        // Check if this file should be excluded based on patterns
        if let Some(file_path) = _delta.new_file().path() {
            let file_path_str = file_path.to_string_lossy();
            for exclude_pattern in &commit_config.exclude {
                if let Ok(pattern) = Pattern::new(exclude_pattern) {
                    if pattern.matches(&file_path_str) {
                        // Skip excluded files silently
                        return true;
                    }
                } else {
                    eprintln!("Warning: Invalid exclude pattern: {}", exclude_pattern);
                }
            }
        }
        
        match line.origin() {
            '+' | '-' | ' ' => diff_str.push(line.origin()),
            _ => {}
        }
        
        diff_str.push_str(content);
        true
    })?;
    
    Ok(diff_str)
}

/// Amend the last commit with a new message, including any staged changes
pub fn amend_commit(repo: &Repository, message: &str) -> Result<()> {
    // Get the repository path
    let repo_path = repo.path()
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Failed to get repository path"))?;
    
    // Create a temporary file for the commit message
    let temp_dir = std::env::temp_dir();
    let message_file = temp_dir.join("commit_message.txt");
    std::fs::write(&message_file, message)?;
    
    // Use git command line to amend the commit, including any staged changes
    let output = std::process::Command::new("git")
        .current_dir(repo_path)
        .arg("commit")
        .arg("--amend")
        .arg("--file")
        .arg(&message_file)
        .arg("--no-edit") // Include staged changes without opening editor
        .output()?;
    
    // Clean up the temporary file
    let _ = std::fs::remove_file(message_file);
    
    // Check if the command was successful
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to amend commit: {}", error));
    }
    
    Ok(())
}

/// Get the git signature from config or default
pub fn get_signature(repo: &Repository) -> Result<Signature<'static>> {
    let config = repo.config()?;
    
    let name = config.get_string("user.name")
        .unwrap_or_else(|_| String::from("Unknown"));
    
    let email = config.get_string("user.email")
        .unwrap_or_else(|_| String::from("unknown@example.com"));
    
    Ok(Signature::now(&name, &email)?)
}

/// Add all untracked and modified files to the staging area
pub fn add_all_files(repo: &Repository) -> Result<()> {
    // Use git2 to stage all modified and untracked files
    let mut index = repo.index()?;
    
    // Get the status of all files in the repository
    let statuses = repo.statuses(Some(
        git2::StatusOptions::new()
            .include_untracked(true)
            .recurse_untracked_dirs(true)
    ))?;
    
    // Add each modified or untracked file to the index
    for entry in statuses.iter() {
        if let Some(path) = entry.path() {
            // Check if the file is modified, new, or renamed but not staged
            if entry.status().is_wt_modified() || 
               entry.status().is_wt_new() || 
               entry.status().is_wt_renamed() || 
               entry.status().is_wt_typechange() {
                index.add_path(std::path::Path::new(path))?;
            }
        }
    }
    
    // Write the updated index back to disk
    index.write()?;
    
    Ok(())
}
