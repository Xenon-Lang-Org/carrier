use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct XnConfig {
    pub compiler_path: String,
    pub interpreter_path: String,
    pub vm_path: String,
    pub project_name: String,
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<XnConfig> {
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;
    let config: XnConfig = toml::from_str(&contents)
        .with_context(|| format!("Failed to parse toml from: {}", path.as_ref().display()))?;
    Ok(config)
}

pub fn save_config<P: AsRef<Path>>(config: &XnConfig, path: P) -> Result<()> {
    let serialized = toml::to_string_pretty(config)?;
    fs::write(&path, serialized)
        .with_context(|| format!("Failed to write config file: {}", path.as_ref().display()))?;
    Ok(())
}
