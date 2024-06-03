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

    # Check if the directory contains a .git folder
    if [ -d ".git" ]; then
      echo "Removing .git directory in $dir"
      rm -r .git
    else
      echo "No .git directory found in $dir, skipping..."
    fi

    # Return to the base directory
    cd "$BASE_DIR" || exit
  fi
done

echo "Finished removing .git directories."
