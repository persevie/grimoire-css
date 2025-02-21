use std::path::Path;

use crate::{
    commands::init::init,
    core::{
        CompiledCssInMemory, ConfigInMemory, CssBuilderFs, CssBuilderInMemory, CssOptimizer,
        GrimoireCssError,
    },
};

/// Initiates the CSS build process using the provided optimizer and configuration.
///
/// This function initializes the CSS configuration by invoking the `init` command, then constructs
/// a `CSSBuilder` with the given configuration and optimizer. Finally, it triggers the build process
/// to generate the optimized CSS output.
///
/// # Arguments
///
/// * `current_dir` - A reference to the current directory path where operations should be performed.
/// * `css_optimizer` - A reference to an implementation of the `CSSOptimizer` trait used during the build process.
///
/// # Returns
///
/// * `Ok(())` - If the build process completes successfully.
/// * `Err(GrimoireCSSError)` - If any step in the initialization or build process fails.
///
/// # Errors
///
/// Returns a `GrimoireCSSError` if the initialization of the configuration or the CSS build process fails.
pub fn build<O: CssOptimizer>(
    current_dir: &Path,
    css_optimizer: &O,
    mode: &str,
) -> Result<(), GrimoireCssError> {
    let config = init(current_dir, mode)?;

    CssBuilderFs::new(&config, current_dir, css_optimizer)?.build()
}

pub fn build_in_memory<O: CssOptimizer>(
    css_optimizer: &O,
    config_in_memory: &ConfigInMemory,
) -> Result<Vec<CompiledCssInMemory>, GrimoireCssError> {
    CssBuilderInMemory::new(config_in_memory, css_optimizer)?.build()
}
