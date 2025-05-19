#!/usr/bin/env python3
"""
Metrics Collector for CSS Framework Performance Benchmark
=======================================================
Provides high-precision, cross-platform measurement of performance metrics including:
- Build time
- Memory usage (peak and average)
- CPU usage
- Process resources
- I/O operations
- Output size and efficiency
Designed for maximum accuracy and consistency across Windows, macOS, and Linux.
"""
import time
import psutil
import subprocess
import threading
import statistics
from pathlib import Path
import platform
import traceback


class ProcessMonitor:
    """Monitors a process and all its children for resource usage."""

    def __init__(self):
        """Initialize the process monitor."""
        self.is_windows = platform.system() == "Windows"
        self.is_macos = platform.system() == "Darwin"
        self.memory_samples = []
        self.peak_memory_bytes = 0
        self.cpu_user_time = 0
        self.cpu_system_time = 0
        self.io_read_bytes = 0
        self.io_write_bytes = 0
        # Tracks all processes we're monitoring
        self.monitored_processes = set()
        # Maps PIDs to their last CPU times for delta calculations
        self.last_cpu_times = {}
        # Used to signal when monitoring should stop
        self.should_stop = False
        # Initial process state to capture for better accuracy
        self.initial_io_counters = {}

    def start_monitoring(self, pid):
        """Start monitoring a process and its children."""
        self.should_stop = False
        # Reset metrics for new monitoring session
        self.memory_samples = []
        self.peak_memory_bytes = 0
        self.cpu_user_time = 0
        self.cpu_system_time = 0
        self.io_read_bytes = 0
        self.io_write_bytes = 0

        try:
            # Store initial process state
            process = psutil.Process(pid)
            self.monitored_processes = {process}
            self.last_cpu_times[pid] = process.cpu_times()

            # Try to get initial I/O counters if available
            try:
                io_counters_method = getattr(process, 'io_counters', None)
                if io_counters_method and callable(io_counters_method):
                    initial_io = io_counters_method()
                    if initial_io:
                        self.initial_io_counters[pid] = initial_io
            except (psutil.AccessDenied, psutil.NoSuchProcess, AttributeError):
                # Silently skip I/O monitoring for this process if not available
                pass
        except (psutil.NoSuchProcess, psutil.AccessDenied) as e:
            print(f"Error accessing process {pid} before monitoring: {e}")

        self.monitoring_thread = threading.Thread(
            target=self._monitor_process_tree,
            args=(pid,),
            daemon=True
        )
        self.monitoring_thread.start()
        return self.monitoring_thread

    def stop_monitoring(self):
        """Signal the monitoring thread to stop."""
        self.should_stop = True
        if hasattr(self, 'monitoring_thread') and self.monitoring_thread.is_alive():
            self.monitoring_thread.join(timeout=2.0)

            # Take one final reading of all processes
            for proc in list(self.monitored_processes):
                try:
                    if proc.is_running():
                        self._update_cpu_times(proc)
                        self._update_io_counters(proc)
                except (psutil.NoSuchProcess, psutil.AccessDenied):
                    pass

    def _monitor_process_tree(self, pid):
        """Monitor a process and all its children for resource usage."""
        try:
            sampling_interval = 0.01  # 10ms for high-frequency sampling

            # Begin monitoring loop
            while not self.should_stop:
                # Update monitored processes: add any new children
                self._update_process_list()

                # Reset per-iteration counters
                current_total_memory = 0

                # Check all processes in our monitoring list
                for proc in list(self.monitored_processes):
                    try:
                        if not proc.is_running():
                            # Process has terminated, remove from monitoring
                            self.monitored_processes.remove(proc)
                            continue

                        # Measure memory using the appropriate platform-specific method
                        memory_used = self._get_process_memory(proc)
                        current_total_memory += memory_used

                        # Measure CPU time delta
                        self._update_cpu_times(proc)

                        # Measure I/O if available
                        self._update_io_counters(proc)
                    except (psutil.NoSuchProcess, psutil.AccessDenied):
                        # Process no longer exists or can't be accessed
                        self.monitored_processes.discard(proc)

                # Update memory metrics only if we got a valid reading
                if current_total_memory > 0:
                    self.memory_samples.append(current_total_memory)
                    self.peak_memory_bytes = max(
                        self.peak_memory_bytes, current_total_memory)

                time.sleep(sampling_interval)

                # If the main process isn't running and has no children, we can stop
                if not self.monitored_processes:
                    break

        except Exception as e:
            print(f"Error in monitoring thread: {e}")
            traceback.print_exc()

    def _update_process_list(self):
        """Update the list of processes we're monitoring to include new children."""
        processes_to_check = list(self.monitored_processes)

        for proc in processes_to_check:
            try:
                if proc.is_running():
                    # Add any children that aren't already being monitored
                    children = proc.children(recursive=True)
                    for child in children:
                        if child not in self.monitored_processes:
                            self.monitored_processes.add(child)
                            try:
                                self.last_cpu_times[child.pid] = child.cpu_times(
                                )
                                # Handle I/O counters in a more robust way
                                try:
                                    # Check if the process has io_counters available through callable method
                                    # instead of directly accessing the attribute
                                    io_counters_method = getattr(
                                        child, 'io_counters', None)
                                    if io_counters_method and callable(io_counters_method):
                                        io_counters = io_counters_method()
                                        self.initial_io_counters[child.pid] = io_counters
                                except (psutil.AccessDenied, psutil.NoSuchProcess):
                                    # Skip I/O monitoring for this process if we can't access it
                                    pass
                            except (psutil.NoSuchProcess, psutil.AccessDenied):
                                pass
            except (psutil.NoSuchProcess, psutil.AccessDenied):
                self.monitored_processes.discard(proc)

    def _get_process_memory(self, proc):
        """Get memory usage for a process using the most accurate method for the platform."""
        try:
            if self.is_windows:
                # On Windows, use private working set for exclusive memory usage
                return proc.memory_info().private
            elif self.is_macos:
                # On macOS, use rss - shared memory for better accuracy
                memory_info = proc.memory_info()
                try:
                    # Try to get more accurate measurement on macOS if available
                    memory_full = proc.memory_full_info()
                    return getattr(memory_full, 'uss', memory_info.rss)
                except:
                    return memory_info.rss
            else:
                # On Linux, USS (Unique Set Size) is most accurate
                try:
                    return proc.memory_full_info().uss
                except:
                    return proc.memory_info().rss
        except (psutil.NoSuchProcess, psutil.AccessDenied):
            return 0

    def _update_cpu_times(self, proc):
        """Update CPU time measurements for a process."""
        try:
            if proc.pid in self.last_cpu_times:
                current = proc.cpu_times()
                last = self.last_cpu_times[proc.pid]

                # Calculate deltas
                user_delta = max(0, current.user - last.user)
                system_delta = max(0, current.system - last.system)

                # Update totals
                self.cpu_user_time += user_delta
                self.cpu_system_time += system_delta

                # Update last seen times
                self.last_cpu_times[proc.pid] = current
        except (psutil.NoSuchProcess, psutil.AccessDenied):
            if proc.pid in self.last_cpu_times:
                del self.last_cpu_times[proc.pid]

    def _update_io_counters(self, proc):
        """Update I/O counters if available."""
        try:
            io_counters_method = getattr(proc, 'io_counters', None)
            if not io_counters_method or not callable(io_counters_method):
                # Skip silently if method doesn't exist - this is expected on some platforms
                return

            current_io = io_counters_method()
            pid = proc.pid

            # If we have initial counters, calculate the delta
            if pid in self.initial_io_counters:
                initial_io = self.initial_io_counters[pid]

                # Check if the counters have the required attributes
                if hasattr(current_io, 'read_bytes') and hasattr(initial_io, 'read_bytes'):
                    read_delta = max(0, getattr(
                        current_io, 'read_bytes') - getattr(initial_io, 'read_bytes'))
                    self.io_read_bytes += read_delta

                if hasattr(current_io, 'write_bytes') and hasattr(initial_io, 'write_bytes'):
                    write_delta = max(0, getattr(
                        current_io, 'write_bytes') - getattr(initial_io, 'write_bytes'))
                    self.io_write_bytes += write_delta
            else:
                # For first measurement, store current values as initial
                self.initial_io_counters[pid] = current_io

        except (psutil.NoSuchProcess, psutil.AccessDenied, AttributeError):
            # Process no longer exists or access denied - normal condition
            pass

    def get_metrics(self):
        """Get the collected metrics."""
        metrics = {
            "memory": {
                "peak_bytes": self.peak_memory_bytes,
                "peak_mb": self.peak_memory_bytes / (1024 * 1024),
                "avg_bytes": statistics.mean(self.memory_samples) if self.memory_samples else 0,
                "avg_mb": statistics.mean(self.memory_samples) / (1024 * 1024) if self.memory_samples else 0
            },
            "cpu": {
                "user_time": self.cpu_user_time,
                "system_time": self.cpu_system_time,
                "total_time": self.cpu_user_time + self.cpu_system_time
            },
            "io": {
                "read_bytes": self.io_read_bytes,
                "read_mb": self.io_read_bytes / (1024 * 1024),
                "write_bytes": self.io_write_bytes,
                "write_mb": self.io_write_bytes / (1024 * 1024)
            }
        }

        # Add standard deviation for memory if we have enough samples
        if len(self.memory_samples) > 1:
            metrics["memory"]["std_dev_mb"] = statistics.stdev(
                self.memory_samples) / (1024 * 1024)

        return metrics


class OutputAnalyzer:
    """Analyzes the output files from the CSS build process."""

    def __init__(self, output_dir):
        """Initialize the output analyzer."""
        self.output_dir = Path(output_dir)

    def analyze(self):
        """Analyze the output files and return metrics."""
        # Default values for when directory doesn't exist or is empty
        result = {
            "file_count": 0,
            "total_size_bytes": 0,
            "total_size_kb": 0.0,
            "avg_size_bytes": 0.0,
            "avg_size_kb": 0.0
        }

        if not self.output_dir.exists():
            print(
                f"Warning: Output directory {self.output_dir} does not exist")
            return result

        total_size = 0
        file_count = 0
        css_files = set()

        # Look for CSS files in the main directory
        for css_file in self.output_dir.glob("*.css"):
            css_files.add(css_file)

        # Also check for CSS files in subdirectories
        for css_file in self.output_dir.glob("**/*.css"):
            css_files.add(css_file)

        # Process all unique CSS files
        for css_file in css_files:
            try:
                file_size = css_file.stat().st_size
                total_size += file_size
                file_count += 1
            except Exception as e:
                print(f"Error analyzing CSS file {css_file}: {e}")

        # Update result only if we found files
        if file_count > 0:
            result["file_count"] = file_count
            result["total_size_bytes"] = total_size
            result["total_size_kb"] = float(total_size / 1024)
            result["avg_size_bytes"] = float(total_size / file_count)
            result["avg_size_kb"] = float((total_size / 1024) / file_count)

        return result


class MetricsCollector:
    """Base class for collecting and analyzing performance metrics for CSS frameworks."""

    def __init__(self, output_dir):
        """Initialize the metrics collector."""
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(exist_ok=True)
        self.process_monitor = ProcessMonitor()
        self.output_analyzer = OutputAnalyzer(output_dir)
        # Track direct file operations
        self.input_files_size = 0
        self.output_files_size = 0

    def _count_classes(self, input_dir="input"):
        """Count the total number of classes in the input files."""
        import re
        input_path = Path(input_dir)
        if not input_path.exists():
            print(f"Warning: Input directory {input_path} does not exist")
            return {"unique_class_count": 0, "total_input_size_bytes": 0}

        class_pattern = r'class="([^"]*)"'
        unique_classes = set()

        # Count total input size for I/O efficiency calculation
        total_input_size = 0
        file_count = 0

        # Process each project directory
        for project_dir in input_path.glob("project*"):
            # Process HTML files in each project
            for html_file in project_dir.glob("*.html"):
                try:
                    with open(html_file, 'r') as f:
                        content = f.read()
                        file_count += 1
                        total_input_size += len(content)
                        for match in re.finditer(class_pattern, content):
                            class_values = match.group(1).split()
                            unique_classes.update(class_values)
                except Exception as e:
                    print(f"Error reading {html_file}: {e}")

        print(
            f"Found {len(unique_classes)} unique CSS classes in {file_count} HTML files")
        return {
            "unique_class_count": len(unique_classes),
            "total_input_size_bytes": total_input_size,
            "file_count": file_count
        }

    def prepare_benchmark(self):
        """Count classes and prepare for benchmark (to be implemented by subclasses)."""
        print("Counting classes in input files...")
        input_metrics = self._count_classes()
        self.input_files_size = input_metrics["total_input_size_bytes"]

        # Clear previous output
        if self.output_dir.exists():
            for css_file in self.output_dir.glob("*.css"):
                try:
                    css_file.unlink()
                except Exception as e:
                    print(f"Error removing file {css_file}: {e}")

        return input_metrics

    def run_process(self, cmd, cwd=None):
        """Run a process and collect metrics."""
        # Start timing
        start_time = time.time()

        # Launch the process
        process = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            cwd=cwd
        )

        # Start monitoring
        self.process_monitor.start_monitoring(process.pid)

        # Wait for the process to complete
        stdout, stderr = process.communicate()

        # End timing
        end_time = time.time()

        # Stop monitoring
        self.process_monitor.stop_monitoring()

        # Print process output for debugging
        if stderr and stderr.strip():
            print(f"Process stderr: {stderr.strip()}")

        # Calculate elapsed time
        elapsed_time = end_time - start_time

        # Get process metrics
        process_metrics = self.process_monitor.get_metrics()

        return process, elapsed_time, process_metrics, stdout, stderr

    def calculate_metrics(self, elapsed_time, process_metrics, input_metrics, output_metrics, process):
        """Calculate efficiency metrics with proper fallbacks."""
        # Update IO metrics based on actual file sizes if process monitoring failed to capture
        if process_metrics["io"]["read_bytes"] < self.input_files_size:
            process_metrics["io"]["read_bytes"] = self.input_files_size
            process_metrics["io"]["read_mb"] = self.input_files_size / \
                (1024 * 1024)

        if process_metrics["io"]["write_bytes"] < self.output_files_size:
            process_metrics["io"]["write_bytes"] = self.output_files_size
            process_metrics["io"]["write_mb"] = self.output_files_size / \
                (1024 * 1024)

        # Calculate throughput metrics
        throughput_metrics = {
            "build_time_seconds": elapsed_time,
            "classes_per_second": input_metrics["unique_class_count"] / elapsed_time if elapsed_time > 0 else 0,
            "memory_efficiency": input_metrics["unique_class_count"] / max(0.001, process_metrics["memory"]["peak_mb"]),
            "bytes_processed_per_second": self.input_files_size / elapsed_time if elapsed_time > 0 else 0,
            "bytes_generated_per_second": self.output_files_size / elapsed_time if elapsed_time > 0 else 0
        }

        # Combine all metrics
        result = {
            "input": input_metrics,
            "output": output_metrics,
            "process": process_metrics,
            "throughput": throughput_metrics,
            "exit_code": process.returncode,
            "success": process.returncode == 0,
        }

        return result


class GrimoireMetricsCollector(MetricsCollector):
    """Collects and analyzes performance metrics for Grimoire CSS."""

    def __init__(self, output_dir="grimoire_css_output", executable="../target/release/grimoire_css"):
        """Initialize the Grimoire CSS metrics collector."""
        super().__init__(output_dir)
        self.executable = executable

    def run_benchmark(self):
        """Run the Grimoire CSS benchmark and collect metrics."""
        try:
            # Step 1: Prepare benchmark
            input_metrics = self.prepare_benchmark()

            # Step 2: Run the build process and collect metrics
            print("Running Grimoire CSS build...")
            cmd = [self.executable, "build"]
            process, elapsed_time, process_metrics, stdout, stderr = self.run_process(
                cmd)

            # Step 3: Analyze output files
            output_metrics = self.output_analyzer.analyze()
            self.output_files_size = output_metrics["total_size_bytes"]

            # Step 4: Calculate and combine metrics
            result = self.calculate_metrics(
                elapsed_time,
                process_metrics,
                input_metrics,
                output_metrics,
                process
            )

            return result
        except Exception as e:
            print(f"Error running Grimoire CSS benchmark: {e}")
            traceback.print_exc()
            return None


class TailwindMetricsCollector(MetricsCollector):
    """Collects and analyzes performance metrics for Tailwind CSS."""

    def __init__(self, output_dir="tailwind_css_output"):
        """Initialize the Tailwind CSS metrics collector."""
        super().__init__(output_dir)

    def run_benchmark(self):
        """Run the Tailwind CSS benchmark and collect metrics."""
        try:
            # Step 1: Prepare benchmark
            input_metrics = self.prepare_benchmark()

            # Step 2: Run Tailwind CSS for each project
            print("Running Tailwind CSS build...")

            # Track total metrics across all projects
            total_elapsed_time = 0
            combined_process_metrics = {
                "memory": {
                    "peak_bytes": 0,
                    "peak_mb": 0.0,
                    "avg_bytes": 0,
                    "avg_mb": 0.0,
                    "std_dev_mb": 0.0
                },
                "cpu": {
                    "user_time": 0,
                    "system_time": 0,
                    "total_time": 0
                },
                "io": {
                    "read_bytes": 0,
                    "read_mb": 0.0,
                    "write_bytes": 0,
                    "write_mb": 0.0
                }
            }

            # Track all memory samples to calculate overall average
            all_memory_samples = []

            # Process each project directory
            input_path = Path("input")
            project_dirs = list(input_path.glob("project*"))

            for project_dir in project_dirs:
                project_name = project_dir.name
                print(f"Processing {project_name}...")

                output_file = f"{self.output_dir}/{project_name}.css"

                # Run Tailwind CSS for this project
                cmd = ["npx", "@tailwindcss/cli",
                       "-c", "tailwind.config.js",
                       "-i", "./input.css",
                       "-o", f"../../{output_file}",
                       "-m"]

                project_process, project_elapsed_time, project_metrics, stdout, stderr = self.run_process(
                    cmd, cwd=str(project_dir))

                # Accumulate metrics
                total_elapsed_time += project_elapsed_time
                combined_process_metrics["cpu"]["user_time"] += project_metrics["cpu"]["user_time"]
                combined_process_metrics["cpu"]["system_time"] += project_metrics["cpu"]["system_time"]
                combined_process_metrics["cpu"]["total_time"] += project_metrics["cpu"]["total_time"]
                combined_process_metrics["io"]["read_bytes"] += project_metrics["io"]["read_bytes"]
                combined_process_metrics["io"]["write_bytes"] += project_metrics["io"]["write_bytes"]

                # Track peak memory
                combined_process_metrics["memory"]["peak_bytes"] = max(
                    combined_process_metrics["memory"]["peak_bytes"],
                    project_metrics["memory"]["peak_bytes"]
                )

                # Collect memory samples for overall average calculation
                process_monitor_samples = getattr(
                    self.process_monitor, 'memory_samples', [])
                if process_monitor_samples:
                    all_memory_samples.extend(process_monitor_samples)

            # Calculate combined averages
            if all_memory_samples:
                combined_process_metrics["memory"]["avg_bytes"] = statistics.mean(
                    all_memory_samples)
                combined_process_metrics["memory"]["avg_mb"] = combined_process_metrics["memory"]["avg_bytes"] / (
                    1024 * 1024)
                if len(all_memory_samples) > 1:
                    std_dev = statistics.stdev(all_memory_samples)
                    combined_process_metrics["memory"]["std_dev_mb"] = std_dev / (
                        1024 * 1024)

            # Update peak MB
            combined_process_metrics["memory"]["peak_mb"] = combined_process_metrics["memory"]["peak_bytes"] / (
                1024 * 1024)
            # Update IO in MB
            combined_process_metrics["io"]["read_mb"] = combined_process_metrics["io"]["read_bytes"] / (
                1024 * 1024)
            combined_process_metrics["io"]["write_mb"] = combined_process_metrics["io"]["write_bytes"] / (
                1024 * 1024)

            # Step 3: Analyze output files
            output_metrics = self.output_analyzer.analyze()
            self.output_files_size = output_metrics["total_size_bytes"]

            # Use a dummy process object for the final results
            dummy_process = type('obj', (object,), {'returncode': 0})

            # Step 4: Calculate and combine metrics
            result = self.calculate_metrics(
                total_elapsed_time,
                combined_process_metrics,
                input_metrics,
                output_metrics,
                dummy_process
            )

            return result
        except Exception as e:
            print(f"Error running Tailwind CSS benchmark: {e}")
            traceback.print_exc()
            return None
