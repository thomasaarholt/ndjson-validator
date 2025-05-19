use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ndjson_validator::{validate_directory_with_summary, validate_file, validate_multiple, ValidationError, ValidatorConfig};

/// Tool for validating and cleaning ND-JSON files
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a single ND-JSON file
    ValidateFile {
        /// Path to the ND-JSON file
        #[arg(required = true)]
        file_path: PathBuf,
        
        /// Clean the file by removing invalid JSON lines
        #[arg(short, long)]
        clean: bool,
        
        /// Directory to output cleaned files to
        #[arg(short, long, required_if_eq("clean", "true"))]
        output_dir: Option<PathBuf>,
    },
    
    /// Validate multiple ND-JSON files
    ValidateFiles {
        /// Paths to ND-JSON files
        #[arg(required = true)]
        file_paths: Vec<PathBuf>,
        
        /// Clean files by removing invalid JSON lines
        #[arg(short, long)]
        clean: bool,
        
        /// Directory to output cleaned files to
        #[arg(short, long, required_if_eq("clean", "true"))]
        output_dir: Option<PathBuf>,
    },
    
    /// Validate all ND-JSON files in a directory
    ValidateDir {
        /// Path to directory containing ND-JSON files
        #[arg(required = true)]
        dir_path: PathBuf,
        
        /// Clean files by removing invalid JSON lines
        #[arg(short, long)]
        clean: bool,
        
        /// Directory to output cleaned files to
        #[arg(short, long, required_if_eq("clean", "true"))]
        output_dir: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ValidateFile { file_path, clean, output_dir } => {
            println!("Validating file: {}", file_path.display());
            
            let _config = ValidatorConfig {
                clean_files: *clean,
                output_dir: output_dir.clone(),
                parallel: true,
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
                
                if *clean {
                    process_cleaning(file_path, output_dir.as_ref().unwrap(), &errors)?;
                }
            }
        },
        
        Commands::ValidateFiles { file_paths, clean, output_dir } => {
            println!("Validating {} files", file_paths.len());
            
            let config = ValidatorConfig {
                clean_files: *clean,
                output_dir: output_dir.clone(),
                parallel: true,
            };
            
            let start = Instant::now();
            let (summary, errors) = validate_multiple(file_paths, &config)
                .with_context(|| "Failed to validate files")?;
            let duration = start.elapsed();
            
            print_summary(&summary, duration);
            
            if !errors.is_empty() {
                print_errors(&errors);
            }
        },
        
        Commands::ValidateDir { dir_path, clean, output_dir } => {
            println!("Validating all ND-JSON files in: {}", dir_path.display());
            
            let config = ValidatorConfig {
                clean_files: *clean,
                output_dir: output_dir.clone(),
                parallel: true,
            };
            
            let start = Instant::now();
            let (summary, errors) = validate_directory_with_summary(dir_path, &config)
                .with_context(|| format!("Failed to validate files in directory: {}", dir_path.display()))?;
            let duration = start.elapsed();
            
            print_summary(&summary, duration);
            
            if !errors.is_empty() {
                print_errors(&errors);
            }
        },
    }

    Ok(())
}

fn print_summary(summary: &ndjson_validator::ValidationSummary, duration: std::time::Duration) {
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

fn print_errors(errors: &[ValidationError]) {
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

fn process_cleaning(input_path: &Path, output_dir: &Path, errors: &[ValidationError]) -> Result<()> {
    if errors.is_empty() {
        println!("No errors to clean up.");
        return Ok(());
    }
    
    let file_name = input_path.file_name().unwrap_or_default();
    let output_path = output_dir.join(file_name);
    
    println!("Cleaned file written to: {}", output_path.display());
    println!("Removed {} invalid lines", errors.len());
    
    Ok(())
}
