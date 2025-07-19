#!/bin/bash

# Navigate to project root
cd "$(dirname "$0")/.."

# Create temporary file for new RELEASES.md content
temp_file=$(mktemp)

# Add header
cat > "$temp_file" << EOF
# Grimoire CSS Releases

## Overview

This document combines all release notes in chronological order, providing a comprehensive view of Grimoire CSS's evolution.

EOF

# Function to extract version number for sorting
version_sort() {
    # Remove 'v' prefix and sort by version number
    sed 's/^v//' | sort -t. -k1,1nr -k2,2nr -k3,3nr
}

# Find all release files, sort them by version number (newest first)
find ./releases -name "v*.md" | version_sort | while read -r file; do
    # Add the entire content of the release file
    echo -e "\n---\n" >> "$temp_file"
    cat "$file" >> "$temp_file"
done

# Add initial release info if not in release files
if ! grep -q "v1.0.0" "$temp_file"; then
    cat >> "$temp_file" << EOF

---

# v1.0.0: Initial Release

The debut release of Grimoire CSS, introducing a powerful CSS engine designed for flexibility and performance.
EOF
fi

# Replace the old RELEASES.md with new content
mv "$temp_file" RELEASES.md

echo "RELEASES.md has been updated with full release notes in chronological order."
