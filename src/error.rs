use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NdJsonError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("JSON parsing error at line {line} in file {file}: {error}")]
    JsonParse {
        file: String,
        line: usize,
        error: serde_json::Error,
    },
    
    #[error("No ND-JSON files found in directory: {0}")]
    NoFilesFound(String),
    
    #[error("Failed to create output directory: {0}")]
    FailedToCreateOutputDir(String),
    
    #[error("File system error: {0}")]
    Walkdir(#[from] walkdir::Error),
}

pub type Result<T> = std::result::Result<T, NdJsonError>;

/// Represents a validation error in an ND-JSON file
#[derive(Debug)]
pub struct ValidationError {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub line_content: String,
    pub error: String,
}

/// Summary of validation results
#[derive(Debug)]
pub struct ValidationSummary {
    pub total_files: usize,
    pub files_with_errors: usize,
    pub total_errors: usize,
}
