# CSS Frameworks Performance Benchmark

This benchmark is designed to compare the performance of CSS frameworks Grimoire CSS and Tailwind CSS. It provides accurate measurement of build time, memory usage, CPU load, and output file size.

## Overview

The benchmark creates a series of standardized test projects, each containing a large number of HTML files with utility classes for both Grimoire CSS and Tailwind CSS. Then each framework is run to process these projects, and various performance metrics are recorded and analyzed.

### Measured Metrics

- **Build Time** — total time required to process all projects
- **Class Processing Speed** — number of processed classes per second
- **Memory Usage** — peak and average memory consumption
- **Memory Efficiency** — number of processed classes per MB of used memory
- **I/O Operations** — volume of data read and written
- **Output File Size** — total size of generated CSS files

## Benchmark Structure

The benchmark is organized as follows:

- `benchmark/main.py` — main script for running the benchmark
- `benchmark/core/` — modules for creating projects and collecting metrics
- `benchmark/input/` — generated test projects
- `benchmark/results/` — benchmark results in JSON and TXT formats

## Installation and Running

### Prerequisites

- Python 3.6 or higher
- Node.js and npm (for Tailwind CSS)
- Compiled Grimoire CSS in `../target/release/grimoire_css`

### Installing Dependencies

```bash
cd benchmark
pip install -r requirements.txt
```

### Running the Benchmark

Preparing test projects without running the benchmark:

```bash
python main.py --prepare
```

Running a full benchmark for both frameworks:

```bash
python main.py
```

Saving a text report to a file:

```bash
python main.py --save-text
```

Testing only one framework:

```bash
python main.py --framework grimoire
python main.py --framework tailwind
```

## Interpreting Results

### JSON Results Format

Results are saved in JSON format with the following structure:

```json
{
  "system_info": { ... },
  "grimoire": {
    "input": { ... },
    "output": { ... },
    "process": { ... },
    "throughput": { ... }
  },
  "tailwind": {
    "input": { ... },
    "output": { ... },
    "process": { ... },
    "throughput": { ... }
  }
}
```

### Key Metrics for Comparison

1. **Build Time**

   - Lower is better
   - Measured in seconds
   - Shows how quickly the framework processes utility classes and generates CSS

2. **Classes/sec**

   - Higher is better
   - Number of unique classes processed per second
   - Reflects the overall speed of the framework

3. **Peak Memory**

   - Lower is better
   - Measured in MB
   - Shows the maximum amount of memory used during operation

4. **Memory Efficiency**

   - Higher is better
   - Measured in "classes per MB"
   - Shows how many classes the framework can process per MB of used memory

5. **Output Size**
   - Lower is better
   - Measured in bytes (displayed in KB/MB)
   - Reflects the efficiency of minification and optimization

### How to Read the Comparison Report

The comparison report (generated when testing both frameworks) includes a **PERFORMANCE COMPARISON** section that contains a table with several key columns:

- **Metric** — name of the metric
- **Grimoire CSS** — value for Grimoire CSS
- **Tailwind CSS** — value for Tailwind CSS
- **Difference** — absolute difference between values and indication of which framework showed better results
- **Ratio (G/T)** — ratio of Grimoire to Tailwind performance

## Features and Limitations

1. **Different Execution Approaches**:

   - Grimoire CSS runs once from the root directory and processes all projects using its configuration
   - Tailwind CSS runs separately for each project from the project directory

2. **Node.js Overhead**:

   - Tailwind CSS runs through Node.js, which adds some overhead not present in Grimoire CSS

3. **Synthetic Nature of Tests**:

   - Test projects are automatically generated and may not reflect all real-world project scenarios
   - Each project contains identical class patterns

4. **System Differences**:
   - Performance can vary significantly depending on the operating system, CPU, and available memory
   - Always run tests under the same conditions to get comparable results

## Working with Results Over Time

All benchmark results are saved in the `benchmark/results/` directory with timestamps in the filenames. This allows:

- Tracking performance changes from version to version
- Comparing results across different machines and systems
- Analyzing the impact of optimizations on performance

For quick visual comparison, it's recommended to use text reports (saved using the `--save-text` option).

## Conclusion

This benchmark provides an objective basis for comparing the performance of CSS frameworks. When interpreting results, it's important to consider the context of framework usage, their capabilities, and implementation details.
