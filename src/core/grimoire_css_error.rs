//! This module defines the `GrimoireCssError` enum, which encapsulates various error types
//! that can occur within the GrimoireCSS project.
//!
//! The `GrimoireCssError` enum covers errors originating from IO operations, regular expressions,
//! serialization/deserialization processes, and custom application-specific errors related to
//! invalid input or spell formats.

use regex;
use serde_json;
use std::fmt;
use std::io;

/// Represents all possible errors that can occur in the Grimoire CSS system.
///
/// This enum consolidates different error types from various operations into
/// a single error type, making error handling consistent throughout the application.
#[derive(Debug)]
pub enum GrimoireCssError {
    /// IO errors during file operations
    Io(io::Error),
    /// Regular expression parsing or execution errors
    Regex(regex::Error),
    /// JSON serialization/deserialization errors
    Serde(serde_json::Error),
    /// Invalid spell format (e.g., malformed class names or templates)
    InvalidSpellFormat(String),
    /// General input validation errors
    InvalidInput(String),
    /// Invalid file or directory path errors
    InvalidPath(String),
    /// Errors in glob pattern syntax or matching
    GlobPatternError(String),
    /// Runtime errors that don't fit other categories
    RuntimeError(String),
}

impl fmt::Display for GrimoireCssError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrimoireCssError::Io(e) => write!(f, "IO error: {e}"),
            GrimoireCssError::Regex(e) => write!(f, "Regex error: {e}"),
            GrimoireCssError::Serde(e) => write!(f, "Serialization/Deserialization error: {e}"),
            GrimoireCssError::InvalidSpellFormat(s) => write!(f, "Invalid spell format: {s}"),
            GrimoireCssError::InvalidInput(s) => write!(f, "Invalid input: {s}"),
            GrimoireCssError::InvalidPath(s) => write!(f, "Invalid path: {s}"),
            GrimoireCssError::GlobPatternError(s) => write!(f, "Glob pattern error: {s}"),
            GrimoireCssError::RuntimeError(s) => write!(f, "Runtime error: {s}"),
        }
    }
}

impl std::error::Error for GrimoireCssError {}

impl From<io::Error> for GrimoireCssError {
    fn from(error: io::Error) -> Self {
        GrimoireCssError::Io(error)
    }
}

impl From<regex::Error> for GrimoireCssError {
    fn from(error: regex::Error) -> Self {
        GrimoireCssError::Regex(error)
    }
}

impl From<serde_json::Error> for GrimoireCssError {
    fn from(error: serde_json::Error) -> Self {
        GrimoireCssError::Serde(error)
    }
}
