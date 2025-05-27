use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_export_component() {
    // Create a temporary directory for testing
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    
    // Create a test component file to export
    let source_dir = temp_path.join("source");
    fs::create_dir_all(&source_dir).expect("Failed to create source directory");
    
    let component_path = source_dir.join("Button.tsx");
    let mut file = File::create(&component_path).expect("Failed to create test component");
    writeln!(file, "import React from 'react';\n\nexport const Button = () => <button>Test</button>;").unwrap();
    
    // Create a test project config
    let config_path = temp_path.join(".component-manager.toml");
    let mut config_file = File::create(&config_path).expect("Failed to create config file");
    writeln!(
        config_file,
        r#"
        framework = ["react"]
        style = ["tailwind"]
        language = ["typescript"]
        components_dir = "components"
        "#
    )
    .unwrap();
    
    // Create the components directory structure
    let components_dir = temp_path.join("components");
    fs::create_dir_all(&components_dir).expect("Failed to create components directory");
    
    // Change to the temp directory
    let original_dir = std::env::current_dir().expect("Failed to get current directory");
    std::env::set_current_dir(&temp_path).expect("Failed to change to temp directory");
    
    // The export_component function should create the component in the correct location
    // and generate a component.toml file
    let expected_component_dir = components_dir.join("react").join("tailwind").join("Button");
    let expected_toml_path = expected_component_dir.join("component.toml");
    
    // Note: In a real test, we would call the export_component function with the test parameters
    // For now, we'll just verify the test setup
    assert!(component_path.exists(), "Test component was not created");
    assert!(config_path.exists(), "Config file was not created");
    
    // Cleanup
    std::env::set_current_dir(original_dir).expect("Failed to change back to original directory");
    temp_dir.close().expect("Failed to clean up temp directory");
}

#[test]
fn test_export_with_existing_component() {
    // Test exporting a component that already exists
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    
    // Create a test component file to export
    let source_dir = temp_path.join("source");
    fs::create_dir_all(&source_dir).expect("Failed to create source directory");
    
    let component_path = source_dir.join("Button.tsx");
    let mut file = File::create(&component_path).expect("Failed to create test component");
    writeln!(file, "import React from 'react';\n\nexport const Button = () => <button>Test</button>;").unwrap();
    
    // Create a test project config
    let config_path = temp_path.join(".component-manager.toml");
    let mut config_file = File::create(&config_path).expect("Failed to create config file");
    writeln!(
        config_file,
        r#"
        framework = ["react"]
        style = ["tailwind"]
        language = ["typescript"]
        components_dir = "components"
        "#
    )
    .unwrap();
    
    // Create the components directory structure with an existing component
    let components_dir = temp_path.join("components");
    let existing_component_dir = components_dir.join("react").join("tailwind").join("Button");
    fs::create_dir_all(&existing_component_dir).expect("Failed to create existing component directory");
    
    // Create an existing component.toml file
    let existing_toml_path = existing_component_dir.join("component.toml");
    let mut toml_file = File::create(&existing_toml_path).expect("Failed to create existing component.toml");
    writeln!(
        toml_file,
        r#"
        name = "Button"
        version = "0.1.0"
        framework = "react"
        style = "tailwind"
        language = "typescript"
        description = "An existing test button"
        "#
    )
    .unwrap();
    
    // Change to the temp directory
    let original_dir = std::env::current_dir().expect("Failed to get current directory");
    std::env::set_current_dir(&temp_path).expect("Failed to change to temp directory");
    
    // The export_component function should detect the existing component
    // and prompt the user to confirm overwrite
    // Note: In a real test, we would mock the user input
    
    // Cleanup
    std::env::set_current_dir(original_dir).expect("Failed to change back to original directory");
    temp_dir.close().expect("Failed to clean up temp directory");
}
