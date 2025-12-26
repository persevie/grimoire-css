use crate::core::{GrimoireCssError, SourceFile};
use miette::{Diagnostic, LabeledSpan, SourceCode};
use std::sync::Arc;
use thiserror::Error;

fn named_source_from(source: &Arc<SourceFile>) -> miette::NamedSource<String> {
    miette::NamedSource::new(source.name.clone(), (*source.content).clone())
}

#[derive(Debug, Error)]
pub enum GrimoireCssDiagnostic {
    #[error("IO error: {0}")]
    Io(String),

    #[error("Regex error: {0}")]
    Regex(String),

    #[error("Serialization/Deserialization error: {0}")]
    Serde(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Glob pattern error: {0}")]
    GlobPatternError(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("CSS Optimization failed: {0}")]
    OptimizationError(String),

    #[error("Invalid spell format: {message}")]
    InvalidSpellFormat {
        message: String,
        src: miette::NamedSource<String>,
        span: (usize, usize),
        label: String,
        help: Option<String>,
    },

    #[error("{message}")]
    CompileError {
        message: String,
        src: miette::NamedSource<String>,
        span: (usize, usize),
        label: String,
        help: Option<String>,
    },
}

impl Diagnostic for GrimoireCssDiagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match self {
            GrimoireCssDiagnostic::Io(_) => Some(Box::new("grimoire_css::io")),
            GrimoireCssDiagnostic::Regex(_) => Some(Box::new("grimoire_css::regex")),
            GrimoireCssDiagnostic::Serde(_) => Some(Box::new("grimoire_css::serde")),
            GrimoireCssDiagnostic::InvalidInput(_) => Some(Box::new("grimoire_css::invalid_input")),
            GrimoireCssDiagnostic::InvalidPath(_) => Some(Box::new("grimoire_css::invalid_path")),
            GrimoireCssDiagnostic::GlobPatternError(_) => {
                Some(Box::new("grimoire_css::glob_pattern"))
            }
            GrimoireCssDiagnostic::RuntimeError(_) => Some(Box::new("grimoire_css::runtime")),
            GrimoireCssDiagnostic::OptimizationError(_) => {
                Some(Box::new("grimoire_css::optimization"))
            }
            GrimoireCssDiagnostic::InvalidSpellFormat { .. } => {
                Some(Box::new("grimoire_css::invalid_spell_format"))
            }
            GrimoireCssDiagnostic::CompileError { .. } => {
                Some(Box::new("grimoire_css::compile_error"))
            }
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match self {
            GrimoireCssDiagnostic::InvalidSpellFormat { help, .. }
            | GrimoireCssDiagnostic::CompileError { help, .. } => help
                .as_deref()
                .map(|h| Box::new(h) as Box<dyn std::fmt::Display>),
            _ => None,
        }
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        match self {
            GrimoireCssDiagnostic::InvalidSpellFormat { src, .. }
            | GrimoireCssDiagnostic::CompileError { src, .. } => Some(src),
            _ => None,
        }
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        match self {
            GrimoireCssDiagnostic::InvalidSpellFormat { span, label, .. }
            | GrimoireCssDiagnostic::CompileError { span, label, .. } => {
                Some(Box::new(std::iter::once(
                    LabeledSpan::new_primary_with_span(Some(label.clone()), *span),
                )))
            }
            _ => None,
        }
    }
}

impl From<&GrimoireCssError> for GrimoireCssDiagnostic {
    fn from(value: &GrimoireCssError) -> Self {
        match value {
            GrimoireCssError::Io(e) => GrimoireCssDiagnostic::Io(e.to_string()),
            GrimoireCssError::Regex(e) => GrimoireCssDiagnostic::Regex(e.to_string()),
            GrimoireCssError::Serde(e) => GrimoireCssDiagnostic::Serde(e.to_string()),
            GrimoireCssError::InvalidInput(msg) => GrimoireCssDiagnostic::InvalidInput(msg.clone()),
            GrimoireCssError::InvalidPath(msg) => GrimoireCssDiagnostic::InvalidPath(msg.clone()),
            GrimoireCssError::GlobPatternError(msg) => {
                GrimoireCssDiagnostic::GlobPatternError(msg.clone())
            }
            GrimoireCssError::RuntimeError(msg) => GrimoireCssDiagnostic::RuntimeError(msg.clone()),
            GrimoireCssError::OptimizationError(msg) => {
                GrimoireCssDiagnostic::OptimizationError(msg.clone())
            }
            GrimoireCssError::InvalidSpellFormat {
                message,
                span,
                label,
                help,
                source_file,
            } => {
                let src = source_file
                    .as_ref()
                    .map(named_source_from)
                    .unwrap_or_else(|| miette::NamedSource::new("unknown", "".to_string()));

                GrimoireCssDiagnostic::InvalidSpellFormat {
                    message: message.clone(),
                    src,
                    span: *span,
                    label: label.clone(),
                    help: help.clone(),
                }
            }
            GrimoireCssError::CompileError {
                message,
                span,
                label,
                help,
                source_file,
            } => {
                let src = source_file
                    .as_ref()
                    .map(named_source_from)
                    .unwrap_or_else(|| miette::NamedSource::new("unknown", "".to_string()));

                GrimoireCssDiagnostic::CompileError {
                    message: message.clone(),
                    src,
                    span: *span,
                    label: label.clone(),
                    help: help.clone(),
                }
            }
        }
    }
}
