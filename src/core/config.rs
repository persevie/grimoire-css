//! This module provides the configuration management for GrimoireCSS.

use crate::{buffer::add_message, core::GrimoireCSSError};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use super::Filesystem;

/// Represents the main configuration structure for GrimoireCSS.
#[derive(Debug, Clone)]
pub struct Config {
    pub variables: Option<Vec<(String, String)>>,
    pub scrolls: Option<HashMap<String, Vec<String>>>,
    pub projects: Vec<ConfigProject>,
    pub shared: Option<Vec<ConfigShared>>,
    pub critical: Option<Vec<ConfigCritical>>,
    /// A set of shared spells used across different projects.
    pub shared_spells: HashSet<String>,
    pub lock: Option<bool>,

    pub custom_animations: HashMap<String, String>,
}

/// Shared configuration for GrimoireCSS projects.
#[derive(Debug, Clone)]
pub struct ConfigShared {
    pub output_path: String,
    pub styles: Option<Vec<String>>,
    pub css_custom_properties: Option<Vec<ConfigCSSCustomProperties>>,
}

/// Critical styles configuration to be inlined into specific HTML files.
#[derive(Debug, Clone)]
pub struct ConfigCritical {
    pub file_to_inline_paths: Vec<String>,
    pub styles: Option<Vec<String>>,
    pub css_custom_properties: Option<Vec<ConfigCSSCustomProperties>>,
}

/// Represents custom CSS properties associated with specific elements.
#[derive(Debug, Clone)]
pub struct ConfigCSSCustomProperties {
    pub element: String,
    pub data_param: String,
    pub data_value: String,
    pub css_variables: Vec<(String, String)>,
}

/// Represents a project in GrimoireCSS.
#[derive(Debug, Clone)]
pub struct ConfigProject {
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
struct ConfigJSON {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    /// Optional framework-level variables used during compilation.
    pub variables: Option<HashMap<String, String>>,
    /// Optional shared configuration settings used across multiple projects.
    pub scrolls: Option<Vec<ConfigScrollJSON>>,
    /// A list of projects included in the configuration.
    pub projects: Vec<ConfigProjectJSON>,
    pub shared: Option<Vec<ConfigSharedJSON>>,
    pub critical: Option<Vec<ConfigCriticalJSON>>,
    pub lock: Option<bool>,
}

/// Represents a scrolls which may contain external or combined CSS rules.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigScrollJSON {
    pub name: String,
    pub spells: Vec<String>,
    pub extends: Option<Vec<String>>,
}

/// A struct representing a project within GrimoireCSS.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ConfigProjectJSON {
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
struct ConfigSharedJSON {
    pub output_path: String,
    pub styles: Option<Vec<String>>,
    pub css_custom_properties: Option<Vec<ConfigCSSCustomPropertiesJSON>>,
}

/// Represents critical styles configuration for inlining into HTML files.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ConfigCriticalJSON {
    pub file_to_inline_paths: Vec<String>,
    pub styles: Option<Vec<String>>,
    pub css_custom_properties: Option<Vec<ConfigCSSCustomPropertiesJSON>>,
}

/// Represents a custom CSS property item, including associated variables.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ConfigCSSCustomPropertiesJSON {
    /// The optional DOM element (`tag`, `class`, `id`, `:root` (default)) associated with the CSS variables.
    pub element: Option<String>,
    /// A parameter name used within the CSS configuration.
    pub data_param: String,
    /// A value corresponding to the data parameter.
    pub data_value: String,
    /// A set of associated CSS variables and their values.
    pub css_variables: HashMap<String, String>,
}

impl Default for Config {
    /// Provides a default configuration for `Config`, initializing the `scrolls`, `projects`, and other fields.
    fn default() -> Self {
        let projects = vec![ConfigProject {
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

impl Config {
    /// Loads the configuration from the file system.
    ///
    /// Reads a JSON configuration file from the file system and deserializes it into a `Config` object.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if reading or parsing the file fails.
    pub fn load(current_dir: &Path) -> Result<Self, GrimoireCSSError> {
        let config_path = Filesystem::get_config_path(current_dir)?;
        let content = fs::read_to_string(&config_path)?;
        let json_config: ConfigJSON = serde_json::from_str(&content)?;
        let mut config = Self::from_json(json_config);

        config.custom_animations = Self::find_custom_animations(current_dir)?;

        Ok(config)
    }

    /// Saves the current configuration to the file system.
    ///
    /// Serializes the current configuration into JSON format and writes it to the file system.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if writing to the file system fails.
    pub fn save(&self, current_dir: &Path) -> Result<(), GrimoireCSSError> {
        let config_path = Filesystem::get_config_path(current_dir)?;
        let json_config: ConfigJSON = self.to_json();
        let content = serde_json::to_string_pretty(&json_config)?;
        fs::write(&config_path, &content)?;

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
    fn get_common_spells_set(config: &ConfigJSON) -> HashSet<String> {
        let mut common_spells = HashSet::new();

        if let Some(shared) = &config.shared {
            for shared_item in shared {
                if let Some(shared_styles) = &shared_item.styles {
                    for spell in shared_styles {
                        common_spells.insert(spell.to_string());
                    }
                }
            }
        }

        if let Some(critical) = &config.critical {
            for critical_item in critical {
                if let Some(shared_styles) = &critical_item.styles {
                    for spell in shared_styles {
                        common_spells.insert(spell.to_string());
                    }
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
    fn from_json(json_config: ConfigJSON) -> Self {
        let variables = json_config.variables.clone().map(|vars| {
            let mut sorted_vars: Vec<_> = vars.into_iter().collect();
            sorted_vars.sort_unstable_by(|a, b| a.0.cmp(&b.0));
            sorted_vars
        });

        let projects = Self::projects_from_json(json_config.projects.clone());

        // Expand glob patterns in shared and critical configurations
        let shared = Self::shared_from_json(json_config.shared.clone());
        let critical = Self::critical_from_json(json_config.critical.clone());

        let shared_spells = Self::get_common_spells_set(&json_config);
        let scrolls = Self::scrolls_from_json(json_config.scrolls.clone());

        Config {
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
    fn shared_from_json(shared: Option<Vec<ConfigSharedJSON>>) -> Option<Vec<ConfigShared>> {
        shared.map(|shared_vec| {
            shared_vec
                .into_iter()
                .map(|c| ConfigShared {
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
        critical: Option<Vec<ConfigCriticalJSON>>,
    ) -> Option<Vec<ConfigCritical>> {
        critical.map(|critical_vec| {
            critical_vec
                .into_iter()
                .map(|c| ConfigCritical {
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
        scrolls: Option<Vec<ConfigScrollJSON>>,
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
        scroll: &ConfigScrollJSON,
        scrolls_vec: &[ConfigScrollJSON],
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
        css_custom_properties_vec: Option<Vec<ConfigCSSCustomPropertiesJSON>>,
    ) -> Option<Vec<ConfigCSSCustomProperties>> {
        css_custom_properties_vec.map(|items: Vec<ConfigCSSCustomPropertiesJSON>| {
            items
                .into_iter()
                .map(|item| ConfigCSSCustomProperties {
                    element: item.element.unwrap_or_else(|| String::from(":root")),
                    data_param: item.data_param,
                    data_value: item.data_value,
                    css_variables: {
                        let mut vars: Vec<_> = item.css_variables.into_iter().collect();
                        vars.sort_unstable_by(|a, b| a.0.cmp(&b.0));
                        vars
                    },
                })
                .collect()
        })
    }

    /// Converts a list of project JSON configurations to the internal `Project` type.
    fn projects_from_json(projects: Vec<ConfigProjectJSON>) -> Vec<ConfigProject> {
        projects
            .into_iter()
            .map(|p| {
                let input_paths = Self::expand_glob_patterns(p.input_paths);
                ConfigProject {
                    project_name: p.project_name,
                    input_paths,
                    output_dir_path: p.output_dir_path,
                    single_output_file_name: p.single_output_file_name,
                }
            })
            .collect()
    }

    /// Converts the internal `Config` into its JSON representation.
    fn to_json(&self) -> ConfigJSON {
        let variables_hash_map = self.variables.as_ref().map(|vars| {
            let mut sorted_vars: Vec<_> = vars.iter().collect();
            sorted_vars.sort_unstable_by(|a, b| a.0.cmp(&b.0));
            sorted_vars
                .into_iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect()
        });

        ConfigJSON {
            schema: Some("https://raw.githubusercontent.com/persevie/grimoire-css/main/src/core/config-schema.json".to_string()),
            variables: variables_hash_map,
            scrolls: Self::scrolls_to_json(self.scrolls.clone()),
            projects: Self::projects_to_json(self.projects.clone()),
            shared: Self::shared_to_json(self.shared.as_ref()),
            critical: Self::critical_to_json(self.critical.as_ref()),
            lock: self.lock,
        }
    }

    /// Converts the internal list of shared configurations into JSON.
    fn shared_to_json(shared: Option<&Vec<ConfigShared>>) -> Option<Vec<ConfigSharedJSON>> {
        shared.map(|common_vec: &Vec<ConfigShared>| {
            common_vec
                .iter()
                .map(|c| ConfigSharedJSON {
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
    fn critical_to_json(critical: Option<&Vec<ConfigCritical>>) -> Option<Vec<ConfigCriticalJSON>> {
        critical.map(|common_vec| {
            common_vec
                .iter()
                .map(|c| ConfigCriticalJSON {
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
        css_custom_properties_vec: Option<&Vec<ConfigCSSCustomProperties>>,
    ) -> Option<Vec<ConfigCSSCustomPropertiesJSON>> {
        css_custom_properties_vec.map(|items: &Vec<ConfigCSSCustomProperties>| {
            items
                .iter()
                .map(|item| ConfigCSSCustomPropertiesJSON {
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
    ) -> Option<Vec<ConfigScrollJSON>> {
        config_scrolls.map(|scrolls| {
            let mut scrolls_vec = Vec::new();
            for (name, spells) in scrolls {
                scrolls_vec.push(ConfigScrollJSON {
                    name,
                    spells,
                    extends: None,
                });
            }
            scrolls_vec
        })
    }

    /// Converts the internal list of `Project` into its JSON representation.
    fn projects_to_json(projects: Vec<ConfigProject>) -> Vec<ConfigProjectJSON> {
        projects
            .into_iter()
            .map(|p| ConfigProjectJSON {
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
    ) -> Result<HashMap<String, String>, GrimoireCSSError> {
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
                if path.extension().and_then(|s| s.to_str()) == Some("css") {
                    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                        let content = fs::read_to_string(&path)?;
                        map.insert(file_stem.to_string(), content);
                    }
                } else {
                    add_message(format!("Only CSS files are supported in the 'animations' directory. Skipping non-CSS file: {}", path.display()));
                }
            } else {
                add_message(format!("Only files are supported in the 'animations' directory. Skipping directory: {}", path.display()));
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
                    add_message(format!("Failed to read glob pattern {}: {}", pattern, e));
                }
            }
        }
        paths
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
        let config = Config::default();
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
        let result = Config::load(dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_save_and_load_config() {
        let dir = tempdir().unwrap();
        let config = Config::default();
        config.save(dir.path()).expect("Failed to save config");

        let loaded_config = Config::load(dir.path()).expect("Failed to load config");
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
        let expanded = Config::expand_glob_patterns(patterns);
        assert_eq!(expanded.len(), 1);
        assert!(expanded[0].ends_with("test.txt"));
    }

    #[test]
    fn test_find_custom_animations_empty() {
        let dir = tempdir().unwrap();
        let animations = Config::find_custom_animations(dir.path()).unwrap();
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

        let animations = Config::find_custom_animations(dir.path()).unwrap();
        assert_eq!(animations.len(), 1);
        assert!(animations.contains_key("fade_in"));
    }

    #[test]
    fn test_get_common_spells_set() {
        let json = ConfigJSON {
            schema: None,
            variables: None,
            scrolls: None,
            projects: vec![],
            shared: Some(vec![ConfigSharedJSON {
                output_path: "styles.css".to_string(),
                styles: Some(vec!["spell1".to_string(), "spell2".to_string()]),
                css_custom_properties: None,
            }]),
            critical: Some(vec![ConfigCriticalJSON {
                file_to_inline_paths: vec!["index.html".to_string()],
                styles: Some(vec!["spell3".to_string()]),
                css_custom_properties: None,
            }]),
            lock: None,
        };

        let common_spells = Config::get_common_spells_set(&json);
        assert_eq!(common_spells.len(), 3);
        assert!(common_spells.contains("spell1"));
        assert!(common_spells.contains("spell2"));
        assert!(common_spells.contains("spell3"));
    }
}
