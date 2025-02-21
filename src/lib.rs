//! Core library module that orchestrates the core functionality of the Grimoire CSS system engine.
//!
//! This module provides two main functions:
//! - [`start`] - Pure function that executes core CSS processing logic
//! - [`start_in_memory`] - Function for processing CSS in memory without file I/O
//! - [`start_as_cli`] - CLI wrapper with logging and user feedback (spinners, colors), it is _not idiomatic_
//!   for a typical Rust library because it introduces side effects and depends on
//!   console/UI crates. Use it only if you specifically want the same CLI behavior
//!   outside of the main binary (e.g. in a Node.js wrapper or CLI application).
//!
//! Choose [`start`] for library usage and [`start_as_cli`] for CLI applications.

mod buffer;
mod commands;
pub mod core;
mod infrastructure;

use crate::core::GrimoireCssError;
use commands::{handle_in_memory, process_mode_and_handle};
use console::{style, Emoji};
use core::{CompiledCssInMemory, ConfigInMemory};
use indicatif::{ProgressBar, ProgressStyle};
use infrastructure::LightningCssOptimizer;
use std::time::{Duration, Instant};

pub static SUCCESS: Emoji<'_, '_> = Emoji("🪄", "✔️");
pub static FAILURE: Emoji<'_, '_> = Emoji("☠️", "X");
pub static INFO: Emoji<'_, '_> = Emoji("📖", "i");

pub static SPINNER: [&str; 10] = ["🜁", "🜂", "🜃", "🜄", "✷", "☽", "☾", "🜇", "✶", ""];

/// Starts the Grimoire CSS system based on the given mode,
/// **without** performing any CLI-specific side effects.
///
/// This function determines the current working directory, initializes
/// the `LightningCSSOptimizer`, and then processes the mode, invoking the
/// appropriate command handlers.
///
/// # Arguments
///
/// * `mode` - A string representing the mode of operation (e.g., "build", "init").
///
/// # Returns
///
/// * `Ok(())` - If the mode is processed successfully.
/// * `Err(GrimoireCSSError)` - If there is an error during initialization or command execution.
///
/// # Errors
///
/// This function returns a `GrimoireCSSError` if the current directory cannot be determined,
/// the optimizer initialization fails, or the mode processing encounters an error.
///
/// # Examples
///
/// ```ignore
/// use grimoire_css_lib::start;
/// if let Err(e) = start("build".to_string()) {
///     eprintln!("Error: {e}");
/// }
/// ```
pub fn start(mode: &str) -> Result<(), GrimoireCssError> {
    let current_dir = std::env::current_dir()?;
    let css_optimizer = LightningCssOptimizer::new(&current_dir)?;

    process_mode_and_handle(mode, &current_dir, &css_optimizer)
}

pub fn start_in_memory(
    config: &ConfigInMemory,
) -> Result<Vec<CompiledCssInMemory>, GrimoireCssError> {
    let css_optimizer = LightningCssOptimizer::new_from(
        config.browserslist_content.as_deref().unwrap_or_default(),
    )?;

    handle_in_memory(config, &css_optimizer)
}

/// Public function to read the saved messages from the buffer.
/// This function is accessible from the main.rs (or any other crate/binary)
/// for reading the buffer content.
pub fn get_logged_messages() -> Vec<String> {
    buffer::read_messages()
}

/// A convenience function that **simulates CLI behavior** (timing, logging, spinner)
/// but is placed in the library crate to avoid duplicating code in multiple binaries
/// or wrappers.
///
/// # Warning
///
/// - This is **not** an idiomatic approach for a typical Rust library since it
///   introduces console-based side effects.
/// - If you do not want logs, spinners, or colorized output, **do not** call this
///   function. Instead, call [`start`] directly.
/// - This function **depends** on `console` and `indicatif` crates for styling
///   and progress-bar support, which might not be desired in all contexts.
///
/// # Arguments
///
/// * `args` - A vector of strings typically representing command-line arguments.
///   The first argument is expected to be the binary name, and the second argument
///   the mode (e.g. "build", "init").
///
/// # Returns
///
/// * `Ok(())` on success, or an `Err(GrimoireCSSError)` if invalid arguments or runtime
///   issues occur.
///
/// # Examples
///
/// ```ignore
/// use grimoire_css_lib::start_as_cli;
///
/// // Typically used in a real CLI setup:
/// let args = vec!["grimoire_css".to_string(), "build".to_string()];
/// if let Err(err) = start_as_cli(args) {
///     eprintln!("Failed: {err}");
/// }
/// ```
pub fn start_as_cli(args: Vec<String>) -> Result<(), GrimoireCssError> {
    // print empty line for better readability
    println!();

    // Check if the user provided at least one argument (mode)
    if args.len() < 2 {
        let message = format!(
            "{}\n    {} ",
            style(format!("{} Wrong usage!", FAILURE)).red().bold(),
            style("Follow: grimoire_css <mode> ('build' or 'init')").italic()
        );
        println!("{}", message);

        return Err(GrimoireCssError::InvalidInput(message));
    }

    println!(
        "{} {}",
        INFO,
        style("Open the Grimoire").color256(55).bold()
    );

    let pb = ProgressBar::new_spinner();
    pb.set_message(style("Casting spells...").color256(55).italic().to_string());
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&SPINNER)
            .template("{spinner:.magenta} {msg}")
            .unwrap(),
    );

    let start_time = Instant::now();

    // Proceed with the main function, passing the first argument (mode)
    match start(args[1].as_str()) {
        Ok(_) => {
            pb.finish_and_clear();

            let duration = start_time.elapsed();
            println!(
                "{}",
                style(format!(
                    "{} {:.2?}",
                    style("✨ Magic complete in").color256(55).bold(),
                    style(duration).color256(55).bold().underlined(),
                ))
            );

            output_saved_messages();

            // print empty line for better readability
            println!();
            Ok(())
        }
        Err(e) => {
            pb.finish();

            println!(
                "{}",
                style(format!("{} Dark magic interfered!\n {}", FAILURE, e))
                    .red()
                    .bold()
            );

            // print empty line for better readability
            println!();
            Err(e)
        }
    }
}

fn output_saved_messages() {
    let messages = get_logged_messages();

    if !messages.is_empty() {
        for msg in &messages {
            println!(
                "{} {}",
                style("    •").color256(53),
                style(msg).color256(53)
            );
        }
    }
}
