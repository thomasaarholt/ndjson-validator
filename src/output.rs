use std::path::Path;
use std::time::Duration;

use crate::error::{ValidationError, ValidationSummary};

/// Prints a summary of validation results
pub fn print_summary(summary: &ValidationSummary, duration: Duration) {
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
