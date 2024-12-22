//! The `core` module contains the essential building blocks of the Grimoire CSS system.
//!
//! This module defines the core structures and logic required to build and optimize CSS.
//! It includes utilities for parsing, generating, and managing CSS, as well as error handling
//! and configuration management. The public submodules provide the necessary components
//! to work with the Grimoire CSS pipeline, from initial configuration to the final optimized output.

pub mod color;
pub mod component;
pub mod config;
pub mod css_builder;
pub mod css_optimizer;
mod file_tracker;
mod filesystem;
pub mod grimoire_css_error;
pub mod spell;

mod animations;
mod build_info;
mod css_generator;
mod parser;

pub use color::*;
pub use config::*;
pub use css_builder::*;
pub use css_optimizer::*;
pub use filesystem::*;
pub use grimoire_css_error::*;
