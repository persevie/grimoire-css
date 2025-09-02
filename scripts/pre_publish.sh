#!/usr/bin/env bash

# Pre-publish helper: generate animations list and compose README.md from content blocks.

set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]:-$0}")" && pwd)
REPO_ROOT=$(cd "$SCRIPT_DIR/.." && pwd)
CONTENT_DIR="$REPO_ROOT/content"
ANIM_CSS_DIR="$REPO_ROOT/src/core/animations/css"

echo "[pre_publish] repo_root=$REPO_ROOT"

# 1) Create content/animations.txt from src/core/animations/css using ls (remove .css)
#    Keep it sorted and handle the case when no files exist.
animations_txt="$CONTENT_DIR/animations.txt"
{
    shopt -s nullglob
    css_files=("$ANIM_CSS_DIR"/*.css)
    if ((${#css_files[@]})); then
        for f in "${css_files[@]}"; do
            bname=$(basename "$f")
            printf '%s\n' "${bname%.css}"
        done | LC_ALL=C sort
    fi
} > "$animations_txt"
echo "[pre_publish] generated $(wc -l < "$animations_txt") entries in content/animations.txt"

# 2) Insert animations from content/animations.txt into content/animations.md
#    between the markers, adding "- " at the beginning of each line.
animations_md="$CONTENT_DIR/animations.md"
tmp_file="$animations_md.tmp"

awk -v list_file="$animations_txt" '
    BEGIN {
        i = 0
        while ((getline l < list_file) > 0) { list[i++] = l }
        close(list_file)
    }
    {
        if (state == 1) {
            # Skip lines until END marker, then print the new block and the END marker.
            if ($0 ~ /<!--[[:space:]]*END[[:space:]]+ANIMATIONS[[:space:]]+LIST[[:space:]]*-->/) {
                for (j=0; j<i; j++) print "- " list[j]
                print
                state = 0
            }
            next
        }
        print
        if ($0 ~ /<!--[[:space:]]*START[[:space:]]+ANIMATIONS[[:space:]]+LIST[[:space:]]*-->/) {
            state = 1
        }
    }
' "$animations_md" > "$tmp_file" && mv "$tmp_file" "$animations_md"
echo "[pre_publish] updated content/animations.md with bulleted list"

# 3) Build README.md by inserting content files into blocks in content/about.md
about_md="$CONTENT_DIR/about.md"
readme_md="$REPO_ROOT/README.md"

awk \
    -v hero="$CONTENT_DIR/hero_image.md" \
    -v note="$CONTENT_DIR/note.md" \
    -v badges="$CONTENT_DIR/badges.md" \
    -v animsec="$CONTENT_DIR/animations.md" \
    -v desk="$CONTENT_DIR/desk.md" \
    -v circle="$CONTENT_DIR/circle.md" \
    -v release="$CONTENT_DIR/release_info.md" \
    -v slogan="$CONTENT_DIR/slogan.md" '
    function print_file(path,    line, opened) {
        # Print file only if it exists and is readable.
        if ((getline line < path) <= 0) { close(path); return }
        print line
        while ((getline line < path) > 0) print line
        close(path)
    }
    {
        if (skip) {
            # Pass through until any END marker for safety (sections are not nested).
            if ($0 ~ /<!--[[:space:]]*END[[:space:]]+[A-Z][A-Z ]*[[:space:]]*-->/) {
                print
                skip = 0
            }
            next
        }

        # Section handlers: print START, inject file content, then skip until END.
        if ($0 ~ /<!--[[:space:]]*START[[:space:]]+HERO[[:space:]]+IMAGE[[:space:]]*-->/) { print; print_file(hero); skip=1; next }
        if ($0 ~ /<!--[[:space:]]*START[[:space:]]+NOTE[[:space:]]*-->/) { print; print_file(note); skip=1; next }
        if ($0 ~ /<!--[[:space:]]*START[[:space:]]+BADGES[[:space:]]*-->/) { print; print_file(badges); skip=1; next }
        if ($0 ~ /<!--[[:space:]]*START[[:space:]]+ANIMATIONS[[:space:]]+SECTION[[:space:]]*-->/) { print; print_file(animsec); skip=1; next }
        if ($0 ~ /<!--[[:space:]]*START[[:space:]]+DESK[[:space:]]*-->/) { print; print_file(desk); skip=1; next }
        if ($0 ~ /<!--[[:space:]]*START[[:space:]]+CIRCLE[[:space:]]*-->/) { print; print_file(circle); skip=1; next }
        if ($0 ~ /<!--[[:space:]]*START[[:space:]]+RELEASE[[:space:]]+INFO[[:space:]]*-->/) { print; print_file(release); skip=1; next }
        if ($0 ~ /<!--[[:space:]]*START[[:space:]]+SLOGAN[[:space:]]*-->/) { print; print_file(slogan); skip=1; next }

        print
    }
' "$about_md" > "$readme_md"

echo "[pre_publish] generated README.md from content/about.md"

# 4) Update Table of Contents in README.md using doctoc (if available)
if command -v doctoc >/dev/null 2>&1; then
    echo "[pre_publish] running doctoc on README.md"
    doctoc --github "$readme_md" || echo "[pre_publish] doctoc encountered an error but continuing"
elif command -v npx >/dev/null 2>&1; then
    echo "[pre_publish] running npx doctoc on README.md"
    npx -y doctoc --github "$readme_md" || echo "[pre_publish] npx doctoc encountered an error but continuing"
else
    echo "[pre_publish] doctoc not found. Skipping TOC update. Install with: npm i -g doctoc" >&2
fi

echo "[pre_publish] done."
