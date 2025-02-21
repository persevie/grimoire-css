//! This module defines the `CssOptimizer` trait, which provides an interface for optimizing raw CSS.
//!
//! Implementations of this trait are responsible for taking raw CSS input and producing optimized,
//! minified output. The specific optimization techniques are left to the implementation details.
//! This trait allows for different optimization backends to be easily swapped or replaced within the system.

use super::GrimoireCssError;

/// The `CssOptimizer` trait provides an interface for optimizing CSS.
///
/// This trait is designed to be implemented by different CSS optimization engines. Implementations
/// of this trait can apply various techniques to minify, clean up, or otherwise optimize raw CSS code.
///
/// # Example
///
/// ```ignore
/// struct MyOptimizer;
///
/// impl CssOptimizer for MyOptimizer {
///     fn optimize(&self, raw_css: &str) -> Result<String, GrimoireCSSError> {
///         // Perform optimization here
///         Ok(minified_css)
///     }
/// }
/// ```
///
/// # Errors
///
/// This method returns a `GrimoireCSSError` if the optimization process fails for any reason.
pub trait CssOptimizer: Sync + Send {
    /// Optimizes a given raw CSS string and returns the optimized result.
    ///
    /// # Arguments
    ///
    /// * `raw_css` - A string containing the raw CSS code that needs to be optimized.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The optimized and minified CSS string.
    /// * `Err(GrimoireCSSError)` - An error indicating that the optimization process failed.
    fn optimize(&self, raw_css: &str) -> Result<String, GrimoireCssError>;
}
