#!/usr/bin/env python3
"""
CSS Framework Performance Benchmark
=================================

A high-precision, cross-platform benchmark for measuring CSS framework performance metrics.
This benchmark provides accurate measurements for build time, memory usage, CPU usage,
and output size with detailed reporting capabilities for both Grimoire CSS and Tailwind CSS.

Usage:
    python main.py [--prepare] [--run-only] [--output-format=json|text|both] [--output-file=filename]
                  [--framework=grimoire|tailwind|both] [--format-pretty]

Options:
    --prepare       Only prepare benchmark projects without running benchmarks
    --run-only      Only run benchmarks (assumes projects are already prepared)
    --output-format Format for results output (default: both)
    --output-file   File to save results to (default: auto-generated with timestamp)
    --framework     Which framework to benchmark (default: both)
    --save-text     Also save text report to a file (default: False)
    --format-pretty Generate a pretty, visualization-ready version of results (default: True)
    --format-only   Only convert existing raw results to pretty format without running benchmarks
"""

import argparse
import json
import platform
import psutil
import time
import datetime
from pathlib import Path

from core.project_creator import create_benchmark_projects
from core.metrics_collector import GrimoireMetricsCollector, TailwindMetricsCollector
from core.report_generator import generate_report, generate_comparison_report
from core.benchmark_formatter import format_benchmark_results


def parse_args():
    """Parse command line arguments."""
    timestamp = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
    default_output = f"results/result_{timestamp}.json"

    parser = argparse.ArgumentParser(
        description="CSS Framework Performance Benchmark")
    parser.add_argument('--prepare', action='store_true',
                        help='Only prepare benchmark projects')
    parser.add_argument('--run-only', action='store_true',
                        help='Only run benchmarks (assumes projects exist)')
    parser.add_argument('--output-format', choices=['json', 'text', 'both'], default='both',
                        help='Format for results output (default: both)')
    parser.add_argument('--output-file', default=default_output,
                        help=f'File to save results to (default: {default_output})')
    parser.add_argument('--framework', choices=['grimoire', 'tailwind', 'both'], default='both',
                        help='Which framework to benchmark (default: both)')
    parser.add_argument('--save-text', action='store_true',
                        help='Also save text report to a file (default: False)')
    parser.add_argument('--format-pretty', action='store_true', default=True,
                        help='Generate a pretty, visualization-ready version of results (default: True)')
    parser.add_argument('--format-only', action='store_true',
                        help='Only convert existing raw results to pretty format without running benchmarks')
    parser.add_argument('--input-file',
                        help='Input file for formatting (used with --format-only)')
    return parser.parse_args()


def collect_system_info():
    """Collect information about the system for benchmark context."""
    return {
        "os": {
            "name": platform.system(),
            "version": platform.version(),
            "release": platform.release()
        },
        "cpu": {
            "name": platform.processor() or platform.machine(),
            "cores_logical": psutil.cpu_count(logical=True),
            "cores_physical": psutil.cpu_count(logical=False)
        },
        "memory": {
            "total_gb": round(psutil.virtual_memory().total / (1024**3), 1)
        },
        "python_version": platform.python_version(),
        "timestamp": time.time(),
        "timestamp_human": time.strftime("%Y-%m-%d %H:%M:%S")
    }


def handle_format_only(args):
    """Handle the format-only mode."""
    if args.input_file:
        success = format_benchmark_results(args.input_file)
    else:
        success = format_benchmark_results()

    if success:
        print("Successfully formatted benchmark results")
    else:
        print("Failed to format benchmark results")
    return True


def prepare_benchmark_projects():
    """Prepare the benchmark projects."""
    print("Preparing benchmark projects...")
    create_benchmark_projects()
    print("Benchmark projects prepared. Run without --prepare to execute benchmarks.")
    return True


def ensure_input_directory():
    """Ensure the input directory exists with projects."""
    input_dir = Path("input")
    if not input_dir.exists() or not any(input_dir.glob("project*")):
        print("Input directory is missing or empty. Creating benchmark projects...")
        create_benchmark_projects()


def run_framework_benchmark(framework, system_info):
    """Run benchmark for the specified framework."""
    framework_results = {}

    if framework in ['grimoire', 'both']:
        print("\n=== Starting Grimoire CSS Performance Benchmark ===")
        grimoire_collector = GrimoireMetricsCollector()
        grimoire_results = grimoire_collector.run_benchmark()

        if not grimoire_results:
            print("Grimoire CSS benchmark failed to complete.")
        else:
            framework_results["grimoire"] = grimoire_results
            print("Grimoire CSS benchmark completed.")

    if framework in ['tailwind', 'both']:
        print("\n=== Starting Tailwind CSS Performance Benchmark ===")
        tailwind_collector = TailwindMetricsCollector()
        tailwind_results = tailwind_collector.run_benchmark()

        if not tailwind_results:
            print("Tailwind CSS benchmark failed to complete.")
        else:
            framework_results["tailwind"] = tailwind_results
            print("Tailwind CSS benchmark completed.")

    return framework_results


def generate_and_save_reports(framework_results, system_info, args):
    """Generate and save the benchmark reports."""
    # Ensure the results directory exists
    results_dir = Path("results")
    results_dir.mkdir(exist_ok=True)

    # Generate output paths
    output_path = Path(args.output_file)

    # Combine results with system info
    full_results = {
        "system_info": system_info,
        **framework_results
    }

    # Generate framework reports and comparison report for text output
    text_reports = []
    for framework, metrics in framework_results.items():
        framework_report = generate_report({
            "system_info": system_info,
            "metrics": metrics,
            "framework": framework.capitalize()
        })
        text_reports.append(framework_report)

    # Generate comparison report if both frameworks were benchmarked
    if len(framework_results) > 1:
        comparison_report = generate_comparison_report(full_results)
        text_reports.append(comparison_report)

    # Combined text report
    full_text_report = "\n\n" + "="*80 + "\n\n".join(text_reports)

    # Save and display results based on format preference
    if args.output_format in ['json', 'both']:
        # Make sure the parent directory exists
        output_path.parent.mkdir(exist_ok=True, parents=True)

        with open(output_path, 'w') as f:
            json.dump(full_results, f, indent=2)
        print(f"\nResults saved to {output_path}")

        # Generate pretty version if requested
        if args.format_pretty and len(framework_results) > 1:
            format_benchmark_results(output_path)

    # Save text report if requested
    if args.save_text:
        text_output_path = output_path.with_suffix('.txt')
        with open(text_output_path, 'w') as f:
            f.write(full_text_report)
        print(f"Text report saved to {text_output_path}")

    # Display reports if text output is requested
    if args.output_format in ['text', 'both']:
        for report in text_reports:
            print(report)
            print("\n" + "="*80 + "\n")

    return full_results


def main():
    """Main entry point for the benchmark."""
    args = parse_args()

    # Handle format-only mode
    if args.format_only:
        return handle_format_only(args)

    # Create benchmark projects only if specifically requested
    if args.prepare:
        return prepare_benchmark_projects()

    # Check if input directory exists for benchmarking
    ensure_input_directory()

    # Skip benchmark if only preparation was requested
    if args.prepare:
        return

    # Collect system information
    system_info = collect_system_info()
    print("\n=== System Information ===")
    print(f"OS: {system_info['os']['name']} {system_info['os']['release']}")
    print(f"CPU: {system_info['cpu']['name']}")
    print(f"Memory: {system_info['memory']['total_gb']} GB")

    # Run benchmarks
    framework_results = run_framework_benchmark(args.framework, system_info)

    # If no benchmarks were successful, exit
    if not framework_results:
        print("All benchmarks failed to complete.")
        return

    # Generate and save reports
    return generate_and_save_reports(framework_results, system_info, args)


if __name__ == "__main__":
    main()
