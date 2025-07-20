//! CSS optimization and minification implementation using the LightningCSS engine.
//!
//! This module provides browser-aware CSS optimization through `.browserslistrc` configuration,
//! handling both file-based and in-memory CSS processing.

use lightningcss::{
    stylesheet::{MinifyOptions, ParserOptions, StyleSheet},
    targets::{Browsers, Targets},
};
use std::{env, fs, path::Path};

use crate::{
    buffer::add_message,
    core::{CssOptimizer, GrimoireCssError},
};

/// CSS optimizer implementation using the LightningCSS engine.
///
/// Handles CSS optimization and minification while respecting browser compatibility
/// settings defined in `.browserslistrc`. Creates a default configuration if none exists.
pub struct LightningCssOptimizer {
    targets: Targets,
}

impl LightningCssOptimizer {
    fn from_content(browserslist_content: &str) -> Result<Self, GrimoireCssError> {
        let browsers = Browsers::from_browserslist(browserslist_content.lines()).map_err(|e| {
            GrimoireCssError::InvalidInput(format!("Failed to parse browserslist: {e}"))
        })?;

        Ok(Self {
            targets: Targets {
                browsers,
                include: Default::default(),
                exclude: Default::default(),
            },
        })
    }

    pub fn new(current_dir: &Path) -> Result<Self, GrimoireCssError> {
        let browserslist_config_path = current_dir.join(".browserslistrc");

        if !browserslist_config_path.exists() {
            fs::write(&browserslist_config_path, "defaults")
                .expect("Failed to create '.browserslistrc' with defaults");

            add_message("Created missing '.browserslistrc' file with 'defaults'".to_string());
        }

        // SAFETY: We're setting an environment variable in a controlled manner.
        // This is safe as long as no other threads are concurrently reading this variable.
        unsafe {
            env::set_var("BROWSERSLIST_CONFIG", &browserslist_config_path);
        }

        let content = fs::read_to_string(&browserslist_config_path)
            .expect("Failed to read '.browserslistrc' file");

        Self::from_content(&content)
    }

    pub fn new_from(browserslist_content: &str) -> Result<Self, GrimoireCssError> {
        Self::from_content(browserslist_content)
    }
}

impl CssOptimizer for LightningCssOptimizer {
    /// Optimizes and minifies raw CSS input using `lightningcss`.
    ///
    /// This method parses the raw CSS string, applies the specified minification and optimization
    /// options, and returns the final optimized CSS output. It leverages the browser targets
    /// that were set up during initialization.
    ///
    /// # Arguments
    ///
    /// * `raw_css` - The raw CSS string to be optimized.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the optimized CSS string or a `GrimoireCSSError` if optimization fails.
    fn optimize(&self, raw_css: &str) -> Result<String, GrimoireCssError> {
        let mut stylesheet = StyleSheet::parse(raw_css, ParserOptions::default())
            .map_err(|e| GrimoireCssError::InvalidInput(format!("Failed to parse CSS: {e}")))?;

        // Apply minification and optimization based on the browser targets.
        stylesheet
            .minify(MinifyOptions {
                targets: self.targets,
                unused_symbols: Default::default(),
            })
            .map_err(|e| GrimoireCssError::InvalidInput(format!("Failed to minify CSS: {e}")))?;

        // Generate the final CSS as a string.
        stylesheet
            .to_css(lightningcss::printer::PrinterOptions {
                minify: true,
                ..Default::default()
            })
            .map(|res| res.code)
            .map_err(|e| GrimoireCssError::InvalidInput(format!("Failed to generate CSS: {e}")))
    }
}
