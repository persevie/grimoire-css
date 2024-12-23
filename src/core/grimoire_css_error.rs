//! This module defines the `GrimoireCSSError` enum, which encapsulates various error types
//! that can occur within the GrimoireCSS project.
//!
//! The `GrimoireCSSError` enum covers errors originating from IO operations, regular expressions,
//! serialization/deserialization processes, and custom application-specific errors related to
//! invalid input or spell formats.

use regex;
use serde_json;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum GrimoireCSSError {
    Io(io::Error),
    Regex(regex::Error),
    Serde(serde_json::Error),
    InvalidSpellFormat(String),
    InvalidInput(String),
    InvalidPath(String),
    GlobPatternError(String),
    RuntimeError(String),
}

impl fmt::Display for GrimoireCSSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrimoireCSSError::Io(e) => write!(f, "IO error: {}", e),
            GrimoireCSSError::Regex(e) => write!(f, "Regex error: {}", e),
            GrimoireCSSError::Serde(e) => write!(f, "Serialization/Deserialization error: {}", e),
            GrimoireCSSError::InvalidSpellFormat(s) => write!(f, "Invalid spell format: {}", s),
            GrimoireCSSError::InvalidInput(s) => write!(f, "Invalid input: {}", s),
            GrimoireCSSError::InvalidPath(s) => write!(f, "Invalid path: {}", s),
            GrimoireCSSError::GlobPatternError(s) => write!(f, "Glob pattern error: {}", s),
            GrimoireCSSError::RuntimeError(s) => write!(f, "Runtime error: {}", s),
        }
    }
}

impl std::error::Error for GrimoireCSSError {}

impl From<io::Error> for GrimoireCSSError {
    fn from(error: io::Error) -> Self {
        GrimoireCSSError::Io(error)
    }
}

impl From<regex::Error> for GrimoireCSSError {
    fn from(error: regex::Error) -> Self {
        GrimoireCSSError::Regex(error)
    }
}

impl From<serde_json::Error> for GrimoireCSSError {
    fn from(error: serde_json::Error) -> Self {
        GrimoireCSSError::Serde(error)
    }
}
