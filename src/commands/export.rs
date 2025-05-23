use std::fs;
use std::path::{Path, PathBuf};
use inquire::{Select, Text};
use crate::config::ProjectConfig;

pub fn export_component() {
	let name = Text::new("Component name (e.g., Button.svelte):")
		.prompt()
		.expect("Failed to read component name");

	let path_input = Text::new("Path to the existing component file:")
		.prompt()
		.expect("Failed to read source path");

	let path = PathBuf::from(&path_input);

	if !path.exists() || !path.is_file() {
		eprintln!("❌ The path does not exist or is not a file.");
		return;
	}

	let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
	if !crate::utils::SUPPORTED_EXTENSIONS.contains(&ext.as_str()) {
		eprintln!("❌ Unsupported file type: .{}", ext);
		return;
	}

	// Read config file
	let config: ProjectConfig = match fs::read_to_string(".component-manager.toml")
		.ok()
		.and_then(|content| toml::from_str(&content).ok()) {
		Some(cfg) => cfg,
		None => {
			eprintln!("❌ Failed to load or parse `.component-manager.toml`.");
			return;
		}
	};

	// Use first values or "unknown"
	let default = "unknown".to_string();
	let framework = config.framework.get(0).unwrap_or(&default);
	let style = config.style.get(0).unwrap_or(&default);

	let destination_dir = Path::new("components").join(framework).join(style);
	if let Err(e) = fs::create_dir_all(&destination_dir) {
		eprintln!("❌ Failed to create destination directory: {}", e);
		return;
	}

	let destination = destination_dir.join(format!("{}.{}", name.trim_end_matches(&format!(".{}", ext)), ext));

	if destination.exists() {
		let overwrite = Select::new("File already exists. Overwrite?", vec!["Yes", "No"])
			.prompt()
			.expect("Failed to read selection");

		if overwrite == "No" {
			println!("❌ Export cancelled.");
			return;
		}
	}

	match fs::copy(&path, &destination) {
		Ok(_) => println!("✅ Exported to {:?}", destination),
		Err(e) => eprintln!("❌ Error exporting component: {}", e),
	}
}
