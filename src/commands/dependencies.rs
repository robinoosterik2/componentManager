use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DependencyType {
    Npm(String),
    Internal(String),  // Path to internal component
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentDependencies {
    pub dependencies: BTreeSet<DependencyType>,
}

impl Default for ComponentDependencies {
    fn default() -> Self {
        Self {
            dependencies: BTreeSet::new(),
        }
    }
}

impl ComponentDependencies {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new dependency
    pub fn add_dependency(&mut self, dep: DependencyType) {
        self.dependencies.insert(dep);
    }

    /// Detect dependencies from component files
    pub fn detect_from_component(&self, _component_path: &Path) -> Result<()> {
        // TODO: Implement actual dependency detection
        // 1. Parse component files for imports
        // 2. Detect npm packages
        // 3. Detect internal component references
        Ok(())
    }

    /// Check for conflicts with other dependencies
    pub fn check_conflicts(&self, other: &Self) -> Vec<(DependencyType, DependencyType)> {
        let mut conflicts = Vec::new();
        
        // Check for version conflicts in npm packages
        let self_npm: BTreeMap<_, _> = self.dependencies.iter()
            .filter_map(|d| match d {
                DependencyType::Npm(pkg) => Some((pkg.split('@').next().unwrap_or(""), pkg)),
                _ => None
            })
            .collect();

        for dep in &other.dependencies {
            if let DependencyType::Npm(other_pkg) = dep {
                if let Some(pkg_name) = other_pkg.split('@').next() {
                    if let Some(self_pkg) = self_npm.get(&pkg_name) {
                        if self_pkg != &other_pkg {
                            conflicts.push((
                                DependencyType::Npm((*self_pkg).to_string()),
                                dep.clone()
                            ));
                        }
                    }
                }
            }
        }

        conflicts
    }

    /// Generate installation commands for missing dependencies
    pub fn generate_install_commands(&self, base_path: &Path) -> Vec<String> {
        let mut commands = Vec::new();
        
        // Group npm packages by version requirements
        let mut npm_deps = Vec::new();
        
        for _dep in &self.dependencies {
            match _dep {
                DependencyType::Npm(pkg) => npm_deps.push(pkg.clone()),
                DependencyType::Internal(path) => {
                    let _full_path = base_path.join(path);
                    // TODO: Handle internal component linking
                    commands.push(format!("# Internal component: {}", path));
                }
            }
        }
        
        if !npm_deps.is_empty() {
            commands.insert(0, format!("npm install --save {}", npm_deps.join(" ")));
        }
        
        commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_conflicts() {
        let mut deps1 = ComponentDependencies::new();
        deps1.add_dependency(DependencyType::Npm("react@^18.0.0".to_string()));
        
        let mut deps2 = ComponentDependencies::new();
        deps2.add_dependency(DependencyType::Npm("react@^17.0.0".to_string()));
        
        let conflicts = deps1.check_conflicts(&deps2);
        assert!(!conflicts.is_empty(), "Should detect version conflict");
    }
    
    #[test]
    fn test_generate_install_commands() {
        let mut deps = ComponentDependencies::new();
        deps.add_dependency(DependencyType::Npm("react@^18.0.0".to_string()));
        deps.add_dependency(DependencyType::Npm("react-dom@^18.0.0".to_string()));
        
        let commands = deps.generate_install_commands(Path::new("."));
        assert!(commands[0].contains("npm install"));
        assert!(commands[0].contains("react@^18.0.0"));
        assert!(commands[0].contains("react-dom@^18.0.0"));
    }
}
