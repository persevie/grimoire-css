use std::path::Path;

use super::{build::build, init::init};
use crate::core::{CSSOptimizer, GrimoireCSSError};

/// Processes the provided mode and delegates handling to the appropriate functionality.
///
/// This function acts as the entry point for various operational modes such as `init` and `build`.
/// Depending on the mode, it will either initialize the environment or trigger the build process,
/// utilizing the provided `CSSOptimizer` implementation for CSS optimization during the build.
///
/// # Arguments
///
/// * `mode` - A string representing the mode of operation. Supported values are `"init"` and `"build"`.
/// * `current_dir` - A reference to the current directory path where operations should be performed.
/// * `css_optimizer` - A reference to an implementation of the `CSSOptimizer` trait used during the build process.
///
/// # Errors
///
/// Returns a `GrimoireCSSError` if an invalid mode is provided or if an underlying operation (e.g., initialization or build) fails.
pub fn process_mode_and_handle<O: CSSOptimizer>(
    mode: &str,
    current_dir: &Path,
    css_optimizer: &O,
) -> Result<(), GrimoireCSSError> {
    match mode {
        "init" => {
            init(current_dir, mode)?;
        }
        "build" => {
            build(current_dir, css_optimizer, mode)?;
        }
        _ => {
            let err_msg = format!("Unknown mode: {}", mode);
            return Err(GrimoireCSSError::InvalidInput(err_msg));
        }
    }
    Ok(())
}
