#!/usr/bin/env python3

from __future__ import annotations

import re
import sys
from pathlib import Path


VERSION_HEADER_RE = re.compile(r"^## \[(v\d+\.\d+\.\d+)\] - (.+)$")
SECTION_RE = re.compile(r"^### (Added|Improved|Fixed|Changed|Deprecated|Removed|Security)\s*$")


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    changelog_path = repo_root / "CHANGELOG.md"
    out_dir = repo_root / "releases" / "changelog"

    if not changelog_path.exists():
        print(f"error: not found: {changelog_path}", file=sys.stderr)
        return 1

    out_dir.mkdir(parents=True, exist_ok=True)

    lines = changelog_path.read_text(encoding="utf-8").splitlines()

    current_version: str | None = None
    current_body: list[str] = []

    def flush() -> None:
        nonlocal current_version, current_body
        if current_version is None:
            return

        # Trim boilerplate and links; keep only section headings + bullets.
        body_lines: list[str] = []
        for line in current_body:
            if line.strip() == "---":
                continue
            if line.strip().startswith("> Full release notes:"):
                continue
            body_lines.append(line.rstrip())

        # Remove leading/trailing blank lines.
        while body_lines and body_lines[0].strip() == "":
            body_lines.pop(0)
        while body_lines and body_lines[-1].strip() == "":
            body_lines.pop()

        # Normalize headings to Keep a Changelog sections.
        normalized: list[str] = []
        for line in body_lines:
            m = SECTION_RE.match(line)
            if m:
                section = m.group(1)
                if section == "Improved":
                    section = "Changed"
                normalized.append(f"### {section}")
                continue
            normalized.append(line)

        out_path = out_dir / f"{current_version}.md"
        out_path.write_text("\n".join(normalized).rstrip("\n") + "\n", encoding="utf-8")

        current_version = None
        current_body = []

    # Parse file.
    for line in lines:
        m = VERSION_HEADER_RE.match(line)
        if m:
            flush()
            current_version = m.group(1)
            current_body = []
            continue

        if current_version is None:
            continue

        # Stop at next version delimiter handled above; otherwise keep.
        current_body.append(line)

    flush()

    print(f"Seeded changelog entries into: {out_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
