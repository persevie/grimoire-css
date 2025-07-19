//! This module provides the configuration management for GrimoireCSS.

use crate::{
    buffer::add_message,
    core::{Filesystem, GrimoireCssError},
};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

/// Represents the main configuration structure for GrimoireCSS.
#[derive(Debug, Clone)]
pub struct ConfigFs {
    pub variables: Option<Vec<(String, String)>>,
    pub scrolls: Option<HashMap<String, Vec<String>>>,
    pub projects: Vec<ConfigFsProject>,
    pub shared: Option<Vec<ConfigFsShared>>,
    pub critical: Option<Vec<ConfigFsCritical>>,
    /// A set of shared spells used across different projects.
    pub shared_spells: HashSet<String>,
    pub lock: Option<bool>,

    pub custom_animations: HashMap<String, String>,
}

/// Shared configuration for GrimoireCSS projects.
#[derive(Debug, Clone)]
pub struct ConfigFsShared {
    pub output_path: String,
    pub styles: Option<Vec<String>>,
    pub css_custom_properties: Option<Vec<ConfigFsCssCustomProperties>>,
}

/// Critical styles configuration to be inlined into specific HTML files.
#[derive(Debug, Clone)]
pub struct ConfigFsCritical {
    pub file_to_inline_paths: Vec<String>,
    pub styles: Option<Vec<String>>,
    pub css_custom_properties: Option<Vec<ConfigFsCssCustomProperties>>,
}

/// Represents custom CSS properties associated with specific elements.
#[derive(Debug, Clone)]
pub struct ConfigFsCssCustomProperties {
    pub element: String,
    pub data_param: String,
    pub data_value: String,
    pub css_variables: Vec<(String, String)>,
}

/// Represents a project in GrimoireCSS.
#[derive(Debug, Clone)]
pub struct ConfigFsProject {
    pub project_name: String,
    pub input_paths: Vec<String>,
    pub output_dir_path: Option<String>,
    pub single_output_file_name: Option<String>,
}

// ---

/// The main struct used to represent the JSON structure of the GrimoireCSS configuration.
///
/// This struct is used internally to serialize and deserialize the configuration data.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConfigFsJSON {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    /// Optional framework-level variables used during compilation.
    pub variables: Option<HashMap<String, String>>,
    /// Optional shared configuration settings used across multiple projects.
    pub scrolls: Option<Vec<ConfigFsScrollJSON>>,
    /// A list of projects included in the configuration.
    pub projects: Vec<ConfigFsProjectJSON>,
    pub shared: Option<Vec<ConfigFsSharedJSON>>,
    pub critical: Option<Vec<ConfigFsCriticalJSON>>,
    pub lock: Option<bool>,
}

/// Represents a scrolls which may contain external or combined CSS rules.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigFsScrollJSON {
    pub name: String,
    pub spells: Vec<String>,
    pub extends: Option<Vec<String>>,
}

/// A struct representing a project within GrimoireCSS.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ConfigFsProjectJSON {
    /// The name of the project.
    pub project_name: String,
    /// A list of input paths for the project.
    pub input_paths: Vec<String>,
    /// Optional output directory path for the project.
    pub output_dir_path: Option<String>,
    /// Optional file name for a single output file.
    pub single_output_file_name: Option<String>,
}

/// Represents shared configuration settings used across multiple projects.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ConfigFsSharedJSON {
    pub output_path: String,
    pub styles: Option<Vec<String>>,
    pub css_custom_properties: Option<Vec<ConfigFsCSSCustomPropertiesJSON>>,
}

/// Represents critical styles configuration for inlining into HTML files.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ConfigFsCriticalJSON {
    pub file_to_inline_paths: Vec<String>,
    pub styles: Option<Vec<String>>,
    pub css_custom_properties: Option<Vec<ConfigFsCSSCustomPropertiesJSON>>,
}

/// Represents a custom CSS property item, including associated variables.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ConfigFsCSSCustomPropertiesJSON {
    /// The optional DOM element (`tag`, `class`, `id`, `:root` (default)) associated with the CSS variables.
    pub element: Option<String>,
    /// A parameter name used within the CSS configuration.
    pub data_param: String,
    /// A value corresponding to the data parameter.
    pub data_value: String,
    /// A set of associated CSS variables and their values.
    pub css_variables: HashMap<String, String>,
}

impl Default for ConfigFs {
    /// Provides a default configuration for `Config`, initializing the `scrolls`, `projects`, and other fields.
    fn default() -> Self {
        let projects = vec![ConfigFsProject {
            project_name: "main".to_string(),
            input_paths: Vec::new(),
            output_dir_path: None,
            single_output_file_name: None,
        }];

        Self {
            scrolls: None,
            shared: None,
            critical: None,
            projects,
            variables: None,
            shared_spells: HashSet::new(),
            custom_animations: HashMap::new(),
            lock: None,
        }
    }
}

impl ConfigFs {
    /// Loads the configuration from the file system.
    ///
    /// Reads a JSON configuration file from the file system and deserializes it into a `Config` object.
    /// Also searches for and loads any external scroll files (grimoire.*.scrolls.json)
    /// and any external variables files (grimoire.*.variables.json).
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if reading or parsing the file fails.
    pub fn load(current_dir: &Path) -> Result<Self, GrimoireCssError> {
        let config_path = Filesystem::get_config_path(current_dir)?;
        let content = fs::read_to_string(&config_path)?;
        let json_config: ConfigFsJSON = serde_json::from_str(&content)?;
        let mut config = Self::from_json(json_config);

        // Load custom animations
        config.custom_animations = Self::find_custom_animations(current_dir)?;

        // Load external scroll files
        config.scrolls = Self::load_external_scrolls(current_dir, config.scrolls)?;

        // Load external variable files
        config.variables = Self::load_external_variables(current_dir, config.variables)?;

        Ok(config)
    }

    /// Saves the current configuration to the file system.
    ///
    /// Serializes the current configuration into JSON format and writes it to the file system.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if writing to the file system fails.
    pub fn save(&self, current_dir: &Path) -> Result<(), GrimoireCssError> {
        let config_path = Filesystem::get_config_path(current_dir)?;
        let json_config = self.to_json();
        let content = serde_json::to_string_pretty(&json_config)?;
        fs::write(&config_path, content)?;

        Ok(())
    }

    /// Extracts common spells from the configuration and adds them to a `HashSet`.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the `ConfigJSON` structure that holds the spells data.
    ///
    /// # Returns
    ///
    /// A `HashSet` of common spell names used across projects.
    fn get_common_spells_set(config: &ConfigFsJSON) -> HashSet<String> {
        let mut common_spells = HashSet::new();

        if let Some(shared) = &config.shared {
            for shared_item in shared {
                if let Some(styles) = &shared_item.styles {
                    common_spells.extend(styles.iter().cloned());
                }
            }
        }

        if let Some(critical) = &config.critical {
            for critical_item in critical {
                if let Some(styles) = &critical_item.styles {
                    common_spells.extend(styles.iter().cloned());
                }
            }
        }

        common_spells
    }

    /// Converts a JSON representation of the configuration into a `Config` instance.
    ///
    /// # Arguments
    ///
    /// * `json_config` - A `ConfigJSON` object representing the deserialized configuration data.
    ///
    /// # Returns
    ///
    /// A new `Config` instance.
    fn from_json(json_config: ConfigFsJSON) -> Self {
        let shared_spells = Self::get_common_spells_set(&json_config);

        let variables = json_config.variables.map(|vars| {
            let mut sorted_vars: Vec<_> = vars.into_iter().collect();
            sorted_vars.sort_by(|a, b| a.0.cmp(&b.0));
            sorted_vars
        });

        let projects = Self::projects_from_json(json_config.projects);

        // Expand glob patterns in shared and critical configurations
        let shared = Self::shared_from_json(json_config.shared);
        let critical = Self::critical_from_json(json_config.critical);
        let scrolls = Self::scrolls_from_json(json_config.scrolls);

        ConfigFs {
            variables,
            scrolls,
            projects,
            shared,
            critical,
            shared_spells,
            custom_animations: HashMap::new(),
            lock: json_config.lock,
        }
    }

    /// Converts shared JSON configuration into internal structure.
    fn shared_from_json(shared: Option<Vec<ConfigFsSharedJSON>>) -> Option<Vec<ConfigFsShared>> {
        shared.map(|shared_vec| {
            shared_vec
                .into_iter()
                .map(|c| ConfigFsShared {
                    output_path: c.output_path,
                    styles: c.styles,
                    css_custom_properties: Self::convert_css_custom_properties_from_json(
                        c.css_custom_properties,
                    ),
                })
                .collect()
        })
    }

    /// Converts critical JSON configuration into internal structure.
    fn critical_from_json(
        critical: Option<Vec<ConfigFsCriticalJSON>>,
    ) -> Option<Vec<ConfigFsCritical>> {
        critical.map(|critical_vec| {
            critical_vec
                .into_iter()
                .map(|c| ConfigFsCritical {
                    file_to_inline_paths: Self::expand_glob_patterns(c.file_to_inline_paths),
                    styles: c.styles,
                    css_custom_properties: Self::convert_css_custom_properties_from_json(
                        c.css_custom_properties,
                    ),
                })
                .collect()
        })
    }

    fn scrolls_from_json(
        scrolls: Option<Vec<ConfigFsScrollJSON>>,
    ) -> Option<HashMap<String, Vec<String>>> {
        scrolls.map(|scrolls_vec| {
            let mut scrolls_map = HashMap::new();

            for scroll in &scrolls_vec {
                let mut scroll_spells = Vec::new();

                // Recursively resolve parent spells
                Self::resolve_spells(scroll, &scrolls_vec, &mut scroll_spells);

                // Add the spells of the current scroll
                scroll_spells.extend_from_slice(&scroll.spells);

                // Insert the resolved spells into the map
                scrolls_map.insert(scroll.name.clone(), scroll_spells);
            }

            scrolls_map
        })
    }

    /// Recursively resolve spells for a given scroll, including extended scrolls
    fn resolve_spells(
        scroll: &ConfigFsScrollJSON,
        scrolls_vec: &[ConfigFsScrollJSON],
        collected_spells: &mut Vec<String>,
    ) {
        if let Some(extends) = &scroll.extends {
            for ext_name in extends {
                // Find the parent scroll
                if let Some(parent_scroll) = scrolls_vec.iter().find(|s| &s.name == ext_name) {
                    // Recursively resolve parent spells if it also extends other scrolls
                    Self::resolve_spells(parent_scroll, scrolls_vec, collected_spells);

                    // Add the spells of the parent scroll
                    collected_spells.extend_from_slice(&parent_scroll.spells);
                }
            }
        }
    }

    /// Converts custom CSS properties from JSON to internal structure.
    fn convert_css_custom_properties_from_json(
        css_custom_properties_vec: Option<Vec<ConfigFsCSSCustomPropertiesJSON>>,
    ) -> Option<Vec<ConfigFsCssCustomProperties>> {
        css_custom_properties_vec.map(|items: Vec<ConfigFsCSSCustomPropertiesJSON>| {
            items
                .into_iter()
                .map(|item| ConfigFsCssCustomProperties {
                    element: item.element.unwrap_or_else(|| String::from(":root")),
                    data_param: item.data_param,
                    data_value: item.data_value,
                    css_variables: {
                        let mut vars: Vec<_> = item.css_variables.into_iter().collect();
                        vars.sort_by(|a, b| a.0.cmp(&b.0));
                        vars
                    },
                })
                .collect()
        })
    }

    /// Converts a list of project JSON configurations to the internal `Project` type.
    fn projects_from_json(projects: Vec<ConfigFsProjectJSON>) -> Vec<ConfigFsProject> {
        projects
            .into_iter()
            .map(|p| {
                let input_paths = Self::expand_glob_patterns(p.input_paths);
                ConfigFsProject {
                    project_name: p.project_name,
                    input_paths,
                    output_dir_path: p.output_dir_path,
                    single_output_file_name: p.single_output_file_name,
                }
            })
            .collect()
    }

    /// Converts the internal `Config` into its JSON representation.
    fn to_json(&self) -> ConfigFsJSON {
        let variables_hash_map = self.variables.as_ref().map(|vars| {
            let mut sorted_vars: Vec<_> = vars.iter().collect();
            sorted_vars.sort_by(|a, b| a.0.cmp(&b.0));
            sorted_vars
                .into_iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect()
        });

        ConfigFsJSON {
            schema: Some("https://raw.githubusercontent.com/persevie/grimoire-css/main/src/core/config/config-schema.json".to_string()),
            variables: variables_hash_map,
            scrolls: Self::scrolls_to_json(self.scrolls.clone()),
            projects: Self::projects_to_json(self.projects.clone()),
            shared: Self::shared_to_json(self.shared.as_ref()),
            critical: Self::critical_to_json(self.critical.as_ref()),
            lock: self.lock,
        }
    }

    /// Converts the internal list of shared configurations into JSON.
    fn shared_to_json(shared: Option<&Vec<ConfigFsShared>>) -> Option<Vec<ConfigFsSharedJSON>> {
        shared.map(|common_vec: &Vec<ConfigFsShared>| {
            common_vec
                .iter()
                .map(|c| ConfigFsSharedJSON {
                    output_path: c.output_path.clone(),
                    styles: c.styles.clone(),
                    css_custom_properties: Self::css_custom_properties_to_json(
                        c.css_custom_properties.as_ref(),
                    ),
                })
                .collect()
        })
    }

    /// Converts the internal list of critical configurations into JSON.
    fn critical_to_json(
        critical: Option<&Vec<ConfigFsCritical>>,
    ) -> Option<Vec<ConfigFsCriticalJSON>> {
        critical.map(|common_vec| {
            common_vec
                .iter()
                .map(|c| ConfigFsCriticalJSON {
                    file_to_inline_paths: c.file_to_inline_paths.clone(),
                    styles: c.styles.clone(),
                    css_custom_properties: Self::css_custom_properties_to_json(
                        c.css_custom_properties.as_ref(),
                    ),
                })
                .collect()
        })
    }

    /// Converts custom CSS properties to JSON format.
    fn css_custom_properties_to_json(
        css_custom_properties_vec: Option<&Vec<ConfigFsCssCustomProperties>>,
    ) -> Option<Vec<ConfigFsCSSCustomPropertiesJSON>> {
        css_custom_properties_vec.map(|items: &Vec<ConfigFsCssCustomProperties>| {
            items
                .iter()
                .map(|item| ConfigFsCSSCustomPropertiesJSON {
                    element: Some(item.element.clone()),
                    data_param: item.data_param.clone(),
                    data_value: item.data_value.clone(),
                    css_variables: item.css_variables.clone().into_iter().collect(),
                })
                .collect()
        })
    }

    fn scrolls_to_json(
        config_scrolls: Option<HashMap<String, Vec<String>>>,
    ) -> Option<Vec<ConfigFsScrollJSON>> {
        config_scrolls.map(|scrolls| {
            let mut scrolls_vec = Vec::new();
            for (name, spells) in scrolls {
                scrolls_vec.push(ConfigFsScrollJSON {
                    name,
                    spells,
                    extends: None,
                });
            }
            scrolls_vec
        })
    }

    /// Converts the internal list of `Project` into its JSON representation.
    fn projects_to_json(projects: Vec<ConfigFsProject>) -> Vec<ConfigFsProjectJSON> {
        projects
            .into_iter()
            .map(|p| ConfigFsProjectJSON {
                project_name: p.project_name,
                input_paths: p.input_paths,
                output_dir_path: p.output_dir_path,
                single_output_file_name: p.single_output_file_name,
            })
            .collect()
    }

    /// Searches for and loads custom animation files from the "animations" subdirectory.
    ///
    /// This function scans the "animations" subdirectory within the given `current_dir/grimoire`,
    /// reads the content of each file, and stores it in a `HashMap`. The key of the
    /// HashMap is the file name (without extension), and the value is the file content.
    ///
    /// # Arguments
    ///
    /// * `current_dir` - A reference to a `Path` representing the directory to search in.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// - `Ok(HashMap<String, String>)`: A HashMap where keys are file names (without extension)
    ///   and values are the contents of the animation files.
    /// - `Err(GrimoireCSSError)`: An error if there's an issue reading the directory or files.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The "animations" subdirectory cannot be read.
    /// - There's an issue reading any of the files in the subdirectory.
    /// - File names cannot be converted to valid UTF-8 strings.
    fn find_custom_animations(
        current_dir: &Path,
    ) -> Result<HashMap<String, String>, GrimoireCssError> {
        let animations_dir =
            Filesystem::get_or_create_grimoire_path(current_dir)?.join("animations");

        if !animations_dir.exists() {
            return Ok(HashMap::new());
        }

        let mut entries = animations_dir.read_dir()?.peekable();

        if entries.peek().is_none() {
            add_message("No custom animations were found in the 'animations' directory. Deleted unnecessary 'animations' directory".to_string());
            fs::remove_dir(&animations_dir)?;
            return Ok(HashMap::new());
        }

        let mut map = HashMap::new();

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if ext == "css" {
                        if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                            let content = fs::read_to_string(&path)?;
                            map.insert(file_stem.to_owned(), content);
                        }
                    } else {
                        add_message(format!(
                            "Only CSS files are supported in the 'animations' directory. Skipping non-CSS file: {}.",
                            path.display()
                        ));
                    }
                }
            } else {
                add_message(format!(
                    "Only files are supported in the 'animations' directory. Skipping directory: {}.",
                    path.display()
                ));
            }
        }

        Ok(map)
    }

    fn expand_glob_patterns(patterns: Vec<String>) -> Vec<String> {
        let mut paths = Vec::new();
        for pattern in patterns {
            match glob(&pattern) {
                Ok(glob_paths) => {
                    for path_result in glob_paths.flatten() {
                        if let Some(path_str) = path_result.to_str() {
                            paths.push(path_str.to_string());
                        }
                    }
                }
                Err(e) => {
                    add_message(format!("Failed to read glob pattern {pattern}: {e}"));
                }
            }
        }
        paths
    }

    /// Loads external scrolls from files matching the pattern "grimoire.*.scrolls.json" in the config directory.
    /// If the main config already has scrolls, they will be merged with the external ones.
    /// Scrolls from the main configuration have higher priority and are not overwritten.
    ///
    /// # Arguments
    ///
    /// * `current_dir` - A reference to the current working directory
    /// * `existing_scrolls` - Optional HashMap of existing scrolls from main config
    ///
    /// # Returns
    ///
    /// * `Option<HashMap<String, Vec<String>>>` - Merged scrolls from main config and external files
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if reading or parsing any external scroll file fails.
    fn load_external_scrolls(
        current_dir: &Path,
        existing_scrolls: Option<HashMap<String, Vec<String>>>,
    ) -> Result<Option<HashMap<String, Vec<String>>>, GrimoireCssError> {
        // Get the config directory path
        let config_dir = Filesystem::get_or_create_grimoire_path(current_dir)?.join("config");

        // Initialize with existing scrolls or create new HashMap
        let mut all_scrolls = existing_scrolls.unwrap_or_default();
        let mut existing_scroll_names: HashSet<String> = all_scrolls.keys().cloned().collect();
        let mut external_files_found = false;

        // Use glob pattern to directly find matching files instead of reading entire directory
        let pattern = config_dir
            .join("grimoire.*.scrolls.json")
            .to_string_lossy()
            .to_string();

        match glob::glob(&pattern) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    if let Some(file_name) = entry.file_name().and_then(|s| s.to_str()) {
                        // Read and parse the external scroll file
                        match fs::read_to_string(&entry) {
                            Ok(content) => {
                                match serde_json::from_str::<serde_json::Value>(&content) {
                                    Ok(json) => {
                                        // Extract and process scrolls from the JSON
                                        if let Some(scrolls) =
                                            json.get("scrolls").and_then(|s| s.as_array())
                                        {
                                            external_files_found = true;

                                            // Parse each scroll from the array
                                            for scroll in scrolls {
                                                if let (Some(name), Some(spells_arr)) = (
                                                    scroll.get("name").and_then(|n| n.as_str()),
                                                    scroll.get("spells").and_then(|s| s.as_array()),
                                                ) {
                                                    // Don't override existing scrolls from main config, just add new ones
                                                    if !existing_scroll_names.contains(name) {
                                                        // Convert the spell array to Vec<String>
                                                        let spells: Vec<String> = spells_arr
                                                            .iter()
                                                            .filter_map(|s| {
                                                                s.as_str().map(|s| s.to_string())
                                                            })
                                                            .collect();

                                                        // Insert new scroll
                                                        all_scrolls
                                                            .insert(name.to_string(), spells);
                                                        existing_scroll_names
                                                            .insert(name.to_string());
                                                    }
                                                    // Existing scrolls from main config have higher priority
                                                }
                                            }

                                            add_message(format!(
                                                "Loaded external scrolls from '{file_name}'"
                                            ));
                                        }
                                    }
                                    Err(err) => {
                                        add_message(format!(
                                            "Failed to parse external scroll file '{file_name}': {err}"
                                        ));
                                    }
                                }
                            }
                            Err(err) => {
                                add_message(format!(
                                    "Failed to read external scroll file '{file_name}': {err}"
                                ));
                            }
                        }
                    }
                }
            }
            Err(err) => {
                add_message(format!("Failed to search for external scroll files: {err}"));
            }
        }

        // Only return Some if we have scrolls, otherwise None
        if all_scrolls.is_empty() {
            Ok(None)
        } else {
            // Add a message if we loaded external scrolls
            if external_files_found {
                add_message("External scroll files were merged with configuration".to_string());
            }
            Ok(Some(all_scrolls))
        }
    }

    /// Loads external variables from files matching the pattern "grimoire.*.variables.json" in the config directory.
    /// If the main config already has variables, they will be merged with the external ones.
    ///
    /// # Arguments
    ///
    /// * `current_dir` - A reference to the current working directory
    /// * `existing_variables` - Optional Vector of existing variables from main config
    ///
    /// # Returns
    ///
    /// * `Option<Vec<(String, String)>>` - Merged variables from main config and external files
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if reading or parsing any external variables file fails.
    fn load_external_variables(
        current_dir: &Path,
        existing_variables: Option<Vec<(String, String)>>,
    ) -> Result<Option<Vec<(String, String)>>, GrimoireCssError> {
        // Get the config directory path
        let config_dir = Filesystem::get_or_create_grimoire_path(current_dir)?.join("config");

        // Initialize with existing variables or create new Vec
        let mut all_variables = existing_variables.unwrap_or_default();
        let mut existing_keys: HashSet<String> =
            all_variables.iter().map(|(key, _)| key.clone()).collect();
        let mut external_files_found = false;

        // Use glob pattern to directly find matching files
        let pattern = config_dir
            .join("grimoire.*.variables.json")
            .to_string_lossy()
            .to_string();

        match glob::glob(&pattern) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    if let Some(file_name) = entry.file_name().and_then(|s| s.to_str()) {
                        // Read and parse the external variables file
                        match fs::read_to_string(&entry) {
                            Ok(content) => {
                                match serde_json::from_str::<serde_json::Value>(&content) {
                                    Ok(json) => {
                                        // Extract and process variables from the JSON
                                        if let Some(variables) =
                                            json.get("variables").and_then(|v| v.as_object())
                                        {
                                            external_files_found = true;

                                            // Parse each variable from the object
                                            for (key, value) in variables {
                                                if let Some(value_str) = value.as_str() {
                                                    // If the key doesn't exist yet, add it
                                                    if !existing_keys.contains(key) {
                                                        all_variables.push((
                                                            key.clone(),
                                                            value_str.to_string(),
                                                        ));
                                                        existing_keys.insert(key.clone());
                                                    }
                                                    // If the key exists, we don't override it - first come, first served
                                                }
                                            }

                                            add_message(format!(
                                                "Loaded external variables from '{file_name}'"
                                            ));
                                        }
                                    }
                                    Err(err) => {
                                        add_message(format!(
                                            "Failed to parse external variables file '{file_name}': {err}"
                                        ));
                                    }
                                }
                            }
                            Err(err) => {
                                add_message(format!(
                                    "Failed to read external variables file '{file_name}': {err}"
                                ));
                            }
                        }
                    }
                }
            }
            Err(err) => {
                add_message(format!(
                    "Failed to search for external variables files: {err}"
                ));
            }
        }

        // Sort variables by key for consistency
        if !all_variables.is_empty() {
            all_variables.sort_by(|a, b| a.0.cmp(&b.0));

            // Add a message if we loaded external variables
            if external_files_found {
                add_message("External variable files were merged with configuration".to_string());
            }
            Ok(Some(all_variables))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = ConfigFs::default();
        assert!(config.variables.is_none());
        assert!(config.scrolls.is_none());
        assert!(config.shared.is_none());
        assert!(config.critical.is_none());
        assert_eq!(config.projects.len(), 1);
        assert_eq!(config.projects[0].project_name, "main");
    }

    #[test]
    fn test_load_nonexistent_config() {
        let dir = tempdir().unwrap();
        let result = ConfigFs::load(dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_save_and_load_config() {
        let dir = tempdir().unwrap();
        let config = ConfigFs::default();
        config.save(dir.path()).expect("Failed to save config");

        let loaded_config = ConfigFs::load(dir.path()).expect("Failed to load config");
        assert_eq!(
            config.projects[0].project_name,
            loaded_config.projects[0].project_name
        );
    }

    #[test]
    fn test_expand_glob_patterns() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let patterns = vec![format!("{}/**/*.txt", dir.path().to_str().unwrap())];
        let expanded = ConfigFs::expand_glob_patterns(patterns);
        assert_eq!(expanded.len(), 1);
        assert!(expanded[0].ends_with("test.txt"));
    }

    #[test]
    fn test_find_custom_animations_empty() {
        let dir = tempdir().unwrap();
        let animations = ConfigFs::find_custom_animations(dir.path()).unwrap();
        assert!(animations.is_empty());
    }

    #[test]
    fn test_find_custom_animations_with_files() {
        let dir = tempdir().unwrap();
        let animations_dir = dir.path().join("grimoire").join("animations");
        fs::create_dir_all(&animations_dir).unwrap();

        let animation_file = animations_dir.join("fade_in.css");
        let mut file = File::create(&animation_file).unwrap();
        writeln!(
            file,
            "@keyframes fade_in {{ from {{ opacity: 0; }} to {{ opacity: 1; }} }}"
        )
        .unwrap();

        let animations = ConfigFs::find_custom_animations(dir.path()).unwrap();
        assert_eq!(animations.len(), 1);
        assert!(animations.contains_key("fade_in"));
    }

    #[test]
    fn test_get_common_spells_set() {
        let json = ConfigFsJSON {
            schema: None,
            variables: None,
            scrolls: None,
            projects: vec![],
            shared: Some(vec![ConfigFsSharedJSON {
                output_path: "styles.css".to_string(),
                styles: Some(vec!["spell1".to_string(), "spell2".to_string()]),
                css_custom_properties: None,
            }]),
            critical: Some(vec![ConfigFsCriticalJSON {
                file_to_inline_paths: vec!["index.html".to_string()],
                styles: Some(vec!["spell3".to_string()]),
                css_custom_properties: None,
            }]),
            lock: None,
        };

        let common_spells = ConfigFs::get_common_spells_set(&json);
        assert_eq!(common_spells.len(), 3);
        assert!(common_spells.contains("spell1"));
        assert!(common_spells.contains("spell2"));
        assert!(common_spells.contains("spell3"));
    }

    #[test]
    fn test_load_external_scrolls_no_files() {
        let dir = tempdir().unwrap();
        let config_dir = dir.path().join("grimoire").join("config");
        fs::create_dir_all(&config_dir).unwrap();

        // Create a basic config file to prevent load() from failing
        let config_file = config_dir.join("grimoire.config.json");
        let config_content = r#"{
            "projects": [
                {
                    "projectName": "main",
                    "inputPaths": []
                }
            ]
        }"#;
        fs::write(&config_file, config_content).unwrap();

        // No external scroll files
        let result = ConfigFs::load_external_scrolls(dir.path(), None).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_load_external_scrolls_single_file() {
        let dir = tempdir().unwrap();
        let config_dir = dir.path().join("grimoire").join("config");
        fs::create_dir_all(&config_dir).unwrap();

        // Create a basic config file to prevent load() from failing
        let config_file = config_dir.join("grimoire.config.json");
        let config_content = r#"{
            "projects": [
                {
                    "projectName": "main",
                    "inputPaths": []
                }
            ]
        }"#;
        fs::write(&config_file, config_content).unwrap();

        // Create an external scrolls file
        let scrolls_file = config_dir.join("grimoire.tailwindcss.scrolls.json");
        let scrolls_content = r#"{
            "scrolls": [
                {
                    "name": "tw-btn",
                    "spells": [
                        "p=4px",
                        "bg=blue",
                        "c=white",
                        "br=4px"
                    ]
                }
            ]
        }"#;
        fs::write(&scrolls_file, scrolls_content).unwrap();

        // Load external scrolls
        let result = ConfigFs::load_external_scrolls(dir.path(), None).unwrap();
        assert!(result.is_some());

        let scrolls = result.unwrap();
        assert_eq!(scrolls.len(), 1);
        assert!(scrolls.contains_key("tw-btn"));
        assert_eq!(scrolls.get("tw-btn").unwrap().len(), 4);
    }

    #[test]
    fn test_load_external_scrolls_multiple_files() {
        let dir = tempdir().unwrap();
        let config_dir = dir.path().join("grimoire").join("config");
        fs::create_dir_all(&config_dir).unwrap();

        // Create a basic config file
        let config_file = config_dir.join("grimoire.config.json");
        let config_content = r#"{
            "projects": [
                {
                    "projectName": "main",
                    "inputPaths": []
                }
            ]
        }"#;
        fs::write(&config_file, config_content).unwrap();

        // Create first external scrolls file
        let scrolls_file1 = config_dir.join("grimoire.tailwindcss.scrolls.json");
        let scrolls_content1 = r#"{
            "scrolls": [
                {
                    "name": "tw-btn",
                    "spells": [
                        "p=4px",
                        "bg=blue",
                        "c=white",
                        "br=4px"
                    ]
                }
            ]
        }"#;
        fs::write(&scrolls_file1, scrolls_content1).unwrap();

        // Create second external scrolls file
        let scrolls_file2 = config_dir.join("grimoire.bootstrap.scrolls.json");
        let scrolls_content2 = r#"{
            "scrolls": [
                {
                    "name": "bs-card",
                    "spells": [
                        "border=1px_solid_#ccc",
                        "br=8px",
                        "shadow=0_2px_8px_rgba(0,0,0,0.1)"
                    ]
                }
            ]
        }"#;
        fs::write(&scrolls_file2, scrolls_content2).unwrap();

        // Load external scrolls
        let result = ConfigFs::load_external_scrolls(dir.path(), None).unwrap();
        assert!(result.is_some());

        let scrolls = result.unwrap();
        assert_eq!(scrolls.len(), 2);
        assert!(scrolls.contains_key("tw-btn"));
        assert!(scrolls.contains_key("bs-card"));
        assert_eq!(scrolls.get("tw-btn").unwrap().len(), 4);
        assert_eq!(scrolls.get("bs-card").unwrap().len(), 3);
    }

    #[test]
    fn test_merge_with_existing_scrolls() {
        let dir = tempdir().unwrap();
        let config_dir = dir.path().join("grimoire").join("config");
        fs::create_dir_all(&config_dir).unwrap();

        // Create a basic config file
        let config_file = config_dir.join("grimoire.config.json");
        let config_content = r#"{
            "scrolls": [
                {
                    "name": "main-btn",
                    "spells": [
                        "p=10px",
                        "fw=bold",
                        "c=black"
                    ]
                }
            ],
            "projects": [
                {
                    "projectName": "main",
                    "inputPaths": []
                }
            ]
        }"#;
        fs::write(&config_file, config_content).unwrap();

        // Create an external scrolls file
        let scrolls_file = config_dir.join("grimoire.extra.scrolls.json");
        let scrolls_content = r#"{
            "scrolls": [
                {
                    "name": "main-btn",
                    "spells": [
                        "bg=green",
                        "hover:bg=darkgreen"
                    ]
                },
                {
                    "name": "extra-btn",
                    "spells": [
                        "fs=16px",
                        "m=10px"
                    ]
                }
            ]
        }"#;
        fs::write(&scrolls_file, scrolls_content).unwrap();

        // Create mock existing scrolls
        let mut existing_scrolls = HashMap::new();
        existing_scrolls.insert(
            "main-btn".to_string(),
            vec![
                "p=10px".to_string(),
                "fw=bold".to_string(),
                "c=black".to_string(),
            ],
        );

        // Load and merge external scrolls
        let result = ConfigFs::load_external_scrolls(dir.path(), Some(existing_scrolls)).unwrap();
        assert!(result.is_some());

        let scrolls = result.unwrap();
        assert_eq!(scrolls.len(), 2);

        // Check that main-btn has combined spells from both sources
        assert!(scrolls.contains_key("main-btn"));
        assert_eq!(scrolls.get("main-btn").unwrap().len(), 3);

        // Check that extra-btn was added
        assert!(scrolls.contains_key("extra-btn"));
        assert_eq!(scrolls.get("extra-btn").unwrap().len(), 2);
    }

    #[test]
    fn test_full_config_with_external_scrolls() {
        let dir = tempdir().unwrap();
        let config_dir = dir.path().join("grimoire").join("config");
        fs::create_dir_all(&config_dir).unwrap();

        // Create a basic config file
        let config_file = config_dir.join("grimoire.config.json");
        let config_content = r#"{
            "scrolls": [
                {
                    "name": "base-btn",
                    "spells": [
                        "p=10px",
                        "br=4px"
                    ]
                }
            ],
            "projects": [
                {
                    "projectName": "main",
                    "inputPaths": []
                }
            ]
        }"#;
        fs::write(&config_file, config_content).unwrap();

        // Create an external scrolls file
        let scrolls_file = config_dir.join("grimoire.theme.scrolls.json");
        let scrolls_content = r#"{
            "scrolls": [
                {
                    "name": "theme-btn",
                    "spells": [
                        "bg=purple",
                        "c=white"
                    ]
                }
            ]
        }"#;
        fs::write(&scrolls_file, scrolls_content).unwrap();

        // Load the full configuration
        let config = ConfigFs::load(dir.path()).expect("Failed to load config");

        // Check that both scrolls are loaded
        assert!(config.scrolls.is_some());
        let scrolls = config.scrolls.unwrap();
        assert_eq!(scrolls.len(), 2);
        assert!(scrolls.contains_key("base-btn"));
        assert!(scrolls.contains_key("theme-btn"));
    }

    #[test]
    fn test_load_external_variables_no_files() {
        let dir = tempdir().unwrap();
        let config_dir = dir.path().join("grimoire").join("config");
        fs::create_dir_all(&config_dir).unwrap();

        // Create a basic config file to prevent load() from failing
        let config_file = config_dir.join("grimoire.config.json");
        let config_content = r#"{
            "projects": [
                {
                    "projectName": "main",
                    "inputPaths": []
                }
            ]
        }"#;
        fs::write(&config_file, config_content).unwrap();

        // No external variable files
        let result = ConfigFs::load_external_variables(dir.path(), None).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_load_external_variables_single_file() {
        let dir = tempdir().unwrap();
        let config_dir = dir.path().join("grimoire").join("config");
        fs::create_dir_all(&config_dir).unwrap();

        // Create a basic config file to prevent load() from failing
        let config_file = config_dir.join("grimoire.config.json");
        let config_content = r#"{
            "projects": [
                {
                    "projectName": "main",
                    "inputPaths": []
                }
            ]
        }"#;
        fs::write(&config_file, config_content).unwrap();

        // Create an external variables file
        let vars_file = config_dir.join("grimoire.theme.variables.json");
        let vars_content = r##"{
            "variables": {
                "primary-color": "#3366ff",
                "secondary-color": "#ff6633",
                "font-size-base": "16px"
            }
        }"##;
        fs::write(&vars_file, vars_content).unwrap();

        // Load external variables
        let result = ConfigFs::load_external_variables(dir.path(), None).unwrap();
        assert!(result.is_some());

        let variables = result.unwrap();
        assert_eq!(variables.len(), 3);

        // Check that variables are sorted by key
        assert_eq!(variables[0].0, "font-size-base");
        assert_eq!(variables[0].1, "16px");
        assert_eq!(variables[1].0, "primary-color");
        assert_eq!(variables[1].1, "#3366ff");
        assert_eq!(variables[2].0, "secondary-color");
        assert_eq!(variables[2].1, "#ff6633");
    }

    #[test]
    fn test_load_external_variables_multiple_files() {
        let dir = tempdir().unwrap();
        let config_dir = dir.path().join("grimoire").join("config");
        fs::create_dir_all(&config_dir).unwrap();

        // Create a basic config file
        let config_file = config_dir.join("grimoire.config.json");
        let config_content = r#"{
            "projects": [
                {
                    "projectName": "main",
                    "inputPaths": []
                }
            ]
        }"#;
        fs::write(&config_file, config_content).unwrap();

        // Create first external variables file
        let vars_file1 = config_dir.join("grimoire.colors.variables.json");
        let vars_content1 = r##"{
            "variables": {
                "primary-color": "#3366ff",
                "secondary-color": "#ff6633"
            }
        }"##;
        fs::write(&vars_file1, vars_content1).unwrap();

        // Create second external variables file
        let vars_file2 = config_dir.join("grimoire.typography.variables.json");
        let vars_content2 = r##"{
            "variables": {
                "font-size-base": "16px",
                "font-family-sans": "Arial, sans-serif"
            }
        }"##;
        fs::write(&vars_file2, vars_content2).unwrap();

        // Load external variables
        let result = ConfigFs::load_external_variables(dir.path(), None).unwrap();
        assert!(result.is_some());

        let variables = result.unwrap();
        assert_eq!(variables.len(), 4);

        // Create a map for easier testing
        let var_map: HashMap<String, String> = variables.into_iter().collect();
        assert!(var_map.contains_key("primary-color"));
        assert!(var_map.contains_key("secondary-color"));
        assert!(var_map.contains_key("font-size-base"));
        assert!(var_map.contains_key("font-family-sans"));

        assert_eq!(var_map.get("primary-color").unwrap(), "#3366ff");
        assert_eq!(
            var_map.get("font-family-sans").unwrap(),
            "Arial, sans-serif"
        );
    }

    #[test]
    fn test_merge_with_existing_variables() {
        let dir = tempdir().unwrap();
        let config_dir = dir.path().join("grimoire").join("config");
        fs::create_dir_all(&config_dir).unwrap();

        // Create a basic config file with variables
        let config_file = config_dir.join("grimoire.config.json");
        let config_content = r##"{
            "variables": {
                "primary-color": "#3366ff",
                "font-size-base": "16px"
            },
            "projects": [
                {
                    "projectName": "main",
                    "inputPaths": []
                }
            ]
        }"##;
        fs::write(&config_file, config_content).unwrap();

        // Create an external variables file
        let vars_file = config_dir.join("grimoire.extra.variables.json");
        let vars_content = r##"{
            "variables": {
                "secondary-color": "#ff6633",
                "primary-color": "#ff0000",
                "spacing-unit": "8px"
            }
        }"##;
        fs::write(&vars_file, vars_content).unwrap();

        // Create mock existing variables
        let existing_variables = vec![
            ("primary-color".to_string(), "#3366ff".to_string()),
            ("font-size-base".to_string(), "16px".to_string()),
        ];

        // Load and merge external variables
        let result =
            ConfigFs::load_external_variables(dir.path(), Some(existing_variables)).unwrap();
        assert!(result.is_some());

        let variables = result.unwrap();
        assert_eq!(variables.len(), 4); // primary-color, font-size-base, secondary-color, spacing-unit

        // Create a map for easier testing
        let var_map: HashMap<String, String> = variables.into_iter().collect();

        // Primary color should remain from the original config (not overwritten)
        assert_eq!(var_map.get("primary-color").unwrap(), "#3366ff");

        // New variables should be added
        assert_eq!(var_map.get("secondary-color").unwrap(), "#ff6633");
        assert_eq!(var_map.get("spacing-unit").unwrap(), "8px");

        // Original variables should be preserved
        assert_eq!(var_map.get("font-size-base").unwrap(), "16px");
    }

    #[test]
    fn test_full_config_with_external_variables() {
        let dir = tempdir().unwrap();
        let config_dir = dir.path().join("grimoire").join("config");
        fs::create_dir_all(&config_dir).unwrap();

        // Create a basic config file with variables
        let config_file = config_dir.join("grimoire.config.json");
        let config_content = r##"{
            "variables": {
                "primary-color": "#3366ff"
            },
            "projects": [
                {
                    "projectName": "main",
                    "inputPaths": []
                }
            ]
        }"##;
        fs::write(&config_file, config_content).unwrap();

        // Create an external variables file
        let vars_file = config_dir.join("grimoire.theme.variables.json");
        let vars_content = r##"{
            "variables": {
                "secondary-color": "#ff6633",
                "spacing-unit": "8px"
            }
        }"##;
        fs::write(&vars_file, vars_content).unwrap();

        // Load the full configuration
        let config = ConfigFs::load(dir.path()).expect("Failed to load config");

        // Check that variables from both sources are loaded
        assert!(config.variables.is_some());
        let variables = config.variables.unwrap();
        assert_eq!(variables.len(), 3);

        // Variables should be sorted by key
        assert_eq!(variables[0].0, "primary-color");
        assert_eq!(variables[1].0, "secondary-color");
        assert_eq!(variables[2].0, "spacing-unit");
    }
}
