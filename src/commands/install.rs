use std::path::Path;
use anyhow::Result;
use colored::Colorize;

use crate::config::get_config;
use crate::commands::dependencies::ComponentDependencies;

pub fn install_dependencies(component_name: Option<&str>) -> Result<()> {
    let project_config = get_config().unwrap();
    let components_dir = Path::new(&project_config.components_dir);
    
    if let Some(name) = component_name {
        // Install dependencies for a specific component
        let component_path = components_dir.join(name);
        if !component_path.exists() {
            anyhow::bail!("Component '{}' not found", name);
        }
        
        let deps = load_component_dependencies(&component_path)?;
        install_dependencies_for(&deps, components_dir)
    } else {
        // Install dependencies for all components
        let mut all_deps = ComponentDependencies::new();
        
        // Load all components' dependencies
        if let Ok(entries) = std::fs::read_dir(components_dir) {
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    if let Ok(deps) = load_component_dependencies(&entry.path()) {
                        all_deps.dependencies.extend(deps.dependencies);
                    }
                }
            }
        }
        
        install_dependencies_for(&all_deps, components_dir)
    }
}

fn load_component_dependencies(component_path: &Path) -> Result<ComponentDependencies> {
    let config_path = component_path.join("component.toml");
    if !config_path.exists() {
        anyhow::bail!("No component.toml found in {:?}", component_path);
    }
    
    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| anyhow::anyhow!("Failed to read component.toml: {}", e))?;
        
    let metadata: super::export::ComponentMetadata = toml::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Failed to parse component.toml: {}", e))?;
    
    Ok(metadata.dependencies)
}

fn install_dependencies_for(deps: &ComponentDependencies, base_path: &Path) -> Result<()> {
    let commands = deps.generate_install_commands(base_path);
    
    if commands.is_empty() {
        println!("{} No dependencies to install", "✓".green().bold());
        return Ok(());
    }
    
    println!("{} The following commands will be executed:", "ℹ".blue().bold());
    for cmd in &commands {
        println!("  {}", cmd);
    }
    
    // TODO: Actually execute the commands after user confirmation
    // For now, just print what would be done
    println!("\n{} Run the above commands to install dependencies", "ℹ".blue().bold());
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    
    #[test]
    fn test_load_component_dependencies() {
        let temp_dir = tempdir().unwrap();
        let component_dir = temp_dir.path().join("test-component");
        fs::create_dir_all(&component_dir).unwrap();
        
        // Create a test component.toml
        let toml_content = r#"
            name = "test"
            version = "0.1.0"
            framework = "vue"
            style = "tailwind"
            language = "typescript"
            description = "Test component"
            author = "test"
            created_at = "2023-01-01T00:00:00Z"
            updated_at = "2023-01-01T00:00:00Z"
            tags = []
            
            [dependencies]
            dependencies = [
                { Npm = "vue@^3.0.0" },
                { Npm = "tailwindcss@^3.0.0" },
            ]
        "#;
        
        fs::write(component_dir.join("component.toml"), toml_content).unwrap();
        
        let deps = load_component_dependencies(&component_dir).unwrap();
        assert_eq!(deps.dependencies.len(), 2);
    }
}
