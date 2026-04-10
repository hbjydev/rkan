use serde::Serialize;

use crate::config::{self, DependencySpecifier};

pub const LATEST_SPEC_VERSION: &str = "v1.34";

#[derive(Serialize, Default)]
pub struct CkanFile {
    pub spec_version: String,
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

#[derive(Clone, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum CkanReleaseStatus {
    #[default]
    Stable,
    Testing,
    #[allow(dead_code)]
    Development,
}

#[derive(Clone, Serialize, Default)]
pub struct CkanResources {
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub bugtracker: Option<String>,
    pub manual: Option<String>,
}

impl CkanResources {
    pub fn from_config(resources: config::ModResources, repo: &str) -> Self {
        fn resolve(url: Option<String>, repo: &str) -> Option<String> {
            url.map(|u| {
                if u.starts_with('/') {
                    format!("https://github.com/{}{}", repo, u)
                } else {
                    u
                }
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

#[derive(Clone, Serialize, Default)]
pub struct CkanDependency {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice_help_text: Option<String>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub version_spec: Option<CkanDependencyVersionSpecifier>,
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum CkanDependencyVersionSpecifier {
    Exact(String),
    MinMax {
        min_version: Option<String>,
        max_version: Option<String>,
    },
}

impl Serialize for CkanDependencyVersionSpecifier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        match self {
            Self::Exact(v) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("version", v)?;
                map.end()
            }
            Self::MinMax {
                min_version,
                max_version,
            } => {
                let len = min_version.is_some() as usize + max_version.is_some() as usize;
                let mut map = serializer.serialize_map(Some(len))?;
                if let Some(v) = min_version {
                    map.serialize_entry("min_version", v)?;
                }
                if let Some(v) = max_version {
                    map.serialize_entry("max_version", v)?;
                }
                map.end()
            }
        }
    }
}

impl From<(String, String)> for CkanDependency {
    fn from((identifier, _version_req): (String, String)) -> Self {
        // TODO: This parsing is pretty naive and drops version requirements currently.
        // We need to properly support them.
        Self {
            name: identifier,
            choice_help_text: None,
            version_spec: None,
        }
    }
}

impl From<(String, DependencySpecifier)> for CkanDependency {
    fn from((identifier, spec): (String, DependencySpecifier)) -> Self {
        Self {
            name: identifier,
            choice_help_text: match &spec {
                DependencySpecifier::Version(_) => None,
                DependencySpecifier::Config { help_text, .. } => help_text.clone(),
            },
            version_spec: match spec {
                DependencySpecifier::Version(ver) => get_ver_bounds(ver),
                DependencySpecifier::Config { version, .. } => get_ver_bounds(version),
            },
        }
    }
}

fn get_ver_bounds(ver: String) -> Option<CkanDependencyVersionSpecifier> {
    if ver == "*" {
        // Wildcard -- CKAN just assumes "any" if the version is unset.
        return None;
    }

    let mut min_version: Option<String> = None;
    let mut max_version: Option<String> = None;
    let mut exact: Option<String> = None;

    for part in ver.split(',').map(str::trim) {
        if let Some(rest) = part.strip_prefix(">=") {
            min_version = Some(rest.to_string());
        } else if let Some(rest) = part.strip_prefix("<=") {
            max_version = Some(rest.to_string());
        } else if let Some(rest) = part.strip_prefix('>') {
            min_version = Some(rest.to_string());
        } else if let Some(rest) = part.strip_prefix('<') {
            max_version = Some(rest.to_string());
        } else if let Some(rest) = part.strip_prefix("==") {
            exact = Some(rest.to_string());
        } else if let Some(rest) = part.strip_prefix('=') {
            exact = Some(rest.to_string());
        } else {
            // bare version string, treat as exact
            exact = Some(part.to_string());
        }
    }

    if let Some(v) = exact {
        Some(CkanDependencyVersionSpecifier::Exact(v))
    } else if min_version.is_some() || max_version.is_some() {
        Some(CkanDependencyVersionSpecifier::MinMax {
            min_version,
            max_version,
        })
    } else {
        None
    }
}

#[derive(Clone, Serialize, Default)]
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

#[derive(Serialize, Default)]
pub struct CkanDownloadHash {
    pub sha1: String,
    pub sha256: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcard_returns_none() {
        assert!(get_ver_bounds("*".to_string()).is_none());
    }

    #[test]
    fn gte_sets_min() {
        let Some(CkanDependencyVersionSpecifier::MinMax {
            min_version,
            max_version,
        }) = get_ver_bounds(">=v0.9.5-0".to_string())
        else {
            panic!("expected MinMax")
        };
        assert_eq!(min_version, Some("v0.9.5-0".to_string()));
        assert_eq!(max_version, None);
    }

    #[test]
    fn lt_sets_max() {
        let Some(CkanDependencyVersionSpecifier::MinMax {
            min_version,
            max_version,
        }) = get_ver_bounds("<1.6.0".to_string())
        else {
            panic!("expected MinMax")
        };
        assert_eq!(min_version, None);
        assert_eq!(max_version, Some("1.6.0".to_string()));
    }

    #[test]
    fn lte_sets_max() {
        let Some(CkanDependencyVersionSpecifier::MinMax {
            min_version,
            max_version,
        }) = get_ver_bounds("<=2.0.0".to_string())
        else {
            panic!("expected MinMax")
        };
        assert_eq!(min_version, None);
        assert_eq!(max_version, Some("2.0.0".to_string()));
    }

    #[test]
    fn gt_sets_min() {
        let Some(CkanDependencyVersionSpecifier::MinMax {
            min_version,
            max_version,
        }) = get_ver_bounds(">1.0.0".to_string())
        else {
            panic!("expected MinMax")
        };
        assert_eq!(min_version, Some("1.0.0".to_string()));
        assert_eq!(max_version, None);
    }

    #[test]
    fn compound_range_sets_both() {
        let Some(CkanDependencyVersionSpecifier::MinMax {
            min_version,
            max_version,
        }) = get_ver_bounds(">=1.0.0,<2.0.0".to_string())
        else {
            panic!("expected MinMax")
        };
        assert_eq!(min_version, Some("1.0.0".to_string()));
        assert_eq!(max_version, Some("2.0.0".to_string()));
    }

    #[test]
    fn exact_with_double_eq() {
        let Some(CkanDependencyVersionSpecifier::Exact(v)) = get_ver_bounds("==1.2.3".to_string())
        else {
            panic!("expected Exact")
        };
        assert_eq!(v, "1.2.3");
    }

    #[test]
    fn exact_with_single_eq() {
        let Some(CkanDependencyVersionSpecifier::Exact(v)) = get_ver_bounds("=1.2.3".to_string())
        else {
            panic!("expected Exact")
        };
        assert_eq!(v, "1.2.3");
    }

    #[test]
    fn bare_version_is_exact() {
        let Some(CkanDependencyVersionSpecifier::Exact(v)) = get_ver_bounds("1.2.3".to_string())
        else {
            panic!("expected Exact")
        };
        assert_eq!(v, "1.2.3");
    }

    #[test]
    fn arbitrary_string_value_preserved() {
        let Some(CkanDependencyVersionSpecifier::MinMax { min_version, .. }) =
            get_ver_bounds(">=some-arbitrary-string".to_string())
        else {
            panic!("expected MinMax")
        };
        assert_eq!(min_version, Some("some-arbitrary-string".to_string()));
    }
}
