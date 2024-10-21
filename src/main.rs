//! The main executable entry point for the Grimoire CSS system.
//!
//! This file handles command-line argument parsing and initiates the main processing by calling
//! the `start` function from the core library. It ensures that errors are handled gracefully
//! and reported to the user with styled output.

use console::{style, Emoji};
use grimoire_css_lib::{get_logged_messages, start};
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::time::{Duration, Instant};

pub static SUCCESS: Emoji<'_, '_> = Emoji("🪄", "✔️");
pub static FAILURE: Emoji<'_, '_> = Emoji("☠️", "X");
pub static INFO: Emoji<'_, '_> = Emoji("📖", "i");

pub static SPINNER: [&str; 10] = ["🜁", "🜂", "🜃", "🜄", "✷", "☽", "☾", "🜇", "✶", ""];

/// The entry point for the Grimoire CSS system.
///
/// This function collects command-line arguments, passes the first argument as the mode to the
/// `start` function from the core library, and handles any errors that occur during execution.
///
/// If an error is encountered, it prints the error message to `stderr` and exits with a non-zero status code.
fn main() {
    let args: Vec<String> = env::args().collect();

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
        let message = format!("{} Usage: {} <mode>", FAILURE, args[0]);
        pb.finish_with_message(message);
        eprintln!("{} Example: {} transmute", FAILURE, args[0]);
        std::process::exit(1);
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
        }
        Err(e) => {
            let message = format!("{} Dark magic interfered: {}", FAILURE, e);
            pb.finish_with_message(message);
            eprintln!("{}", style(format!("Error: {}", e)).red().bold());
            std::process::exit(1);
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
