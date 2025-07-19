use crate::{
    buffer::add_message,
    core::{ConfigFs, GrimoireCssError},
};
use std::path::Path;

/// Initializes the Grimoire CSS configuration by loading an existing config or creating a new one if necessary.
///
/// This function attempts to load the configuration from the given directory. If the configuration file
/// already exists, it is loaded and returned. If the file is missing or the loading process encounters
/// an error other than a parsing failure, a new configuration is created and saved to the directory.
///
/// # Arguments
///
/// * `current_dir` - The directory where the configuration file is expected to be found or created.
/// * `mode` - The mode in which the function operates. If mode is "init", it will notify if the config already exists.
///
/// # Returns
///
/// * `Ok(Config)` - If the configuration is successfully loaded or created.
/// * `Err(GrimoireCSSError)` - If there is an error during loading or saving the configuration.
pub fn init(current_dir: &Path, mode: &str) -> Result<ConfigFs, GrimoireCssError> {
    match ConfigFs::load(current_dir) {
        Ok(config) => {
            if mode == "init" {
                add_message(format!(
                    "Configuration file already exists at {}.",
                    current_dir.display()
                ));
            }
            Ok(config)
        }
        Err(err) => match err {
            GrimoireCssError::Serde(_) => {
                let err_msg = format!("Failed to parse config. {err}");
                Err(GrimoireCssError::InvalidInput(err_msg))
            }
            _ => {
                let config = ConfigFs::default();
                ConfigFs::save(&config, current_dir)?;
                Ok(config)
            }
        },
    }
}
