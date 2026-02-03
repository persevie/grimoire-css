#!/usr/bin/env python3

from __future__ import annotations

import re
import sys
from pathlib import Path


HEADER = """# Grimoire CSS Releases

## Overview

This document combines all release notes in chronological order, providing a comprehensive view of Grimoire CSS's evolution.
"""


FALLBACK_V1_0_0 = """

---

# v1.0.0: Initial Release

The debut release of Grimoire CSS, introducing a powerful CSS engine designed for flexibility and performance.
"""


VERSION_RE = re.compile(r"^v(\d+)\.(\d+)\.(\d+)\.md$")


def parse_version(file_name: str) -> tuple[int, int, int] | None:
    match = VERSION_RE.match(file_name)
    if not match:
        return None
    major, minor, patch = (int(match.group(1)), int(match.group(2)), int(match.group(3)))
    return major, minor, patch


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    releases_dir = repo_root / "releases"
    output_path = repo_root / "RELEASES.md"

    if not releases_dir.exists():
        print(f"error: releases dir not found: {releases_dir}", file=sys.stderr)
        return 1

    release_files: list[tuple[tuple[int, int, int], Path]] = []

    for path in releases_dir.glob("v*.md"):
        version = parse_version(path.name)
        if version is None:
            continue
        release_files.append((version, path))

    # Sort order: true semver descending (newest version first)
    release_files.sort(key=lambda x: (-x[0][0], -x[0][1], -x[0][2], str(x[1])))

    parts: list[str] = [HEADER]

    for _, file_path in release_files:
        parts.append("\n---\n\n")
        content = file_path.read_text(encoding="utf-8")
        parts.append(content)
        if not content.endswith("\n"):
            parts.append("\n")

    full = "".join(parts)

    if "v1.0.0" not in full:
        full += FALLBACK_V1_0_0

    output_path.write_text(full, encoding="utf-8")

    print("RELEASES.md has been updated with full release notes in chronological order.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
