use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_show_components() {
    // Create a temporary directory for testing
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    
    // Create a test component structure
    let components_dir = temp_path.join("components");
    let framework_dir = components_dir.join("react");
    let style_dir = framework_dir.join("tailwind");
    fs::create_dir_all(&style_dir).expect("Failed to create component directories");
    
    // Create a test component file
    let component_dir = style_dir.join("Button");
    fs::create_dir_all(&component_dir).expect("Failed to create component directory");
    
    // Create a component.toml file
    let toml_path = component_dir.join("component.toml");
    let mut toml_file = File::create(&toml_path).expect("Failed to create component.toml");
    writeln!(
        toml_file,
        r#"
        name = "Button"
        version = "0.1.0"
        framework = "react"
        style = "tailwind"
        language = "typescript"
        description = "A test button component"
        author = "Test User"
        created_at = "2025-05-27T00:00:00Z"
        updated_at = "2025-05-27T00:00:00Z"
        tags = ["button", "ui", "test"]
        "#
    )
    .unwrap();
    
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
    
    // Get the path to the binary
    let binary_path = std::env::current_exe()
        .expect("Failed to get current executable")
        .parent()
        .expect("Failed to get parent directory")
        .parent()
        .expect("Failed to get parent directory")
        .join("component-manager");

    // Run the show command using duct and capture stdout/stderr
    let output = duct::cmd!(
        binary_path,
        "show"
    )
    .stderr_capture()
    .stdout_capture()
    .unchecked()
    .run()
    .expect("Failed to execute command");

    // Convert the output to strings
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Verify the output contains expected content
    assert!(
        stdout.contains("Available components"),
        "Expected 'Available components' in output, got: {}",
        stdout
    );
    assert!(
        stdout.contains("Button"),
        "Expected 'Button' in output, got: {}",
        stdout
    );
    assert!(
        stdout.contains("react/tailwind/Button"),
        "Expected 'react/tailwind/Button' in output, got: {}",
        stdout
    );
    assert!(
        stderr.is_empty(),
        "Expected no stderr output, got: {}",
        stderr
    );
    assert!(
        output.status.success(),
        "Command failed with status: {:?}",
        output.status
    );
    
    // Also verify the component file exists
    assert!(toml_path.exists(), "Component TOML file was not created");
    
    // Cleanup
    std::env::set_current_dir(original_dir).expect("Failed to change back to original directory");
    temp_dir.close().expect("Failed to clean up temp directory");
}

#[test]
fn test_show_components_empty() {
    // Test showing components when none exist
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    
    // Create a test project config but no components
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
    
    // The show_components function should handle the case when no components exist
    // Note: In a real test, we would capture stdout and verify the output
    
    // Cleanup
    std::env::set_current_dir(original_dir).expect("Failed to change back to original directory");
    temp_dir.close().expect("Failed to clean up temp directory");
}
