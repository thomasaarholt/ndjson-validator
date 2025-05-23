use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use serde_json::Value;

use crate::error::{Result, ValidationError};

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
mod tests {
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
