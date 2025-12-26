use crate::core::{GrimoireCssError, SourceFile};
use miette::Diagnostic;
use std::sync::Arc;
use thiserror::Error;

fn named_source_from(source: &Arc<SourceFile>) -> miette::NamedSource<String> {
    let content = if let Some(content) = &source.content {
        (**content).clone()
    } else if let Some(path) = &source.path {
        std::fs::read_to_string(path).unwrap_or_default()
    } else {
        String::new()
    };

    miette::NamedSource::new(source.name.clone(), content)
}

#[derive(Debug, Error, Diagnostic)]
pub enum GrimoireCssDiagnostic {
    #[error("IO error: {0}")]
    #[diagnostic(code(grimoire_css::io))]
    Io(String),

    #[error("Regex error: {0}")]
    #[diagnostic(code(grimoire_css::regex))]
    Regex(String),

    #[error("Serialization/Deserialization error: {0}")]
    #[diagnostic(code(grimoire_css::serde))]
    Serde(String),

    #[error("Invalid input: {0}")]
    #[diagnostic(code(grimoire_css::invalid_input))]
    InvalidInput(String),

    #[error("Invalid path: {0}")]
    #[diagnostic(code(grimoire_css::invalid_path))]
    InvalidPath(String),

    #[error("Glob pattern error: {0}")]
    #[diagnostic(code(grimoire_css::glob_pattern))]
    GlobPatternError(String),

    #[error("Runtime error: {0}")]
    #[diagnostic(code(grimoire_css::runtime))]
    RuntimeError(String),

    #[error("CSS Optimization failed: {0}")]
    #[diagnostic(code(grimoire_css::optimization))]
    OptimizationError(String),

    #[error("Invalid spell format: {message}")]
    #[diagnostic(code(grimoire_css::invalid_spell_format))]
    InvalidSpellFormat {
        message: String,
        #[source_code]
        src: miette::NamedSource<String>,
        #[label("{label}")]
        span: (usize, usize),
        label: String,
        #[help]
        help: Option<String>,
    },

    #[error("{message}")]
    #[diagnostic(code(grimoire_css::compile_error))]
    CompileError {
        message: String,
        #[source_code]
        src: miette::NamedSource<String>,
        #[label("{label}")]
        span: (usize, usize),
        label: String,
        #[help]
        help: Option<String>,
    },
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
