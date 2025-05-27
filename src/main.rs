mod commands;
mod config;
mod utils;

use clap::{Parser, Subcommand, Args};
use commands::{
    export::export_component, 
    import::import_component, 
    init::init_config, 
    show::show_components,
    install::install_dependencies,
};

#[derive(Parser)]
#[command(name = "Component CLI", version, about = "Manage frontend components")]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Export a component to the component library
    Export,
    /// Import a component from the component library
    Import,
    /// Initialize component manager configuration
    Init,
    /// List available components
    Show {
        /// Show all components, regardless of project configuration
        #[arg(short, long)]
        all: bool,
    },
    /// Install dependencies for components
    Install {
        /// Specific component to install dependencies for (default: all components)
        component: Option<String>,
    },
}

fn main() {
	let cli = Cli::parse();
	match cli.command {
		Commands::Export => export_component(),
		Commands::Import => import_component(),
		Commands::Init => init_config(),
        Commands::Show { all } => show_components(all),
        Commands::Install { component } => {
            if let Err(e) = install_dependencies(component.as_deref()) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
	}
}
