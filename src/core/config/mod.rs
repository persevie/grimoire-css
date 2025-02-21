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
