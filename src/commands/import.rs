use std::{fs, path::PathBuf};
use inquire::{Select, Text, Confirm};
use walkdir::WalkDir;
use crate::config::ProjectConfig;

fn load_project_config() -> Option<ProjectConfig> {
	let content = fs::read_to_string(".component-manager.toml").ok()?;
	toml::from_str(&content).ok()
}

fn extract_framework_and_style(path: &PathBuf) -> Option<(String, String)> {
	let components: Vec<_> = path.components()
		.map(|c| c.as_os_str().to_string_lossy().to_string())
		.collect();

	if components.get(0).map(|c| c.as_str()) != Some("components") {
		return None;
	}

	if components.len() >= 4 {
		let framework = components.get(1)?.to_string();
		let style = components.get(2)?.to_string();
		return Some((framework, style));
	}

	None
}

pub fn import_component() {
	let config = match load_project_config() {
		Some(cfg) => cfg,
		None => {
			eprintln!("❌ Could not load `.component-manager.toml`. Make sure you run init first.");
			return;
		}
	};

	let mut filtered_paths = vec![];
	let mut filtered_names = vec![];

	for entry in WalkDir::new("components").into_iter().filter_map(|e| e.ok()) {
		let path = entry.path().to_path_buf();
		if !path.is_file() {
			continue;
		}

		if let Some((framework, style)) = extract_framework_and_style(&path) {
			if config.framework.contains(&framework) && config.style.contains(&style) {
				if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
					filtered_paths.push(path.clone());
					filtered_names.push(fname.to_string());
				}
			}
		}
	}

	if filtered_names.is_empty() {
		println!("No components matching your project config.");
		return;
	}

	let selected_name = Select::new("Select a component to import:", filtered_names)
		.prompt()
		.expect("Failed to select component");

	let source = filtered_paths.iter()
		.find(|p| p.file_name().and_then(|n| n.to_str()) == Some(&selected_name))
		.expect("Selected file not found");

	let target_dir = Text::new("Target project directory:")
		.prompt()
		.expect("Failed to read target path");

	let destination = PathBuf::from(&target_dir).join(source.file_name().unwrap());

	if destination.exists() {
		let overwrite = Confirm::new(&format!(
			"File {:?} already exists. Overwrite?", destination.file_name().unwrap()
		))
		.with_default(false)
		.prompt();

		match overwrite {
			Ok(true) => { /* proceed */ }
			Ok(false) => {
				println!("Import cancelled.");
				return;
			}
			Err(_) => {
				eprintln!("Prompt failed, aborting.");
				return;
			}
		}
	}

	match fs::copy(&source, &destination) {
		Ok(_) => println!("✅ Imported to {:?}", destination),
		Err(e) => eprintln!("❌ Error importing component: {}", e),
	}
}
