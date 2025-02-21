//! File system operations for the Grimoire CSS system.
//!
//! This module provides a unified interface for filesystem operations such as:
//! - Managing configuration directories and files
//! - Creating and maintaining the Grimoire CSS directory structure
//! - Handling path resolution for configuration and other resources
//!
//! The directory structure follows the convention:
//! ```text
//! ./grimoire/
//!   └── config/
//!       └── grimoire.config.json
//! ```

use super::GrimoireCssError;
use crate::buffer::add_message;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Provides filesystem operations for the Grimoire CSS system.
///
/// This struct implements methods for managing the file system structure
/// required by Grimoire CSS, including configuration directories and files.
pub struct Filesystem;

impl Filesystem {
    /// Retrieves or creates the path for the configuration file.
    ///
    /// This method ensures that the necessary directory structure exists
    /// and returns the path to the grimoire.config.json file.
    ///
    /// # Arguments
    ///
    /// * `current_dir` - The base directory path to create the configuration in
    ///
    /// # Returns
    ///
    /// Returns a `PathBuf` pointing to the configuration file location
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if:
    /// - The directory structure cannot be created
    /// - File system permissions prevent access
    pub fn get_config_path(current_dir: &Path) -> Result<PathBuf, GrimoireCssError> {
        let grimoire_dir = Self::get_or_create_grimoire_path(current_dir)?;
        let config_path = grimoire_dir.join("config");
        if !config_path.exists() {
            fs::create_dir(&config_path)?;
        }
        Ok(config_path.join("grimoire.config.json"))
    }

    /// Gets or creates the path for the GrimoireCSS folder.
    ///
    /// This method ensures the existence of the Grimoire directory structure.
    /// If the directories don't exist, it creates them and adds a success message
    /// to the notification buffer.
    ///
    /// # Arguments
    ///
    /// * `cwd` - The current working directory where the Grimoire folder should be created
    ///
    /// # Returns
    ///
    /// Returns a `PathBuf` pointing to the Grimoire directory
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if:
    /// - Directory creation fails
    /// - File system operations are not permitted
    pub fn get_or_create_grimoire_path(cwd: &Path) -> Result<PathBuf, GrimoireCssError> {
        let grimoire_path = cwd.join("grimoire");
        if !grimoire_path.exists() {
            fs::create_dir(&grimoire_path)?;
            let config_path = grimoire_path.join("config");
            if !config_path.exists() {
                fs::create_dir(&config_path)?;
            }
            add_message(format!(
                "Configuration and directories created successfully at `{}`.",
                "./grimoire"
            ));
        }
        Ok(grimoire_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_get_or_create_grimoire_path_creates_directory() {
        let temp_dir = tempdir().unwrap();
        let cwd = temp_dir.path();

        let grimoire_path = cwd.join("grimoire");
        assert!(!grimoire_path.exists());

        let result = Filesystem::get_or_create_grimoire_path(cwd)
            .expect("Failed to get or create grimoire path");
        assert_eq!(result, grimoire_path);

        assert!(grimoire_path.exists());
        assert!(grimoire_path.is_dir());

        let config_path = grimoire_path.join("config");
        assert!(config_path.exists());
        assert!(config_path.is_dir());
    }

    #[test]
    fn test_get_or_create_grimoire_path_does_not_create_if_exists() {
        let temp_dir = tempdir().unwrap();
        let cwd = temp_dir.path();

        let grimoire_path = cwd.join("grimoire");
        let config_path = grimoire_path.join("config");
        fs::create_dir(&grimoire_path).unwrap();
        fs::create_dir(&config_path).unwrap();

        let result = Filesystem::get_or_create_grimoire_path(cwd)
            .expect("Failed to get or create grimoire path");
        assert_eq!(result, grimoire_path);

        assert!(grimoire_path.exists());
        assert!(grimoire_path.is_dir());
        assert!(config_path.exists());
        assert!(config_path.is_dir());
    }

    #[test]
    fn test_get_config_path_creates_config_file_path() {
        let temp_dir = tempdir().unwrap();
        let cwd = temp_dir.path();

        let result = Filesystem::get_config_path(cwd).expect("Failed to get or create config path");

        let expected_path = cwd.join("grimoire/config/grimoire.config.json");
        assert_eq!(result, expected_path);

        let grimoire_path = cwd.join("grimoire");
        let config_dir = grimoire_path.join("config");
        assert!(grimoire_path.exists());
        assert!(grimoire_path.is_dir());
        assert!(config_dir.exists());
        assert!(config_dir.is_dir());
    }

    #[test]
    fn test_get_config_path_does_not_create_if_exists() {
        let temp_dir = tempdir().unwrap();
        let cwd = temp_dir.path();

        let grimoire_path = cwd.join("grimoire");
        let config_dir = grimoire_path.join("config");
        let config_file_path = config_dir.join("grimoire.config.json");
        fs::create_dir(&grimoire_path).unwrap();
        fs::create_dir(&config_dir).unwrap();
        fs::write(&config_file_path, b"{}").unwrap();

        let result = Filesystem::get_config_path(cwd).expect("Failed to get or create config path");
        assert_eq!(result, config_file_path);

        assert!(grimoire_path.exists());
        assert!(grimoire_path.is_dir());
        assert!(config_dir.exists());
        assert!(config_dir.is_dir());
        assert!(config_file_path.exists());
    }
}
