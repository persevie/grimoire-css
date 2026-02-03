//! `grim` is a short alias for the `grimoire_css` CLI.
//!
//! Installing this crate via `cargo install grimoire_css` will install both binaries.

use grimoire_css_lib::start_as_cli;
use std::env;

#[cfg(feature = "heap-profile")]
use dhat::Alloc;

#[cfg(feature = "heap-profile")]
#[global_allocator]
static ALLOC: Alloc = Alloc;

fn main() {
    #[cfg(feature = "heap-profile")]
    let _profiler = dhat::Profiler::new_heap();

    let args: Vec<String> = env::args().collect();

    if start_as_cli(args).is_err() {
        std::process::exit(1);
    }
}
