# ND-JSON Validator

A high-performance Rust library and CLI tool for validating and cleaning ND-JSON (Newline Delimited JSON) files.

## Features

- ✅ Fast validation of ND-JSON files with detailed error reporting
- ✅ Parallel processing for multi-file validation
- ✅ Clean mode to remove invalid JSON lines and create corrected files
- ✅ Support for validating individual files, multiple files, or entire directories
- ✅ Easy-to-use programmatic API for integration into other tools
- ✅ Well-organized modular codebase for easy maintenance and extension

## Project Structure

This project is organized into several modules for better maintainability:

```
src/
├── lib.rs           # Main library entry point and public API
├── main.rs          # CLI application entry point
├── cli.rs           # Command-line interface definitions
├── commands.rs      # Command handlers and output formatting
├── config.rs        # Configuration structures
├── error.rs         # Error types and definitions
├── validator.rs     # Core validation logic
├── cleaner.rs       # File cleaning functionality
└── processor.rs     # High-level processing functions

tests/
└── integration.rs   # Integration tests
```

## Installation

### From Cargo

```bash
cargo install ndjson-validator
```

### From Source

```bash
git clone https://github.com/yourusername/ndjson-validator.git
cd ndjson-validator
cargo build --release
```

The binary will be available at `target/release/ndjson-validator`.

## CLI Usage

### Validate a Single File

```bash
ndjson-validator validate-file path/to/file.ndjson
```

### Validate Multiple Files

```bash
ndjson-validator validate-files file1.ndjson file2.ndjson file3.ndjson
```

### Validate All Files in a Directory

```bash
ndjson-validator validate-dir path/to/directory
```

### Clean Invalid JSON Lines

Add the `--clean` flag and specify an output directory with `--output-dir`:

```bash
ndjson-validator validate-file path/to/file.ndjson --clean --output-dir path/to/output
```

## Library Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ndjson-validator = "0.1.0"
```

### Example: Validating a Single File

```rust
use std::path::Path;
use ndjson_validator::{validate_file, ValidatorConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("data.ndjson");
    let errors = validate_file(file_path)?;
    
    if errors.is_empty() {
        println!("File is valid!");
    } else {
        println!("Found {} errors in file", errors.len());
        for error in errors {
            println!("Line {}: {}", error.line_number, error.error);
        }
    }
    
    Ok(())
}
```

### Example: Validating and Cleaning Multiple Files

```rust
use std::path::{Path, PathBuf};
use ndjson_validator::{validate_multiple, ValidatorConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = vec![
        PathBuf::from("file1.ndjson"),
        PathBuf::from("file2.ndjson"),
        PathBuf::from("file3.ndjson"),
    ];
    
    let config = ValidatorConfig {
        clean_files: true,
        output_dir: Some(PathBuf::from("cleaned")),
        parallel: true,
    };
    
    let (summary, errors) = validate_multiple(&files, &config)?;
    
    println!("Processed {} files", summary.total_files);
    println!("Found {} errors in {} files", summary.total_errors, summary.files_with_errors);
    
    Ok(())
}
```

## Library API

The library provides a clean, modular API organized into focused modules:

### Core Functions

- `validate_file()` - Validate a single ND-JSON file
- `validate_files()` - Validate multiple files with optional parallel processing
- `validate_directory()` - Validate all ND-JSON files in a directory
- `process_file()` - Validate and optionally clean a single file
- `validate_multiple()` - Validate multiple files and return summary statistics

### Configuration

```rust
use ndjson_validator::ValidatorConfig;

let config = ValidatorConfig {
    clean_files: true,                          // Enable cleaning mode
    output_dir: Some(PathBuf::from("output")),  // Where to write cleaned files
    parallel: true,                             // Use parallel processing
};
```

### Error Types

The library uses custom error types for better error handling:

```rust
use ndjson_validator::{NdJsonError, ValidationError, ValidationSummary};

// Detailed error information for each invalid line
struct ValidationError {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub line_content: String,
    pub error: String,
}

// Summary of validation results
struct ValidationSummary {
    pub total_files: usize,
    pub files_with_errors: usize,
    pub total_errors: usize,
}
```

## Performance

The library uses parallel processing with [Rayon](https://github.com/rayon-rs/rayon) to validate multiple files simultaneously, making it very efficient for large datasets.

## License

MIT
