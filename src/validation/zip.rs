use crate::validation::{ValidationContext, ValidationError, Validator};

pub struct ZipFormatValidator;
impl Validator for ZipFormatValidator {
    fn validate(&self, ctx: &ValidationContext) -> Result<(), ValidationError> {
        let file = std::fs::File::open(&ctx.zip_path)
            .map_err(|e| ValidationError::InvalidZip(format!("Failed to open ZIP file: {}", e)))?;

        zip::ZipArchive::new(file)
            .map_err(|e| ValidationError::InvalidZip(format!("Failed to read ZIP archive: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{ckan::types::CkanFile, validation::{ValidationContext, ValidationError}};
    use super::*;

    #[test]
    fn test_valid_zip() {
        let validator = ZipFormatValidator;
        let ctx = ValidationContext {
            metadata: &CkanFile::default(),
            zip_path: "tests/fixtures/valid_mod.zip".to_string(),
        };
        assert!(validator.validate(&ctx).is_ok());
    }

    #[test]
    fn test_invalid_zip() {
        let validator = ZipFormatValidator;
        let ctx = ValidationContext {
            metadata: &CkanFile::default(),
            zip_path: "tests/fixtures/invalid_mod.zip".to_string(),
        };
        assert!(matches!(validator.validate(&ctx), Err(ValidationError::InvalidZip(_))));
    }
}