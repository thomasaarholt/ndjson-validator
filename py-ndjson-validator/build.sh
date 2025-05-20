#!/usr/bin/env bash

# List the Python versions you want to build for
PY_VERSIONS=(3.9 3.10 3.11 3.12 3.13)

for version in "${PY_VERSIONS[@]}"; do
    echo "=== Building for Python $version ==="

    # Remove existing virtual environment if present
    rm -rf .venv

    # Create new virtual environment with uv
    uv venv --python $version

    # Activate virtual environment
    source .venv/bin/activate

    # Optional: upgrade pip and install maturin
    uv pip install --upgrade maturin

    # Build the wheel
    maturin build --release

    # Deactivate the virtual environment
    deactivate
done
