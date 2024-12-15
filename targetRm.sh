#!/bin/bash

# Function to recursively remove 'target' directories
remove_target_dirs() {
    local dir="$1"

    # Find all directories named 'target' and delete them
    find "$dir" -type d -name "target" -exec rm -rf {} +

    echo "All 'target' directories have been removed from $dir"
}

# Check if a directory is provided as an argument
if [ $# -eq 0 ]; then
    echo "Usage: $0 <directory>"
    exit 1
fi

# Call the function with the given directory
remove_target_dirs "$1"

