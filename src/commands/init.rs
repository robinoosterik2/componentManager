use inquire::MultiSelect;
use std::fs;
use std::path::PathBuf;
use crate::config::ProjectConfig;
use crate::utils::{SUPPORTED_FRAMEWORKS, SUPPORTED_STYLES, SUPPORTED_LANGUAGES};

fn prompt_with_validation(prompt_text: &str, options: Vec<String>) -> Vec<String> {
	loop {
		let ans = MultiSelect::new(prompt_text, options.clone())
			.with_vim_mode(true)
			.prompt();

		match ans {
			Ok(selection) if !selection.is_empty() => return selection,
			Ok(_) => println!("❌ You must select at least one option. Please try again."),
			Err(e) => {
				eprintln!("❌ Prompt failed: {}", e);
				std::process::exit(1);
			}
		}
	}
}

/// Initializes a `.component-manager.toml` file with selected framework and styling options.
pub fn init_config() {
	let frameworks = SUPPORTED_FRAMEWORKS
		.iter()
		.map(|&f| f.to_string())
		.collect::<Vec<String>>();

	let styles = SUPPORTED_STYLES
		.iter()
		.map(|&s| s.to_string())
		.collect::<Vec<String>>();

	let languages = SUPPORTED_LANGUAGES
		.iter()
		.map(|&l| l.to_string())
		.collect::<Vec<String>>();

	let selected_framework = prompt_with_validation("Select a framework:", frameworks);
	let selected_style = prompt_with_validation("Select a styling library:", styles);
	let selected_language = prompt_with_validation("Select a language:", languages);

	let config = ProjectConfig {
		framework: selected_framework,
		style: selected_style,
		language: selected_language,
		components_dir: PathBuf::from("./components"),
	};

	let toml_string = toml::to_string(&config)
		.expect("Failed to serialize config");

	match fs::write(".component-manager.toml", toml_string) {
		Ok(_) => println!("✅ Created `.component-manager.toml` in current directory."),
		Err(e) => eprintln!("❌ Failed to write config: {}", e),
	}
}
