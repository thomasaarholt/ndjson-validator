mod cleaner;
mod config;
mod error;
mod processor;
mod validator;

// Re-export public API
pub use config::ValidatorConfig;
pub use error::{NdJsonError, Result, ValidationError, ValidationSummary};
pub use processor::{
    process_file_serde, validate_directory_with_summary_serde, 
    validate_files_serde, validate_files_with_summary_serde,
    process_file_sonic, validate_files_sonic, validate_files_with_summary_sonic,
    validate_directory_with_summary_sonic
};
pub use validator::{validate_file_serde, validate_file_sonic};


