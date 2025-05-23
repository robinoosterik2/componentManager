mod commands;
mod config;
mod utils;

use clap::{Parser, Subcommand};
use commands::{export::export_component, import::import_component, init::init_config};

#[derive(Parser)]
#[command(name = "Component CLI", version, about = "Manage frontend components")]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	Export,
	Import,
	Init,
}

fn main() {
	let cli = Cli::parse();
	match cli.command {
		Commands::Export => export_component(),
		Commands::Import => import_component(),
		Commands::Init => init_config(),
	}
}
