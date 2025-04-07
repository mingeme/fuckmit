use std::process::Command;
use once_cell::sync::Lazy;

// Define a constant for the version
pub static VERSION: Lazy<String> = Lazy::new(|| {
    get_version().unwrap_or_else(|_| String::from("0.1.0"))
});

// Get version from git tags or fallback to default
fn get_version() -> Result<String, Box<dyn std::error::Error>> {
    // Try to get the latest tag
    let tag_output = Command::new("git")
        .args(["describe", "--tags", "--abbrev=0"])
        .output()?;
    
    if tag_output.status.success() {
        let tag = String::from_utf8(tag_output.stdout)?.trim().to_string();
        
        // Get the current commit hash
        let hash_output = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()?;
        
        if hash_output.status.success() {
            let hash = String::from_utf8(hash_output.stdout)?.trim().to_string();
            
            // Check if the current commit is tagged
            let exact_output = Command::new("git")
                .args(["describe", "--exact-match", "--tags"])
                .output()?;
            
            if exact_output.status.success() {
                // Current commit is tagged, use the tag as version
                Ok(tag)
            } else {
                // Current commit is not tagged, use tag-hash format
                Ok(format!("{}-{}", tag, hash))
            }
        } else {
            // Fallback to just the tag if we can't get the hash
            Ok(tag)
        }
    } else {
        // No tags found, use default version with hash
        let hash_output = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()?;
        
        if hash_output.status.success() {
            let hash = String::from_utf8(hash_output.stdout)?.trim().to_string();
            Ok(format!("v0.1.0-{}", hash))
        } else {
            // Fallback to default version
            Ok(String::from("v0.1.0"))
        }
    }
}
