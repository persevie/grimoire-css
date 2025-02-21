//! CSS generation and color manipulation.
//!
//! This module provides:
//! - CSS code generation from spells
//! - Color function processing (lighten, darken, etc.)
//! - CSS class name generation with proper escaping

pub mod color_functions;
pub mod css_generator_base;

pub use css_generator_base::*;
