//! This module defines the `Parser` struct, which is responsible for collecting CSS class names
//! and templated spells from HTML or other input files. It can traverse directories, parse files,
//! and extract class names based on specific patterns, using regular expressions.

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use regex::Regex;

use crate::buffer::add_message;

use super::GrimoireCSSError;

/// `Parser` is responsible for extracting CSS class names and templated spells from HTML or other
/// files. It uses regular expressions to find class names and spell-like patterns, and supports
/// processing both single files and entire directories.
///
/// # Fields
///
/// * `current_dir` - The base directory used for resolving relative paths.
/// * `tepmplated_spell_regex` - Regex for detecting templated spells (starting with `g!` and ending with `;`).
/// * `class_name_regex` - Regex for detecting `className` attributes in JSX-like syntax.
/// * `class_regex` - Regex for detecting `class` attributes in HTML-like syntax.
pub struct Parser<'a> {
    current_dir: &'a Path,
    tepmplated_spell_regex: Regex,
    class_name_regex: Regex,
    class_regex: Regex,
}

impl<'a> Parser<'a> {
    /// Creates a new `Parser` instance with predefined regular expressions for extracting class names
    /// and templated spells.
    ///
    /// # Arguments
    ///
    /// * `current_dir` - A reference to the base directory where the parser will operate.
    pub fn new(current_dir: &'a Path) -> Self {
        let class_name_regex = Regex::new(r#"className=["|'|`](.*)["|'|`]"#).unwrap();
        let class_regex = Regex::new(r#"class=["|'|`](.*)["|'|`]"#).unwrap();
        let tepmplated_spell_regex = Regex::new(r#"(g!\S*?;)"#).unwrap();

        Self {
            current_dir,
            tepmplated_spell_regex,
            class_name_regex,
            class_regex,
        }
    }

    /// Collects all class names and templated spells from a set of input files, storing them in a vector.
    ///
    /// This function processes multiple input paths and collects class names or templated spells
    /// found in each file. It ensures uniqueness of collected classes using a `HashSet` and returns
    /// a list of class names.
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
    ) -> Result<Vec<String>, GrimoireCSSError> {
        let mut class_names: Vec<String> = Vec::new();
        let mut seen_class_names: HashSet<String> = HashSet::new();

        for input_path in input_paths {
            let path = self.current_dir.join(input_path);
            self.collect_spells_from_path(&path, &mut class_names, &mut seen_class_names)?;
        }

        Ok(class_names)
    }

    /// Recursively collects CSS class names or templated spells from a given file or directory path.
    ///
    /// If the path is a file, it reads the file and collects classes. If it's a directory, it
    /// recursively processes each file in the directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the file or directory to process.
    /// * `class_names` - A mutable reference to a vector that stores the collected class names.
    /// * `seen_class_names` - A mutable reference to a `HashSet` for tracking seen class names to ensure uniqueness.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if there is an issue reading a file.
    fn collect_spells_from_path(
        &self,
        path: &Path,
        class_names: &mut Vec<String>,
        seen_class_names: &mut HashSet<String>,
    ) -> Result<(), GrimoireCSSError> {
        if path.is_file() {
            let file_content = fs::read_to_string(path)?;
            self.collect_candidates(&file_content, class_names, seen_class_names)?;
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

    fn collect_candidates(
        &self,
        file_content: &str,
        class_names: &mut Vec<String>,
        seen_class_names: &mut HashSet<String>,
    ) -> Result<(), GrimoireCSSError> {
        let whitespace_splitter = |input: &str| {
            input
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
        };

        // Collect all 'className' matches
        Self::collect_classes::<fn(&str) -> bool, fn(&str) -> Vec<String>>(
            file_content,
            &self.class_name_regex,
            None,
            Some(whitespace_splitter),
            class_names,
            seen_class_names,
        )?;

        // Collect all 'class' matches
        Self::collect_classes::<fn(&str) -> bool, fn(&str) -> Vec<String>>(
            file_content,
            &self.class_regex,
            None,
            Some(whitespace_splitter),
            class_names,
            seen_class_names,
        )?;

        // Collect all 'templated class' (starts with 'g!', ends with ';') matches
        Self::collect_classes::<fn(&str) -> bool, fn(&str) -> Vec<String>>(
            file_content,
            &self.tepmplated_spell_regex,
            None,
            None,
            class_names,
            seen_class_names,
        )?;

        Ok(())
    }

    /// Collects class names based on the given regular expression and optional predicate/splitter functions.
    ///
    /// This is a utility function that runs a regex search over the file content and optionally
    /// filters or splits the results before adding them to the collection of class names.
    ///
    /// # Arguments
    ///
    /// * `file_content` - The content of the file to be parsed.
    /// * `regex` - A regular expression used to search for class names.
    /// * `predicate` - An optional function used to filter the results.
    /// * `splitter` - An optional function used to split the result into multiple class names.
    /// * `class_names` - A mutable reference to a vector to store the collected class names.
    /// * `seen_class_names` - A mutable reference to a `HashSet` used to track seen class names.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if there is an issue during processing.
    fn collect_classes<P, S>(
        file_content: &str,
        regex: &Regex,
        mut predicate: Option<P>,
        mut splitter: Option<S>,
        class_names: &mut Vec<String>,
        seen_class_names: &mut HashSet<String>,
    ) -> Result<(), GrimoireCSSError>
    where
        P: FnMut(&str) -> bool,
        S: FnMut(&str) -> Vec<String>,
    {
        for cap in regex.captures_iter(file_content) {
            let classes = if let Some(splitter_fn) = &mut splitter {
                splitter_fn(&cap[1])
            } else {
                vec![cap[1].to_string()]
            };

            for class in classes {
                let should_include = predicate.as_mut().map_or(true, |p| p(&class));

                if should_include && !seen_class_names.contains(&class) {
                    seen_class_names.insert(class.clone());
                    class_names.push(class);
                }
            }
        }

        Ok(())
    }

    /// Retrieves and sorts all entries in a given directory.
    ///
    /// This method reads the contents of a directory, collects the paths of all files and subdirectories,
    /// and sorts them to ensure deterministic processing order.
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
    fn get_sorted_directory_entries(&self, path: &Path) -> Result<Vec<PathBuf>, GrimoireCSSError> {
        let mut entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, _>>()?;

        entries.sort_unstable();

        Ok(entries)
    }

    /// Collects class names or templated spells from multiple input paths, producing multiple outputs.
    ///
    /// This method processes multiple input files or directories, and for each file found, it collects
    /// the class names or spells and associates them with an output CSS file path. It handles both single
    /// files and directories recursively.
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
    ) -> Result<Vec<(PathBuf, Vec<String>)>, GrimoireCSSError> {
        let mut res: Vec<(PathBuf, Vec<String>)> = Vec::new();

        for input_path_string in input_paths {
            let path = self.current_dir.join(input_path_string);

            if path.is_file() {
                let mut class_names: Vec<String> = Vec::new();
                let mut seen_class_names: HashSet<String> = HashSet::new();

                let output_file_path = path.with_extension("css");
                let bundle_output_full_path =
                    output_dir_path.join(output_file_path.file_name().ok_or_else(|| {
                        GrimoireCSSError::InvalidPath(output_file_path.to_string_lossy().into())
                    })?);

                let file_content = fs::read_to_string(path)?;
                self.collect_candidates(&file_content, &mut class_names, &mut seen_class_names)?;

                res.push((bundle_output_full_path, class_names));
            } else if path.is_dir() {
                let entries = &self.get_sorted_directory_entries(&path)?;

                let dir_entry_strings = entries
                    .iter()
                    .map(|p| {
                        p.strip_prefix(self.current_dir)
                            .unwrap_or(p)
                            .to_string_lossy()
                            .into()
                    })
                    .collect::<Vec<String>>();

                self.collect_classes_multiple_output(&dir_entry_strings, output_dir_path)?;
            } else {
                add_message(format!("Invalid path: {}", path.display()));
            }
        }

        Ok(res)
    }
}
