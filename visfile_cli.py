#!/usr/bin/env python3
"""
CLI interface for visfile library.
Usage: visfile <directory> [output.png]
"""

import sys
import os
import argparse
from pathlib import Path

def main():
    parser = argparse.ArgumentParser(
        description="🗂️ VisFile - Directory visualization tool",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  visfile .                          # Current directory -> treemap.png
  visfile /path/to/project           # Project -> treemap.png
  visfile ~/Documents docs.png       # Documents -> docs.png
  visfile . --type pie               # Pie chart visualization
  visfile src/ chart.png -t pie      # Source folder pie chart

🚀 Fast directory scanning with Rust + Beautiful visualizations
        """
    )
    
    parser.add_argument(
        'directory',
        help='Directory path to analyze'
    )
    
    parser.add_argument(
        'output',
        nargs='?',
        default='treemap.png',
        help='Output PNG file path (default: treemap.png)'
    )
    
    parser.add_argument(
        '--type', '-t',
        choices=['treemap', 'pie'],
        default='treemap',
        help='Visualization type: treemap (hierarchical) or pie (percentage)'
    )
    
    parser.add_argument(
        '--version',
        action='version',
        version='visfile 0.1.0'
    )

    args = parser.parse_args()

    # Validate input directory
    input_path = Path(args.directory).expanduser().resolve()
    if not input_path.exists():
        print(f"❌ Error: Directory '{args.directory}' does not exist", file=sys.stderr)
        sys.exit(1)
    
    if not input_path.is_dir():
        print(f"❌ Error: '{args.directory}' is not a directory", file=sys.stderr)
        sys.exit(1)

    # Validate output path
    output_path = Path(args.output).expanduser().resolve()
    output_dir = output_path.parent
    
    if not output_dir.exists():
        try:
            output_dir.mkdir(parents=True, exist_ok=True)
        except PermissionError:
            print(f"❌ Error: Cannot create output directory '{output_dir}'", file=sys.stderr)
            sys.exit(1)

    # Import and use visfile
    try:
        import visfile
    except ImportError as e:
        print("❌ Error: visfile library not found", file=sys.stderr)
        print("Please install visfile first:", file=sys.stderr)
        print("  1. git clone https://github.com/sohprivate/visfile.git", file=sys.stderr)
        print("  2. cd visfile", file=sys.stderr)
        print("  3. pip install maturin", file=sys.stderr)
        print("  4. maturin develop", file=sys.stderr)
        sys.exit(1)

    # Generate visualization
    try:
        print(f"🔍 Scanning directory: {input_path}")
        
        if args.type == 'pie':
            print("📊 Generating pie chart...")
            visfile.piechart(str(input_path), str(output_path))
            viz_type = "pie chart"
        else:
            print("🗂️ Generating treemap...")
            visfile.treemap(str(input_path), str(output_path))
            viz_type = "treemap"
        
        # Check if file was created and get size
        if output_path.exists():
            file_size = output_path.stat().st_size
            size_str = format_file_size(file_size)
            print(f"✅ {viz_type.title()} generated: {output_path} ({size_str})")
        else:
            print("❌ Error: Output file was not created", file=sys.stderr)
            sys.exit(1)
            
    except Exception as e:
        print(f"❌ Error generating {args.type}: {e}", file=sys.stderr)
        sys.exit(1)

def format_file_size(size_bytes):
    """Format file size in human readable format"""
    if size_bytes == 0:
        return "0 B"
    
    size_names = ["B", "KB", "MB", "GB"]
    size = float(size_bytes)
    i = 0
    
    while size >= 1024.0 and i < len(size_names) - 1:
        size /= 1024.0
        i += 1
    
    if i == 0:
        return f"{int(size)} {size_names[i]}"
    else:
        return f"{size:.1f} {size_names[i]}"

if __name__ == "__main__":
    main()