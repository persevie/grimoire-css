//! The `commands` module handles the execution of different modes in the Grimoire CSS system.
//!
//! This module provides functionality for initializing and executing different commands such as
//! building and initializing configurations. It serves as a command dispatcher, routing requests
//! based on the selected mode or action.

mod build;
pub(crate) mod fi;
pub mod handler;
mod init;
mod shorten;
pub(crate) mod transmute;

pub use handler::*;
pub(crate) use init::init as init_project;
pub(crate) use shorten::shorten as shorten_project;
