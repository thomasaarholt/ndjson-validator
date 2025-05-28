#!/usr/bin/env python3
"""
Test script to compare the performance and functionality of the serde_json vs sonic-rs implementations.
"""

import py_ndjson_validator
import tempfile
import os
import time
from pathlib import Path

def create_test_files():
    """Create test NDJSON files with valid and invalid JSON lines."""
    
    # Create a temporary directory
    temp_dir = tempfile.mkdtemp()
    
    # Create test file with mixed valid/invalid JSON
    test_file = os.path.join(temp_dir, "test.ndjson")
    with open(test_file, 'w') as f:
        f.write('{"name": "Alice", "age": 30}\n')  # Valid
        f.write('{"name": "Bob", "age": 25\n')     # Invalid - missing closing brace
        f.write('{"name": "Charlie", "age": 35}\n') # Valid
        f.write('[1, 2, 3\n')                      # Invalid - missing closing bracket
        f.write('{"valid": true}\n')               # Valid
    
    return [test_file], temp_dir

def test_both_methods():
    """Test both serde_json and sonic-rs methods."""
    
    print("🧪 Testing both serde_json and sonic-rs implementations...")
    
    # Create test files
    test_files, base_temp_dir = create_test_files()
    
    # Create output directories
    serde_output = os.path.join(base_temp_dir, "serde_output")
    sonic_output = os.path.join(base_temp_dir, "sonic_output")
    os.makedirs(serde_output, exist_ok=True)
    os.makedirs(sonic_output, exist_ok=True)
    
    print(f"📁 Test file: {test_files[0]}")
    print(f"📁 Serde output: {serde_output}")
    print(f"📁 Sonic output: {sonic_output}")
    
    # Test serde_json implementation
    print("\n🔧 Testing serde_json implementation...")
    start_time = time.time()
    serde_cleaned, serde_errors = py_ndjson_validator.clean_ndjson([Path(f) for f in test_files], Path(serde_output))
    serde_time = time.time() - start_time
    
    print(f"⏱️  Serde time: {serde_time:.4f} seconds")
    print(f"📄 Cleaned files: {[str(f) for f in serde_cleaned]}")
    print(f"❌ Errors found: {len(serde_errors)}")
    for error in serde_errors:
        print(f"   Line {error.line_number}: {error.error}")
    
    # Test sonic-rs implementation
    print("\n⚡ Testing sonic-rs implementation...")
    start_time = time.time()
    sonic_cleaned, sonic_errors = py_ndjson_validator.clean_ndjson_sonic([Path(f) for f in test_files], Path(sonic_output))
    sonic_time = time.time() - start_time
    
    print(f"⏱️  Sonic time: {sonic_time:.4f} seconds")
    print(f"📄 Cleaned files: {[str(f) for f in sonic_cleaned]}")
    print(f"❌ Errors found: {len(sonic_errors)}")
    for error in sonic_errors:
        print(f"   Line {error.line_number}: {error.error}")
    
    # Compare results
    print("\n📊 Comparison:")
    print(f"   Serde errors: {len(serde_errors)}")
    print(f"   Sonic errors: {len(sonic_errors)}")
    print(f"   Same error count: {len(serde_errors) == len(sonic_errors)}")
    
    if sonic_time > 0:
        speedup = serde_time / sonic_time
        print(f"   Speed ratio (serde/sonic): {speedup:.2f}x")
    
    # Check cleaned file contents
    if serde_cleaned and sonic_cleaned:
        serde_file = str(serde_cleaned[0])
        sonic_file = str(sonic_cleaned[0])
        
        if os.path.exists(serde_file) and os.path.exists(sonic_file):
            with open(serde_file, 'r') as f:
                serde_content = f.read()
            with open(sonic_file, 'r') as f:
                sonic_content = f.read()
            
            print(f"   Cleaned files identical: {serde_content == sonic_content}")
            print(f"   Serde cleaned lines: {len(serde_content.strip().split(chr(10))) if serde_content.strip() else 0}")
            print(f"   Sonic cleaned lines: {len(sonic_content.strip().split(chr(10))) if sonic_content.strip() else 0}")
    
    # Cleanup
    import shutil
    shutil.rmtree(base_temp_dir)
    
    print("\n✅ Test completed successfully!")

if __name__ == "__main__":
    test_both_methods()
