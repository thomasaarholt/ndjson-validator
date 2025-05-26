use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::time::Instant;

use ndjson_validator::{
    validate_directory_with_summary, validate_file, validate_files_with_summary, 
    ValidationError, ValidationSummary, ValidatorConfig
};

/// Prints a summary of validation results
pub fn print_summary(summary: &ValidationSummary, duration: std::time::Duration) {
    println!("Validation Summary:");
    println!("  Total files processed: {}", summary.total_files);
    println!("  Files with errors: {}", summary.files_with_errors);
    println!("  Total errors found: {}", summary.total_errors);
    println!("  Time taken: {:.2?}", duration);
    
    if summary.total_errors == 0 {
        println!("✅ All files are valid!");
    } else {
        println!("❌ Found {} errors in {} files", summary.total_errors, summary.files_with_errors);
    }
}

/// Prints detailed error information
pub fn print_errors(errors: &[ValidationError]) {
    if errors.is_empty() {
        return;
    }
    
    let max_errors_to_display = 10;
    let display_count = std::cmp::min(errors.len(), max_errors_to_display);
    
    println!("\nError Details (showing first {}/{}):", display_count, errors.len());
    
    for (i, error) in errors.iter().take(display_count).enumerate() {
        println!("{}. File: {}", i + 1, error.file_path.display());
        println!("   Line {}: {}", error.line_number, error.line_content);
        println!("   Error: {}", error.error);
        println!();
    }
    
    if errors.len() > max_errors_to_display {
        println!("... and {} more errors", errors.len() - max_errors_to_display);
    }
}

/// Prints information about the cleaning process
pub fn print_cleaning_info(input_path: &Path, output_dir: &Path, error_count: usize) {
    let file_name = input_path.file_name().unwrap_or_default();
    let output_path = output_dir.join(file_name);
    
    if error_count == 0 {
        println!("No errors to clean up.");
    } else {
        println!("Cleaned file written to: {}", output_path.display());
        println!("Removed {} invalid lines", error_count);
    }
}

pub fn handle_validate_file(file_path: &PathBuf, clean: bool, output_dir: &Option<PathBuf>) -> Result<()> {
    println!("Validating file: {}", file_path.display());
    
    let _config = ValidatorConfig {
        clean_files: clean,
        output_dir: output_dir.clone(),
    };
    
    let start = Instant::now();
    let errors = validate_file(file_path)
        .with_context(|| format!("Failed to validate file: {}", file_path.display()))?;
    let duration = start.elapsed();
    
    if errors.is_empty() {
        println!("✅ File is valid! Validation took {:.2?}", duration);
    } else {
        println!("❌ Found {} errors in file. Validation took {:.2?}", errors.len(), duration);
        print_errors(&errors);
        
        if clean {
            print_cleaning_info(file_path, output_dir.as_ref().unwrap(), errors.len());
        }
    }
    
    Ok(())
}

pub fn handle_validate_files(file_paths: &[PathBuf], clean: bool, output_dir: &Option<PathBuf>) -> Result<()> {
    println!("Validating {} files", file_paths.len());
    
    let config = ValidatorConfig {
        clean_files: clean,
        output_dir: output_dir.clone(),
    };
    
    let start = Instant::now();
    let (summary, errors) = validate_files_with_summary(file_paths, &config)
        .with_context(|| "Failed to validate files")?;
    let duration = start.elapsed();
    
    print_summary(&summary, duration);
    
    if !errors.is_empty() {
        print_errors(&errors);
    }
    
    Ok(())
}

pub fn handle_validate_dir(dir_path: &PathBuf, clean: bool, output_dir: &Option<PathBuf>) -> Result<()> {
    println!("Validating all ND-JSON files in: {}", dir_path.display());
    
    let config = ValidatorConfig {
        clean_files: clean,
        output_dir: output_dir.clone(),
    };
    
    let start = Instant::now();
    let (summary, errors) = validate_directory_with_summary(dir_path, &config)
        .with_context(|| format!("Failed to validate files in directory: {}", dir_path.display()))?;
    let duration = start.elapsed();
    
    print_summary(&summary, duration);
    
    if !errors.is_empty() {
        print_errors(&errors);
    }
    
    Ok(())
}
