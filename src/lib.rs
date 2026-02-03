//! Core library module that orchestrates the core functionality of the Grimoire CSS engine.
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
mod core;
mod infrastructure;

#[cfg(feature = "analyzer")]
pub mod analyzer;

#[cfg(feature = "lsp")]
pub mod lsp;

use commands::{handle_in_memory, process_mode_and_handle, process_mode_and_handle_with_options};
use console::style;
use core::{compiled_css::CompiledCssInMemory, config::ConfigInMemory};
use indicatif::{ProgressBar, ProgressStyle};
use infrastructure::{GrimoireCssDiagnostic, LightningCssOptimizer};
use miette::GraphicalReportHandler;
use std::path::Path;
use std::time::{Duration, Instant};

pub use core::{GrimoireCssError, color, component, config, spell::Spell};

static GRIMM_CALM: &str = " |(• ε •)|";
static GRIMM_HAPPY: &str = " ヽ(• ε •)ﾉ";
static GRIMM_CURSED: &str = " |(x ~ x)|";
static GRIMM_CASTING: [&str; 8] = [
    GRIMM_CALM,
    " (|¬ヘ¬)|",
    " \\(°o°)/",
    " (∩¬ロ¬)⊃━▪ ～",
    " (∩¬ロ¬)⊃━▪ ～·",
    " (∩¬ロ¬)⊃━▪ ～·•",
    " (∩¬ロ¬)⊃━▪ ～·•●",
    GRIMM_HAPPY,
];

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

/// Analyzer/LSP helper: same as [`start_in_memory`], but prints CSS in a human-readable format.
///
/// This is feature-gated to avoid affecting default runtime/CLI output.
#[cfg(feature = "analyzer")]
pub fn start_in_memory_pretty(
    config: &ConfigInMemory,
) -> Result<Vec<CompiledCssInMemory>, GrimoireCssError> {
    let css_optimizer = LightningCssOptimizer::new_from_with_printer_minify(
        config.browserslist_content.as_deref().unwrap_or_default(),
        false,
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
    let bin_name = args
        .first()
        .and_then(|s| Path::new(s).file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("grimoire_css");

    // Special-case: `fi` supports clean JSON output (no banners/spinners).
    if args.get(1).is_some_and(|m| m == "fi") {
        return commands::fi::run_fi_cli(args);
    }

    println!();

    println!(
        "{}  Ritual initiated",
        style(" Grimoire CSS ").white().on_color256(55).bright(),
    );

    // Check if the user provided at least one argument (mode)
    if args.len() < 2 {
        let message = format!(
            "{}  {} ",
            style(" Cursed! ").white().on_red().bright(),
            format_args!("Follow: {bin_name} <mode> ('build', 'init', 'shorten', 'fi')")
        );

        println!();
        println!("{GRIMM_CURSED}");
        println!();
        println!("{message}");

        return Err(GrimoireCssError::InvalidInput(message));
    }

    println!();

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().tick_strings(&GRIMM_CASTING));
    pb.enable_steady_tick(Duration::from_millis(220));
    pb.set_draw_target(indicatif::ProgressDrawTarget::stdout_with_hz(10));

    let start_time = Instant::now();

    let mode = args[1].as_str();

    let cli_options = commands::CliOptions {
        force_version_update: mode == "build" && args.iter().any(|a| a == "--force-version-update"),
    };

    // Proceed with the main function, passing the first argument (mode)
    let current_dir = std::env::current_dir()?;
    let css_optimizer = LightningCssOptimizer::new(&current_dir)?;

    match process_mode_and_handle_with_options(mode, &current_dir, &css_optimizer, cli_options) {
        Ok(_) => {
            pb.finish_and_clear();

            print!("\r\x1b[2K{GRIMM_HAPPY}  Spells cast successfully.\n");

            let duration = start_time.elapsed();

            output_saved_messages();

            println!();

            println!(
                "{}",
                style(format!(
                    "{}",
                    style(format!(" Enchanted in {duration:.2?}! "))
                        .white()
                        .on_color256(55)
                        .bright(),
                ))
            );

            println!();

            Ok(())
        }
        Err(e) => {
            pb.finish_and_clear();
            print!("\r\x1b[2K{GRIMM_CURSED}\n");

            println!();
            println!("{}", style(" Cursed! ").white().on_red().bright());
            println!();

            let diagnostic: GrimoireCssDiagnostic = (&e).into();
            let mut out = String::new();
            GraphicalReportHandler::new()
                .render_report(&mut out, &diagnostic)
                .unwrap();
            println!("{out}");

            Err(e)
        }
    }
}

fn output_saved_messages() {
    let messages = get_logged_messages();

    if !messages.is_empty() {
        println!();
        for msg in &messages {
            println!("  • {msg}");
        }
    }
}
