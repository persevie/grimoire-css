#!/usr/bin/env python3
"""
Report Generator for CSS Framework Performance Benchmark
======================================================
Formats and displays benchmark results with meaningful interpretations.
Highlights key metrics for assessing the performance of CSS frameworks.
"""
import time
from datetime import datetime


def format_bytes(bytes_value, precision=2):
    """Format bytes value to human-readable format."""
    if bytes_value < 1024:
        return f"{bytes_value} bytes"
    elif bytes_value < 1024**2:
        return f"{bytes_value/1024:.{precision}f} KB"
    elif bytes_value < 1024**3:
        return f"{bytes_value/(1024**2):.{precision}f} MB"
    else:
        return f"{bytes_value/(1024**3):.{precision}f} GB"


def format_time(seconds):
    """Format time in seconds to appropriate units."""
    if seconds < 0.001:
        return f"{seconds*1000000:.2f} µs"
    elif seconds < 1:
        return f"{seconds*1000:.2f} ms"
    else:
        return f"{seconds:.2f} s"


def generate_report(results):
    """Generate a formatted report from benchmark results."""
    if not results or "metrics" not in results or not results["metrics"]:
        return "No valid benchmark results to report."

    metrics = results["metrics"]
    system_info = results.get("system_info", {})
    framework_name = results.get("framework", "CSS Framework")

    # Format the report
    lines = []
    lines.append("=" * 80)
    lines.append(f"{framework_name.upper()} PERFORMANCE BENCHMARK REPORT")
    lines.append("=" * 80)

    # Add timestamp
    timestamp = system_info.get("timestamp_human",
                                datetime.fromtimestamp(time.time()).strftime("%Y-%m-%d %H:%M:%S"))
    lines.append(f"Generated: {timestamp}")

    # Add system info if available
    if system_info:
        lines.append("\nSYSTEM INFORMATION")
        lines.append("-" * 80)
        lines.append(
            f"OS:      {system_info.get('os', {}).get('name', 'Unknown')} {system_info.get('os', {}).get('release', '')}")
        lines.append(
            f"CPU:     {system_info.get('cpu', {}).get('name', 'Unknown')}")
        lines.append(
            f"Cores:   {system_info.get('cpu', {}).get('cores_physical', 'Unknown')} physical, {system_info.get('cpu', {}).get('cores_logical', 'Unknown')} logical")
        lines.append(
            f"Memory:  {system_info.get('memory', {}).get('total_gb', 'Unknown')} GB")

    # Input summary
    lines.append("\nINPUT SUMMARY")
    lines.append("-" * 80)
    lines.append(
        f"Unique Utility Classes: {metrics['input']['unique_class_count']}")
    lines.append(
        f"Total Input Size: {format_bytes(metrics['input']['total_input_size_bytes'])}")
    if 'file_count' in metrics['input']:
        lines.append(f"Input HTML Files: {metrics['input']['file_count']}")

    # Performance metrics
    lines.append("\nPERFORMANCE METRICS")
    lines.append("-" * 80)
    lines.append(
        f"Build Time: {format_time(metrics['throughput']['build_time_seconds'])}")
    lines.append(
        f"Processing Speed: {metrics['throughput']['classes_per_second']:.2f} classes/second")

    # Memory metrics
    lines.append("\nMEMORY USAGE")
    lines.append("-" * 80)
    lines.append(
        f"Peak Memory: {metrics['process']['memory']['peak_mb']:.2f} MB")
    lines.append(
        f"Average Memory: {metrics['process']['memory']['avg_mb']:.2f} MB")
    if "memory_efficiency" in metrics["throughput"]:
        lines.append(
            f"Memory Efficiency: {metrics['throughput']['memory_efficiency']:.2f} classes/MB")
    if "std_dev_mb" in metrics["process"]["memory"]:
        lines.append(
            f"Memory Stability (Std Dev): {metrics['process']['memory']['std_dev_mb']:.2f} MB")
    lines.append(
        f"Memory per Class: {metrics['process']['memory']['peak_bytes'] / max(1, metrics['input']['unique_class_count']):.2f} bytes/class")

    # CPU metrics
    lines.append("\nCPU USAGE")
    lines.append("-" * 80)
    lines.append(
        f"User CPU Time: {format_time(metrics['process']['cpu']['user_time'])}")
    lines.append(
        f"System CPU Time: {format_time(metrics['process']['cpu']['system_time'])}")
    lines.append(
        f"Total CPU Time: {format_time(metrics['process']['cpu']['total_time'])}")

    # I/O and Output metrics
    lines.append("\nI/O & OUTPUT METRICS")
    lines.append("-" * 80)
    lines.append(
        f"Total Read: {format_bytes(metrics['process']['io']['read_bytes'])}")
    lines.append(
        f"Total Written: {format_bytes(metrics['process']['io']['write_bytes'])}")
    lines.append(f"Output File Count: {metrics['output']['file_count']}")
    lines.append(
        f"Output Size: {format_bytes(metrics['output']['total_size_bytes'])}")

    return "\n".join(lines)


def generate_comparison_report(results):
    """Generate a report comparing different CSS frameworks."""
    if not results:
        return "No valid benchmark results to compare."

    framework_results = {}
    system_info = results.get("system_info", {})

    # Extract framework-specific results
    for framework in ["grimoire", "tailwind"]:
        if framework in results:
            framework_results[framework] = results[framework]

    if not framework_results or len(framework_results) < 2:
        return "Insufficient data for framework comparison."

    # Format the comparison report
    lines = []
    lines.append("=" * 80)
    lines.append("CSS FRAMEWORKS PERFORMANCE COMPARISON")
    lines.append("=" * 80)

    # Add timestamp
    timestamp = system_info.get("timestamp_human",
                                datetime.fromtimestamp(time.time()).strftime("%Y-%m-%d %H:%M:%S"))
    lines.append(f"Generated: {timestamp}")

    # Add system info
    if system_info:
        lines.append("\nSYSTEM INFORMATION")
        lines.append("-" * 80)
        lines.append(
            f"OS:      {system_info.get('os', {}).get('name', 'Unknown')} {system_info.get('os', {}).get('release', '')}")
        lines.append(
            f"CPU:     {system_info.get('cpu', {}).get('name', 'Unknown')}")
        lines.append(
            f"Cores:   {system_info.get('cpu', {}).get('cores_physical', 'Unknown')} physical, {system_info.get('cpu', {}).get('cores_logical', 'Unknown')} logical")
        lines.append(
            f"Memory:  {system_info.get('memory', {}).get('total_gb', 'Unknown')} GB")

    # Performance comparison
    lines.append("\nPERFORMANCE COMPARISON")
    lines.append("-" * 80)

    # Build comparison table
    headers = ["Metric", "Grimoire CSS",
               "Tailwind CSS", "Difference", "Ratio (G/T)"]
    rows = []

    # Get metrics for comparison
    metrics = {
        "Build Time": (lambda m: m["throughput"]["build_time_seconds"], format_time, "lower is better"),
        "Classes/sec": (lambda m: m["throughput"]["classes_per_second"], lambda v: f"{v:.2f}", "higher is better"),
        "Peak Memory": (lambda m: m["process"]["memory"]["peak_mb"], lambda v: f"{v:.2f} MB", "lower is better"),
        "Memory Efficiency": (lambda m: m["throughput"]["memory_efficiency"], lambda v: f"{v:.2f} classes/MB", "higher is better"),
        "Output Size": (lambda m: m["output"]["total_size_bytes"], format_bytes, "lower is better"),
    }

    # Add each metric to the comparison table
    for metric_name, (extractor, formatter, note) in metrics.items():
        g_value = extractor(
            framework_results["grimoire"]) if "grimoire" in framework_results else None
        t_value = extractor(
            framework_results["tailwind"]) if "tailwind" in framework_results else None

        if g_value is not None and t_value is not None and t_value != 0:
            if metric_name == "Build Time" or metric_name == "Peak Memory" or metric_name == "Output Size":
                # Lower is better
                difference = t_value - g_value
                ratio = t_value / g_value if g_value != 0 else float("inf")

                diff_text = f"{formatter(abs(difference))}"

                ratio_text = f"{ratio:.2f}x"

                rows.append([
                    metric_name,
                    formatter(g_value),
                    formatter(t_value),
                    diff_text,
                    ratio_text if metric_name == "Build Time" else f"{ratio:.2f}x"
                ])
            else:
                # Higher is better
                difference = g_value - t_value
                ratio = g_value / t_value if t_value != 0 else float("inf")

                diff_text = f"{formatter(abs(difference))}"

                rows.append([
                    metric_name,
                    formatter(g_value),
                    formatter(t_value),
                    diff_text,
                    f"{ratio:.2f}x"
                ])

    # Format the table
    column_widths = [max(len(row[i]) for row in [headers] + rows)
                     for i in range(len(headers))]

    # Add header
    lines.append(" | ".join(headers[i].ljust(
        column_widths[i]) for i in range(len(headers))))
    lines.append("-" * (sum(column_widths) + 3 * len(column_widths)))

    # Add rows
    for row in rows:
        lines.append(" | ".join(row[i].ljust(column_widths[i])
                     for i in range(len(row))))

    # Add notes
    lines.append("\nNotes:")
    for metric_name, (_, _, note) in metrics.items():
        lines.append(f"- {metric_name}: {note}")

    return "\n".join(lines)


if __name__ == "__main__":
    # Example usage
    sample_results = {
        "system_info": {
            "os": {"name": "Sample OS", "release": "1.0"},
            "cpu": {"name": "Sample CPU", "cores_physical": 4, "cores_logical": 8},
            "memory": {"total_gb": 16},
            "timestamp_human": "2023-01-01 12:00:00"
        },
        "metrics": {
            "input": {"unique_class_count": 1000, "total_input_size_bytes": 1024 * 1024},
            "output": {"file_count": 10, "total_size_bytes": 51200, "avg_size_bytes": 5120},
            "process": {
                "memory": {"peak_bytes": 104857600, "peak_mb": 100, "avg_mb": 80, "std_dev_mb": 10},
                "cpu": {"user_time": 2.5, "system_time": 0.5, "total_time": 3.0},
                "io": {"read_bytes": 1048576, "write_bytes": 51200}
            },
            "throughput": {
                "build_time_seconds": 3.5,
                "classes_per_second": 285.7,
                "memory_efficiency": 10.0
            }
        }
    }
    print(generate_report(sample_results))
