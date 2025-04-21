use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use super::get_config_dir;

/// Represents the mappings between repository paths and configuration names
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Mappings {
    #[serde(default)]
    pub mappings: HashMap<String, String>,
}

/// Get the path to the mappings file
pub fn get_file_path() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    fs::create_dir_all(&config_dir)?;
    Ok(config_dir.join("mappings.toml"))
}

/// Load mappings from the mappings file
pub fn load() -> Result<Mappings> {
    let mapping_file = get_file_path()?;

    if !mapping_file.exists() {
        return Ok(Mappings::default());
    }

    let content = fs::read_to_string(&mapping_file).context(format!(
        "Failed to read mappings file: {}",
        mapping_file.display()
    ))?;

    let mappings: Mappings =
        toml::from_str(&content).context("Failed to parse mappings file as TOML")?;

    Ok(mappings)
}

/// Save mappings to the mappings file
pub fn save(mappings: &Mappings) -> Result<()> {
    let mapping_file = get_file_path()?;

    let content =
        toml::to_string_pretty(mappings).context("Failed to serialize mappings to TOML")?;

    fs::write(&mapping_file, content).context(format!(
        "Failed to write mappings file: {}",
        mapping_file.display()
    ))?;

    Ok(())
}
