use anyhow::Result;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

// Import the Config struct from our crate
// Note: This assumes the config module is public in our crate
use fuckmit::config::{Config, CommitConfig};

#[test]
fn test_default_commit_config() {
    let default_config = CommitConfig::default();

    // Check that the default system prompt is not empty
    assert!(!default_config.prompt.system.is_empty());

    // Check that the default user prompt contains the diff placeholder
    assert!(default_config.prompt.user.contains("{{diff}}"));

    // Check that default excludes are empty
    assert!(default_config.exclude.is_empty());

    // Check that with_excludes method works correctly
    let exclude_patterns = vec!["test-pattern.json".to_string()];
    let config_with_excludes = CommitConfig::with_excludes(exclude_patterns.clone());
    assert_eq!(config_with_excludes.exclude, exclude_patterns);
}

#[test]
fn test_save_and_load_config() -> Result<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let config_dir = temp_dir.path().join("config");
    fs::create_dir_all(&config_dir)?;

    // Set the config directory environment variable for testing
    std::env::set_var("FUCKMIT_CONFIG_DIR", config_dir.to_str().unwrap());

    // Create a test config
    let mut config = Config::default();
    config.add_provider("test_provider", "test_api_key")?;
    config.set_active_provider("test_provider")?;

    // Save the config
    config.save()?;

    // Check that the config file was created
    let config_path = config_dir.join("config.yml");
    assert!(Path::new(&config_path).exists());

    // Load the config
    let loaded_config = Config::load()?;

    // Check that the loaded config has the correct provider
    let provider_config = loaded_config.get_provider_config("test_provider")?;
    assert_eq!(provider_config.api_key, "test_api_key");

    // Check that the active provider is correct
    assert_eq!(loaded_config.get_active_provider()?, "test_provider");

    // Clean up
    std::env::remove_var("FUCKMIT_CONFIG_DIR");

    Ok(())
}
