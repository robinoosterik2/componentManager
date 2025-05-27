use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
    pub framework: Vec<String>,
    pub style: Vec<String>,
    pub language: Vec<String>,
    #[serde(default = "default_components_dir")]
    pub components_dir: PathBuf,
}

fn default_components_dir() -> PathBuf {
    PathBuf::from("./components")
}

impl ProjectConfig {
    pub fn load_from_file() -> Option<Self> {
        let content = std::fs::read_to_string(".component-manager.toml").ok()?;
        toml::from_str(&content).ok()
    }
}

pub fn get_config() -> Result<ProjectConfig, String> {
    ProjectConfig::load_from_file()
        .or_else(|| {
            // If no config file exists, return default config
            Some(ProjectConfig {
                framework: vec!["vue".to_string()],
                style: vec!["css".to_string()],
                language: vec!["javascript".to_string()],
                components_dir: default_components_dir(),
            })
        })
        .ok_or_else(|| "Failed to load or create config".to_string())
}
