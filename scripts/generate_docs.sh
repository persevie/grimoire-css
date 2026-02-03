#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$REPO_ROOT"

python3 scripts/generate_changelog.py
python3 scripts/generate_releases.py

echo "Done: CHANGELOG.md + RELEASES.md regenerated."
