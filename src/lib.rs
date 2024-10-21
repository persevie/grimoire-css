//! The main library module that orchestrates the core functionality of the Grimoire CSS system.
//!
//! This module initializes the CSS optimizer and manages the execution of commands based on the
//! provided mode. It serves as the entry point for building and optimizing CSS based on various
//! configurations and inputs.

mod buffer;
mod commands;
pub mod core;
mod infrastructure;

use crate::core::GrimoireCSSError;
use commands::process_mode_and_handle;
use infrastructure::LightningCSSOptimizer;

/// Starts the Grimoire CSS system based on the given mode.
///
/// This function determines the current working directory, initializes the `LightningCSSOptimizer`,
/// and then processes the mode, invoking the appropriate command handlers.
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
pub fn start(mode: String) -> Result<(), GrimoireCSSError> {
    let current_dir = std::env::current_dir()?;
    let css_optimizer = LightningCSSOptimizer::new(&current_dir)?;

    process_mode_and_handle(&mode, &current_dir, &css_optimizer)
}

/// Public function to read the saved messages from the buffer.
/// This function is accessible from the main.rs for reading the buffer content.
pub fn get_logged_messages() -> Vec<String> {
    buffer::read_messages()
}
