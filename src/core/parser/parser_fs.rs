//! This module defines the `ParserFs` struct, which extends the base Parser
//! with filesystem-specific functionality for collecting CSS classes from files and directories.

use super::Parser;
use crate::core::SourceFile;
use crate::{buffer::add_message, core::GrimoireCssError};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

type Span = (usize, usize);
type ClassWithSpan = (String, Span);
type ClassesWithSpans = Vec<ClassWithSpan>;

type SingleOutputFileClasses = (PathBuf, ClassesWithSpans);
type MultipleOutputFileClasses = (PathBuf, PathBuf, ClassesWithSpans);

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
    /// A vector of tuples containing file path and found classes with spans.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if there is an issue reading any of the files.
    pub fn collect_classes_single_output(
        &self,
        input_paths: &Vec<String>,
    ) -> Result<Vec<SingleOutputFileClasses>, GrimoireCssError> {
        let mut results = Vec::new();
        let mut seen_class_names: HashSet<String> = HashSet::new();

        for input_path in input_paths {
            let path = self.current_dir.join(input_path);
            self.collect_spells_from_path(&path, &mut results, &mut seen_class_names)?;
        }

        Ok(results)
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
    /// A vector of tuples: (OutputCssPath, InputSourcePath, ClassesWithSpans).
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if any file or directory cannot be processed.
    #[allow(dead_code)]
    pub fn collect_classes_multiple_output(
        &self,
        input_paths: &Vec<String>,
        output_dir_path: &Path,
    ) -> Result<Vec<MultipleOutputFileClasses>, GrimoireCssError> {
        let mut res = Vec::new();

        for input_path_string in input_paths {
            let path = self.current_dir.join(input_path_string);

            if path.is_file() {
                let mut class_names = Vec::new();
                let mut seen_class_names: HashSet<String> = HashSet::new();

                let output_file_path = path.with_extension("css");
                let bundle_output_full_path =
                    output_dir_path.join(output_file_path.file_name().ok_or_else(|| {
                        GrimoireCssError::InvalidPath(output_file_path.to_string_lossy().into())
                    })?);

                let file_content = fs::read_to_string(&path)?;
                if let Err(e) = self.base_parser.collect_candidates(
                    &file_content,
                    &mut class_names,
                    &mut seen_class_names,
                ) {
                    let src = Arc::new(SourceFile::new(
                        Some(path.clone()),
                        path.to_string_lossy().to_string(),
                        file_content.clone(),
                    ));
                    return Err(e.with_source(src));
                }

                res.push((bundle_output_full_path, path, class_names));
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

    /// Streaming variant of `collect_classes_multiple_output`.
    ///
    /// Calls `visitor` once per input file instead of building a large in-memory vector.
    pub fn for_each_classes_multiple_output<F>(
        &self,
        input_paths: &Vec<String>,
        output_dir_path: &Path,
        mut visitor: F,
    ) -> Result<(), GrimoireCssError>
    where
        F: FnMut(PathBuf, PathBuf, ClassesWithSpans) -> Result<(), GrimoireCssError>,
    {
        for input_path_string in input_paths {
            let path = self.current_dir.join(input_path_string);
            self.visit_classes_multiple_output_path(&path, output_dir_path, &mut visitor)?;
        }

        Ok(())
    }

    fn visit_classes_multiple_output_path<F>(
        &self,
        path: &Path,
        output_dir_path: &Path,
        visitor: &mut F,
    ) -> Result<(), GrimoireCssError>
    where
        F: FnMut(PathBuf, PathBuf, ClassesWithSpans) -> Result<(), GrimoireCssError>,
    {
        if path.is_file() {
            let mut class_names = Vec::new();
            let mut seen_class_names: HashSet<String> = HashSet::new();

            let output_file_path = path.with_extension("css");
            let bundle_output_full_path =
                output_dir_path.join(output_file_path.file_name().ok_or_else(|| {
                    GrimoireCssError::InvalidPath(output_file_path.to_string_lossy().into())
                })?);

            let file_content = fs::read_to_string(path)?;
            if let Err(e) = self.base_parser.collect_candidates(
                &file_content,
                &mut class_names,
                &mut seen_class_names,
            ) {
                let src = Arc::new(SourceFile::new(
                    Some(path.to_path_buf()),
                    path.to_string_lossy().to_string(),
                    file_content.clone(),
                ));
                return Err(e.with_source(src));
            }

            visitor(bundle_output_full_path, path.to_path_buf(), class_names)?;
            return Ok(());
        }

        if path.is_dir() {
            let entries = self.get_sorted_directory_entries(path)?;
            for entry in entries {
                self.visit_classes_multiple_output_path(&entry, output_dir_path, visitor)?;
            }
            return Ok(());
        }

        add_message(format!("Invalid path: {}", path.display()));
        Ok(())
    }

    /// Recursively collects CSS class names or templated spells from a given file or directory path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the file or directory to process.
    /// * `results` - A mutable reference to a vector that stores the collected results.
    /// * `seen_class_names` - A mutable reference to a HashSet for tracking seen class names.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if there is an issue reading a file.
    fn collect_spells_from_path(
        &self,
        path: &Path,
        results: &mut Vec<SingleOutputFileClasses>,
        seen_class_names: &mut HashSet<String>,
    ) -> Result<(), GrimoireCssError> {
        if path.is_file() {
            let file_content = fs::read_to_string(path)?;
            let mut class_names = Vec::new();

            if let Err(e) = self.base_parser.collect_candidates(
                &file_content,
                &mut class_names,
                seen_class_names,
            ) {
                let src = Arc::new(SourceFile::new(
                    Some(path.to_path_buf()),
                    path.to_string_lossy().to_string(),
                    file_content.clone(),
                ));
                return Err(e.with_source(src));
            }

            if !class_names.is_empty() {
                results.push((path.to_path_buf(), class_names));
            }
        } else if path.is_dir() {
            let entries = &self.get_sorted_directory_entries(path)?;

            for entry in entries {
                self.collect_spells_from_path(entry, results, seen_class_names)?;
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
        Ok(raw_spells.into_iter().map(|(s, _)| s).collect())
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
        let results = result.unwrap();
        assert_eq!(results.len(), 1);
        let (path, classes) = &results[0];
        assert_eq!(path, &test_file);
        assert_eq!(classes.len(), 3);

        let class_names: Vec<String> = classes.iter().map(|(n, _)| n.clone()).collect();
        assert!(class_names.contains(&"test1".to_string()));
        assert!(class_names.contains(&"test2".to_string()));
        assert!(class_names.contains(&"test3".to_string()));
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
        let (_, _, classes1) = &outputs[0];
        assert_eq!(classes1.len(), 1);
        assert_eq!(classes1[0].0, "file1-class");

        // Check second file output
        let (_, _, classes2) = &outputs[1];
        assert_eq!(classes2.len(), 1);
        assert_eq!(classes2[0].0, "file2-class");
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
        let results = result.unwrap();
        assert_eq!(results.len(), 1);
        let (_, classes) = &results[0];
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].0, "nested-class");
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

    #[test]
    fn test_collect_classes_single_output_dedups_across_files() {
        let temp_dir = tempdir().unwrap();
        let a = temp_dir.path().join("a.html");
        let b = temp_dir.path().join("b.html");

        // Same token appears in both files.
        fs::write(&a, r#"<div class="h=10px"></div>"#).unwrap();
        fs::write(&b, r#"<div class="h=10px"></div>"#).unwrap();

        let parser = ParserFs::new(temp_dir.path());
        let input_paths = vec!["a.html".to_string(), "b.html".to_string()];

        let results = parser.collect_classes_single_output(&input_paths).unwrap();

        // Dedup is global across all inputs in single-output mode.
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, a);
        assert_eq!(results[0].1.len(), 1);
        assert_eq!(results[0].1[0].0, "h=10px".to_string());
    }
}
