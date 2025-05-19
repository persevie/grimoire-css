//! The `commands` module handles the execution of different modes in the Grimoire CSS system.
//!
//! This module provides functionality for initializing and executing different commands such as
//! building and initializing configurations. It serves as a command dispatcher, routing requests
//! based on the selected mode or action.

mod build;
pub mod handler;
mod init;
mod shorten;

pub use handler::*;
