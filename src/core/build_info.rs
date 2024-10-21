//! This module defines the `BuildInfo` struct, which encapsulates information about the build process.
//!
//! The `BuildInfo` struct is used to track the output file path and associated spells that are generated
//! during the CSS build process. This information is essential for managing the state of the build and
//! ensuring that all necessary CSS files and components are correctly produced.

use std::path::PathBuf;

use super::spell::Spell;

/// Represents information about a CSS build output.
///
/// `BuildInfo` contains the output file path and the list of spells that were generated during the
/// build process. This structure is used to store the results of a build, helping to track where
/// the CSS output should be written and which spells were included.
///
/// # Fields
///
/// * `file_path` - The path to the output CSS file.
/// * `spells` - A list of `Spell` objects that represent the CSS transformations or declarations to be included.
#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub file_path: PathBuf,
    pub spells: Vec<Spell>,
}
