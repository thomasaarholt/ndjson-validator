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

def clean_ndjson_rust_serde(
    files: list[str], output_dir: str
) -> tuple[list[str], list[ErrorEntry]]:
    """
    Validates and cleans NDJSON files using the serde parser.

    Args:
        files: list of file paths to validate and clean
        output_dir: Directory to write cleaned files to

    Returns:
        A tuple containing:
        - list of paths to the cleaned files
        - list of errors found during validation

    Raises:
        ValueError: If there's an error during validation
    """
    ...

def clean_ndjson_rust_sonic(
    files: list[str], output_dir: str
) -> tuple[list[str], list[ErrorEntry]]:
    """
    Validates and cleans NDJSON files using the Sonic parser.

    Args:
        files: list of file paths to validate and clean
        output_dir: Directory to write cleaned files to

    Returns:
        A tuple containing:
        - list of paths to the cleaned files
        - list of errors found during validation

    Raises:
        ValueError: If there's an error during validation
    """
    ...
