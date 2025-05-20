use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::collections::HashSet;

use rayon::prelude::*;
use serde_json::Value;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Error)]
pub enum NdJsonError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("JSON parsing error at line {line} in file {file}: {error}")]
    JsonParse {
        file: String,
        line: usize,
        error: serde_json::Error,
    },
    
    #[error("No ND-JSON files found in directory: {0}")]
    NoFilesFound(String),
    
    #[error("Failed to create output directory: {0}")]
    FailedToCreateOutputDir(String),
    
    #[error("File system error: {0}")]
    Walkdir(#[from] walkdir::Error),
}

pub type Result<T> = std::result::Result<T, NdJsonError>;

/// Represents a validation error in an ND-JSON file
#[derive(Debug)]
pub struct ValidationError {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub line_content: String,
    pub error: String,
}

/// Configuration options for the ND-JSON validator
#[derive(Debug, Clone)]
pub struct ValidatorConfig {
    /// Whether to clean files by removing invalid JSON lines
    pub clean_files: bool,
    
    /// Directory to write cleaned files to (if clean_files is true)
    pub output_dir: Option<PathBuf>,
    
    /// Whether to use parallel processing for faster validation
    pub parallel: bool,
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            clean_files: false,
            output_dir: None,
            parallel: true,
        }
    }
}

/// Validates a single ND-JSON file and returns a list of validation errors
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use ndjson_validator::validate_file;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let file_path = Path::new("tests/valid.ndjson");
/// let errors = validate_file(file_path)?;
/// assert_eq!(errors.len(), 0); // No errors in a valid file
///
/// let invalid_file_path = Path::new("tests/invalid1.ndjson");
/// let errors = validate_file(invalid_file_path)?;
/// assert_eq!(errors.len(), 1); // One error in this file
/// assert_eq!(errors[0].line_number, 1); // Error is on line 1
/// # Ok(())
/// # }
/// ```
pub fn validate_file(file_path: &Path) -> Result<Vec<ValidationError>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut errors = Vec::new();

    for (i, line_result) in reader.lines().enumerate() {
        let line_number = i + 1;
        let line = line_result?;
        
        if line.trim().is_empty() {
            continue;
        }
        
        match serde_json::from_str::<Value>(&line) {
            Ok(_) => {}
            Err(e) => {
                errors.push(ValidationError {
                    file_path: file_path.to_path_buf(),
                    line_number,
                    line_content: line,
                    error: e.to_string(),
                });
            }
        }
    }

    Ok(errors)
}

#[cfg(test)]
mod validate_file_tests {
    use super::*;
    
    #[test]
    fn test_valid_ndjson() {
        let file_path = Path::new("tests/valid.ndjson");
        let errors = validate_file(file_path).unwrap();
        assert_eq!(errors.len(), 0);
    }
    
    #[test]
    fn test_invalid_ndjson1() {
        let file_path = Path::new("tests/invalid1.ndjson");
        let errors = validate_file(file_path).unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line_number, 1);
    }
    
    #[test]
    fn test_invalid_ndjson2() {
        let file_path = Path::new("tests/invalid2.ndjson");
        let errors = validate_file(file_path).unwrap();
        assert_eq!(errors.len(), 8); // All lines except first and last are invalid
    }
}

/// Validates and optionally cleans a single ND-JSON file
///
/// # Examples
///
/// ```
/// use std::path::{Path, PathBuf};
/// use ndjson_validator::{process_file, ValidatorConfig};
/// use std::fs;
/// use tempfile::tempdir;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let temp_dir = tempdir()?;
/// let output_dir = temp_dir.path();
/// 
/// // Create a config with cleaning enabled
/// let config = ValidatorConfig {
///     clean_files: true,
///     output_dir: Some(output_dir.to_path_buf()),
///     parallel: false,
/// };
/// 
/// // Process a file with invalid JSON
/// let file_path = Path::new("tests/invalid1.ndjson");
/// let errors = process_file(file_path, &config)?;
/// 
/// // Verify results
/// assert_eq!(errors.len(), 1); // One error was found
/// 
/// let output_file = output_dir.join("invalid1.ndjson");
/// let content = fs::read_to_string(output_file)?;
/// 
/// // Cleaned file should only have 2 valid lines
/// let line_count = content.lines().count();
/// assert_eq!(line_count, 2);
/// # Ok(())
/// # }
/// ```
pub fn process_file(file_path: &Path, config: &ValidatorConfig) -> Result<Vec<ValidationError>> {
    let errors = validate_file(file_path)?;
    
    if config.clean_files && config.output_dir.is_some() { // MODIFIED: Removed !errors.is_empty()
        let output_dir = config.output_dir.as_ref().unwrap();
        fs::create_dir_all(output_dir)
            .map_err(|_| NdJsonError::FailedToCreateOutputDir(output_dir.display().to_string()))?;
        
        let relative_path = file_path.file_name().unwrap_or_default();
        let output_path = output_dir.join(relative_path);
        
        clean_file(file_path, &output_path, &errors)?;
    }
    
    Ok(errors)
}

#[cfg(test)]
mod process_file_tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs; // Added for reading file content in new test
    
    #[test]
    fn test_cleaning_ndjson() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        let file_path = Path::new("tests/invalid1.ndjson");
        let config = ValidatorConfig {
            clean_files: true,
            output_dir: Some(output_dir.to_path_buf()),
            parallel: false,
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
            parallel: false,
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
            parallel: false,
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
            parallel: false,
        };

        let errors = process_file(&input_file_path, &config).unwrap();
        assert_eq!(errors.len(), 2, "Should find two errors in the input file");

        let expected_output_file_path = output_dir_path.join(input_file_name);
        assert!(!expected_output_file_path.exists(), "Output file {:?} should not exist if cleaning results in an empty file", expected_output_file_path);
    }
}

/// Writes a cleaned version of the file without the invalid JSON lines
fn clean_file(input_path: &Path, output_path: &Path, errors: &[ValidationError]) -> Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    
    let invalid_lines: HashSet<usize> = errors.iter() // MODIFIED: Use HashSet
        .map(|e| e.line_number)
        .collect();
    
    let mut lines_written = 0; // ADDED: Counter for lines written
    
    // Create the output file. It will be empty initially or truncated if it exists.
    let output_file_handle = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file_handle);
    
    for (i, line_result) in reader.lines().enumerate() {
        let line_number = i + 1;
        let line = line_result?; // Propagates IO errors from reading lines
        
        if !invalid_lines.contains(&line_number) {
            writeln!(writer, "{}", line)?; // Propagates IO errors from writing lines
            lines_written += 1; // ADDED: Increment counter
        }
    }
    
    writer.flush()?; // Ensure all buffered data is written to the underlying file.
    drop(writer); // ADDED: Explicitly drop writer to close the file before potential deletion.

    if lines_written == 0 { // ADDED: Check if any lines were written
        // If no lines were written, the file is effectively empty. Remove it.
        fs::remove_file(output_path)?;
    }
    
    Ok(())
}

#[cfg(test)]
mod clean_file_tests {
    use super::*;
    use tempfile::{NamedTempFile, tempdir}; // ADDED tempdir
    use std::fs;

    #[test]
    fn test_clean_file_removes_invalid_lines() {
        // Create a temporary input file
        let input_file = NamedTempFile::new().unwrap();
        let input_path = input_file.path();
        
        // Write test content to the input file
        fs::write(input_path, "line1\\nline2\\nline3\\nline4\\n").unwrap();
        
        // Create a temporary output file
        let output_file = NamedTempFile::new().unwrap();
        let output_path = output_file.path();
        
        // Create validation errors for lines 2 and 4
        let errors = vec![
            ValidationError {
                file_path: input_path.to_path_buf(),
                line_number: 2,
                line_content: "line2".to_string(),
                error: "test error".to_string(),
            },
            ValidationError {
                file_path: input_path.to_path_buf(), // FIXED TYPO: added .before to_path_buf()
                line_number: 4,
                line_content: "line4".to_string(),
                error: "test error".to_string(),
            },
        ];
        
        // Clean the file
        clean_file(input_path, output_path, &errors).unwrap();
        
        // Read the output file
        let content = fs::read_to_string(output_path).unwrap();
        
        // Check that lines 2 and 4 were removed
        assert_eq!(content, "line1\nline3\n");
    }

    #[test]
    fn test_clean_file_all_invalid_lines_no_output() {
        // Create a temporary input file
        let input_file = NamedTempFile::new().unwrap();
        let input_path = input_file.path();
        fs::write(input_path, "corrupt1\\ncorrupt2\\n").unwrap();
        
        // Create a path for the output file (it might not be created)
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("cleaned_output.ndjson");

        let errors = vec![
            ValidationError {
                file_path: input_path.to_path_buf(),
                line_number: 1,
                line_content: "corrupt1".to_string(),
                error: "test error".to_string(),
            },
            ValidationError {
                file_path: input_path.to_path_buf(),
                line_number: 2,
                line_content: "corrupt2".to_string(),
                error: "test error".to_string(),
            },
        ];
        
        clean_file(input_path, &output_path, &errors).unwrap();
        
        assert!(!output_path.exists(), "Output file should not exist when all lines are invalid");
    }
}

/// Validates a list of ND-JSON files
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use ndjson_validator::{validate_files, ValidatorConfig};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let files = vec![
///     PathBuf::from("tests/valid.ndjson"),
///     PathBuf::from("tests/invalid1.ndjson"),
/// ];
///
/// let config = ValidatorConfig::default();
/// let errors = validate_files(&files, &config)?;
///
/// assert_eq!(errors.len(), 1); // One error from invalid1.ndjson
/// # Ok(())
/// # }
/// ```
pub fn validate_files(file_paths: &[PathBuf], config: &ValidatorConfig) -> Result<Vec<ValidationError>> {
    let process_fn = |file_path: &PathBuf| -> Result<Vec<ValidationError>> {
        process_file(file_path, config)
    };
    
    let results = if config.parallel {
        // Process files in parallel
        file_paths.par_iter()
            .map(process_fn)
            .collect::<Vec<Result<Vec<ValidationError>>>>()
    } else {
        // Process files sequentially
        file_paths.iter()
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

#[cfg(test)]
mod validate_files_tests {
    use super::*;
    
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
        
        // Test with parallel processing
        let parallel_config = ValidatorConfig {
            clean_files: false,
            output_dir: None,
            parallel: true,
        };
        let parallel_errors = validate_files(&files, &parallel_config).unwrap();
        
        // Test with sequential processing
        let sequential_config = ValidatorConfig {
            clean_files: false,
            output_dir: None,
            parallel: false,
        };
        let sequential_errors = validate_files(&files, &sequential_config).unwrap();
        
        // Both should have the same number of errors
        assert_eq!(parallel_errors.len(), sequential_errors.len());
        assert_eq!(parallel_errors.len(), 1 + 8); // 1 from invalid1.ndjson + 8 from invalid2.ndjson
    }
}

/// Validates all ND-JSON files in a directory
pub fn validate_directory(dir_path: &Path, config: &ValidatorConfig) -> Result<Vec<ValidationError>> {
    let mut file_paths = Vec::new();
    
    // Find all NDJSON files in the directory
    for entry_result in WalkDir::new(dir_path).max_depth(1).into_iter() {
        let entry = entry_result?;
        let path = entry.path();
        if path.is_file() && 
           (path.extension().map_or(false, |ext| ext == "ndjson" || ext == "jsonl") || 
            path.to_string_lossy().contains(".nd.json")) {
            file_paths.push(path.to_path_buf());
        }
    }
    
    if file_paths.is_empty() {
        return Err(NdJsonError::NoFilesFound(dir_path.display().to_string()));
    }
    
    validate_files(&file_paths, config)
}

#[cfg(test)]
mod validate_directory_tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_validate_directory() {
        let config = ValidatorConfig::default();
        let errors = validate_directory(Path::new("tests"), &config).unwrap();
        
        // We should have errors from both invalid files
        assert!(errors.len() > 0);
        
        // Check that errors came from the invalid files
        let invalid_files: HashSet<_> = errors.iter()
            .map(|e| e.file_path.file_name().unwrap().to_string_lossy().to_string())
            .collect();
        
        assert!(invalid_files.contains("invalid1.ndjson"));
        assert!(invalid_files.contains("invalid2.ndjson"));
    }
    
    #[test]
    fn test_empty_directory() {
        let temp_dir = tempdir().unwrap();
        let config = ValidatorConfig::default();
        
        // Should return an error for an empty directory
        let result = validate_directory(temp_dir.path(), &config);
        assert!(result.is_err());
        
        if let Err(err) = result {
            match err {
                NdJsonError::NoFilesFound(_) => {}, // This is the expected error
                _ => panic!("Unexpected error type: {:?}", err),
            }
        }
    }
}

/// Summary of validation results
#[derive(Debug)]
pub struct ValidationSummary {
    pub total_files: usize,
    pub files_with_errors: usize,
    pub total_errors: usize,
}

/// Validates multiple ND-JSON files and returns a summary along with detailed errors
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use ndjson_validator::{validate_multiple, ValidatorConfig};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let files = vec![
///     PathBuf::from("tests/valid.ndjson"),
///     PathBuf::from("tests/invalid1.ndjson"),
///     PathBuf::from("tests/invalid2.ndjson"),
/// ];
///
/// let config = ValidatorConfig::default();
/// let (summary, errors) = validate_multiple(&files, &config)?;
///
/// assert_eq!(summary.total_files, 3);
/// assert_eq!(summary.files_with_errors, 2);
/// assert!(summary.total_errors > 0);
/// # Ok(())
/// # }
/// ```
pub fn validate_multiple(
    files: &[PathBuf], 
    config: &ValidatorConfig
) -> Result<(ValidationSummary, Vec<ValidationError>)> {
    let errors = validate_files(files, config)?;
    
    // Count unique files with errors
    let files_with_errors = errors.iter()
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

#[cfg(test)]
mod validate_multiple_tests {
    use super::*;
    
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
}

/// Validates all ND-JSON files in a directory and returns a summary along with detailed errors
pub fn validate_directory_with_summary(
    dir_path: &Path, 
    config: &ValidatorConfig
) -> Result<(ValidationSummary, Vec<ValidationError>)> {
    let mut file_paths = Vec::new();
    
    // Find all NDJSON files in the directory
    for entry_result in WalkDir::new(dir_path).max_depth(1).into_iter() {
        let entry = entry_result?;
        let path = entry.path();
        if path.is_file() && 
           (path.extension().map_or(false, |ext| ext == "ndjson" || ext == "jsonl") || 
            path.to_string_lossy().contains(".nd.json")) {
            file_paths.push(path.to_path_buf());
        }
    }
    
    if file_paths.is_empty() {
        return Err(NdJsonError::NoFilesFound(dir_path.display().to_string()));
    }
    
    validate_multiple(&file_paths, config)
}

#[cfg(test)]
mod validate_directory_with_summary_tests {
    use super::*;
    
    #[test]
    fn test_directory_summary() {
        let config = ValidatorConfig::default();
        let (summary, errors) = validate_directory_with_summary(Path::new("tests"), &config).unwrap();
        
        assert_eq!(summary.total_files, 3); // valid.ndjson, invalid1.ndjson, invalid2.ndjson
        assert_eq!(summary.files_with_errors, 2); // Two files with errors
        assert_eq!(summary.total_errors, errors.len());
    }
}
