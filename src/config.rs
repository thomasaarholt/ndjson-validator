use std::path::PathBuf;

/// Configuration options for the ND-JSON validator
#[derive(Debug, Clone)]
pub struct ValidatorConfig {
    /// Whether to clean files by removing invalid JSON lines
    pub clean_files: bool,
    
    /// Directory to write cleaned files to (if clean_files is true)
    pub output_dir: Option<PathBuf>,
    
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            clean_files: false,
            output_dir: None,
        }
    }
}
