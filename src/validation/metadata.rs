use std::sync::LazyLock;

use crate::validation::{CKAN_SCHEMA, ValidationContext, ValidationError, Validator};

static CKAN_IDENTIFIER_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    let schema_regex = CKAN_SCHEMA
        .get("definitions")
        .and_then(|props| props.get("identifier"))
        .and_then(|identifier| identifier.get("pattern"))
        .and_then(|pattern| pattern.as_str())
        .expect("Failed to get CKAN identifier pattern from schema");

    regex::Regex::new(schema_regex).unwrap()
});

/// Validates that the CKAN mod identifier is valid according to CKAN's rules.
pub struct CkanIdentifierValidator;
impl Validator for CkanIdentifierValidator {
    fn validate(&self, ctx: &ValidationContext) -> Result<(), ValidationError> {
        if !CKAN_IDENTIFIER_REGEX.is_match(&ctx.metadata.identifier) {
            return Err(ValidationError::InvalidIdentifier);
        }
        Ok(())
    }
}

static CKAN_LICENSES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    CKAN_SCHEMA
        .get("definitions")
        .and_then(|props| props.get("license"))
        .and_then(|license| license.get("enum"))
        .and_then(|enum_values| enum_values.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<&'static str>>()
        })
        .unwrap_or_default()
});

pub struct CkanLicenseValidator;
impl Validator for CkanLicenseValidator {
    fn validate(&self, ctx: &ValidationContext) -> Result<(), ValidationError> {
        if !CKAN_LICENSES.contains(&ctx.metadata.license.as_str()) {
            return Err(ValidationError::InvalidLicense(
                ctx.metadata.license.clone(),
            ));
        }
        Ok(())
    }
}

static CKAN_KSP_VERSION_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    CKAN_SCHEMA
        .get("definitions")
        .and_then(|props| props.get("ksp_version"))
        .and_then(|ksp_version| ksp_version.get("pattern"))
        .map(|arr| regex::Regex::new(arr.as_str().unwrap_or("")).unwrap())
        .expect("Failed to build KSP version regex from CKAN schema")
});

pub struct CkanKspVersionValidator;
impl Validator for CkanKspVersionValidator {
    fn validate(&self, ctx: &ValidationContext) -> Result<(), ValidationError> {
        if !CKAN_KSP_VERSION_REGEX.is_match(&ctx.metadata.ksp_version) {
            return Err(ValidationError::InvalidKspVersion(
                ctx.metadata.ksp_version.clone(),
            ));
        }
        Ok(())
    }
}

const CKAN_TAGS: &[&str] = &[
    "parts",
    "physics",
    "plugin",
    "app",
    "config",
    "library",
    "flags",
    "agency",
    "suits",
    "control",
    "convenience",
    "information",
    "editor",
    "planet-pack",
    "graphics",
    "sound",
    "resources",
    "science",
    "tech-tree",
    "career",
    "combat",
    "comms",
    "buildings",
    "crewed",
    "uncrewed",
    "stock-inventory",
    "first-person",
];

/// Validates that all tags in the CKAN metadata are from the allowed list of tags.
pub struct CkanTagsValidator;
impl Validator for CkanTagsValidator {
    fn validate(&self, ctx: &ValidationContext) -> Result<(), ValidationError> {
        let invalid_tags: Vec<String> = ctx
            .metadata
            .tags
            .iter()
            .filter(|tag| !CKAN_TAGS.contains(&tag.as_str()))
            .cloned()
            .collect();

        if !invalid_tags.is_empty() {
            return Err(ValidationError::InvalidTags(invalid_tags));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ckan::types::CkanFile;

    #[test]
    fn test_valid_identifier() {
        let validator = CkanIdentifierValidator;
        let ctx = ValidationContext {
            metadata: &CkanFile {
                identifier: "valid-id".to_string(),
                ..Default::default()
            },
            zip_path: "".to_string(),
        };
        assert!(validator.validate(&ctx).is_ok());
    }

    #[test]
    fn test_invalid_identifier() {
        let validator = CkanIdentifierValidator;
        let ctx = ValidationContext {
            metadata: &CkanFile {
                identifier: "invalid id".to_string(),
                ..Default::default()
            },
            zip_path: "".to_string(),
        };
        assert!(matches!(
            validator.validate(&ctx),
            Err(ValidationError::InvalidIdentifier)
        ));
    }

    #[test]
    fn test_valid_license() {
        let validator = CkanLicenseValidator;
        let ctx = ValidationContext {
            metadata: &CkanFile {
                license: "CC-BY-SA-4.0".to_string(),
                ..Default::default()
            },
            zip_path: "".to_string(),
        };
        assert!(validator.validate(&ctx).is_ok());
    }

    #[test]
    fn test_invalid_license() {
        let validator = CkanLicenseValidator;
        let ctx = ValidationContext {
            metadata: &CkanFile {
                license: "invalid license".to_string(),
                ..Default::default()
            },
            zip_path: "".to_string(),
        };
        assert!(matches!(
            validator.validate(&ctx),
            Err(ValidationError::InvalidLicense(_))
        ));
    }

    #[test]
    fn test_valid_ksp_version() {
        let validator = CkanKspVersionValidator;
        let ctx = ValidationContext {
            metadata: &CkanFile {
                ksp_version: "1.12.2".to_string(),
                ..Default::default()
            },
            zip_path: "".to_string(),
        };
        assert!(validator.validate(&ctx).is_ok());
    }

    #[test]
    fn test_invalid_ksp_version() {
        let validator = CkanKspVersionValidator;
        let ctx = ValidationContext {
            metadata: &CkanFile {
                ksp_version: "invalid version".to_string(),
                ..Default::default()
            },
            zip_path: "".to_string(),
        };
        assert!(matches!(
            validator.validate(&ctx),
            Err(ValidationError::InvalidKspVersion(_))
        ));
    }

    #[test]
    fn test_valid_tags() {
        let validator = CkanTagsValidator;
        let ctx = ValidationContext {
            metadata: &CkanFile {
                tags: vec!["parts".to_string(), "physics".to_string()],
                ..Default::default()
            },
            zip_path: "".to_string(),
        };
        assert!(validator.validate(&ctx).is_ok());
    }

    #[test]
    fn test_invalid_tags() {
        let validator = CkanTagsValidator;
        let invalid_tags = vec!["invalid_tag".to_string(), "parts".to_string()];
        let ctx = ValidationContext {
            metadata: &CkanFile {
                tags: invalid_tags.clone(),
                ..Default::default()
            },
            zip_path: "".to_string(),
        };
        assert!(matches!(
            validator.validate(&ctx),
            Err(ValidationError::InvalidTags(_))
        ));
    }
}
