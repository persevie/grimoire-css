//! The main library module that orchestrates the core functionality of the Grimoire CSS system.
//!
//! This module provides:
//! - A **pure** function `start` that executes the core logic of Grimoire CSS without
//!   printing or side effects.
//! - A **convenience** function [`start_as_cli`] for CLI usage. It includes logging,
//!   timing, and user-facing output (spinners, styled text), but is _not idiomatic_
//!   for a typical Rust library because it introduces side effects and depends on
//!   console/UI crates. Use it only if you specifically want the same CLI behavior
//!   outside of the main binary (e.g. in a Node.js wrapper).

mod buffer;
mod commands;
pub mod core;
mod infrastructure;

use crate::core::GrimoireCSSError;
use commands::process_mode_and_handle;
use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressStyle};
use infrastructure::LightningCSSOptimizer;
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
pub fn start(mode: String) -> Result<(), GrimoireCSSError> {
    let current_dir = std::env::current_dir()?;
    let css_optimizer = LightningCSSOptimizer::new(&current_dir)?;

    process_mode_and_handle(&mode, &current_dir, &css_optimizer)
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
pub fn start_as_cli(args: Vec<String>) -> Result<(), GrimoireCSSError> {
    let start_time = Instant::now();

    println!("{} {}", INFO, style("Opened the Grimoire").blue().bold());

    let pb = ProgressBar::new_spinner();
    pb.set_message(style("Casting spells...").blue().italic().to_string());
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&SPINNER)
            .template("{spinner:.magenta} {msg}")
            .unwrap(),
    );

    // Check if the user provided at least one argument (mode)
    if args.len() < 2 {
        pb.finish();
        return Err(GrimoireCSSError::InvalidInput(format!(
            "{}\n    Follow: {} <mode>",
            style(format!("{} Wrong usage!", FAILURE)).red().bold(),
            style(&args[0]).yellow().italic()
        )));
    }

    // Proceed with the main function, passing the first argument (mode)
    match start(args[1].to_owned()) {
        Ok(_) => {
            pb.finish_and_clear();
            output_saved_messages();

            let duration = start_time.elapsed();
            println!(
                "{}",
                style(format!("✨ Magic complete in {:.2?}!", duration))
                    .magenta()
                    .bold()
            );
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

            Err(e)
        }
    }
}

fn output_saved_messages() {
    let messages = get_logged_messages();

    if !messages.is_empty() {
        println!("{}", style("History:").dim().bold());

        for msg in messages.iter() {
            println!("{} {}", style("    •").dim(), style(msg).dim());
        }
    }
}
