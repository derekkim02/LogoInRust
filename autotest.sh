#!/bin/bash

# Directory containing the correct images
CORRECT_DIR="./correct_images"

# Directory containing the user-generated images
USER_DIR="./user_images"

bash generate_pics.sh

# Iterate over the files in the correct images directory
find "$CORRECT_DIR" -type f -name "*.svg" | sort | while read correct_file; do
    # Extract just the filename, no path
    filename=$(basename "$correct_file")

    # Construct the corresponding filename in the user directory
    # This replaces "_actual" with "_mine" in the filename
    user_filename="${filename/_actual/_mine}"

    # Construct the full path to the user file
    user_file="${USER_DIR}/${user_filename}"

    # Compare the files
    if [ -f "$user_file" ]; then
        echo "Comparing $filename with $user_filename"
        diff -s "$correct_file" "$user_file"
    else
        echo "User file $user_filename not found."
    fi

    echo "======================================================================"
done