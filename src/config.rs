use std::collections::HashMap;

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

    pub install: Vec<ModInstallDirective>,

    #[serde(default)]
    pub provides: Vec<String>,
    // map of identifier -> version requirement
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    // map of identifier -> version requirement
    #[serde(default)]
    pub conflicts: HashMap<String, String>,
    // map of identifier -> version requirement
    #[serde(default)]
    pub recommends: HashMap<String, String>,

    #[serde(default)]
    pub variants: Vec<ModVariant>,
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
