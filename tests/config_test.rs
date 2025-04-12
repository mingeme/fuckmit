use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

// Helper function to set up a temporary config directory
fn setup_temp_config_dir() -> TempDir {
    let temp_dir = tempfile::tempdir().unwrap();
    // Create the fuckmit directory inside the temp directory
    let fuckmit_dir = temp_dir.path().join("fuckmit");
    fs::create_dir_all(&fuckmit_dir).unwrap();
    temp_dir
}

#[test]
fn test_config_init_global() {
    let temp_dir = setup_temp_config_dir();
    let config_dir = temp_dir.path().to_str().unwrap();

    // Run the config init --global command
    let mut cmd = Command::cargo_bin("fuckmit").unwrap();
    let assert = cmd
        .env("FUCKMIT_CONFIG_DIR", config_dir)
        .arg("config")
        .arg("init")
        .arg("--global")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("Created new config file"));

    // Verify that default.fuckmit.yml was created
    let default_config = temp_dir.path().join("fuckmit").join("default.fuckmit.yml");
    assert!(default_config.exists(), "default.fuckmit.yml should exist");
}

#[test]
fn test_config_list() {
    let temp_dir = setup_temp_config_dir();
    let config_dir = temp_dir.path().to_str().unwrap();
    let fuckmit_dir = temp_dir.path().join("fuckmit");

    // Create default config first to ensure it exists
    let mut cmd = Command::cargo_bin("fuckmit").unwrap();
    cmd.env("FUCKMIT_CONFIG_DIR", config_dir)
        .arg("config")
        .arg("init")
        .arg("--global")
        .assert()
        .success();

    // Create additional test configurations
    fs::write(
        fuckmit_dir.join("custom1.fuckmit.yml"),
        "prompt:\n  system: Custom system prompt 1\n  user: Custom user prompt 1",
    )
    .unwrap();
    fs::write(
        fuckmit_dir.join("custom2.fuckmit.yml"),
        "prompt:\n  system: Custom system prompt 2\n  user: Custom user prompt 2",
    )
    .unwrap();

    // Run the config list command
    let mut cmd = Command::cargo_bin("fuckmit").unwrap();
    let assert = cmd
        .env("FUCKMIT_CONFIG_DIR", config_dir)
        .arg("config")
        .arg("list")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("Available configurations:"))
        .stdout(predicate::str::contains("default"))
        .stdout(predicate::str::contains("custom1"))
        .stdout(predicate::str::contains("custom2"));
}

#[test]
fn test_config_use() {
    let temp_dir = setup_temp_config_dir();
    let config_dir = temp_dir.path().to_str().unwrap();
    let fuckmit_dir = temp_dir.path().join("fuckmit");

    // Create default config first to ensure it exists
    let mut cmd = Command::cargo_bin("fuckmit").unwrap();
    cmd.env("FUCKMIT_CONFIG_DIR", config_dir)
        .arg("config")
        .arg("init")
        .arg("--global")
        .assert()
        .success();

    // Create a custom configuration
    let custom_content = "prompt:\n  system: Custom system prompt\n  user: Custom user prompt";
    let custom_path = fuckmit_dir.join("custom.fuckmit.yml");
    fs::write(&custom_path, custom_content).unwrap();

    // Use the custom configuration
    let mut cmd = Command::cargo_bin("fuckmit").unwrap();
    cmd.env("FUCKMIT_CONFIG_DIR", config_dir)
        .arg("config")
        .arg("use")
        .arg("custom")
        .assert()
        .success();

    // Verify that the symlink was created and points to the custom config
    let symlink = fuckmit_dir.join(".fuckmit.yml");
    assert!(symlink.exists(), ".fuckmit.yml symlink should exist");

    // Run the config list command to verify that custom is marked as active
    let mut cmd = Command::cargo_bin("fuckmit").unwrap();
    let assert = cmd
        .env("FUCKMIT_CONFIG_DIR", config_dir)
        .arg("config")
        .arg("list")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("custom (active)"));
}

#[test]
fn test_config_use_nonexistent() {
    let temp_dir = setup_temp_config_dir();
    let config_dir = temp_dir.path().to_str().unwrap();

    // Try to use a non-existent configuration
    let mut cmd = Command::cargo_bin("fuckmit").unwrap();
    cmd.env("FUCKMIT_CONFIG_DIR", config_dir)
        .arg("config")
        .arg("use")
        .arg("nonexistent")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Configuration 'nonexistent' not found",
        ));
}

#[test]
fn test_config_show() {
    // Test showing the default global configuration when no local config exists
    let temp_dir = setup_temp_config_dir();
    let config_dir = temp_dir.path().to_str().unwrap();

    // Change to the temporary directory to use the global config
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Create default config first to ensure it exists
    let mut cmd = Command::cargo_bin("fuckmit").unwrap();
    cmd.env("FUCKMIT_CONFIG_DIR", config_dir)
        .arg("config")
        .arg("init")
        .arg("--global")
        .assert()
        .success();

    // Run the config show command (should use the global default config)
    let mut cmd = Command::cargo_bin("fuckmit").unwrap();
    let assert = cmd
        .env("FUCKMIT_CONFIG_DIR", config_dir)
        .arg("config")
        .arg("show")
        .assert();

    assert.success().stdout(predicate::str::contains(
        "generates clear and concise git commit messages",
    ));

    // Restore the original working directory
    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_config_show_local() {
    // Test showing a custom configuration when it's set as active
    let temp_dir = setup_temp_config_dir();
    let config_dir = temp_dir.path().to_str().unwrap();
    let fuckmit_dir = temp_dir.path().join("fuckmit");

    // Create a custom configuration
    let custom_content =
        "prompt:\n  system: Custom system prompt\n  user: Custom user prompt\nexclude: []";
    let custom_path = fuckmit_dir.join(".fuckmit.yml");
    fs::write(&custom_path, custom_content).unwrap();

    // Run the config show command (should use the local config)
    let mut cmd = Command::cargo_bin("fuckmit").unwrap();
    let assert = cmd
        .env("FUCKMIT_CONFIG_DIR", config_dir)
        .current_dir(fuckmit_dir)
        .arg("config")
        .arg("show")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("Custom system prompt"))
        .stdout(predicate::str::contains("Custom user prompt"));
}
