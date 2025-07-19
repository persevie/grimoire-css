//! This module defines the `ParserFs` struct, which extends the base Parser
//! with filesystem-specific functionality for collecting CSS classes from files and directories.

use super::Parser;
use crate::{buffer::add_message, core::GrimoireCssError};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

/// `ParserFs` extends the base `Parser` with filesystem-specific functionality.
/// It handles file reading, directory traversal, and path resolution.
pub struct ParserFs {
    current_dir: PathBuf,
    base_parser: Parser,
}

impl ParserFs {
    /// Creates a new `ParserFs` instance.
    ///
    /// # Arguments
    ///
    /// * `current_dir` - The base directory where the parser will operate.
    pub fn new(current_dir: &Path) -> Self {
        Self {
            current_dir: current_dir.to_path_buf(),
            base_parser: Parser::new(),
        }
    }

    /// Collects all class names and templated spells from a set of input files, storing them in a vector.
    ///
    /// # Arguments
    ///
    /// * `input_paths` - A vector of file paths (as strings) relative to the `current_dir`.
    ///
    /// # Returns
    ///
    /// A vector of unique class names found in the input files.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if there is an issue reading any of the files.
    pub fn collect_classes_single_output(
        &self,
        input_paths: &Vec<String>,
    ) -> Result<Vec<String>, GrimoireCssError> {
        let mut class_names: Vec<String> = Vec::new();
        let mut seen_class_names: HashSet<String> = HashSet::new();

        for input_path in input_paths {
            let path = self.current_dir.join(input_path);
            self.collect_spells_from_path(&path, &mut class_names, &mut seen_class_names)?;
        }

        Ok(class_names)
    }

    /// Collects class names or templated spells from multiple input paths, producing multiple outputs.
    ///
    /// # Arguments
    ///
    /// * `input_paths` - A vector of file paths to process.
    /// * `output_dir_path` - The directory where the output CSS files will be placed.
    ///
    /// # Returns
    ///
    /// A vector of tuples, where each tuple contains the path to the output CSS file and a vector of class names.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if any file or directory cannot be processed.
    pub fn collect_classes_multiple_output(
        &self,
        input_paths: &Vec<String>,
        output_dir_path: &Path,
    ) -> Result<Vec<(PathBuf, Vec<String>)>, GrimoireCssError> {
        let mut res: Vec<(PathBuf, Vec<String>)> = Vec::new();

        for input_path_string in input_paths {
            let path = self.current_dir.join(input_path_string);

            if path.is_file() {
                let mut class_names: Vec<String> = Vec::new();
                let mut seen_class_names: HashSet<String> = HashSet::new();

                let output_file_path = path.with_extension("css");
                let bundle_output_full_path =
                    output_dir_path.join(output_file_path.file_name().ok_or_else(|| {
                        GrimoireCssError::InvalidPath(output_file_path.to_string_lossy().into())
                    })?);

                let file_content = fs::read_to_string(&path)?;
                self.base_parser.collect_candidates(
                    &file_content,
                    &mut class_names,
                    &mut seen_class_names,
                )?;

                res.push((bundle_output_full_path, class_names));
            } else if path.is_dir() {
                let entries = &self.get_sorted_directory_entries(&path)?;

                let dir_entry_strings = entries
                    .iter()
                    .map(|p| {
                        p.strip_prefix(&self.current_dir)
                            .unwrap_or(p)
                            .to_string_lossy()
                            .into()
                    })
                    .collect::<Vec<String>>();

                res.extend(
                    self.collect_classes_multiple_output(&dir_entry_strings, output_dir_path)?,
                );
            } else {
                add_message(format!("Invalid path: {}", path.display()));
            }
        }

        Ok(res)
    }

    /// Recursively collects CSS class names or templated spells from a given file or directory path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the file or directory to process.
    /// * `class_names` - A mutable reference to a vector that stores the collected class names.
    /// * `seen_class_names` - A mutable reference to a HashSet for tracking seen class names.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if there is an issue reading a file.
    fn collect_spells_from_path(
        &self,
        path: &Path,
        class_names: &mut Vec<String>,
        seen_class_names: &mut HashSet<String>,
    ) -> Result<(), GrimoireCssError> {
        if path.is_file() {
            let file_content = fs::read_to_string(path)?;
            self.base_parser
                .collect_candidates(&file_content, class_names, seen_class_names)?;
        } else if path.is_dir() {
            let entries = &self.get_sorted_directory_entries(path)?;

            for entry in entries {
                self.collect_spells_from_path(entry, class_names, seen_class_names)?;
            }
        } else {
            add_message(format!("Invalid path: {}", path.display()));
        }

        Ok(())
    }

    /// Retrieves and sorts all entries in a given directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The directory path to read.
    ///
    /// # Returns
    ///
    /// A sorted vector of paths found in the directory.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if there is an issue reading the directory.
    fn get_sorted_directory_entries(&self, path: &Path) -> Result<Vec<PathBuf>, GrimoireCssError> {
        let mut entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, _>>()?;

        entries.sort();

        Ok(entries)
    }

    pub fn collect_raw_spells(
        &self,
        content: &str,
    ) -> Result<Vec<String>, crate::core::GrimoireCssError> {
        let mut raw_spells = Vec::new();
        let mut seen = std::collections::HashSet::new();
        self.base_parser
            .collect_candidates(content, &mut raw_spells, &mut seen)?;
        Ok(raw_spells)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self};
    use tempfile::tempdir;

    #[test]
    fn test_collect_classes_single_output() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.html");
        fs::write(
            &test_file,
            r#"<div class="test1 test2"></div><div className="test3"></div>"#,
        )
        .unwrap();

        let parser = ParserFs::new(temp_dir.path());
        let result =
            parser.collect_classes_single_output(&vec![test_file.to_str().unwrap().to_string()]);

        assert!(result.is_ok());
        let classes = result.unwrap();
        assert_eq!(classes.len(), 3);
        assert!(classes.contains(&"test1".to_string()));
        assert!(classes.contains(&"test2".to_string()));
        assert!(classes.contains(&"test3".to_string()));
    }

    #[test]
    fn test_collect_classes_multiple_output() {
        let temp_dir = tempdir().unwrap();
        let test_file1 = temp_dir.path().join("test1.html");
        let test_file2 = temp_dir.path().join("test2.html");

        fs::write(&test_file1, r#"<div class="file1-class"></div>"#).unwrap();
        fs::write(&test_file2, r#"<div class="file2-class"></div>"#).unwrap();

        let parser = ParserFs::new(temp_dir.path());
        let output_dir = temp_dir.path().join("output");
        fs::create_dir(&output_dir).unwrap();

        let result = parser.collect_classes_multiple_output(
            &vec![
                test_file1.to_str().unwrap().to_string(),
                test_file2.to_str().unwrap().to_string(),
            ],
            &output_dir,
        );

        assert!(result.is_ok());
        let outputs = result.unwrap();
        assert_eq!(outputs.len(), 2);

        // Check first file output
        assert_eq!(outputs[0].1.len(), 1);
        assert!(outputs[0].1.contains(&"file1-class".to_string()));

        // Check second file output
        assert_eq!(outputs[1].1.len(), 1);
        assert!(outputs[1].1.contains(&"file2-class".to_string()));
    }

    #[test]
    fn test_collect_from_directory() {
        let temp_dir = tempdir().unwrap();
        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        fs::write(
            sub_dir.join("test.html"),
            r#"<div class="nested-class"></div>"#,
        )
        .unwrap();

        let parser = ParserFs::new(temp_dir.path());
        let result =
            parser.collect_classes_single_output(&vec![sub_dir.to_str().unwrap().to_string()]);

        assert!(result.is_ok());
        let classes = result.unwrap();
        assert_eq!(classes.len(), 1);
        assert!(classes.contains(&"nested-class".to_string()));
    }

    #[test]
    fn test_invalid_path() {
        let temp_dir = tempdir().unwrap();
        let parser = ParserFs::new(temp_dir.path());
        let result = parser.collect_classes_single_output(&vec!["nonexistent.html".to_string()]);
        println!("{result:?}");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_empty_input() {
        let temp_dir = tempdir().unwrap();
        let parser = ParserFs::new(temp_dir.path());
        let result = parser.collect_classes_single_output(&vec!["empty.html".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
