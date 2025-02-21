//! The main executable entry point for the Grimoire CSS system.
//!
//! This file handles command-line argument parsing and initiates the main processing by calling
//! the [`start_as_cli`] function from the core library. It ensures that errors are handled gracefully
//! and reported to the user by returning a non-zero status code on failure.

use grimoire_css_lib::start_as_cli;
use std::env;

/// The entry point for the Grimoire CSS system (CLI).
///
/// This function:
/// - Collects command-line arguments.
/// - Passes them into [`start_as_cli`] from the core library, which handles
///   logging, error styling, spinners, and time measurements.
/// - If an error is encountered, it exits with a non-zero status code.
fn main() {
    let args: Vec<String> = env::args().collect();

    // By calling `start_as_cli`, we rely on the library's built-in logging,
    // progress bar, and error-handling logic.
    if start_as_cli(args).is_err() {
        std::process::exit(1);
    }
}
