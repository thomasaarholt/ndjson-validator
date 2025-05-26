use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use rayon::prelude::*;
use walkdir::WalkDir;

use crate::cleaner::clean_file;
use crate::config::ValidatorConfig;
use crate::error::{NdJsonError, Result, ValidationError, ValidationSummary};
use crate::validator::validate_file;

/// Validates and optionally cleans a single ND-JSON file
pub fn process_file(file_path: &Path, config: &ValidatorConfig) -> Result<Vec<ValidationError>> {
    let errors = validate_file(file_path)?;

    if config.clean_files && config.output_dir.is_some() {
        let output_dir = config.output_dir.as_ref().unwrap();
        fs::create_dir_all(output_dir)
            .map_err(|_| NdJsonError::FailedToCreateOutputDir(output_dir.display().to_string()))?;

        let relative_path = file_path.file_name().unwrap_or_default();
        let output_path = output_dir.join(relative_path);

        clean_file(file_path, &output_path, &errors)?;
    }

    Ok(errors)
}

/// Validates a list of ND-JSON files
pub fn validate_files(
    file_paths: &[PathBuf],
    config: &ValidatorConfig,
) -> Result<Vec<ValidationError>> {
    let process_fn =
        |file_path: &PathBuf| -> Result<Vec<ValidationError>> { process_file(file_path, config) };

    let results = {
        file_paths
            .par_iter()
            .map(process_fn)
            .collect::<Vec<Result<Vec<ValidationError>>>>()
    };

    // Flatten results and collect errors
    let mut all_errors = Vec::new();
    for result in results {
        match result {
            Ok(errors) => all_errors.extend(errors),
            Err(e) => return Err(e),
        }
    }

    Ok(all_errors)
}

/// Validates multiple ND-JSON files and returns a summary along with detailed errors
pub fn validate_multiple(
    files: &[PathBuf],
    config: &ValidatorConfig,
) -> Result<(ValidationSummary, Vec<ValidationError>)> {
    let errors = validate_files(files, config)?;

    // Count unique files with errors
    let files_with_errors = errors
        .iter()
        .map(|e| &e.file_path)
        .collect::<HashSet<_>>()
        .len();

    let summary = ValidationSummary {
        total_files: files.len(),
        files_with_errors,
        total_errors: errors.len(),
    };

    Ok((summary, errors))
}

/// Validates all ND-JSON files in a directory and returns a summary along with detailed errors
pub fn validate_directory_with_summary(
    dir_path: &Path,
    config: &ValidatorConfig,
) -> Result<(ValidationSummary, Vec<ValidationError>)> {
    let mut file_paths = Vec::new();

    // Find all NDJSON files in the directory
    for entry_result in WalkDir::new(dir_path).max_depth(1).into_iter() {
        let entry = entry_result?;
        let path = entry.path();
        if path.is_file()
            && (path
                .extension()
                .map_or(false, |ext| ext == "ndjson" || ext == "jsonl")
                || path.to_string_lossy().contains(".nd.json"))
        {
            file_paths.push(path.to_path_buf());
        }
    }

    if file_paths.is_empty() {
        return Err(NdJsonError::NoFilesFound(dir_path.display().to_string()));
    }

    validate_multiple(&file_paths, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_cleaning_ndjson() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();

        let file_path = Path::new("tests/invalid1.ndjson");
        let config = ValidatorConfig {
            clean_files: true,
            output_dir: Some(output_dir.to_path_buf()),
        };

        let errors = process_file(file_path, &config).unwrap();
        assert_eq!(errors.len(), 1);

        let output_file = output_dir.join("invalid1.ndjson");
        let content = fs::read_to_string(output_file).unwrap();

        // Cleaned file should only have 2 lines
        let line_count = content.lines().count();
        assert_eq!(line_count, 2);

        // Check that the file contains "Bob" and "Charlie" but not "Alice"
        assert!(!content.contains("Alice"));
        assert!(content.contains("Bob"));
        assert!(content.contains("Charlie"));
    }

    #[test]
    fn test_no_cleaning_when_disabled() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();

        let file_path = Path::new("tests/invalid1.ndjson");
        let config = ValidatorConfig {
            clean_files: false, // Cleaning disabled
            output_dir: Some(output_dir.to_path_buf()),
        };

        let errors = process_file(file_path, &config).unwrap();
        assert_eq!(errors.len(), 1);

        // No output file should be created
        let output_file = output_dir.join("invalid1.ndjson");
        assert!(!output_file.exists());
    }

    #[test]
    fn test_copy_valid_file_when_cleaning_enabled() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();

        let file_path = Path::new("tests/valid.ndjson"); // Use a valid file
        let config = ValidatorConfig {
            clean_files: true, // Cleaning enabled
            output_dir: Some(output_dir.to_path_buf()),
        };

        let errors = process_file(file_path, &config).unwrap();
        assert_eq!(errors.len(), 0); // No errors in valid file

        // Output file should be created and be a copy of the input
        let output_file_path = output_dir.join("valid.ndjson");
        assert!(output_file_path.exists());

        let input_content = fs::read_to_string(file_path).unwrap();
        let output_content = fs::read_to_string(output_file_path).unwrap();
        assert_eq!(input_content, output_content);
    }

    #[test]
    fn test_process_file_all_invalid_cleans_to_nothing() {
        let temp_output_dir = tempdir().unwrap(); // Directory for cleaned output
        let output_dir_path = temp_output_dir.path();

        // Create a temporary input directory and file
        let temp_input_dir = tempdir().unwrap();
        let input_file_name = "all_invalid.ndjson";
        let input_file_path = temp_input_dir.path().join(input_file_name);
        fs::write(&input_file_path, "{\"key\": value}\n[1,2\n").unwrap(); // Two invalid JSON lines

        let config = ValidatorConfig {
            clean_files: true,
            output_dir: Some(output_dir_path.to_path_buf()),
        };

        let errors = process_file(&input_file_path, &config).unwrap();
        assert_eq!(errors.len(), 2, "Should find two errors in the input file");

        let expected_output_file_path = output_dir_path.join(input_file_name);
        assert!(
            !expected_output_file_path.exists(),
            "Output file {:?} should not exist if cleaning results in an empty file",
            expected_output_file_path
        );
    }

    #[test]
    fn test_validate_multiple_files() {
        let files = vec![
            PathBuf::from("tests/valid.ndjson"),
            PathBuf::from("tests/invalid1.ndjson"),
        ];

        let config = ValidatorConfig::default();
        let errors = validate_files(&files, &config).unwrap();

        assert_eq!(errors.len(), 1); // One error from invalid1.ndjson
        assert!(errors[0].file_path.ends_with("invalid1.ndjson"));
    }

    #[test]
    fn test_parallel_vs_sequential() {
        let files = vec![
            PathBuf::from("tests/valid.ndjson"),
            PathBuf::from("tests/invalid1.ndjson"),
            PathBuf::from("tests/invalid2.ndjson"),
        ];

        let parallel_config = ValidatorConfig {
            clean_files: false,
            output_dir: None,
        };
        let parallel_errors = validate_files(&files, &parallel_config).unwrap();

        assert_eq!(parallel_errors.len(), 1 + 8); // 1 from invalid1.ndjson + 8 from invalid2.ndjson
    }



    #[test]
    fn test_validation_summary() {
        let files = vec![
            PathBuf::from("tests/valid.ndjson"),
            PathBuf::from("tests/invalid1.ndjson"),
            PathBuf::from("tests/invalid2.ndjson"),
        ];

        let config = ValidatorConfig::default();
        let (summary, errors) = validate_multiple(&files, &config).unwrap();

        assert_eq!(summary.total_files, 3);
        assert_eq!(summary.files_with_errors, 2); // valid.ndjson has no errors
        assert_eq!(summary.total_errors, errors.len());
    }

    #[test]
    fn test_directory_summary() {
        let config = ValidatorConfig::default();
        let (summary, errors) =
            validate_directory_with_summary(Path::new("tests"), &config).unwrap();

        assert_eq!(summary.total_files, 3); // valid.ndjson, invalid1.ndjson, invalid2.ndjson
        assert_eq!(summary.files_with_errors, 2); // Two files with errors
        assert_eq!(summary.total_errors, errors.len());
    }
}
