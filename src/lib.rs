mod cleaner;
mod config;
mod error;
mod processor;
mod validator;

// Re-export public API
pub use config::ValidatorConfig;
pub use error::{NdJsonError, Result, ValidationError, ValidationSummary};
pub use processor::{
    process_file, validate_directory_with_summary, 
    validate_files, validate_files_with_summary
};
pub use validator::validate_file;


