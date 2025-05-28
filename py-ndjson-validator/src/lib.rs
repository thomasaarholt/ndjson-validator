use std::path::PathBuf;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use ndjson_validator::{validate_files_serde, validate_files_sonic, ValidatorConfig};

#[pyclass]
#[derive(Debug)]
struct ErrorEntry {
    #[pyo3(get)]
    file_path: String,
    #[pyo3(get)]
    line_number: usize,
    #[pyo3(get)]
    line_content: String,
    #[pyo3(get)]
    error: String,
}


#[pyfunction]
fn clean_ndjson_rust_serde(files: Vec<String>, output_dir: &str) -> PyResult<(Vec<String>, Vec<ErrorEntry>)> {
    // Convert Python list of paths to Rust PathBuf
    let file_paths: Vec<PathBuf> = files.into_iter().map(PathBuf::from).collect();

    // Create output directory path
    let output_dir_path = PathBuf::from(output_dir);

    // Create validator config
    let config = ValidatorConfig {
        clean_files: true,
        output_dir: Some(output_dir_path.clone()),
    };

    // Run validation and cleaning
    let errors = match validate_files_serde(&file_paths, &config) {
        Ok(errors) => errors,
        Err(err) => return Err(PyValueError::new_err(format!("Validation error: {}", err))),
    };

    // Create Python list of cleaned file paths
    let mut cleaned_files: Vec<String> = Vec::new();
    for file_path in &file_paths {
        if let Some(file_name) = file_path.file_name() {
            let cleaned_path = output_dir_path.join(file_name);
            cleaned_files.push(cleaned_path.to_string_lossy().to_string());
        }
    }

    // Create Python list of errors
    let mut json_errors: Vec<ErrorEntry> = Vec::new();
    for error in errors {
        let error_entry = ErrorEntry {
            file_path: error.file_path.to_string_lossy().to_string(),
            line_number: error.line_number,
            line_content: error.line_content,
            error: error.error,
        };

        json_errors.push(error_entry);
    }

    // Create result dictionary
    Ok((cleaned_files, json_errors))
}

#[pyfunction]
fn clean_ndjson_rust_sonic(files: Vec<String>, output_dir: &str) -> PyResult<(Vec<String>, Vec<ErrorEntry>)> {
    // Convert Python list of paths to Rust PathBuf
    let file_paths: Vec<PathBuf> = files.into_iter().map(PathBuf::from).collect();

    // Create output directory path
    let output_dir_path = PathBuf::from(output_dir);

    // Create validator config
    let config = ValidatorConfig {
        clean_files: true,
        output_dir: Some(output_dir_path.clone()),
    };

    // Run validation and cleaning using sonic-rs
    let errors = match validate_files_sonic(&file_paths, &config) {
        Ok(errors) => errors,
        Err(err) => return Err(PyValueError::new_err(format!("Validation error: {}", err))),
    };

    // Create Python list of cleaned file paths
    let mut cleaned_files: Vec<String> = Vec::new();
    for file_path in &file_paths {
        if let Some(file_name) = file_path.file_name() {
            let cleaned_path = output_dir_path.join(file_name);
            cleaned_files.push(cleaned_path.to_string_lossy().to_string());
        }
    }

    // Create Python list of errors
    let mut json_errors: Vec<ErrorEntry> = Vec::new();
    for error in errors {
        let error_entry = ErrorEntry {
            file_path: error.file_path.to_string_lossy().to_string(),
            line_number: error.line_number,
            line_content: error.line_content,
            error: error.error,
        };

        json_errors.push(error_entry);
    }

    // Create result dictionary
    Ok((cleaned_files, json_errors))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
#[pyo3(name = "py_ndjson_validator")]
fn py_ndjson_validator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let _ = m.add_function(wrap_pyfunction!(clean_ndjson_rust_serde, m)?);
    let _ = m.add_function(wrap_pyfunction!(clean_ndjson_rust_sonic, m)?);
    m.add_class::<ErrorEntry>()?;
    Ok(())
}
