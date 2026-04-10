use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Mod {
    pub identifier: String,
    pub name: String,
    #[serde(rename = "abstract")]
    pub abstract_: Option<String>,
    pub authors: Vec<String>,
    pub tags: Vec<String>,
    pub license: String,
    pub repo: String,
    pub asset_match: Option<String>,
    pub resources: ModResources,

    #[serde(default = "default_ksp_version")]
    pub ksp_version: String,

    #[serde(default)]
    pub install: Vec<ModInstallDirective>,

    #[serde(default)]
    pub provides: Vec<String>,
    // map of identifier -> version requirement
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpecifier>,
    // map of identifier -> version requirement
    #[serde(default)]
    pub conflicts: HashMap<String, String>,
    // map of identifier -> version requirement
    #[serde(default)]
    pub recommends: HashMap<String, DependencySpecifier>,

    #[serde(default)]
    pub variants: Vec<ModVariant>,
}

fn default_ksp_version() -> String {
    "1.12".to_string()
}

/// A specifier for a KSP mod version.
#[derive(Debug, Serialize, Deserialize)]
pub enum DependencySpecifier {
    /// The version of the dependency to install
    Version(String),

    /// An expanded configuration for the dependency
    Config {
        /// The version to install
        version: String,

        /// The text to show when choosing a variant of this dependency.
        help_text: Option<String>,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModVariant {
    pub identifier: String,
    pub name: String,
    pub asset_match: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModResources {
    pub bugtracker: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub manual: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModInstallDirective {
    pub file: String,
    pub install_to: String,
}

pub fn find_all_configs(
    configs_dir: &Path,
    filter: &[String],
) -> Result<Vec<Mod>, Box<dyn std::error::Error>> {
    let mut configs = Vec::new();

    for entry in std::fs::read_dir(configs_dir)? {
        let entry = entry?;
        let path = entry.path();
        tracing::debug!(?path, "Checking entry: {:?}", entry.file_name());

        if path.is_dir() {
            configs.extend(find_all_configs(&path, filter)?);
        } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let config_data = std::fs::read_to_string(&path)?;
            let config: Mod = toml::from_str(&config_data)?;
            tracing::debug!(?path, "Loaded mod config: {:?}", config.identifier);
            if filter.is_empty() || filter.contains(&config.identifier) {
                configs.push(config);
            } else {
                tracing::debug!(
                    ?path,
                    "Skipping mod config due to filter: {:?}",
                    config.identifier
                );
            }
        }
    }
    Ok(configs)
}
