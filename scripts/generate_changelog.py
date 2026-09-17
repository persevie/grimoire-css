#!/usr/bin/env python3

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from datetime import date
from pathlib import Path


HEADER = """# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This file is auto-generated from per-version entries in `releases/changelog/`.
Do not edit it manually — edit the corresponding entry and run `bash scripts/generate_docs.sh`.
Release dates are set in the per-version entries. Older entries without an explicit date use their Git tag date.
"""

VERSION_RE = re.compile(r"^v(\d+)\.(\d+)\.(\d+)\.md$")


def parse_version(file_name: str) -> tuple[int, int, int] | None:
    match = VERSION_RE.match(file_name)
    if not match:
        return None
    return int(match.group(1)), int(match.group(2)), int(match.group(3))


def try_git_tag_date(repo_root: Path, tag: str) -> str | None:
    try:
        subprocess.run(
            ["git", "rev-parse", "-q", "--verify", f"refs/tags/{tag}"],
            cwd=repo_root,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=True,
        )
    except Exception:
        return None

    try:
        result = subprocess.run(
            ["git", "log", "-1", "--format=%as", tag],
            cwd=repo_root,
            capture_output=True,
            text=True,
            check=True,
        )
        date = result.stdout.strip()
        return date or None
    except Exception:
        return None


def read_release_date(entry_md: str) -> tuple[str | None, str]:
    first_line, _, body = entry_md.partition("\n")
    if not first_line.startswith("Release date:"):
        return None, entry_md
    value = first_line.removeprefix("Release date:").strip()
    if value != "Unreleased":
        if not re.fullmatch(r"\d{4}-\d{2}-\d{2}", value):
            raise ValueError("Release date must be YYYY-MM-DD or Unreleased")
        date.fromisoformat(value)
    return value, body.lstrip("\n")


def validate_entry_format(version: str, entry_md: str) -> None:
    # Deterministic rule: changelog entries must contain only Keep-a-Changelog sections.
    # Supported: Added, Changed, Deprecated, Removed, Fixed, Security.
    allowed = {"Added", "Changed", "Deprecated", "Removed", "Fixed", "Security"}
    for line in entry_md.splitlines():
        if line.startswith("## "):
            raise ValueError(f"{version}: entry must not contain '##' headings (generator provides version headings)")
        if line.startswith("### "):
            section = line[4:].strip()
            if section not in allowed:
                raise ValueError(
                    f"{version}: invalid section '{section}'. Allowed: {', '.join(sorted(allowed))}"
                )


def normalize_entry(entry_md: str) -> str:
    # Ensure entries end with a newline, and normalize multiple blank lines.
    text = entry_md.replace("\r\n", "\n").rstrip("\n") + "\n"
    return text


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check-release", metavar="VERSION", help="Require an explicit release date and an up-to-date CHANGELOG without writing files")
    args = parser.parse_args(argv)
    repo_root = Path(__file__).resolve().parents[1]
    releases_dir = repo_root / "releases"
    entries_dir = releases_dir / "changelog"
    output_path = repo_root / "CHANGELOG.md"

    if not entries_dir.exists():
        print(f"error: changelog entries dir not found: {entries_dir}", file=sys.stderr)
        return 1

    entry_files: list[tuple[tuple[int, int, int], Path]] = []
    for path in entries_dir.glob("v*.md"):
        version = parse_version(path.name)
        if version is None:
            continue
        entry_files.append((version, path))

    # True semver descending.
    entry_files.sort(key=lambda x: (-x[0][0], -x[0][1], -x[0][2], str(x[1])))

    parts: list[str] = [HEADER, "\n", "## [Unreleased]\n\n", "(no unversioned changes recorded)\n\n"]

    release_checked = False
    for (major, minor, patch), entry_path in entry_files:
        version_str = f"v{major}.{minor}.{patch}"
        explicit_date, entry_md = read_release_date(entry_path.read_text(encoding="utf-8"))
        date_str = explicit_date or try_git_tag_date(repo_root, version_str) or "Unreleased"
        if args.check_release == version_str:
            if explicit_date in (None, "Unreleased"):
                raise ValueError(f"{version_str}: confirm Release date: YYYY-MM-DD before publication")
            release_checked = True
        validate_entry_format(version_str, entry_md)
        entry_md = normalize_entry(entry_md)

        parts.append(f"## [{version_str}] - {date_str}\n\n")
        parts.append(f"> Full release notes: [releases/{version_str}.md](./releases/{version_str}.md)\n\n")
        parts.append(entry_md)
        if not entry_md.endswith("\n\n"):
            parts.append("\n")

    output = "".join(parts).rstrip("\n") + "\n"
    if args.check_release:
        if not release_checked:
            raise ValueError(f"Changelog entry not found: {args.check_release}")
        if not output_path.exists() or output_path.read_text(encoding="utf-8") != output:
            raise ValueError("CHANGELOG.md is out of date; run bash scripts/generate_docs.sh")
        return 0
    output_path.write_text(output, encoding="utf-8")
    print("CHANGELOG.md has been updated from releases/changelog/*.md")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
