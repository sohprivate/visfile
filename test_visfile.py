#!/usr/bin/env python3
"""
Test script for the visfile library.
This demonstrates how to use the Rust-based directory visualization tool from Python.
"""

import os
import sys

def test_visfile():
    try:
        import visfile
        print("✓ visfile library imported successfully")
    except ImportError as e:
        print(f"✗ Failed to import visfile: {e}")
        print("Make sure to run 'maturin develop' first to build the library")
        return False

    # Create a test directory structure
    test_dir = "test_project"
    os.makedirs(test_dir, exist_ok=True)
    os.makedirs(f"{test_dir}/src", exist_ok=True)
    os.makedirs(f"{test_dir}/docs", exist_ok=True)
    os.makedirs(f"{test_dir}/tests", exist_ok=True)

    # Create some test files with different sizes
    with open(f"{test_dir}/README.md", "w") as f:
        f.write("# Test Project\n\nThis is a test project for visfile.\n" * 100)

    with open(f"{test_dir}/src/main.rs", "w") as f:
        f.write("fn main() {\n    println!(\"Hello, world!\");\n}\n" * 200)

    with open(f"{test_dir}/src/lib.rs", "w") as f:
        f.write("pub fn add(left: usize, right: usize) -> usize {\n    left + right\n}\n" * 150)

    with open(f"{test_dir}/docs/documentation.md", "w") as f:
        f.write("# Documentation\n\nThis is documentation.\n" * 300)

    with open(f"{test_dir}/tests/test.rs", "w") as f:
        f.write("#[test]\nfn test_add() {\n    assert_eq!(2 + 2, 4);\n}\n" * 100)

    with open(f"{test_dir}/Cargo.toml", "w") as f:
        f.write("""[package]
name = "test_project"
version = "0.1.0"
edition = "2021"

[dependencies]
""" * 50)

    print(f"✓ Created test directory structure: {test_dir}")

    # Test the treemap function
    output_file = "test_treemap.png"
    try:
        visfile.treemap(test_dir, output_file)
        print(f"✓ Treemap generated successfully: {output_file}")
        
        if os.path.exists(output_file):
            file_size = os.path.getsize(output_file)
            print(f"✓ Output file size: {file_size} bytes")
        else:
            print("✗ Output file was not created")
            return False
            
    except Exception as e:
        print(f"✗ Failed to generate treemap: {e}")
        return False

    # Test with non-existent directory
    try:
        visfile.treemap("non_existent_directory", "test_error.png")
        print("✗ Should have failed with non-existent directory")
        return False
    except Exception as e:
        print(f"✓ Correctly handled non-existent directory: {e}")

    print("\n🎉 All tests passed!")
    return True

if __name__ == "__main__":
    print("Testing visfile library...")
    print("=" * 50)
    
    success = test_visfile()
    
    print("\n" + "=" * 50)
    if success:
        print("Test completed successfully!")
        print("\nTo use the library:")
        print("  import visfile")
        print('  visfile.treemap("my_project/", "output.png")')
    else:
        print("Test failed!")
        sys.exit(1)