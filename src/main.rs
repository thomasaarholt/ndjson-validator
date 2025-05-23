use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;

use cli::{Cli, Commands};
use commands::{handle_validate_dir, handle_validate_file, handle_validate_files};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ValidateFile { file_path, clean, output_dir } => {
            handle_validate_file(file_path, *clean, output_dir)
        },
        
        Commands::ValidateFiles { file_paths, clean, output_dir } => {
            handle_validate_files(file_paths, *clean, output_dir)
        },
        
        Commands::ValidateDir { dir_path, clean, output_dir } => {
            handle_validate_dir(dir_path, *clean, output_dir)
        },
    }
}
