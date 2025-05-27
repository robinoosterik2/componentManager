#[derive(Debug, Deserialize)]
pub struct ComponentMetadata {
	pub description: Option<String>,
	pub version: Option<String>,
}
pub struct Component {
	pub name: String,
	pub path: String,
	pub metadata: Option<ComponentMetadata>,
}
impl Component {
	pub fn new(name: String, path: String) -> Self {
		Self {
			name,
			path,
			metadata: None,
		}
	}
	pub fn load_metadata(&mut self) {
		let metadata_path = format!("{}/{}.metadata.toml", self.path, self.name);
		if let Ok(content) = std::fs::read_to_string(&metadata_path) {
			self.metadata = toml::from_str(&content).ok();
		}
	}
}
impl Component {
	pub fn save_metadata(&self) {
		if let Some(metadata) = &self.metadata {
			let metadata_path = format!("{}/{}.metadata.toml", self.path, self.name);
			let content = toml::to_string(metadata).unwrap();
			std::fs::write(metadata_path, content).unwrap();
		}
	}
}
impl Component {
	pub fn load_from_path(path: &str) -> Self {
		let name = path.split('/').last().unwrap_or("unknown").to_string();
		let metadata_path = format!("{}/{}.metadata.toml", path, name);
		let metadata = if std::path::Path::new(&metadata_path).exists() {
			let content = std::fs::read_to_string(&metadata_path).ok();
			toml::from_str(content.unwrap_or("").as_str()).ok()
		} else {
			None
		};
		Self {
			name,
			path: path.to_string(),
			metadata,
		}
	}
}