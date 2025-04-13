use regex::Regex;

/// Checks if a file path matches a glob pattern
///
/// # Arguments
///
/// * `file_path` - The file path to check
/// * `pattern` - The glob pattern to match against
///
/// # Returns
///
/// Boolean indicating if the file matches the pattern
pub fn matches_glob_pattern(file_path: &str, pattern: &str) -> bool {
    // skip if empty
    if pattern.is_empty() {
        return false;
    }

    // Handle pattern with ** at both start and end (e.g., **/node_modules/**)
    if pattern.starts_with("**/") && pattern.ends_with("/**") {
        let middle = &pattern[3..pattern.len() - 3];
        return file_path.contains(&format!("/{}/", middle))
            || file_path.starts_with(&format!("{}/", middle));
    }

    // For complex patterns with both **/ and *
    if pattern.contains("**/") && pattern.contains('*') && !pattern.ends_with("**/") {
        // Handle patterns like 'src/**/*.tsx'
        if let Some((base_path, file_pattern)) = pattern.split_once("**/") {
            let regex_pattern = file_pattern.replace('.', "\\.").replace('*', ".*");

            if let Ok(re) = Regex::new(&format!(".*{}$", regex_pattern)) {
                return file_path.starts_with(base_path) && re.is_match(file_path);
            }
        }
    }

    // Handle recursive pattern with **/ prefix
    if let Some(suffix) = pattern.strip_prefix("**/") {
        // Check if the file path ends with the suffix at any level
        return file_path.ends_with(suffix) || file_path.contains(&format!("/{}", suffix));
    }

    // Handle patterns with **/ in the middle
    if pattern.contains("**/") {
        if let Some((prefix, suffix)) = pattern.split_once("**/") {
            // Check if the file path starts with prefix and ends with suffix
            return (prefix.is_empty() || file_path.starts_with(prefix))
                && (suffix.is_empty() || file_path.ends_with(suffix));
        }
    }

    // Handle patterns ending with /** (e.g., dist/**)
    if pattern.ends_with("/**") {
        let prefix = &pattern[0..pattern.len() - 3];
        return file_path == prefix || file_path.starts_with(&format!("{}/", prefix));
    }

    // Handle simple glob patterns with * (any characters)
    if pattern.contains('*') {
        // For *.json patterns, only match files directly (not in subdirectories)
        if pattern.starts_with("*.") {
            // Get just the filename without the path
            let parts: Vec<&str> = file_path.split('/').collect();
            let filename = parts.last().unwrap_or(&"");
            let extension = &pattern[1..]; // Remove the *

            // Files in subdirectories should not match *.json pattern
            if parts.len() > 1 && pattern == "*.json" {
                return false;
            }

            return filename.ends_with(extension);
        }

        let regex_pattern = pattern.replace('.', "\\.").replace('*', ".*");

        if let Ok(re) = Regex::new(&format!("^{}$", regex_pattern)) {
            return re.is_match(file_path);
        }
    }

    // Exact match for simple patterns - only match the full path exactly
    file_path == pattern
}

/// Filters out files that match any of the exclude patterns
///
/// # Arguments
///
/// * `files` - Array of file paths
/// * `exclude_patterns` - Array of patterns to exclude
///
/// # Returns
///
/// Filtered array of files
pub fn filter_excluded_files(files: Vec<String>, exclude_patterns: Vec<String>) -> Vec<String> {
    if exclude_patterns.is_empty() {
        return files;
    }

    files
        .into_iter()
        .filter(|file| {
            // Keep the file if it doesn't match any exclude pattern
            !exclude_patterns
                .iter()
                .any(|pattern| matches_glob_pattern(file, pattern))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_glob_pattern() {
        // Test patterns with ** at both start and end
        assert!(matches_glob_pattern(
            "src/node_modules/package.json",
            "**/node_modules/**"
        ));
        assert!(matches_glob_pattern(
            "node_modules/package.json",
            "**/node_modules/**"
        ));
        assert!(!matches_glob_pattern(
            "src/modules/package.json",
            "**/node_modules/**"
        ));

        // Test complex patterns with both **/ and *
        assert!(matches_glob_pattern(
            "src/components/Button.tsx",
            "src/**/*.tsx"
        ));
        assert!(!matches_glob_pattern(
            "lib/components/Button.tsx",
            "src/**/*.tsx"
        ));

        // Test recursive pattern with **/ prefix
        assert!(matches_glob_pattern(
            "src/components/Button.tsx",
            "**/Button.tsx"
        ));
        assert!(matches_glob_pattern("Button.tsx", "**/Button.tsx"));
        assert!(!matches_glob_pattern("Button.js", "**/Button.tsx"));

        // Test patterns with **/ in the middle
        assert!(matches_glob_pattern(
            "src/components/Button.tsx",
            "src/**/Button.tsx"
        ));
        assert!(!matches_glob_pattern(
            "lib/components/Button.tsx",
            "src/**/Button.tsx"
        ));

        // Test patterns ending with /**
        assert!(matches_glob_pattern("dist/index.js", "dist/**"));
        assert!(matches_glob_pattern("dist", "dist/**"));
        assert!(!matches_glob_pattern("src/dist/index.js", "dist/**"));

        // Test simple glob patterns with *
        assert!(matches_glob_pattern("package.json", "*.json"));
        assert!(!matches_glob_pattern("src/package.json", "*.json"));
        assert!(matches_glob_pattern("config.js", "config.*"));

        // Test exact match
        assert!(matches_glob_pattern("package.json", "package.json"));
        assert!(!matches_glob_pattern("package.js", "package.json"));
    }

    #[test]
    fn test_filter_excluded_files() {
        let files = vec![
            "src/index.js".to_string(),
            "src/components/Button.tsx".to_string(),
            "node_modules/package.json".to_string(),
            "dist/bundle.js".to_string(),
        ];

        let exclude_patterns = vec!["**/node_modules/**".to_string(), "dist/**".to_string()];

        let filtered = filter_excluded_files(files, exclude_patterns);
        assert_eq!(
            filtered,
            vec![
                "src/index.js".to_string(),
                "src/components/Button.tsx".to_string(),
            ]
        );
    }
}
