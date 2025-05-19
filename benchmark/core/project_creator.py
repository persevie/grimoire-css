#!/usr/bin/env python3
"""
Project Creator for CSS Framework Performance Benchmark
=====================================================

Creates standardized test projects that contain HTML files with utility classes
for benchmarking CSS framework performance.

This module generates HTML files with both Grimoire CSS spells and Tailwind CSS utility classes,
ensuring a fair and equivalent comparison between the frameworks.
"""

import json
import shutil
from pathlib import Path


def create_benchmark_projects():
    """Create test project files for benchmarking.

    This function creates a set of projects, each with multiple HTML files
    containing utility classes for both Grimoire CSS and Tailwind CSS.
    """
    # First, clean up any previous input directory
    input_dir = Path("input")
    if input_dir.exists():
        shutil.rmtree(input_dir)

    # Also clean up output directories
    grimoire_output = Path("grimoire_css_output")
    tailwind_output = Path("tailwind_css_output")

    for output_dir in [grimoire_output, tailwind_output]:
        if output_dir.exists():
            shutil.rmtree(output_dir)
        output_dir.mkdir(parents=True, exist_ok=True)

    print("Creating benchmark projects...")

    # Number of projects and files per project
    num_projects = 10
    files_per_project = 10000

    for j in range(1, num_projects + 1):
        project_dir = input_dir / f"project{j}"
        project_dir.mkdir(parents=True, exist_ok=True)

        # Create tailwind config for this project
        tailwind_config = {
            "content": ["./file*.html"],
            "theme": {
                "extend": {},
            },
            "plugins": [],
        }

        with open(project_dir / "tailwind.config.js", "w") as f:
            f.write(f"module.exports = {json.dumps(tailwind_config, indent=2)}")

        # Create input.css for this project
        with open(project_dir / "input.css", "w") as f:
            f.write('@import "tailwindcss";')

        # Create HTML files for each project
        print(f"Creating files for project {j}...")

        for i in range(1, files_per_project + 1):
            ji = f"{j}{i}"
            content = f"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Document</title>
</head>
<body>
  <div class="h={ji}px w={ji}px h-[{ji}px] w-[{ji}px] bg-c=red bg-[red]">Red square</div>
  <div class="h={ji}px w={ji}px h-[{ji}px] w-[{ji}px] bg-c=yellow bg-[yellow]">Yellow square</div>
  <div class="h={ji}px w={ji}px h-[{ji}px] w-[{ji}px] bg-c=green bg-[green]">Green square</div>
</body>
</html>"""

            with open(project_dir / f"file{i}.html", "w") as f:
                f.write(content)

            if i % 1000 == 0:
                print(f"Project {j}: Created {i} files")

        print(f"Completed project {j}")

    # Create a grimoire config file in the root directory
    grimoire_config = {
        "outDir": "./grimoire_css_output",
        "include": ["./input/**/*.html"],
        "exclude": [],
        "minify": True,
        "addSourceMap": False
    }

    with open("grimoire.config.json", "w") as f:
        json.dump(grimoire_config, f, indent=2)

    # Make sure Tailwind CSS is installed
    ensure_tailwind_installed()

    print("Benchmark projects created successfully!")
    print(f"Total files created: {num_projects * files_per_project}")


def ensure_tailwind_installed():
    """Ensure Tailwind CSS is installed for benchmarking."""
    import subprocess

    print("Checking Tailwind CSS installation...")

    package_json_path = Path("package.json")

    if not package_json_path.exists():
        print("Creating package.json for Tailwind CSS...")
        package_json = {
            "name": "css-benchmark",
            "version": "1.0.0",
            "description": "CSS Framework Benchmark",
            "devDependencies": {
                "@tailwindcss/cli": "^3.3.0"
            }
        }

        with open(package_json_path, "w") as f:
            json.dump(package_json, f, indent=2)

    # Check if node_modules exists
    if not Path("node_modules").exists() or not Path("node_modules/@tailwindcss").exists():
        print("Installing Tailwind CSS...")
        try:
            subprocess.run(["npm", "install"], check=True)
            print("Tailwind CSS installed successfully.")
        except subprocess.CalledProcessError as e:
            print(f"Error installing Tailwind CSS: {e}")
            print("Please install Tailwind CSS manually with: npm install")
        except FileNotFoundError:
            print("npm command not found. Please install Node.js and npm.")
            print("Then install Tailwind CSS with: npm install")


if __name__ == "__main__":
    create_benchmark_projects()
