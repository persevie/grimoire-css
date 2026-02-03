//! Configuration management for GrimoireCSS.
//!
//! Provides two configuration types:
//! - [`ConfigFs`] - File system based configuration with JSON serialization
//! - [`ConfigInMemory`] - In-memory configuration for testing and programmatic use
//!
//! Use [`ConfigFs`] for standard projects and [`ConfigInMemory`] for testing or embedding.

pub mod config_fs;
pub mod config_in_memory;

pub use config_fs::*;
pub use config_in_memory::*;

use std::collections::HashMap;

/// A scroll definition as used by the compiler.
///
/// - `spells`: always included when the scroll is invoked.
/// - `spells_by_args`: optional overloads selected by argument count (keyed by "0", "1", ...).
#[derive(Debug, Clone, Default)]
pub struct ScrollDefinition {
    pub spells: Vec<String>,
    pub spells_by_args: Option<HashMap<String, Vec<String>>>,
}
