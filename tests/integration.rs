use std::fs;
use std::path::Path;
use tempfile::tempdir;

use ndjson_validator::{validate_file, process_file, ValidatorConfig};

#[test]
fn test_integration_valid_ndjson() {
    let file_path = Path::new("tests/valid.ndjson");
    let errors = validate_file(file_path).unwrap();
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_integration_invalid_ndjson1() {
    let file_path = Path::new("tests/invalid1.ndjson");
    let errors = validate_file(file_path).unwrap();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].line_number, 1);
}

#[test]
fn test_integration_invalid_ndjson2() {
    let file_path = Path::new("tests/invalid2.ndjson");
    let errors = validate_file(file_path).unwrap();
    assert_eq!(errors.len(), 8); // All lines except first and last are invalid
}

#[test]
fn test_integration_cleaning_ndjson() {
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
