from pathlib import Path
from typing import List, Tuple
from py_ndjson_validator import clean_ndjson_rust, ErrorEntry



def clean_ndjson(files: List[Path], output_dir: Path) -> Tuple[List[Path], List[ErrorEntry]]:
    """
    Validates and cleans NDJSON files.
    
    Args:
        files: List of file paths to validate and clean
        output_dir: Directory to write cleaned files to
        
    Returns:
        A tuple containing:
        - List of paths to the cleaned files
        - List of errors found during validation
        
    Raises:
        ValueError: If there's an error during validation
    """
    if not output_dir.exists():
        output_dir.mkdir(parents=True, exist_ok=True)

    file_paths = [str(file) for file in files]
    output_dir_str = str(output_dir)

    cleaned_files, errors = clean_ndjson_rust(file_paths, output_dir_str)

    cleaned_file_paths = [Path(file) for file in cleaned_files]

    return cleaned_file_paths, errors
