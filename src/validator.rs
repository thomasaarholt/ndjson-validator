use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use serde_json::Value;
use sonic_rs::LazyValue;

use crate::error::{Result, ValidationError};

/// Validates a single ND-JSON file and returns a list of validation errors
pub fn validate_file_serde(file_path: &Path) -> Result<Vec<ValidationError>> {
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

/// Validates a single ND-JSON file using sonic-rs and returns a list of validation errors
pub fn validate_file_sonic(file_path: &Path) -> Result<Vec<ValidationError>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut errors = Vec::new();

    for (i, line_result) in reader.lines().enumerate() {
        let line_number = i + 1;
        let line = line_result?;
        
        if line.trim().is_empty() {
            continue;
        }
        
        match sonic_rs::from_str::<LazyValue>(&line) {
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
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_ndjson() {
        let file_path = Path::new("tests/valid.ndjson");
        let errors = validate_file_serde(file_path).unwrap();
        assert_eq!(errors.len(), 0);
    }
    
    #[test]
    fn test_invalid_ndjson1() {
        let file_path = Path::new("tests/invalid1.ndjson");
        let errors = validate_file_serde(file_path).unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line_number, 1);
    }
    
    #[test]
    fn test_invalid_ndjson2() {
        let file_path = Path::new("tests/invalid2.ndjson");
        let errors = validate_file_serde(file_path).unwrap();
        assert_eq!(errors.len(), 8); // All lines except first and last are invalid
    }
}
