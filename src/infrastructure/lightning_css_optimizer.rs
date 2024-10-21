use lightningcss::{
    stylesheet::{MinifyOptions, ParserOptions, StyleSheet},
    targets::{Browsers, Targets},
};
use std::{env, fs, path::Path};

use crate::{
    buffer::add_message,
    core::{CSSOptimizer, GrimoireCSSError},
};

/// An implementation of `CSSOptimizer` that uses the `lightningcss` library to optimize and minify CSS.
///
/// This struct is responsible for parsing raw CSS, applying optimizations, and producing the
/// final optimized output. It relies on the `lightningcss` library to perform the actual
/// CSS transformations.
pub struct LightningCSSOptimizer {
    targets: Targets,
}

impl LightningCSSOptimizer {
    /// Initializes a new `LightningCSSOptimizer` instance, setting up the environment for CSS optimization.
    ///
    /// This function ensures the existence of the `.browserslistrc` file in the given directory and configures
    /// the environment by setting the `BROWSERSLIST_CONFIG` variable. The optimizer then loads the browser targets
    /// based on the configuration in `.browserslistrc`.
    ///
    /// # Arguments
    ///
    /// * `current_dir` - The directory where the `.browserslistrc` file is expected to be located.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if there's a failure in initializing the optimizer.
    pub fn new(current_dir: &Path) -> Result<Self, GrimoireCSSError> {
        let browserslist_config_path = current_dir.join(".browserslistrc");

        // Ensure `.browserslistrc` exists with default configuration if missing.
        if !browserslist_config_path.exists() {
            fs::write(&browserslist_config_path, "defaults")
                .expect("Failed to create .browserslistrc with defaults");

            add_message(
                ".browserslistrc file was missing and has been created with 'defaults'."
                    .to_string(),
            );
        }

        // Set the `BROWSERSLIST_CONFIG` environment variable to point to the `.browserslistrc`.
        env::set_var("BROWSERSLIST_CONFIG", &browserslist_config_path);

        // Load browser targets based on the `.browserslistrc` configuration.
        let browsers = Browsers::load_browserslist().expect("Failed to load browserslist");

        // Define targets for minification and optimization based on loaded browsers.
        let targets = Targets {
            browsers,
            include: Default::default(),
            exclude: Default::default(),
        };

        Ok(Self { targets })
    }
}

impl CSSOptimizer for LightningCSSOptimizer {
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
    fn optimize(&self, raw_css: &str) -> Result<String, GrimoireCSSError> {
        let mut stylesheet =
            StyleSheet::parse(raw_css, ParserOptions::default()).expect("Failed to parse CSS");

        // Apply minification and optimization based on the browser targets.
        stylesheet
            .minify(MinifyOptions {
                targets: self.targets,
                unused_symbols: Default::default(),
            })
            .expect("Failed to minify CSS");

        // Generate the final CSS as a string.
        let res = stylesheet
            .to_css(lightningcss::printer::PrinterOptions {
                minify: true,
                ..Default::default()
            })
            .expect("Failed to generate CSS");

        Ok(res.code)
    }
}
