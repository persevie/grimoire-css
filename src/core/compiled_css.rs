//! Types for representing compiled CSS output in filesystem and memory.

use std::path::PathBuf;

/// A vector of tuples containing output file paths and their CSS content
pub type CompiledCssFs = Vec<(PathBuf, String)>;

/// In-memory representation of compiled CSS with a name identifier
#[derive(Debug)]
pub struct CompiledCssInMemory {
    /// Name identifier for the compiled CSS
    pub name: String,
    /// The compiled CSS content
    pub content: String,
}
