use serde::Deserialize;
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct OxideConfig {
    pub project: ProjectConfig,
    pub build: BuildConfig,
}

#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct BuildConfig {
    pub out_dir: PathBuf,
    pub targets: Vec<String>,
}

impl OxideConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config_str = fs::read_to_string("oxidebuild.toml")?;
        let config: OxideConfig = toml::from_str(&config_str)?;
        Ok(config)
    }
}
