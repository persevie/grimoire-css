//! The `infrastructure` module provides integration with external libraries and services
//! that power Grimoire CSS's core functionality. This includes CSS optimization,
//! minification, and other low-level operations that require external dependencies.
pub mod lightning_css_optimizer;

pub use lightning_css_optimizer::*;
