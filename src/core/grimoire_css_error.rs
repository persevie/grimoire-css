//! This module defines the `GrimoireCssError` enum, which encapsulates various error types
//! that can occur within the GrimoireCSS project.
//!
//! The `GrimoireCssError` enum covers errors originating from IO operations, regular expressions,
//! serialization/deserialization processes, and custom application-specific errors related to
//! invalid input or spell formats.

use thiserror::Error;
use std::sync::Arc;

use super::source_file::SourceFile;

/// Represents all possible errors that can occur in the Grimoire CSS system.
///
/// This enum consolidates different error types from various operations into
/// a single error type, making error handling consistent throughout the application.
#[derive(Debug, Error)]
pub enum GrimoireCssError {
    /// IO errors during file operations
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Regular expression parsing or execution errors
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    /// JSON serialization/deserialization errors
    #[error("Serialization/Deserialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// Invalid spell format (e.g., malformed class names or templates)
    #[error("Invalid spell format: {message}")]
    InvalidSpellFormat {
        message: String,
        span: (usize, usize),
        label: String,
        help: Option<String>,
        source_file: Option<Arc<SourceFile>>,
    },

    /// General input validation errors
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Invalid file or directory path errors
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Errors in glob pattern syntax or matching
    #[error("Glob pattern error: {0}")]
    GlobPatternError(String),

    /// Runtime errors that don't fit other categories
    #[error("Runtime error: {0}")]
    RuntimeError(String),

    /// CSS Optimization errors (e.g. from LightningCSS)
    #[error("CSS Optimization failed: {0}")]
    OptimizationError(String),

    /// Error with source context for better reporting
    #[error("{message}")]
    CompileError {
        message: String,
        span: (usize, usize),
        label: String,
        help: Option<String>,
        source_file: Option<Arc<SourceFile>>,
    },
}

impl GrimoireCssError {
    pub fn with_source(self, source: Arc<SourceFile>) -> Self {
        match self {
            GrimoireCssError::InvalidSpellFormat {
                message,
                span,
                label,
                help,
                source_file: existing,
            } => GrimoireCssError::InvalidSpellFormat {
                message,
                span,
                label,
                help,
                source_file: existing.or(Some(source)),
            },
            GrimoireCssError::CompileError {
                message,
                span,
                label,
                help,
                source_file: existing,
            } => GrimoireCssError::CompileError {
                message,
                span,
                label,
                help,
                source_file: existing.or(Some(source)),
            },
            other => other,
        }
    }

    pub fn source(&self) -> Option<&Arc<SourceFile>> {
        match self {
            GrimoireCssError::InvalidSpellFormat { source_file, .. } => source_file.as_ref(),
            GrimoireCssError::CompileError { source_file, .. } => source_file.as_ref(),
            _ => None,
        }
    }
}
