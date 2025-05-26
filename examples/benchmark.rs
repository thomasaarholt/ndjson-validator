use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use ndjson_validator::{validate_directory_with_summary, ValidatorConfig};
use rand::{Rng, thread_rng};

fn main() -> io::Result<()> {
    // Create a directory for test files
    let test_dir = PathBuf::from("bench_files");
    std::fs::create_dir_all(&test_dir)?;
    
    // Generate large test files
    let num_files = 10;
    let lines_per_file = 100_000; // 100K lines per file
    let error_rate = 0.001; // 0.1% error rate
    
    println!("Generating {} test files with {} lines each...", num_files, lines_per_file);
    generate_test_files(&test_dir, num_files, lines_per_file, error_rate)?;
    
    println!("\nRunning benchmark with parallel processing...");
    let parallel_config = ValidatorConfig {
        clean_files: false,
        output_dir: None,
    };
    
    let start = Instant::now();
    let (summary, _) = validate_directory_with_summary(&test_dir, &parallel_config)
        .expect("Failed to validate directory");
    let parallel_duration = start.elapsed();
    
    println!("Parallel processing results:");
    println!("  Total files: {}", summary.total_files);
    println!("  Files with errors: {}", summary.files_with_errors);
    println!("  Total errors: {}", summary.total_errors);
    println!("  Time taken: {:.2?}", parallel_duration);
    
    Ok(())
}

fn generate_test_files(
    dir: &Path,
    num_files: usize,
    lines_per_file: usize,
    error_rate: f64,
) -> io::Result<()> {
    let mut rng = thread_rng();
    
    for i in 0..num_files {
        let file_path = dir.join(format!("test_file_{}.ndjson", i));
        let file = File::create(&file_path)?;
        let mut writer = BufWriter::new(file);
        
        for _ in 0..lines_per_file {
            let has_error = rng.gen::<f64>() < error_rate;
            
            if has_error {
                // Generate an invalid JSON line
                let error_type = rng.gen_range(0..5);
                match error_type {
                    0 => writeln!(writer, "{{\"name\": \"Invalid\", \"age\":}}")?, // Syntax error
                    1 => writeln!(writer, "{{\"name\": \"Unclosed\", \"age\": 25")?, // Unclosed brace
                    2 => writeln!(writer, "{{name: \"No quotes\", \"age\": 30}}")?, // Missing quotes
                    3 => writeln!(writer, "{{\"name\": \"Invalid value\", \"age\": test}}")?, // Invalid value
                    _ => writeln!(writer, "{{\"name\": \"Invalid escape\", \"notes\": \"Bad \\escape\"}}")?, // Invalid escape
                }
            } else {
                // Generate a valid JSON line
                let age = rng.gen_range(20..80);
                let id = rng.gen_range(1000..9999);
                writeln!(
                    writer,
                    "{{\"name\": \"Person-{}\", \"age\": {}, \"id\": {}, \"active\": {}}}",
                    id, age, id, rng.gen::<bool>()
                )?;
            }
        }
        
        writer.flush()?;
    }
    
    Ok(())
}
