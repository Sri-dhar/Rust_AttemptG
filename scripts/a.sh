#!/bin/bash

# Directory to start from
BASE_DIR=$(realpath ".")

# Check if the directory exists
if [ ! -d "$BASE_DIR" ]; then
  echo "Directory $BASE_DIR does not exist."
  exit 1
fi

# Iterate over each item in the base directory
for dir in "$BASE_DIR"/*/; do
  # Check if the item is a directory
  if [ -d "$dir" ]; then
    echo "Entering directory: $dir"
    cd "$dir" || continue

    # Check if the directory contains a Cargo.toml file
    if [ -f "Cargo.toml" ]; then
      echo "Running cargo clean in $dir"
      cargo clean
    else
      echo "No Cargo.toml found in $dir, skipping..."
    fi

    # Return to the base directory
    cd "$BASE_DIR" || exit
  fi
done

echo "Finished cleaning all cargo projects."

