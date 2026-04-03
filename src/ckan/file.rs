use serde::Serialize;

use crate::config;

#[derive(Serialize)]
pub struct CkanFile {
    pub spec_version: u64,
    pub identifier: String,
    pub name: String,
    #[serde(rename = "abstract")]
    pub abstract_: String,
    pub author: Vec<String>,
    pub version: String,
    pub ksp_version: String,
    pub license: String,
    pub release_status: CkanReleaseStatus,
    pub resources: CkanResources,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub provides: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub depends: Vec<CkanDependency>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recommends: Vec<CkanDependency>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conflicts: Vec<CkanDependency>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub install: Vec<CkanInstallDirective>,
    pub download: String,
    pub download_size: u64,
    pub download_hash: CkanDownloadHash,
    pub download_content_type: String,
    pub install_size: u64,
    pub release_date: String,
    pub x_generated_by: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CkanReleaseStatus {
    Stable,
    Testing,
    Development,
}

#[derive(Clone, Serialize)]
pub struct CkanResources {
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub bugtracker: Option<String>,
    pub manual: Option<String>,
}

impl CkanResources {
    pub fn from_config(resources: config::ModResources, repo: &str) -> Self {
        fn resolve(url: Option<String>, repo: &str) -> Option<String> {
            url.map(|u| if u.starts_with('/') {
                format!("https://github.com/{}{}", repo, u)
            } else {
                u
            })
        }
        Self {
            bugtracker: if resources.bugtracker.is_none() {
                Some(format!("https://github.com/{}/issues", repo))
            } else {
                resolve(resources.bugtracker, repo)
            },

            homepage: if resources.homepage.is_none() {
                Some(format!("https://github.com/{}", repo))
            } else {
                resolve(resources.homepage, repo)
            },

            repository: if resources.repository.is_none() {
                Some(format!("https://github.com/{}", repo))
            } else {
                resolve(resources.repository, repo)
            },

            manual: if resources.manual.is_none() {
                Some(format!("https://github.com/{}/wiki", repo))
            } else {
                resolve(resources.manual, repo)
            },
        }
    }
}

#[derive(Clone, Serialize)]
pub struct CkanDependency {
    pub name: String,
}

impl From<(String, String)> for CkanDependency {
    fn from((identifier, _version_req): (String, String)) -> Self {
        Self { name: identifier }
    }
}

#[derive(Clone, Serialize)]
pub struct CkanInstallDirective {
    pub file: String,
    pub install_to: String,
}

impl From<config::ModInstallDirective> for CkanInstallDirective {
    fn from(directive: config::ModInstallDirective) -> Self {
        Self {
            file: directive.file,
            install_to: directive.install_to,
        }
    }
}

#[derive(Serialize)]
pub struct CkanDownloadHash {
    pub sha1: String,
    pub sha256: String,
}