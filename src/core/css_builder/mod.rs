//! The CSS builder module provides flexible CSS generation and processing.
//!
//! Each builder implementation offers:
//! - CSS generation from spells
//! - CSS optimization
//! - Configurable output handling
//!
//! Available implementations:
//! - [`CssBuilderFs`] - File system-based with persistent storage
//! - [`CssBuilderInMemory`] - Entirely in-memory

mod css_builder_base;
mod css_builder_fs;
mod css_builder_in_memory;

pub use css_builder_base::*;
pub use css_builder_fs::*;
pub use css_builder_in_memory::*;
