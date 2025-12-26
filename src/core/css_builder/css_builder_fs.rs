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
        build_info::BuildInfo, file_tracker::FileTracker, parser::ParserFs,
        source_file::SourceFile, spell::Spell,
    },
};
use regex::Regex;
use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
};

use super::CssBuilder;

type CriticalCssEntries = Vec<(PathBuf, Arc<str>)>;
type CriticalCssResult = Option<CriticalCssEntries>;

/// Manages the process of compiling and building CSS files with filesystem persistence.
pub struct CssBuilderFs<'a> {
    css_builder: CssBuilder<'a>,
    config: &'a ConfigFs,
    current_dir: &'a Path,
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
        let inline_css_regex = Regex::new(r#"(?s)<style data-grimoire-critical-css>.*?</style>"#)?;

        Ok(Self {
            css_builder,
            config,
            current_dir,
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
        let lock_enabled = self.config.lock.unwrap_or(false);

        let jobs = Self::jobs_from_env()?;

        // Only collect output paths when we actually need them for file tracking.
        let mut compiled_project_paths: Option<Vec<PathBuf>> = lock_enabled.then(Vec::new);

        if jobs <= 1 || self.config.projects.len() <= 1 {
            for project in &self.config.projects {
                let outputs = self.build_project(project)?;
                if let Some(paths) = &mut compiled_project_paths {
                    paths.extend(outputs);
                }
            }
        } else {
            let mut all_outputs: Vec<PathBuf> = Vec::new();
            let this: &CssBuilderFs<'a> = &*self;

            // NOTE: Parallelism is intentionally limited to project-level isolation. Each project
            // builds its own parser/builder instances to avoid shared mutable state.
            thread::scope(|scope| {
                let projects = &self.config.projects;
                let chunk_size = projects.len().div_ceil(jobs);
                let mut handles = Vec::new();

                for chunk in projects.chunks(chunk_size) {
                    handles.push(scope.spawn(move || {
                        let mut outputs = Vec::new();
                        for project in chunk {
                            outputs.extend(this.build_project(project)?);
                        }
                        Ok::<_, GrimoireCssError>(outputs)
                    }));
                }

                for h in handles {
                    match h.join() {
                        Ok(Ok(outputs)) => all_outputs.extend(outputs),
                        Ok(Err(e)) => return Err(e),
                        Err(_) => {
                            return Err(GrimoireCssError::InvalidInput(
                                "Project build thread panicked".to_string(),
                            ));
                        }
                    }
                }

                Ok(())
            })?;

            if let Some(paths) = &mut compiled_project_paths {
                paths.extend(all_outputs);
            }
        }
        let compiled_shared_css: Option<Vec<(PathBuf, String)>> = self.compile_shared_css()?;
        let compiled_critical_css: CriticalCssResult = self.compile_critical_css()?;

        if let Some(compiled_shared_css) = &compiled_shared_css {
            Self::write_compiled_css(compiled_shared_css)?;
        }

        // Track file changes if locking is enabled
        if lock_enabled {
            let all_compiled_paths = compiled_project_paths
                .as_ref()
                .expect("compiled_project_paths must be collected when lock is enabled")
                .iter()
                .map(|p| p.as_path())
                .chain(
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
    #[allow(dead_code)]
    fn compile_css(
        &self,
        project_build_info: &[BuildInfo],
    ) -> Result<Vec<(PathBuf, String)>, GrimoireCssError> {
        let compiled_css: Result<Vec<(PathBuf, String)>, GrimoireCssError> = project_build_info
            .iter()
            .map(|build_info| {
                let css = self
                    .css_builder
                    .combine_spells_to_optimized_css_string(&build_info.spells)?;
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
    fn compile_critical_css(&self) -> Result<CriticalCssResult, GrimoireCssError> {
        self.config.critical.as_ref().map_or(Ok(None), |critical| {
            let mut compiled_critical_css: CriticalCssEntries = Vec::new();

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
                    let optimized_css: Arc<str> =
                        Arc::from(self.css_builder.optimize_css(&composed_css)?);

                    for path_to_inline in &critical_item.file_to_inline_paths {
                        compiled_critical_css
                            .push((PathBuf::from(&path_to_inline), Arc::clone(&optimized_css)));
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

    /// Composes additional (raw, unoptimized) CSS from shared styles.
    ///
    /// # Arguments
    ///
    /// * `shared_styles` - Vector of style definitions or file paths.
    ///
    /// # Returns
    ///
    /// Composed (raw) CSS string.
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
            } else if let Some(spell) = Spell::new(
                item,
                &self.config.shared_spells,
                &self.config.scrolls,
                (0, 0),
                None,
            )? {
                spells.push(spell);
            }
        }

        let mut raw_css = self.css_builder.combine_spells_to_css_string(&spells)?;

        if !files_content.is_empty() {
            for contents in files_content {
                raw_css.push_str(&contents);
            }
        }

        // Important: callers are responsible for running optimization exactly once.
        Ok(raw_css)
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
        inline_shared_css: &[(PathBuf, Arc<str>)],
    ) -> Result<(), GrimoireCssError> {
        for (file_path, css) in inline_shared_css {
            let path = self.current_dir.join(file_path);
            self.embed_critical_css(&path, css.as_ref())?;
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

    fn build_project(
        &self,
        project: &'a crate::core::ConfigFsProject,
    ) -> Result<Vec<PathBuf>, GrimoireCssError> {
        let project_output_dir_path = project
            .output_dir_path
            .as_deref()
            .map(|d| self.current_dir.join(d))
            .unwrap_or_else(|| self.current_dir.join("grimoire/dist"));

        let parser = ParserFs::new(self.current_dir);

        let mut outputs = Vec::new();

        if let Some(single_output_file_name) = &project.single_output_file_name {
            let parsing_results = parser.collect_classes_single_output(&project.input_paths)?;
            let bundle_output_full_path = project_output_dir_path.join(single_output_file_name);

            let mut all_spells = Vec::new();
            for (file_path, classes) in parsing_results {
                let source = Arc::new(SourceFile::new_path_only(
                    Some(file_path.clone()),
                    file_path.to_string_lossy().to_string(),
                ));
                let spells = Spell::generate_spells_from_classes(
                    classes,
                    &self.config.shared_spells,
                    &self.config.scrolls,
                    Some(source),
                )?;

                // `ParserFs::collect_classes_single_output` already deduplicates class tokens.
                all_spells.extend(spells);
            }

            let css = self
                .css_builder
                .combine_spells_to_optimized_css_string(&all_spells)?;

            Self::create_output_directory_if_needed(&bundle_output_full_path)?;
            fs::write(&bundle_output_full_path, css)?;
            outputs.push(bundle_output_full_path);
        } else {
            let mut out_paths = Vec::new();
            parser.for_each_classes_multiple_output(
                &project.input_paths,
                &project_output_dir_path,
                |output_file_path, source_path, classes| {
                    let source = Arc::new(SourceFile::new_path_only(
                        Some(source_path.clone()),
                        source_path.to_string_lossy().to_string(),
                    ));
                    let spells = Spell::generate_spells_from_classes(
                        classes,
                        &self.config.shared_spells,
                        &self.config.scrolls,
                        Some(source),
                    )?;

                    let css = self
                        .css_builder
                        .combine_spells_to_optimized_css_string(&spells)?;
                    Self::create_output_directory_if_needed(&output_file_path)?;
                    fs::write(&output_file_path, css)?;
                    out_paths.push(output_file_path);
                    Ok(())
                },
            )?;
            outputs.extend(out_paths);
        }

        Ok(outputs)
    }

    fn jobs_from_env() -> Result<usize, GrimoireCssError> {
        match env::var("GRIMOIRE_CSS_JOBS") {
            Ok(v) => Self::parse_jobs(&v).map(Self::cap_jobs_to_machine),
            Err(env::VarError::NotPresent) => Ok(1),
            Err(e) => Err(GrimoireCssError::InvalidInput(format!(
                "Failed to read GRIMOIRE_CSS_JOBS: {e}"
            ))),
        }
    }

    fn parse_jobs(raw: &str) -> Result<usize, GrimoireCssError> {
        let trimmed = raw.trim();
        let jobs: usize = trimmed.parse().map_err(|_| {
            GrimoireCssError::InvalidInput(format!(
                "Invalid GRIMOIRE_CSS_JOBS value '{trimmed}': expected a positive integer"
            ))
        })?;

        if jobs == 0 {
            return Err(GrimoireCssError::InvalidInput(
                "GRIMOIRE_CSS_JOBS must be >= 1".to_string(),
            ));
        }

        Ok(jobs)
    }

    fn cap_jobs_to_machine(requested: usize) -> usize {
        let max = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        requested.clamp(1, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ConfigFsCritical;
    use std::path::Path;

    struct MockOptimizer;

    impl CssOptimizer for MockOptimizer {
        fn optimize(&self, css: &str) -> Result<String, GrimoireCssError> {
            Ok(css.to_string() + "_optimized")
        }

        fn validate(&self, _raw_css: &str) -> Result<(), GrimoireCssError> {
            Ok(())
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

        let spell = Spell::new(
            "d=grid",
            &config.shared_spells,
            &config.scrolls,
            (0, 0),
            None,
        )
        .unwrap()
        .unwrap();

        let build_info = BuildInfo {
            file_path: PathBuf::from("test_output.css"),
            spells: vec![spell],
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

        let spell = Spell::new(
            "d=grid",
            &config.shared_spells,
            &config.scrolls,
            (0, 0),
            None,
        )
        .unwrap()
        .unwrap();

        let spells = vec![spell];

        let result = builder.css_builder.combine_spells_to_css_string(&spells);
        assert!(result.is_ok());

        let assembled_css = result.unwrap();
        assert_eq!(assembled_css, ".d\\=grid{display:grid;}");
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

    #[test]
    fn test_compose_extra_css_is_raw_not_optimized() {
        let config = create_test_config();
        let current_dir = Path::new(".");
        let optimizer = MockOptimizer;
        let builder = CssBuilderFs::new(&config, current_dir, &optimizer).unwrap();

        let raw = builder.compose_extra_css(&["d=grid".to_string()]).unwrap();
        // compose_extra_css returns raw CSS; optimization is the caller's responsibility.
        assert_eq!(raw, ".d\\=grid{display:grid;}");
    }

    #[test]
    fn test_compile_critical_css_shares_payload_across_files() {
        let mut config = create_test_config();
        config.critical = Some(vec![ConfigFsCritical {
            file_to_inline_paths: vec!["a.html".to_string(), "b.html".to_string()],
            styles: Some(vec!["d=grid".to_string()]),
            css_custom_properties: None,
        }]);

        let current_dir = Path::new(".");
        let optimizer = MockOptimizer;
        let builder = CssBuilderFs::new(&config, current_dir, &optimizer).unwrap();

        let compiled = builder.compile_critical_css().unwrap().unwrap();
        assert_eq!(compiled.len(), 2);
        assert!(Arc::ptr_eq(&compiled[0].1, &compiled[1].1));
        assert_eq!(compiled[0].1.as_ref(), ".d\\=grid{display:grid;}_optimized");
    }

    #[test]
    fn test_parse_jobs_defaults_and_validation() {
        assert_eq!(CssBuilderFs::parse_jobs("1").unwrap(), 1);
        assert_eq!(CssBuilderFs::parse_jobs("  4 ").unwrap(), 4);
        assert!(CssBuilderFs::parse_jobs("0").is_err());
        assert!(CssBuilderFs::parse_jobs("-1").is_err());
        assert!(CssBuilderFs::parse_jobs("abc").is_err());
    }
}
