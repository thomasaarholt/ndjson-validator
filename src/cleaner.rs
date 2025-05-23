use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use crate::error::{Result, ValidationError};

/// Writes a cleaned version of the file without the invalid JSON lines
pub fn clean_file(input_path: &Path, output_path: &Path, errors: &[ValidationError]) -> Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    
    let invalid_lines: HashSet<usize> = errors.iter()
        .map(|e| e.line_number)
        .collect();
    
    let mut lines_written = 0;
    
    // Create the output file. It will be empty initially or truncated if it exists.
    let output_file_handle = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file_handle);
    
    for (i, line_result) in reader.lines().enumerate() {
        let line_number = i + 1;
        let line = line_result?; // Propagates IO errors from reading lines
        
        if !invalid_lines.contains(&line_number) {
            writeln!(writer, "{}", line)?; // Propagates IO errors from writing lines
            lines_written += 1;
        }
    }
    
    writer.flush()?; // Ensure all buffered data is written to the underlying file.
    drop(writer); // Explicitly drop writer to close the file before potential deletion.

    if lines_written == 0 {
        // If no lines were written, the file is effectively empty. Remove it.
        fs::remove_file(output_path)?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{NamedTempFile, tempdir};
    use std::fs;

    #[test]
    fn test_clean_file_removes_invalid_lines() {
        // Create a temporary input file
        let input_file = NamedTempFile::new().unwrap();
        let input_path = input_file.path();
        
        // Write test content to the input file
        fs::write(input_path, "line1\nline2\nline3\nline4\n").unwrap();
        
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
                file_path: input_path.to_path_buf(),
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
        fs::write(input_path, "corrupt1\ncorrupt2\n").unwrap();
        
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
