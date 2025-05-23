use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Tool for validating and cleaning ND-JSON files
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
