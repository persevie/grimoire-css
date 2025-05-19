#!/usr/bin/env python3
"""
Benchmark Results Formatter
==========================

Converts raw benchmark results into human-readable, chart-friendly format.
Generates visualization-ready data from benchmark comparison results.
"""
import json
from pathlib import Path


def format_time(seconds):
    """Format time value to human-readable string."""
    if seconds < 0.001:
        return f"{seconds*1000000:.2f}μs"
    elif seconds < 1:
        return f"{seconds*1000:.2f}ms"
    else:
        return f"{seconds:.2f}s"


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


def format_memory(mb_value):
    """Format memory in MB to appropriate units."""
    if mb_value < 1024:
        return f"{round(mb_value, 2)} MB"
    else:
        return f"{round(mb_value/1024, 2)} GB"


def calculate_ratio(value1, value2, higher_is_better=False):
    """Calculate ratio between two values and format as 'Nx better/more/less'."""
    if value1 == 0 or value2 == 0:
        return "∞"

    if higher_is_better:
        ratio = value1 / value2
        return f"{ratio:.1f}x higher" if ratio > 1 else f"{1/ratio:.1f}x lower"
    else:
        ratio = value2 / value1
        return f"{ratio:.1f}x less" if ratio > 1 else f"{1/ratio:.1f}x more"


def calculate_chart_heights(g_value, t_value, lower_is_better=True, max_height=85):
    """Calculate chart heights for visualization based on values."""
    if g_value == 0 and t_value == 0:
        return 0, 0

    # Determine which value is larger
    if lower_is_better:
        larger_value = max(g_value, t_value)
        g_height = (g_value / larger_value) * max_height
        t_height = (t_value / larger_value) * max_height
    else:
        larger_value = max(g_value, t_value)
        g_height = (g_value / larger_value) * max_height
        t_height = (t_value / larger_value) * max_height

    # Ensure minimum height for visibility
    g_height = max(g_height, 2) if g_value > 0 else 0
    t_height = max(t_height, 2) if t_value > 0 else 0

    return g_height, t_height


def generate_pretty_results(input_file):
    """Convert raw benchmark results to pretty, chart-friendly format."""
    # Load raw benchmark results
    with open(input_file, 'r') as f:
        raw_results = json.load(f)

    if "grimoire" not in raw_results or "tailwind" not in raw_results:
        print(f"Error: {input_file} doesn't contain both Grimoire and Tailwind results")
        return None

    grimoire = raw_results["grimoire"]
    tailwind = raw_results["tailwind"]

    # Generate chart data
    charts = []

    # 1. Build Time Chart
    g_build_time = grimoire["throughput"]["build_time_seconds"]
    t_build_time = tailwind["throughput"]["build_time_seconds"]
    g_height, t_height = calculate_chart_heights(g_build_time, t_build_time)
    ratio = t_build_time / g_build_time if g_build_time > 0 else float('inf')

    charts.append({
        "title": "Grimoire CSS vs Tailwind CSS - Build Time",
        "chartTitle": "Build Time",
        "chartSubtitle": "Total time taken to compile CSS (lower is better)",
        "chartId": "chart_time",
        "highlightText": f"{ratio:.1f}x faster",
        "grimoireHeight": g_height,
        "tailwindHeight": t_height,
        "grimoireValue": format_time(g_build_time),
        "tailwindValue": format_time(t_build_time),
        "grimoireRawValue": round(g_build_time, 2),
        "tailwindRawValue": round(t_build_time, 2)
    })

    # 2. Peak Memory Usage Chart
    g_peak_memory = grimoire["process"]["memory"]["peak_mb"]
    t_peak_memory = tailwind["process"]["memory"]["peak_mb"]
    g_height, t_height = calculate_chart_heights(g_peak_memory, t_peak_memory)
    ratio = t_peak_memory / g_peak_memory if g_peak_memory > 0 else float('inf')

    charts.append({
        "title": "Grimoire CSS vs Tailwind CSS - Peak Memory Usage",
        "chartTitle": "Peak Memory Usage",
        "chartSubtitle": "Maximum memory consumed during compilation (lower is better)",
        "chartId": "chart_peak_memory",
        "highlightText": f"{ratio:.1f}x less",
        "grimoireHeight": g_height,
        "tailwindHeight": t_height,
        "grimoireValue": format_memory(g_peak_memory),
        "tailwindValue": format_memory(t_peak_memory),
        "grimoireRawValue": round(g_peak_memory, 2),
        "tailwindRawValue": round(t_peak_memory, 2)
    })

    # 3. Average Memory Usage Chart
    g_avg_memory = grimoire["process"]["memory"]["avg_mb"]
    t_avg_memory = tailwind["process"]["memory"]["avg_mb"]
    g_height, t_height = calculate_chart_heights(g_avg_memory, t_avg_memory)
    ratio = t_avg_memory / g_avg_memory if g_avg_memory > 0 else float('inf')

    charts.append({
        "title": "Grimoire CSS vs Tailwind CSS - Average Memory Usage",
        "chartTitle": "Average Memory Usage",
        "chartSubtitle": "Average memory consumed during compilation (lower is better)",
        "chartId": "chart_avg_memory",
        "highlightText": f"{ratio:.1f}x less",
        "grimoireHeight": g_height,
        "tailwindHeight": t_height,
        "grimoireValue": format_memory(g_avg_memory),
        "tailwindValue": format_memory(t_avg_memory),
        "grimoireRawValue": round(g_avg_memory, 2),
        "tailwindRawValue": round(t_avg_memory, 2)
    })

    # 4. CPU Usage (User Time) Chart
    g_user_time = grimoire["process"]["cpu"]["user_time"]
    t_user_time = tailwind["process"]["cpu"]["user_time"]
    g_height, t_height = calculate_chart_heights(g_user_time, t_user_time)
    ratio = t_user_time / g_user_time if g_user_time > 0 else float('inf')

    charts.append({
        "title": "Grimoire CSS vs Tailwind CSS - CPU Usage (User Time)",
        "chartTitle": "CPU Usage (User Time)",
        "chartSubtitle": "CPU time spent in user mode during compilation (lower is better)",
        "chartId": "chart_cpu_user",
        "highlightText": f"{ratio:.1f}x less",
        "grimoireHeight": g_height,
        "tailwindHeight": t_height,
        "grimoireValue": format_time(g_user_time),
        "tailwindValue": format_time(t_user_time),
        "grimoireRawValue": round(g_user_time, 2),
        "tailwindRawValue": round(t_user_time, 2)
    })

    # 5. CPU Usage (System Time) Chart
    g_system_time = grimoire["process"]["cpu"]["system_time"]
    t_system_time = tailwind["process"]["cpu"]["system_time"]
    g_height, t_height = calculate_chart_heights(g_system_time, t_system_time)
    ratio = t_system_time / g_system_time if g_system_time > 0 else float('inf')

    charts.append({
        "title": "Grimoire CSS vs Tailwind CSS - CPU Usage (System Time)",
        "chartTitle": "CPU Usage (System Time)",
        "chartSubtitle": "CPU time spent in system mode during compilation (lower is better)",
        "chartId": "chart_cpu_system",
        "highlightText": f"{ratio:.1f}x less",
        "grimoireHeight": g_height,
        "tailwindHeight": t_height,
        "grimoireValue": format_time(g_system_time),
        "tailwindValue": format_time(t_system_time),
        "grimoireRawValue": round(g_system_time, 2),
        "tailwindRawValue": round(t_system_time, 2)
    })

    # 6. Output Size Chart
    g_output_size = grimoire["output"]["total_size_kb"]
    t_output_size = tailwind["output"]["total_size_kb"]
    g_height, t_height = calculate_chart_heights(g_output_size, t_output_size)
    ratio = t_output_size / g_output_size if g_output_size > 0 else float('inf')

    charts.append({
        "title": "Grimoire CSS vs Tailwind CSS - Output Size",
        "chartTitle": "Output Size",
        "chartSubtitle": "Size of the generated CSS file (lower is better)",
        "chartId": "chart_output",
        "highlightText": f"{ratio:.1f}x less",
        "grimoireHeight": g_height,
        "tailwindHeight": t_height,
        "grimoireValue": format_bytes(grimoire["output"]["total_size_bytes"]),
        "tailwindValue": format_bytes(tailwind["output"]["total_size_bytes"]),
        "grimoireRawValue": round(g_output_size, 2),
        "tailwindRawValue": round(t_output_size, 2)
    })

    # 7. Classes per Second Chart
    g_classes_per_second = grimoire["throughput"]["classes_per_second"]
    t_classes_per_second = tailwind["throughput"]["classes_per_second"]
    g_height, t_height = calculate_chart_heights(g_classes_per_second, t_classes_per_second, lower_is_better=False)
    ratio = g_classes_per_second / t_classes_per_second if t_classes_per_second > 0 else float('inf')

    charts.append({
        "title": "Grimoire CSS vs Tailwind CSS - Processing Speed",
        "chartTitle": "Processing Speed",
        "chartSubtitle": "Number of utility classes processed per second (higher is better)",
        "chartId": "chart_classes_per_second",
        "highlightText": f"{ratio:.1f}x faster",
        "grimoireHeight": g_height,
        "tailwindHeight": t_height,
        "grimoireValue": f"{g_classes_per_second:.2f} classes/s",
        "tailwindValue": f"{t_classes_per_second:.2f} classes/s",
        "grimoireRawValue": round(g_classes_per_second, 2),
        "tailwindRawValue": round(t_classes_per_second, 2)
    })

    # 8. Memory Efficiency Chart
    g_memory_efficiency = grimoire["throughput"]["memory_efficiency"]
    t_memory_efficiency = tailwind["throughput"]["memory_efficiency"]
    g_height, t_height = calculate_chart_heights(g_memory_efficiency, t_memory_efficiency, lower_is_better=False)
    ratio = g_memory_efficiency / t_memory_efficiency if t_memory_efficiency > 0 else float('inf')

    charts.append({
        "title": "Grimoire CSS vs Tailwind CSS - Memory Efficiency",
        "chartTitle": "Memory Efficiency",
        "chartSubtitle": "Number of utility classes processed per MB of memory (higher is better)",
        "chartId": "chart_memory_efficiency",
        "highlightText": f"{ratio:.1f}x more efficient",
        "grimoireHeight": g_height,
        "tailwindHeight": t_height,
        "grimoireValue": f"{g_memory_efficiency:.2f} classes/MB",
        "tailwindValue": f"{t_memory_efficiency:.2f} classes/MB",
        "grimoireRawValue": round(g_memory_efficiency, 2),
        "tailwindRawValue": round(t_memory_efficiency, 2)
    })

    # Compile pretty results
    pretty_results = {
        "charts": charts,
        # Add metadata from the original results
        "metadata": {
            "timestamp": raw_results["system_info"]["timestamp"],
            "timestamp_human": raw_results["system_info"]["timestamp_human"],
            "system": {
                "os": f"{raw_results['system_info']['os']['name']} {raw_results['system_info']['os']['release']}",
                "cpu": raw_results["system_info"]["cpu"]["name"],
                "cores": f"{raw_results['system_info']['cpu']['cores_physical']} physical, {raw_results['system_info']['cpu']['cores_logical']} logical",
                "memory": f"{raw_results['system_info']['memory']['total_gb']} GB"
            }
        }
    }

    return pretty_results


def format_benchmark_results(input_file=None):
    """Format benchmark results and save them as a pretty JSON file."""
    if input_file:
        # Process a single file
        input_path = Path(input_file)
        if not input_path.exists():
            print(f"Error: Input file {input_path} does not exist")
            return False

        pretty_results = generate_pretty_results(input_path)
        if pretty_results:
            output_path = input_path.with_stem(input_path.stem + "_pretty")
            with open(output_path, 'w') as f:
                json.dump(pretty_results, f, indent=2)
            print(f"Pretty results saved to {output_path}")
            return True
        return False
    else:
        # Process all result files in the results directory
        results_dir = Path("results")
        if not results_dir.exists():
            print(f"Error: Results directory {results_dir} does not exist")
            return False

        success_count = 0
        for json_file in results_dir.glob("result_*.json"):
            # Skip files that already have _pretty in the name
            if "_pretty" in json_file.stem:
                continue

            pretty_results = generate_pretty_results(json_file)
            if pretty_results:
                output_path = json_file.with_stem(json_file.stem + "_pretty")
                with open(output_path, 'w') as f:
                    json.dump(pretty_results, f, indent=2)
                print(f"Pretty results saved to {output_path}")
                success_count += 1

        if success_count > 0:
            print(f"Successfully processed {success_count} benchmark result files")
            return True
        else:
            print("No benchmark result files found to process")
            return False


if __name__ == "__main__":
    # Test with a sample file
    format_benchmark_results()
