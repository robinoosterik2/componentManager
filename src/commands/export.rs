use std::fs;
use std::path::Path;
use std::path::PathBuf;
use chrono::Utc;
use inquire::{Select, Text};
use serde::{Serialize, Deserialize};
use whoami;

use crate::config::ProjectConfig;

use crate::commands::dependencies::{ComponentDependencies, DependencyType};

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentMetadata {
    name: String,
    version: String,
    framework: String,
    style: String,
    language: String,
    description: String,
    author: String,
    created_at: String,
    updated_at: String,
    tags: Vec<String>,
    #[serde(default)]
    pub dependencies: ComponentDependencies,
}

pub fn export_component() {
    // Get component name (without extension)
    let name = Text::new("Component name (e.g., Button):")
        .prompt()
        .expect("Failed to read component name");

    let path_input = Text::new("Path to the existing component file:")
        .prompt()
        .expect("Failed to read source path");

    let source_path = PathBuf::from(&path_input);

    if !source_path.exists() || !source_path.is_file() {
        eprintln!("❌ The path does not exist or is not a file.");
        return;
    }

    let ext = source_path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if !crate::utils::SUPPORTED_EXTENSIONS.contains(&ext.as_str()) {
        eprintln!("❌ Unsupported file type: .{}", ext);
        return;
    }

    // Read project config
    let project_config: ProjectConfig = match fs::read_to_string(".component-manager.toml")
        .ok()
        .and_then(|content| toml::from_str(&content).ok()) {
        Some(cfg) => cfg,
        None => {
            eprintln!("❌ Failed to load or parse `.component-manager.toml`.");
            return;
        }
    };

    // Get framework and style for the component
    let framework = Select::new("Select framework:", project_config.framework.clone())
        .prompt()
        .expect("Failed to select framework");
	let style = Select::new("Select style:", project_config.style.clone())
		.prompt()
		.expect("Failed to select style");
	let destination_dir = Path::new("components").join(framework.clone()).join(style.clone()).join(&name);
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

    // Copy the component file
    if let Err(e) = fs::copy(&source_path, &destination) {
        eprintln!("❌ Error exporting component: {}", e);
        return;
    }

    // Get additional metadata
    let description = Text::new("Enter a short description for the component:")
        .with_help_message("This will be shown in the component list")
        .prompt()
        .unwrap_or_default();

    let author = whoami::username();
    let now = Utc::now().to_rfc3339();

    // Create component metadata
    // Initialize and detect dependencies
    let mut dependencies = ComponentDependencies::new();
    // TODO: Implement actual dependency detection
    // dependencies.detect_from_component(&source_path)?;
    
    // Add any framework/plugin dependencies
    if framework == "vue" {
        dependencies.add_dependency(DependencyType::Npm("vue@^3.0.0".to_string()));
    } else if framework == "react" {
        dependencies.add_dependency(DependencyType::Npm("react@^18.0.0".to_string()));
        dependencies.add_dependency(DependencyType::Npm("react-dom@^18.0.0".to_string()));
    }
    
    // Add style dependencies
    if style == "tailwind" {
        dependencies.add_dependency(DependencyType::Npm("tailwindcss@^3.0.0".to_string()));
    }

    let metadata = ComponentMetadata {
        name: name.clone(),
        version: "0.1.0".to_string(),
        framework,
        style,
        language: project_config.language.first().cloned().unwrap_or_default(),
        description,
        author,
        created_at: now.clone(),
        updated_at: now,
        tags: vec![],
        dependencies,
    };

    // Write metadata to toml file
    let metadata_path = destination_dir.join("component.toml");
    let toml_string = toml::to_string_pretty(&metadata).expect("Failed to serialize metadata");
    
    if let Err(e) = fs::write(&metadata_path, toml_string) {
        eprintln!("⚠️  Warning: Failed to create component metadata: {}", e);
    } else {
        println!("✅ Created component metadata at: {}", metadata_path.display());
    }

    println!("✅ Successfully exported component to: {}", destination.display());
}
