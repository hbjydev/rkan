use crate::validation::{ValidationContext, ValidationError, Validator};

pub struct InstallValidator;
impl Validator for InstallValidator {
    fn validate(&self, ctx: &ValidationContext) -> Result<(), ValidationError> {
        // Open & extract the ZIP file to a temporary directory
        let temp_dir =
            tempfile::tempdir().map_err(|e| ValidationError::InvalidZip(e.to_string()))?;
        let zip_file = std::fs::File::open(&ctx.zip_path)
            .map_err(|e| ValidationError::InvalidZip(e.to_string()))?;
        let mut zip = zip::ZipArchive::new(zip_file)
            .map_err(|e| ValidationError::InvalidZip(e.to_string()))?;
        zip.extract(temp_dir.path())
            .map_err(|e| ValidationError::InvalidZip(e.to_string()))?;

        if ctx.metadata.install.is_empty() {
            // The user hasn't provided any explicit install directives, so we assume the ZIP is structured
            // per the CKAN spec.
            if !temp_dir
                .path()
                .join(ctx.metadata.identifier.clone())
                .exists()
            {
                return Err(ValidationError::MissingFiles(vec![
                    ctx.metadata.identifier.clone(),
                ]));
            }
            // If the expected top-level directory exists, we consider the install directives valid
            Ok(())
        } else {
            // Validate that all specified install paths exist in the extracted ZIP
            let mut missing_files = Vec::new();
            for install_directive in &ctx.metadata.install {
                let full_path = temp_dir.path().join(
                    install_directive
                        .file
                        .split('/')
                        .collect::<std::path::PathBuf>(),
                );
                if !full_path.exists() {
                    missing_files.push(install_directive.file.clone());
                }
            }

            if !missing_files.is_empty() {
                return Err(ValidationError::MissingFiles(missing_files));
            }

            Ok(())
        }
    }
}

const VALID_INSTALL_TO_PATHS: &[&str] = &[
    "GameData",
    "Missions",
    "Ships",
    "Ships/SPH",
    "Ships/VAB",
    "Ships/@thumbs/VAB",
    "Ships/Script",
    "Tutorial",
    "Scenarios",
    "GameRoot",
];
pub struct InstallToValidator;
impl Validator for InstallToValidator {
    fn validate(&self, ctx: &ValidationContext) -> Result<(), ValidationError> {
        for install_directive in &ctx.metadata.install {
            if !VALID_INSTALL_TO_PATHS.contains(&install_directive.install_to.as_str()) {
                return Err(ValidationError::InvalidInstallTo(
                    install_directive.install_to.clone(),
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ckan::types::{CkanFile, CkanInstallDirective};

    #[test]
    fn test_valid_install_directives() {
        let metadata = CkanFile {
            install: vec![CkanInstallDirective {
                file: "GameData/Sol-Configs".to_string(),
                install_to: "GameData".to_string(),
            }],
            ..Default::default()
        };

        let ctx = ValidationContext {
            metadata: &metadata,
            zip_path: "tests/fixtures/valid_mod.zip".to_string(),
        };

        let validator = InstallValidator;
        assert!(validator.validate(&ctx).is_ok());
    }

    #[test]
    fn test_missing_install_files() {
        let metadata = CkanFile {
            install: vec![CkanInstallDirective {
                file: "missing_file.txt".to_string(),
                install_to: "GameData".to_string(),
            }],
            ..Default::default()
        };

        let ctx = ValidationContext {
            metadata: &metadata,
            zip_path: "tests/fixtures/valid_mod.zip".to_string(),
        };

        let validator = InstallValidator;
        let result = validator.validate(&ctx);
        assert!(result.is_err());
        if let Err(ValidationError::MissingFiles(files)) = result {
            assert_eq!(files, vec!["missing_file.txt".to_string()]);
        } else {
            panic!("Expected MissingFiles error");
        }
    }

    #[test]
    fn test_valid_install_to() {
        let metadata = CkanFile {
            install: vec![CkanInstallDirective {
                file: "GameData/Sol-Configs".to_string(),
                install_to: "GameData".to_string(),
            }],
            ..Default::default()
        };

        let ctx = ValidationContext {
            metadata: &metadata,
            zip_path: "tests/fixtures/valid_mod.zip".to_string(),
        };

        let validator = InstallToValidator;
        assert!(validator.validate(&ctx).is_ok());
    }

    #[test]
    fn test_invalid_install_to() {
        let metadata = CkanFile {
            install: vec![CkanInstallDirective {
                file: "GameData/Sol-Configs".to_string(),
                install_to: "InvalidPath".to_string(),
            }],
            ..Default::default()
        };

        let ctx = ValidationContext {
            metadata: &metadata,
            zip_path: "tests/fixtures/valid_mod.zip".to_string(),
        };

        let validator = InstallToValidator;
        let result = validator.validate(&ctx);
        assert!(result.is_err());
        if let Err(ValidationError::InvalidInstallTo(path)) = result {
            assert_eq!(path, "InvalidPath".to_string());
        } else {
            panic!("Expected InvalidInstallTo error");
        }
    }
}
