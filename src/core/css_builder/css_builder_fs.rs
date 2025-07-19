//! File system-based CSS builder with persistent storage and file tracking.
//!
//! This builder provides:
//! - CSS compilation with file output
//! - Shared CSS injection
//! - Critical CSS inlining
//! - File change tracking
//! - File cleanup
//!
//! Use this builder for standard projects that require file output.

use crate::{
    buffer::add_message,
    core::{
        ConfigFs, ConfigFsCssCustomProperties, CssOptimizer, GrimoireCssError,
        build_info::BuildInfo, file_tracker::FileTracker, parser::ParserFs, spell::Spell,
    },
};
use regex::Regex;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use super::CssBuilder;

/// Manages the process of compiling and building CSS files with filesystem persistence.
pub struct CssBuilderFs<'a> {
    css_builder: CssBuilder<'a>,
    config: &'a ConfigFs,
    current_dir: &'a Path,
    parser: ParserFs,
    inline_css_regex: Regex,
}

impl<'a> CssBuilderFs<'a> {
    /// Creates a new `CSSBuilder` instance.
    ///
    /// # Arguments
    ///
    /// * `config` - Reference to the Grimoire CSS configuration.
    /// * `current_dir` - Current working directory path.
    /// * `optimizer` - A reference to an implementation of the `CSSOptimizer` trait, which is responsible for optimizing CSS during the build process.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if the regex initialization fails.
    pub fn new<O: CssOptimizer>(
        config: &'a ConfigFs,
        current_dir: &'a Path,
        optimizer: &'a O,
    ) -> Result<Self, GrimoireCssError> {
        let css_builder = CssBuilder::new(optimizer, &config.variables, &config.custom_animations)?;
        let parser = ParserFs::new(current_dir);
        let inline_css_regex = Regex::new(r#"(?s)<style data-grimoire-critical-css>.*?</style>"#)?;

        Ok(Self {
            css_builder,
            config,
            current_dir,
            parser,
            inline_css_regex,
        })
    }

    /// Executes the build process, compiling CSS based on the provided configuration.
    ///
    /// Processes each project, compiles shared and critical CSS, and writes the output files.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if any step in the build process fails.
    pub fn build(&mut self) -> Result<(), GrimoireCssError> {
        let mut project_build_info = Vec::new();

        for project in &self.config.projects {
            let project_output_dir_path = project
                .output_dir_path
                .as_deref()
                .map(|d| self.current_dir.join(d))
                .unwrap_or_else(|| self.current_dir.join("grimoire/dist"));

            if let Some(single_output_file_name) = &project.single_output_file_name {
                let classes = self
                    .parser
                    .collect_classes_single_output(&project.input_paths)?;
                let bundle_output_full_path = project_output_dir_path.join(single_output_file_name);

                let spells = Spell::generate_spells_from_classes(
                    classes,
                    &self.config.shared_spells,
                    &self.config.scrolls,
                )?;

                project_build_info.push(BuildInfo {
                    file_path: bundle_output_full_path,
                    spells,
                });
            } else {
                let classes = self.parser.collect_classes_multiple_output(
                    &project.input_paths,
                    &project_output_dir_path,
                )?;

                for (file_path, classes) in classes {
                    let spells = Spell::generate_spells_from_classes(
                        classes,
                        &self.config.shared_spells,
                        &self.config.scrolls,
                    )?;

                    project_build_info.push(BuildInfo { file_path, spells });
                }
            }
        }

        let compiled_css: Vec<(PathBuf, String)> = self.compile_css(&project_build_info)?;
        let compiled_shared_css: Option<Vec<(PathBuf, String)>> = self.compile_shared_css()?;
        let compiled_critical_css: Option<Vec<(PathBuf, String)>> = self.compile_critical_css()?;

        Self::write_compiled_css(&compiled_css)?;

        if let Some(compiled_shared_css) = &compiled_shared_css {
            Self::write_compiled_css(compiled_shared_css)?;
        }

        // Track file changes if locking is enabled
        if self.config.lock.unwrap_or(false) {
            let all_compiled_paths = compiled_css.iter().map(|(path, _)| path.as_path()).chain(
                compiled_shared_css
                    .as_ref()
                    .into_iter()
                    .flat_map(|css| css.iter().map(|(path, _)| path.as_path())),
            );

            FileTracker::track(self.current_dir, all_compiled_paths)?;
        }

        if let Some(compiled_critical_css) = compiled_critical_css {
            self.inject_critical_css_into_html(&compiled_critical_css)?;
        }

        Ok(())
    }

    /// Compiles CSS for the provided project build information.
    ///
    /// # Arguments
    ///
    /// * `project_build_info` - Vector of `BuildInfo` containing file paths and spells.
    ///
    /// # Returns
    ///
    /// Vector of tuples with output file paths and compiled CSS strings.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if spell assembly or CSS optimization fails.
    fn compile_css(
        &self,
        project_build_info: &[BuildInfo],
    ) -> Result<Vec<(PathBuf, String)>, GrimoireCssError> {
        let compiled_css: Result<Vec<(PathBuf, String)>, GrimoireCssError> = project_build_info
            .iter()
            .map(|build_info| {
                let assembled_spells =
                    self.css_builder.combine_spells_to_css(&build_info.spells)?;
                let raw_css = if assembled_spells.len() == 1 {
                    assembled_spells[0].clone()
                } else {
                    assembled_spells.concat()
                };
                let css = self.css_builder.optimize_css(&raw_css)?;
                Ok((build_info.file_path.clone(), css))
            })
            .collect();

        compiled_css
    }

    /// Writes compiled CSS to specified file paths.
    ///
    /// # Arguments
    ///
    /// * `compiled_css` - Vector of tuples with file paths and CSS strings.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if writing to files fails.
    fn write_compiled_css(compiled_css: &[(PathBuf, String)]) -> Result<(), GrimoireCssError> {
        for (file_path, css) in compiled_css {
            Self::create_output_directory_if_needed(file_path)?;
            fs::write(file_path, css)?;
        }

        Ok(())
    }

    /// Creates the output directory if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path where the output file will be written.
    fn create_output_directory_if_needed(file_path: &Path) -> std::io::Result<()> {
        if let Some(parent_dir) = file_path.parent() {
            fs::create_dir_all(parent_dir)
        } else {
            Ok(())
        }
    }

    /// Compiles shared CSS defined in the configuration.
    ///
    /// # Returns
    ///
    /// Optional vector of tuples with output paths and compiled CSS strings.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if CSS composition or optimization fails.
    fn compile_shared_css(&self) -> Result<Option<Vec<(PathBuf, String)>>, GrimoireCssError> {
        self.config.shared.as_ref().map_or(Ok(None), |shared| {
            let mut compiled_shared_css = Vec::new();

            for shared_item in shared {
                if shared_item.output_path.is_empty() {
                    add_message("Output path is empty. Skipping.".to_string());
                    continue;
                }

                let mut composed_css = String::new();

                if let Some(custom_properties_css) =
                    Self::compose_custom_css_properties(&shared_item.css_custom_properties)
                {
                    composed_css.push_str(&custom_properties_css);
                }

                if let Some(shared_styles) = &shared_item.styles {
                    let extra_css = self.compose_extra_css(shared_styles)?;
                    composed_css.push_str(&extra_css);
                }

                if !composed_css.is_empty() {
                    compiled_shared_css.push((
                        PathBuf::from(&shared_item.output_path),
                        self.css_builder.optimize_css(&composed_css)?,
                    ));
                }
            }

            Ok(Some(compiled_shared_css))
        })
    }

    /// Compiles critical CSS and prepares it for injection into HTML files.
    ///
    /// # Returns
    ///
    /// Optional vector of tuples with file paths and CSS strings to be inlined.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if CSS composition or optimization fails.
    fn compile_critical_css(&self) -> Result<Option<Vec<(PathBuf, String)>>, GrimoireCssError> {
        self.config.critical.as_ref().map_or(Ok(None), |critical| {
            let mut compiled_critical_css = Vec::new();

            for critical_item in critical {
                if critical_item.file_to_inline_paths.is_empty() {
                    add_message("No file paths provided for inlining. Skipping.".to_string());
                    continue;
                }

                let mut composed_css = String::new();

                if let Some(custom_properties_css) =
                    Self::compose_custom_css_properties(&critical_item.css_custom_properties)
                {
                    composed_css.push_str(&custom_properties_css);
                }

                if let Some(shared_styles) = &critical_item.styles {
                    let extra_css = self.compose_extra_css(shared_styles)?;
                    composed_css.push_str(&extra_css);
                }

                if !composed_css.is_empty() {
                    let optimized_css = self.css_builder.optimize_css(&composed_css)?;

                    for path_to_inline in &critical_item.file_to_inline_paths {
                        compiled_critical_css
                            .push((PathBuf::from(&path_to_inline), optimized_css.clone()));
                    }
                }
            }

            Ok(Some(compiled_critical_css))
        })
    }

    /// Composes custom CSS properties into a CSS string.
    ///
    /// # Arguments
    ///
    /// * `raw_custom_css_properties` - Optional vector of `CSSCustomPropertiesItem`.
    ///
    /// # Returns
    ///
    /// Optional CSS string containing the custom properties.
    fn compose_custom_css_properties(
        raw_custom_css_properties: &Option<Vec<ConfigFsCssCustomProperties>>,
    ) -> Option<String> {
        raw_custom_css_properties.as_ref().map(|items| {
            items
                .iter()
                .map(Self::format_css_custom_properties_item)
                .collect()
        })
    }

    /// Formats a `CSSCustomPropertiesItem` into a CSS string.
    ///
    /// # Arguments
    ///
    /// * `css_custom_properties_item` - Item containing custom CSS properties.
    ///
    /// # Returns
    ///
    /// CSS string representing the custom properties.
    fn format_css_custom_properties_item(
        css_custom_properties_item: &ConfigFsCssCustomProperties,
    ) -> String {
        let variables = css_custom_properties_item
            .css_variables
            .iter()
            .map(|(var_name, var_value)| format!("--{var_name}: {var_value};"))
            .collect::<Vec<_>>()
            .join(" ");
        format!(
            "{}[data-{}='{}'] {{{}}}",
            css_custom_properties_item.element,
            css_custom_properties_item.data_param,
            css_custom_properties_item.data_value,
            variables
        )
    }

    /// Composes additional CSS from shared styles.
    ///
    /// # Arguments
    ///
    /// * `shared_styles` - Vector of style definitions or file paths.
    ///
    /// # Returns
    ///
    /// Composed and optimized CSS string.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if reading files or spell parsing fails.
    fn compose_extra_css(&self, shared_styles: &[String]) -> Result<String, GrimoireCssError> {
        let mut seen = HashSet::new();
        let mut files_content = Vec::new();
        let mut spells = Vec::new();

        for item in shared_styles {
            if !seen.insert(item.clone()) {
                continue;
            }

            if Path::new(item).is_file() {
                match fs::read_to_string(item) {
                    Ok(contents) => files_content.push(contents),
                    Err(err) => {
                        return Err(GrimoireCssError::InvalidInput(format!(
                            "Error reading file {item}; {err}"
                        )));
                    }
                }
            } else if let Some(spell) =
                Spell::new(item, &self.config.shared_spells, &self.config.scrolls)?
            {
                spells.push(spell);
            }
        }

        let assembled_spells = self.css_builder.combine_spells_to_css(&spells)?;
        let mut raw_css = assembled_spells.join("");

        if !files_content.is_empty() {
            raw_css.push_str(&files_content.join(""));
        }

        self.css_builder.optimize_css(&raw_css)
    }

    /// Injects critical CSS into HTML files.
    ///
    /// # Arguments
    ///
    /// * `inline_shared_css` - Vector of tuples with HTML file paths and CSS strings to inject.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if reading or writing HTML files fails.
    fn inject_critical_css_into_html(
        &self,
        inline_shared_css: &[(PathBuf, String)],
    ) -> Result<(), GrimoireCssError> {
        for (file_path, css) in inline_shared_css {
            let path = self.current_dir.join(file_path);
            self.embed_critical_css(&path, css)?;
        }

        Ok(())
    }

    /// Embeds critical CSS into an HTML file.
    ///
    /// # Arguments
    ///
    /// * `html_file_path` - Path to the HTML file.
    /// * `shared_css_str` - Critical CSS string to embed.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if reading or writing the HTML file fails.
    fn embed_critical_css(
        &self,
        html_file_path: &Path,
        shared_css_str: &str,
    ) -> Result<(), GrimoireCssError> {
        let html_content = fs::read_to_string(html_file_path)?;
        let critical_css = format!("<style data-grimoire-critical-css>{shared_css_str}</style>");

        // Remove existing critical CSS
        let cleaned_html_content = self.inline_css_regex.replace(&html_content, "").to_string();

        // Insert the critical CSS just before the closing </head> tag
        let updated_html_content = if let Some(index) = cleaned_html_content.rfind("</head>") {
            let (before_head, after_head) = cleaned_html_content.split_at(index);
            format!("{before_head}{critical_css}{after_head}")
        } else {
            // If </head> is not found, append the critical CSS at the end
            format!("{cleaned_html_content}{critical_css}")
        };

        fs::write(html_file_path, updated_html_content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    struct MockOptimizer;

    impl CssOptimizer for MockOptimizer {
        fn optimize(&self, css: &str) -> Result<String, GrimoireCssError> {
            Ok(css.to_string() + "_optimized")
        }
    }

    fn create_test_config() -> ConfigFs {
        ConfigFs::default()
    }

    #[test]
    fn test_cssbuilder_new() {
        let config = create_test_config();
        let current_dir = Path::new(".");
        let optimizer = MockOptimizer;

        let builder = CssBuilderFs::new(&config, current_dir, &optimizer);
        assert!(builder.is_ok());
    }

    #[test]
    fn test_cssbuilder_compile_css() {
        let config = create_test_config();
        let current_dir = Path::new(".");
        let optimizer = MockOptimizer;
        let builder = CssBuilderFs::new(&config, current_dir, &optimizer).unwrap();

        let build_info = BuildInfo {
            file_path: PathBuf::from("test_output.css"),
            spells: vec![Spell {
                raw_spell: "d=grid".to_string(),
                component: "display".to_string(),
                component_target: "grid".to_string(),
                effects: String::new(),
                area: String::new(),
                focus: String::new(),
                with_template: false,
                scroll_spells: None,
            }],
        };

        let result = builder.compile_css(&[build_info]);
        assert!(result.is_ok());

        let compiled_css = result.unwrap();
        assert_eq!(compiled_css[0].1, ".d\\=grid{display:grid;}_optimized");
    }

    #[test]
    fn test_cssbuilder_combine_spells_to_css() {
        let config = create_test_config();
        let current_dir = Path::new(".");
        let optimizer = MockOptimizer;
        let builder = CssBuilderFs::new(&config, current_dir, &optimizer).unwrap();

        let spells = vec![Spell {
            raw_spell: "d=grid".to_string(),
            component: "display".to_string(),
            component_target: "grid".to_string(),
            effects: String::new(),
            area: String::new(),
            focus: String::new(),
            with_template: false,
            scroll_spells: None,
        }];

        let result = builder.css_builder.combine_spells_to_css(&spells);
        assert!(result.is_ok());

        let assembled_css = result.unwrap();
        assert_eq!(assembled_css[0], ".d\\=grid{display:grid;}");
    }

    #[test]
    fn test_cssbuilder_write_compiled_css() {
        let file_path = PathBuf::from("test_output.css");
        let css = vec![(file_path.clone(), ".d\\=grid{display:grid;}".to_string())];

        let result = CssBuilderFs::write_compiled_css(&css);
        assert!(result.is_ok());

        let written_content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(written_content, ".d\\=grid{display:grid;}");

        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_cssbuilder_optimize_css() {
        let config = create_test_config();
        let current_dir = Path::new(".");
        let optimizer = MockOptimizer;
        let builder = CssBuilderFs::new(&config, current_dir, &optimizer).unwrap();

        let raw_css = ".d\\=grid{display:grid;}";
        let result = builder.css_builder.optimize_css(raw_css);
        assert!(result.is_ok());

        let optimized_css = result.unwrap();
        assert_eq!(optimized_css, ".d\\=grid{display:grid;}_optimized");
    }
}
