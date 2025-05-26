use std::path::{Path, PathBuf};
use ndjson_validator::{validate_file, validate_directory_with_summary, ValidatorConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Validate a single file
    println!("Example 1: Validating a single file");
    let file_path = Path::new("tests/valid.ndjson");
    let errors = validate_file(file_path)?;
    
    if errors.is_empty() {
        println!("✅ File '{}' is valid!", file_path.display());
    } else {
        println!("❌ Found {} errors in file '{}'", errors.len(), file_path.display());
        for error in errors {
            println!("Line {}: {}", error.line_number, error.error);
        }
    }
    
    // Example 2: Validate an invalid file
    println!("\nExample 2: Validating a file with invalid JSON");
    let invalid_file_path = Path::new("tests/invalid1.ndjson");
    let errors = validate_file(invalid_file_path)?;
    
    if errors.is_empty() {
        println!("✅ File '{}' is valid!", invalid_file_path.display());
    } else {
        println!("❌ Found {} errors in file '{}'", errors.len(), invalid_file_path.display());
        for error in &errors {
            println!("Line {}: {}", error.line_number, error.error);
            println!("  Content: {}", error.line_content);
        }
    }
    
    // Example 3: Validate a directory with summary
    println!("\nExample 3: Validating a directory with summary");
    let dir_path = Path::new("tests");
    
    // Create a configuration for cleaning
    let config = ValidatorConfig {
        clean_files: true,
        output_dir: Some(PathBuf::from("cleaned_output")),
    };
    
    let (summary, _errors) = validate_directory_with_summary(dir_path, &config)?;
    
    println!("Validation Summary:");
    println!("  Total files processed: {}", summary.total_files);
    println!("  Files with errors: {}", summary.files_with_errors);
    println!("  Total errors found: {}", summary.total_errors);
    
    if summary.total_errors == 0 {
        println!("✅ All files are valid!");
    } else {
        println!("❌ Found {} errors in {} files", summary.total_errors, summary.files_with_errors);
        println!("  Cleaned files written to: {}", config.output_dir.unwrap().display());
    }
    
    Ok(())
}
