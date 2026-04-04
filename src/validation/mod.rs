use thiserror::Error;

use crate::ckan::types::CkanFile;

pub mod metadata;
pub mod zip;
pub mod install;

#[derive(Debug, Error)]
pub enum ValidationError {
    /// The CKAN mod identifier is invalid.
    #[error("The CKAN mod identifier is invalid.")]
    InvalidIdentifier,

    /// The ZIP file is not a valid ZIP archive
    #[error("The ZIP file is not a valid ZIP archive.")]
    InvalidZip(String),

    /// The ZIP file is missing required files or directories
    #[error("The ZIP file is missing required files or directories: {0:?}")]
    MissingFiles(Vec<String>),

    /// The provided tag(s) are not recognized as valid CKAN tags
    #[error("The provided tag(s) are not recognized as valid CKAN tags: {0:?}")]
    InvalidTags(Vec<String>),
}

pub struct ValidationContext<'a> {
    pub metadata: &'a CkanFile,
    pub zip_path: String,
}

pub trait Validator: Send + Sync {
    fn validate(&self, ctx: &ValidationContext<'_>) -> Result<(), ValidationError>;
}

pub fn default_validators() -> Vec<Box<dyn Validator>> {
    vec![
        // Metadata validators
        Box::new(metadata::CkanIdentifierValidator {}),
        Box::new(metadata::CkanTagsValidator {}),

        // ZIP file validators
        Box::new(zip::ZipFormatValidator {}),

        // Installation validators
        Box::new(install::InstallValidator {}),
    ]
}

pub fn run_validators(validators: &[Box<dyn Validator>], ctx: &ValidationContext) -> Result<(), ValidationError> {
    for validator in validators {
        validator.validate(ctx)?;
    }
    Ok(())
}