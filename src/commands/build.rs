use std::path::Path;

use crate::{
    buffer::add_message,
    commands::init::init,
    core::{
        CompiledCssInMemory, ConfigInMemory, CssBuilderFs, CssBuilderInMemory, CssOptimizer,
        GrimoireCssError,
    },
};

#[derive(Debug, Clone, Copy, Default)]
pub struct BuildOptions {
    /// If true, build will proceed even when config version mismatches the current binary,
    /// and `version` will be updated.
    pub force_version_update: bool,
}

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
    options: BuildOptions,
) -> Result<(), GrimoireCssError> {
    let config = init(current_dir, mode)?;

    let current_version = env!("CARGO_PKG_VERSION");
    let config_version = config.grimoire_css_version.as_deref();

    let mismatch = match config_version {
        Some(v) => v != current_version,
        None => true,
    };

    if mismatch {
        let message = match config_version {
            Some(v) => format!("Config version mismatch: version={v}, current={current_version}."),
            None => format!("Config has no version; current={current_version}."),
        };

        if options.force_version_update {
            add_message(format!(
                "{message} Proceeding due to --force-version-update and updating config."
            ));
            crate::core::ConfigFs::update_config_version_only(current_dir, current_version)?;
        } else {
            add_message(format!(
                "{message} Run `grimoire_css init` to update, or pass --force-version-update to build anyway and update the config."
            ));
            return Err(GrimoireCssError::InvalidInput(message));
        }
    }

    CssBuilderFs::new(&config, current_dir, css_optimizer)?.build()
}

pub fn build_in_memory<O: CssOptimizer>(
    css_optimizer: &O,
    config_in_memory: &ConfigInMemory,
) -> Result<Vec<CompiledCssInMemory>, GrimoireCssError> {
    CssBuilderInMemory::new(config_in_memory, css_optimizer)?.build()
}
