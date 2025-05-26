from typing import List, Tuple

class ErrorEntry:
    """
    Represents an error found during NDJSON validation.
    
    Attributes:
        file_path (str): Path to the file containing the error
        line_number (int): Line number where the error occurred
        line_content (str): Content of the line with the error
        error (str): Description of the error
    """
    file_path: str
    line_number: int
    line_content: str
    error: str

def clean_ndjson_rust(files: List[str], output_dir: str) -> Tuple[List[str], List[ErrorEntry]]:
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
    ...
