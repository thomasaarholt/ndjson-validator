# ND-JSON Validator

A high-performance Rust library and CLI tool for validating and cleaning ND-JSON (Newline Delimited JSON) files.

## Features

- ✅ Fast validation of ND-JSON files
- ✅ Parallel processing for multi-file validation
- ✅ Clean mode to remove invalid JSON lines
- ✅ Detailed error reporting
- ✅ Support for validating individual files, multiple files, or entire directories
- ✅ Easy-to-use programmatic API for integration into other tools

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

## Performance

The library uses parallel processing with [Rayon](https://github.com/rayon-rs/rayon) to validate multiple files simultaneously, making it very efficient for large datasets.

## License

MIT
