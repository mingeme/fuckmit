use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(windows)]
use std::fs as platform_fs;
#[cfg(unix)]
use std::os::unix::fs as platform_fs;

use crate::config::{get_commit_config, CommitConfig};

#[derive(Args)]
pub struct ConfigCommand {
    #[command(subcommand)]
    command: ConfigSubcommand,
}

#[derive(Subcommand)]
enum ConfigSubcommand {
    /// Create a new commit configuration file
    Init {
        /// Create in global config directory instead of current directory
        #[arg(long, short)]
        global: bool,
    },

    /// Show current commit configuration
    Show,

    /// List available configurations
    List,

    /// Use a specific configuration as default
    Use {
        /// Name of the configuration to use (without .fuckmit.yml extension)
        name: String,
    },
}

impl ConfigCommand {
    pub async fn execute(&self) -> Result<()> {
        match &self.command {
            ConfigSubcommand::Init { global } => {
                let target_path = if *global {
                    let config_dir = Self::get_config_dir()?;
                    std::fs::create_dir_all(&config_dir)?;

                    // Use default.fuckmit.yml as the base configuration file
                    config_dir.join("default.fuckmit.yml")
                } else {
                    PathBuf::from(".fuckmit.yml")
                };

                if target_path.exists() {
                    println!(
                        "Commit configuration already exists at {}",
                        target_path.display()
                    );
                } else {
                    // Create default commit config
                    let commit_config = CommitConfig::default();

                    // Save to file
                    let yaml = serde_yaml::to_string(&commit_config)?;
                    std::fs::write(&target_path, yaml)?;

                    println!("Created new config file at {}", target_path.display());
                }

                // If global, also create/update the .fuckmit.yml symlink
                if *global {
                    let config_dir = Self::get_config_dir()?;
                    let symlink_path = config_dir.join(".fuckmit.yml");

                    // Remove existing symlink if it exists
                    if symlink_path.exists() {
                        // Ignore errors when removing existing file
                        let _ = fs::remove_file(&symlink_path);
                    }

                    // Create the symlink to the default configuration
                    if let Err(e) = Self::create_symlink(&target_path, &symlink_path) {
                        // Only warn about symlink creation failure, don't fail the command
                        eprintln!("Warning: Could not create symlink: {}", e);
                    } else {
                        println!("Set as active configuration");
                    }
                }
            }
            ConfigSubcommand::Show => {
                let commit_config = get_commit_config()?;

                println!("{}", serde_yaml::to_string(&commit_config)?);
            }
            ConfigSubcommand::List => {
                let config_dir = Self::get_config_dir()?;
                let entries = fs::read_dir(&config_dir).context(format!(
                    "Failed to read config directory: {}",
                    config_dir.display()
                ))?;

                let mut configs = Vec::new();
                for entry in entries {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() && Self::is_config_file(&path) {
                        configs.push(path);
                    }
                }

                // Check if .fuckmit.yml symlink exists and what it points to
                let symlink_path = config_dir.join(".fuckmit.yml");
                let active_target = if symlink_path.exists() && symlink_path.is_symlink() {
                    fs::read_link(&symlink_path).ok().map(|p| {
                        p.file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string()
                    })
                } else {
                    None
                };

                println!("Available configurations:\n");
                let has_configs = !configs.is_empty();

                for config_path in &configs {
                    let file_name = config_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy();
                    let is_active = match &active_target {
                        Some(target) if target == &file_name => " (active)",
                        _ => "",
                    };

                    // Strip the .fuckmit.yml extension for display
                    let display_name = file_name.to_string();
                    let display_name = display_name
                        .strip_suffix(".fuckmit.yml")
                        .or_else(|| display_name.strip_suffix(".fuckmit.yaml"))
                        .unwrap_or(&display_name);

                    println!("  {}{}", display_name, is_active);
                }

                if !has_configs {
                    println!(
                        "No configurations found. Create one with 'fuckmit config init --global'."
                    );
                }
            }
            ConfigSubcommand::Use { name } => {
                let config_dir = Self::get_config_dir()?;

                // Ensure the config directory exists
                fs::create_dir_all(&config_dir)?;

                // Determine the source file path
                let source_file =
                    if name.ends_with(".fuckmit.yml") || name.ends_with(".fuckmit.yaml") {
                        config_dir.join(name)
                    } else {
                        config_dir.join(format!("{}.fuckmit.yml", name))
                    };

                // Check if the source file exists
                if !source_file.exists() {
                    return Err(anyhow::anyhow!("Configuration '{}' not found", name));
                }

                // Determine the symlink path
                let symlink_path = config_dir.join(".fuckmit.yml");

                // Remove existing symlink if it exists
                if symlink_path.exists() {
                    // Ignore errors when removing existing file
                    let _ = fs::remove_file(&symlink_path);
                }

                // Create the symlink
                match Self::create_symlink(&source_file, &symlink_path) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(anyhow::anyhow!(
                            "Failed to create symlink from {} to {}: {}",
                            source_file.display(),
                            symlink_path.display(),
                            e
                        ));
                    }
                };

                println!("Now using '{}' as the active configuration", name);
            }
        }

        Ok(())
    }

    /// Get the config directory for configurations
    fn get_config_dir() -> Result<PathBuf> {
        // First check if FUCKMIT_CONFIG_DIR environment variable is set
        if let Ok(config_dir) = std::env::var("FUCKMIT_CONFIG_DIR") {
            let mut path = PathBuf::from(config_dir);
            path.push("fuckmit");
            return Ok(path);
        }

        // Fall back to default config directory
        let mut path = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        path.push("fuckmit");
        Ok(path)
    }

    /// Check if a file is a valid config file
    fn is_config_file(path: &Path) -> bool {
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
        file_name.ends_with(".fuckmit.yml") || file_name.ends_with(".fuckmit.yaml")
    }

    /// Create a symlink or copy the file on Windows
    fn create_symlink(source: &Path, dest: &Path) -> Result<()> {
        #[cfg(unix)]
        {
            platform_fs::symlink(source, dest)
                .map_err(|e| anyhow::anyhow!("Failed to create symlink: {}", e))
        }

        #[cfg(windows)]
        {
            // On Windows, we'll just copy the file instead of creating a symlink
            fs::copy(source, dest)
                .map(|_| ())
                .map_err(|e| anyhow::anyhow!("Failed to copy file: {}", e))
        }
    }
}
