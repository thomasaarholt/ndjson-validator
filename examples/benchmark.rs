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

    println!("\nRunning benchmark");
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
                let long_payload = "x".repeat(3000); // Adjusted for ~20000 chars total

                match error_type {
                    0 => { // Syntax error: missing colon after level4_long_key_name_jkl
                        let s = format!(
                            r#"{{ "level1_long_key_name_abc": {{ "level2_long_key_name_def": {{ "level3_long_key_name_ghi": {{ "level4_long_key_name_jkl"  "level5_long_key_name_mno": {{ "level6_long_key_name_pqr": {{ "level7_long_key_name_stu": {{ "level8_long_key_name_vwx": "{}" }} }} }} }} }} }} }}"#,
                            long_payload
                        );
                        writeln!(writer, "{}", s)?;
                    }
                    1 => { // Unclosed brace (missing final '}')
                        let s = format!(
                            r#"{{ "level1_long_key_name_abc": {{ "level2_long_key_name_def": {{ "level3_long_key_name_ghi": {{ "level4_long_key_name_jkl": {{ "level5_long_key_name_mno": {{ "level6_long_key_name_pqr": {{ "level7_long_key_name_stu": {{ "level8_long_key_name_vwx": "{}" }} }} }} }} }} }}"#,
                            long_payload
                        );
                        writeln!(writer, "{}", s)?;
                    }
                    2 => { // Missing quotes for a key (level4_long_key_name_jkl)
                        let s = format!(
                            r#"{{ "level1_long_key_name_abc": {{ "level2_long_key_name_def": {{ "level3_long_key_name_ghi": {{ level4_long_key_name_jkl: {{ "level5_long_key_name_mno": {{ "level6_long_key_name_pqr": {{ "level7_long_key_name_stu": {{ "level8_long_key_name_vwx": "{}" }} }} }} }} }} }} }} }}"#,
                            long_payload
                        );
                        writeln!(writer, "{}", s)?;
                    }
                    3 => { // Invalid value (unquoted literal)
                        let s = format!(
                            r#"{{ "level1_long_key_name_abc": {{ "level2_long_key_name_def": {{ "level3_long_key_name_ghi": {{ "level4_long_key_name_jkl": {{ "level5_long_key_name_mno": {{ "level6_long_key_name_pqr": {{ "level7_long_key_name_stu": {{ "level8_long_key_name_vwx": invalid_value_{} }} }} }} }} }} }} }} }}"#,
                            long_payload
                        );
                        writeln!(writer, "{}", s)?;
                    }
                    _ => { // Invalid escape sequence
                        let s = format!(
                            r#"{{ "level1_long_key_name_abc": {{ "level2_long_key_name_def": {{ "level3_long_key_name_ghi": {{ "level4_long_key_name_jkl": {{ "level5_long_key_name_mno": {{ "level6_long_key_name_pqr": {{ "level7_long_key_name_stu": {{ "level8_long_key_name_vwx": "bad\\escape_{}" }} }} }} }} }} }} }} }}"#,
                            long_payload
                        );
                        writeln!(writer, "{}", s)?;
                    }
                }
            } else {
                // Generate a valid JSON line
                let long_payload = "v".repeat(3000); // Adjusted for ~20000 chars total
                let s = format!(
                    r#"{{ "level1_long_key_name_abc": {{ "level2_long_key_name_def": {{ "level3_long_key_name_ghi": {{ "level4_long_key_name_jkl": {{ "level5_long_key_name_mno": {{ "level6_long_key_name_pqr": {{ "level7_long_key_name_stu": {{ "level8_long_key_name_vwx": "{}" }} }} }} }} }} }} }} }}"#,
                    long_payload
                );
                writeln!(writer, "{}", s)?;
            }
        }
        
        writer.flush()?;
    }
    
    Ok(())
}
