#!/bin/bash
# VisFile installer script

echo "🚀 Installing VisFile..."

# Check if Python is available
if ! command -v python &> /dev/null; then
    echo "❌ Python is not installed. Please install Python 3.8+ first."
    exit 1
fi

# Get Python path
PYTHON_PATH=$(which python)
echo "Found Python at: $PYTHON_PATH"

# Create visfile command
echo "Creating visfile command..."

# Find Python bin directory
PYTHON_BIN_DIR=$(dirname "$PYTHON_PATH")

# Create the visfile executable
cat > "$PYTHON_BIN_DIR/visfile" << 'EOF'
#!/usr/bin/env python
import sys
import os
# Add the visfile directory to Python path
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from visfile_cli import main

if __name__ == '__main__':
    main()
EOF

# Make it executable
chmod +x "$PYTHON_BIN_DIR/visfile"

echo "✅ VisFile CLI installed successfully!"
echo ""
echo "You can now use: visfile --help"
echo "Example: visfile . output.png"