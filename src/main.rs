use clap::{Parser, Subcommand};
use inquire::{Select, Text};
use std::fs;
use std::path::PathBuf;

const SUPPORTED_EXTENSIONS: &[&str] = &["svelte", "vue", "tsx", "jsx"];

#[derive(Parser)]
#[command(name = "Component CLI", version, about = "Manage frontend components")]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// Export a frontend component to the library
	Export,
	/// Import a frontend component into your project
	Import,
}

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Commands::Export => export_component(),
		Commands::Import => import_component(),
	}
}

// /home/robin/Robin/ScoutingDrankKas/drankkas/app/components/AdminCard.vue
fn export_component() {
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

	// Check file extension
	let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
	if !SUPPORTED_EXTENSIONS.contains(&ext.as_str()) {
		eprintln!("❌ Unsupported file type: .{}", ext);
		return;
	}

	let destination = PathBuf::from("components").join(name + "." + &ext);
	if destination.exists() {
		let overwrite = Select::new("File already exists. Overwrite?", vec!["Yes", "No"])
			.prompt()
			.expect("Failed to read selection");

		if overwrite == "No" {
			println!("❌ Export cancelled.");
			return;
		}
	}
	fs::create_dir_all("components").unwrap();

	match fs::copy(&path, &destination) {
		Ok(_) => println!("✅ Exported to {:?}", destination),
		Err(e) => eprintln!("❌ Error exporting component: {}", e),
	}
}


fn import_component() {
	let entries = match fs::read_dir("components") {
		Ok(e) => e,
		Err(_) => {
			eprintln!("No components directory found.");
			return;
		}
	};

	let components: Vec<String> = entries
		.filter_map(|entry| entry.ok())
		.map(|e| e.file_name().into_string().unwrap_or_default())
		.collect();

	if components.is_empty() {
		println!("No components available.");
		return;
	}

	let selected = Select::new("Select a component to import:", components)
		.prompt()
		.expect("Failed to select component");

	let target_dir = Text::new("Target project directory:")
		.prompt()
		.expect("Failed to read target path");

	let source = PathBuf::from("components").join(&selected);
	let destination = PathBuf::from(target_dir).join(&selected);

	match fs::copy(&source, &destination) {
		Ok(_) => println!("✅ Imported to {:?}", destination),
		Err(e) => eprintln!("❌ Error importing component: {}", e),
	}
}
