use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
	pub framework: Vec<String>,
	pub style: Vec<String>,
	pub language: Vec<String>,
}


impl ProjectConfig {
	pub fn load_from_file() -> Option<Self> {
		let content = std::fs::read_to_string(".component-manager.toml").ok()?;
		toml::from_str(&content).ok()
	}
}
