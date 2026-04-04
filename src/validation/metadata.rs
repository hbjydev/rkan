use lazy_static::lazy_static;

use crate::validation::{ValidationContext, ValidationError, Validator};

lazy_static! {
    static ref CKAN_IDENTIFIER_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_-]+$").unwrap();
}

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

const CKAN_TAGS: &[&str] = &[
    "parts", "physics", "plugin", "app", "config", "library", "flags", "agency",
    "suits", "control", "convenience", "information", "editor", "planet-pack",
    "graphics", "sound", "resources", "science", "tech-tree", "career",
    "combat", "comms", "buildings", "crewed", "uncrewed", "stock-inventory",
    "first-person"
];

/// Validates that all tags in the CKAN metadata are from the allowed list of tags.
pub struct CkanTagsValidator;
impl Validator for CkanTagsValidator {
    fn validate(&self, ctx: &ValidationContext) -> Result<(), ValidationError> {
        let invalid_tags: Vec<String> = ctx.metadata.tags.iter()
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
    use crate::ckan::types::CkanFile;
    use super::*;

    #[test]
    fn test_valid_identifier() {
        let validator = CkanIdentifierValidator;
        let ctx = ValidationContext {
            metadata: &CkanFile {
                identifier: "valid_id".to_string(),
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
        assert!(matches!(validator.validate(&ctx), Err(ValidationError::InvalidIdentifier)));
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
        assert!(matches!(validator.validate(&ctx), Err(ValidationError::InvalidTags(_))));
    }
}