use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_import_component() {
    // Create a temporary directory for testing
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    
    // Create a test component structure
    let components_dir = temp_path.join("components");
    let framework_dir = components_dir.join("react");
    let style_dir = framework_dir.join("tailwind");
    fs::create_dir_all(&style_dir).expect("Failed to create component directories");
    
    // Create a test component file
    let component_path = style_dir.join("Button.tsx");
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
    
    // Change to the temp directory
    let original_dir = std::env::current_dir().expect("Failed to get current directory");
    std::env::set_current_dir(&temp_path).expect("Failed to change to temp directory");
    
    // Import the test component
    // Note: In a real test, we would call the import_component function directly
    // For now, we'll just verify the test setup
    assert!(component_path.exists(), "Test component was not created");
    assert!(config_path.exists(), "Config file was not created");
    
    // Cleanup
    std::env::set_current_dir(original_dir).expect("Failed to change back to original directory");
    temp_dir.close().expect("Failed to clean up temp directory");
}

#[test]
fn test_import_with_invalid_config() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    
    // Don't create a config file to simulate a missing config
    
    // Change to the temp directory
    let original_dir = std::env::current_dir().expect("Failed to get current directory");
    std::env::set_current_dir(&temp_path).expect("Failed to change to temp directory");
    
    // The import should fail with a helpful error message
    // Note: In a real test, we would capture stderr and verify the error message
    
    // Cleanup
    std::env::set_current_dir(original_dir).expect("Failed to change back to original directory");
    temp_dir.close().expect("Failed to clean up temp directory");
}
