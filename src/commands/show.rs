use std::fs;
use std::path::Path;
use std::collections::BTreeMap;

use serde::Deserialize;
use toml;

use crate::config::get_config;
use crate::utils::{SUPPORTED_FRAMEWORKS, SUPPORTED_STYLES};

#[derive(Debug, Deserialize)]
struct ComponentMetadata {
    name: String,
    version: String,
    framework: String,
    style: String,
    language: String,
    description: Option<String>,
    author: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    tags: Option<Vec<String>>,
    dependencies: Option<Vec<String>>,
}

fn get_component_config(component_path: &Path) -> Option<ComponentMetadata> {
    let config_path = component_path.join("component.toml");
    if !config_path.exists() {
        return None;
    }
    
    let content = match std::fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(_) => {
            println!("Failed to read component config: {}", &config_path.display());
            return None;
        },
    };
    
    match toml::from_str(&content) {
        Ok(config) => Some(config),
        Err(e) => {
            println!("Error parsing component config {}: {}", config_path.display(), e);
            None
        }
    }
}

fn is_compatible(component_config: &ComponentMetadata, project_config: &crate::config::ProjectConfig) -> bool {
    // Check if the component's framework matches the project's frameworks
    let framework_match = project_config.framework.is_empty() ||
        project_config.framework.contains(&component_config.framework);
    
    // Check if the component's style matches the project's styles
    let style_match = project_config.style.is_empty() ||
        project_config.style.contains(&component_config.style);
    
    // Check if the component's language matches the project's languages
    let language_match = project_config.language.is_empty() ||
        project_config.language.contains(&component_config.language);
    
    framework_match && style_match && language_match
}

pub fn show_components(show_all: bool) {
    let project_config = match get_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("Error loading project config: {}", e);
            return;
        }
    };

    let components_dir = Path::new(&project_config.components_dir);
    
    if !components_dir.exists() {
        println!("Components directory not found at: {}", components_dir.display());
        return;
    }

    if !show_all {
        println!("Available components (matching project configuration):");
        println!("Framework: {:?}", project_config.framework);
        println!("Style: {:?}", project_config.style);
        println!("Language: {:?}", project_config.language);
        println!("{:-<40} {}", "-", "-");
        
        let mut has_components = false;
        
        // Check for components in framework/style/component structure
        for framework in &project_config.framework {
            let framework_dir = components_dir.join(framework);
            if !framework_dir.exists() {
                continue;
            }
            
            for style in &project_config.style {
                let style_dir = framework_dir.join(style);
                if !style_dir.exists() {
                    continue;
                }
                
                if let Ok(component_dirs) = fs::read_dir(&style_dir) {
                    for component_dir in component_dirs.flatten() {
                        if let Ok(dir_type) = component_dir.file_type() {
                            if dir_type.is_dir() {
                                if let Some(component_name) = component_dir.file_name().to_str() {
                                    let component_path = component_dir.path();
                                    let config_path = component_path.join("component.toml");
                                    
                                    if config_path.exists() {
                                        match get_component_config(&component_path) {
                                            Some(component_config) => {
                                                if is_compatible(&component_config, &project_config) {
                                                    println!("- {}/{}/{}", framework, style, component_name);
                                                    has_components = true;
                                                }
                                            }
                                            None => {
                                                println!("⚠️  Invalid component config in {}/{}/{}", framework, style, component_name);
                                            }
                                        }
                                    } else {
                                        // If no config found, just show the component
                                        println!("- {}/{}/{}", framework, style, component_name);
                                        has_components = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }        
        if !has_components {
            println!("No compatible components found.");
        }
    } else {
        // Show all components grouped by framework and style
        println!("All available components in {}:\n", components_dir.display());
        
        // Group components by framework and style
        let mut components_by_framework: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();
        
        // Process all components in the components directory
        if let Err(e) = process_components_directory(components_dir, &mut components_by_framework) {
            println!("Error processing components directory: {}", e);
        }
        
        // Sort frameworks and styles for consistent output
        let mut sorted_frameworks: Vec<_> = components_by_framework.keys().collect();
        sorted_frameworks.sort_by_key(|f| {
            SUPPORTED_FRAMEWORKS.iter().position(|&x| x == **f).unwrap_or(usize::MAX)
        });
        
        for framework in sorted_frameworks {
            let styles = components_by_framework.get(framework).unwrap();
            println!("{}:", framework);
            
            // Sort styles
            let mut sorted_styles: Vec<_> = styles.keys().collect();
            sorted_styles.sort_by_key(|s| {
                SUPPORTED_STYLES.iter().position(|&x| x == **s).unwrap_or(usize::MAX)
            });
            
            for style in sorted_styles {
                let components = styles.get(style).unwrap();
                if !components.is_empty() {
                    println!("  - {} ({}):", style, components.len());
                    for component in components {
                        println!("    • {}", component);
                    }
                }
            }
            println!();
        }
    }
}

// TODO combine the following functions into one
/// Process all components in the components directory and populate the components map
fn process_components_directory(
    components_dir: &Path,
    components_by_framework: &mut BTreeMap<String, BTreeMap<String, Vec<String>>>,
) -> std::io::Result<()> {
    for framework_entry in fs::read_dir(components_dir)? {
        let framework_entry = framework_entry?;
        
        // Skip non-directory entries
        if !framework_entry.file_type()?.is_dir() {
            continue;
        }
        
        // Process framework directory
        if let (Some(framework_name), framework_path) = (framework_entry.file_name().to_str(), framework_entry.path()) {
            process_style_directories(framework_name, &framework_path, components_by_framework)?;
        }
    }
    Ok(())
}

/// Process all style directories within a framework directory
fn process_style_directories(
    framework_name: &str,
    framework_path: &Path,
    components_by_framework: &mut BTreeMap<String, BTreeMap<String, Vec<String>>>,
) -> std::io::Result<()> {
    for style_entry in fs::read_dir(framework_path)? {
        let style_entry = style_entry?;
        
        // Skip non-directory entries
        if !style_entry.file_type()?.is_dir() {
            continue;
        }
        
        // Process style directory
        if let (Some(style_name), style_path) = (style_entry.file_name().to_str(), style_entry.path()) {
            process_component_directories(
                framework_name,
                style_name,
                &style_path,
                components_by_framework,
            )?;
        }
    }
    Ok(())
}

/// Process all component directories within a style directory
fn process_component_directories(
    framework_name: &str,
    style_name: &str,
    style_path: &Path,
    components_by_framework: &mut BTreeMap<String, BTreeMap<String, Vec<String>>>,
) -> std::io::Result<()> {
    for component_entry in fs::read_dir(style_path)? {
        let component_entry = component_entry?;
        
        // Skip non-directory entries
        if !component_entry.file_type()?.is_dir() {
            continue;
        }
        
        // Process component directory
        if let Some(component_name) = component_entry.file_name().to_str() {
            let component_path = component_entry.path();
            
            // Only include components with a valid config
            if get_component_config(&component_path).is_some() {
                components_by_framework
                    .entry(framework_name.to_string())
                    .or_default()
                    .entry(style_name.to_string())
                    .or_default()
                    .push(component_name.to_string());
            }
        }
    }
    Ok(())
}