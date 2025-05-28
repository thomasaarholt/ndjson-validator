from __future__ import annotations
from pathlib import Path
import json

from py_ndjson_validator.py_ndjson_validator import clean_ndjson_rust_serde, clean_ndjson_rust_sonic, ErrorEntry


def py_clean_ndjson(
    files: list[Path], output_dir: Path
) -> tuple[list[Path], list[str]]:
    output_dir.mkdir(exist_ok=True)

    new_files = []
    errors = []
    for file in files:
        new_file = output_dir / file.name
        lines = file.read_text().splitlines()

        new_lines: list[str] = []
        for line_no, line in enumerate(lines, start=1):
            try:

                _ = json.loads(line)
                new_lines.append(line)
            except Exception:
                errors.append(f"File {file.name} line {line_no}")
        new_file.write_text("\n".join(new_lines))
        new_files.append(new_file)
    return new_files, errors


def clean_ndjson_serde(
    files: list[Path], output_dir: Path
) -> tuple[list[Path], list[ErrorEntry]]:
    """
    Validates and cleans NDJSON files using serde_json.

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
    if not output_dir.exists():
        output_dir.mkdir(parents=True, exist_ok=True)

    file_paths = [str(file) for file in files]
    output_dir_str = str(output_dir)

    cleaned_files, errors = clean_ndjson_rust_serde(file_paths, output_dir_str)

    cleaned_file_paths = [Path(file) for file in cleaned_files]

    return cleaned_file_paths, errors


def clean_ndjson_sonic(
    files: list[Path], output_dir: Path
) -> tuple[list[Path], list[ErrorEntry]]:
    """
    Validates and cleans NDJSON files using sonic-rs (faster alternative).

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
    if not output_dir.exists():
        output_dir.mkdir(parents=True, exist_ok=True)

    file_paths = [str(file) for file in files]
    output_dir_str = str(output_dir)

    cleaned_files, errors = clean_ndjson_rust_sonic(file_paths, output_dir_str)

    cleaned_file_paths = [Path(file) for file in cleaned_files]

    return cleaned_file_paths, errors
