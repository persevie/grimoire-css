#[cfg(feature = "mcp")]
use crate::transmutator::{Transmutation, TransmuteOptions, transmute_css, transmute_paths};
use crate::{
    GrimoireCssError, Spell,
    config::{ConfigFs, external_config_files},
    core::{Filesystem, css_builder::CssBuilder, parser::Parser},
    infrastructure::LightningCssOptimizer,
};
use serde::Serialize;
use serde_json::{Value, json};
#[cfg(feature = "mcp")]
use std::io::Write;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use glob::glob;

#[derive(Debug, Clone, Serialize)]
pub struct ExplainClassTokenResult {
    pub class_token: String,
    pub expanded_spells: Vec<String>,
    pub css: String,
}

pub struct Analyzer;

#[derive(Debug, Clone, Serialize)]
pub struct DryOccurrence {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub tokens: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DryCandidate {
    pub tokens: Vec<String>,
    pub support: usize,
    pub occurrences: Vec<DryOccurrence>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DryCandidatesResult {
    pub files_scanned: usize,
    pub class_occurrences: usize,
    pub candidates: Vec<DryCandidate>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenOccurrence {
    pub token: String,
    pub file: String,
    pub byte_offset: usize,
    pub byte_len: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScrollReference {
    pub scroll: String,
    pub arity: usize,
    pub occurrence: TokenOccurrence,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpellReference {
    pub spell: String,
    pub occurrence: TokenOccurrence,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpellFrequency {
    pub spell: String,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct IndexError {
    pub file: String,
    pub byte_offset: usize,
    pub byte_len: usize,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IndexResult {
    pub files_scanned: usize,
    pub token_occurrences: usize,
    pub scroll_references: Vec<ScrollReference>,
    pub top_expanded_spells: Vec<SpellFrequency>,
    pub css_variables_read: Vec<String>,
    pub css_variables_written: Vec<String>,
    pub errors: Vec<IndexError>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VariableReference {
    pub variable: String,
    pub kind: String,
    pub spell: String,
    pub occurrence: TokenOccurrence,
}

#[derive(Debug, Clone, Serialize)]
pub struct GrimoireVariableDefinition {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GrimoireVariableReference {
    pub variable: String,
    pub spell: String,
    pub occurrence: TokenOccurrence,
}

#[derive(Debug, Clone, Serialize)]
pub struct LintMessage {
    pub level: String,
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrence: Option<TokenOccurrence>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LintResult {
    pub errors: Vec<LintMessage>,
    pub warnings: Vec<LintMessage>,
    pub notes: Vec<LintMessage>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfigProjectSummary {
    pub name: String,
    pub input_paths: Vec<String>,
    pub output_dir_path: Option<String>,
    pub single_output_file_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfigSummary {
    pub config_path: String,
    pub projects: Vec<ConfigProjectSummary>,
    pub scrolls: Vec<String>,
    pub variables: Vec<GrimoireVariableDefinition>,
    pub shared_spells: Vec<String>,
    pub custom_animations: Vec<String>,
    pub css_custom_properties: Vec<String>,
    pub external_scroll_files: Vec<String>,
    pub external_variable_files: Vec<String>,
}

#[cfg(feature = "mcp")]
#[derive(Debug, Clone, Serialize)]
pub struct ValidationIssue {
    pub stage: String,
    pub path: String,
    pub message: String,
}

#[cfg(feature = "mcp")]
#[derive(Debug, Clone, Serialize)]
pub struct ConfigValidationResult {
    pub valid: bool,
    pub config_path: String,
    pub schema_valid: bool,
    pub engine_load_valid: bool,
    pub issues: Vec<ValidationIssue>,
}

#[cfg(feature = "mcp")]
#[derive(Debug, Clone, Serialize)]
pub struct SpellValidationItem {
    pub token: String,
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded_spells: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[cfg(feature = "mcp")]
#[derive(Debug, Clone, Serialize)]
pub struct SpellsValidationResult {
    pub valid: bool,
    pub checked: usize,
    pub items: Vec<SpellValidationItem>,
}

#[cfg(feature = "mcp")]
#[derive(Debug, Clone, Serialize)]
pub struct BuildCheckResult {
    pub attempted: bool,
    pub successful: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[cfg(feature = "mcp")]
#[derive(Debug, Clone, Serialize)]
pub struct ProjectCheckResult {
    pub valid: bool,
    pub config: ConfigValidationResult,
    pub spells_valid: bool,
    pub spell_errors: Vec<IndexError>,
    pub lint_clean: bool,
    pub lint: LintResult,
    pub build: BuildCheckResult,
}

#[cfg(feature = "mcp")]
#[derive(Debug, Clone, Serialize)]
pub struct TransmutationValidationResult {
    pub valid: bool,
    pub transmutation: Transmutation,
    pub validation: SpellsValidationResult,
}

#[cfg(feature = "mcp")]
#[derive(Debug, Clone, Serialize)]
pub struct CssImportResult {
    pub valid: bool,
    pub import_path: String,
    pub transmutation: Transmutation,
    pub validation: SpellsValidationResult,
    pub project_check: Option<ProjectCheckResult>,
    pub rolled_back: bool,
    pub rollback_error: Option<String>,
    pub error: Option<String>,
    pub build_outputs_transactional: bool,
    pub warning: String,
}

#[cfg(feature = "mcp")]
fn is_valid_import_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
}

#[cfg(feature = "mcp")]
fn collect_existing_scroll_names(
    config_path: &Path,
    config_dir: &Path,
    excluded: &Path,
) -> Result<HashSet<String>, GrimoireCssError> {
    let mut names = scroll_names_from_file(config_path)?;
    for path in external_config_files(config_dir, ".scrolls.json")? {
        if path != excluded {
            names.extend(scroll_names_from_file(&path)?);
        }
    }
    Ok(names)
}

#[cfg(feature = "mcp")]
fn scroll_names_from_file(path: &Path) -> Result<HashSet<String>, GrimoireCssError> {
    let value: Value = serde_json::from_slice(&fs::read(path)?)?;
    Ok(value
        .get("scrolls")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|scroll| scroll.get("name").and_then(Value::as_str))
        .map(str::to_string)
        .collect())
}

#[cfg(feature = "mcp")]
fn validate_external_config_files(
    current_dir: &Path,
    validator: &jsonschema::Validator,
) -> Vec<ValidationIssue> {
    let config_dir = current_dir.join("grimoire/config");
    let mut issues = Vec::new();

    for (suffix, property) in [
        (".scrolls.json", "scrolls"),
        (".variables.json", "variables"),
    ] {
        let paths = match external_config_files(&config_dir, suffix) {
            Ok(paths) => paths,
            Err(error) => {
                issues.push(ValidationIssue {
                    stage: "external_discovery".to_string(),
                    path: Analyzer::to_rel(current_dir, &config_dir),
                    message: error.to_string(),
                });
                continue;
            }
        };

        for path in paths {
            let display_path = Analyzer::to_rel(current_dir, &path);
            let content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(error) => {
                    issues.push(ValidationIssue {
                        stage: "external_read".to_string(),
                        path: display_path,
                        message: error.to_string(),
                    });
                    continue;
                }
            };
            let external: Value = match serde_json::from_str(&content) {
                Ok(external) => external,
                Err(error) => {
                    issues.push(ValidationIssue {
                        stage: "external_json".to_string(),
                        path: display_path,
                        message: error.to_string(),
                    });
                    continue;
                }
            };
            let Some(object) = external.as_object() else {
                issues.push(ValidationIssue {
                    stage: "external_schema".to_string(),
                    path: display_path,
                    message: "external config must be a JSON object".to_string(),
                });
                continue;
            };
            if object.len() != 1 || !object.contains_key(property) {
                issues.push(ValidationIssue {
                    stage: "external_schema".to_string(),
                    path: display_path,
                    message: format!("external config must contain only '{property}'"),
                });
                continue;
            }

            let mut synthetic = json!({"projects":[]});
            synthetic[property] = object[property].clone();
            issues.extend(
                validator
                    .iter_errors(&synthetic)
                    .map(|error| ValidationIssue {
                        stage: "external_schema".to_string(),
                        path: display_path.clone(),
                        message: error.to_string(),
                    }),
            );
        }
    }

    issues
}

#[cfg(feature = "mcp")]
fn atomic_write(path: &Path, bytes: &[u8], replace: bool) -> Result<(), GrimoireCssError> {
    let parent = path.parent().ok_or_else(|| {
        GrimoireCssError::InvalidPath(format!("Path has no parent: {}", path.display()))
    })?;
    fs::create_dir_all(parent)?;
    let mut builder = tempfile::Builder::new();
    builder.prefix(".grimoire-import-");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        builder.permissions(fs::Permissions::from_mode(0o666));
    }
    let mut file = builder.tempfile_in(parent)?;
    file.write_all(bytes)?;
    file.as_file().sync_all()?;
    if replace {
        file.persist(path)
    } else {
        file.persist_noclobber(path)
    }
    .map_err(|error| GrimoireCssError::Io(error.error))?;
    Ok(())
}

#[cfg(feature = "mcp")]
fn restore_import(
    path: &Path,
    previous: Option<&[u8]>,
    installed: &[u8],
) -> Result<(), GrimoireCssError> {
    match fs::read(path) {
        Ok(current) if current == installed => {}
        Ok(_) => {
            return Err(GrimoireCssError::RuntimeError(format!(
                "Import target changed concurrently; refusing to roll it back: {}",
                path.display()
            )));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(GrimoireCssError::RuntimeError(format!(
                "Import target was removed concurrently; refusing to recreate it: {}",
                path.display()
            )));
        }
        Err(error) => return Err(GrimoireCssError::Io(error)),
    }

    if let Some(bytes) = previous {
        atomic_write(path, bytes, true)
    } else {
        match fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(GrimoireCssError::Io(error)),
        }
    }
}

impl Analyzer {
    pub fn load_config(current_dir: &Path) -> Result<ConfigFs, GrimoireCssError> {
        ConfigFs::load_read_only(current_dir)
    }

    #[cfg(feature = "mcp")]
    /// Validates main and external configs against the schema, then loads them.
    pub fn validate_config(current_dir: &Path) -> Result<ConfigValidationResult, GrimoireCssError> {
        let config_path = current_dir.join("grimoire/config/grimoire.config.json");
        let display_path = Self::to_rel(current_dir, &config_path);
        let content = match fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(error) => {
                return Ok(ConfigValidationResult {
                    valid: false,
                    config_path: display_path.clone(),
                    schema_valid: false,
                    engine_load_valid: false,
                    issues: vec![ValidationIssue {
                        stage: "read".to_string(),
                        path: display_path,
                        message: error.to_string(),
                    }],
                });
            }
        };

        let instance: Value = match serde_json::from_str(&content) {
            Ok(instance) => instance,
            Err(error) => {
                return Ok(ConfigValidationResult {
                    valid: false,
                    config_path: display_path.clone(),
                    schema_valid: false,
                    engine_load_valid: false,
                    issues: vec![ValidationIssue {
                        stage: "json".to_string(),
                        path: display_path,
                        message: error.to_string(),
                    }],
                });
            }
        };

        let schema: Value =
            serde_json::from_str(include_str!("../core/config/config-schema.json"))?;
        let validator = jsonschema::validator_for(&schema).map_err(|error| {
            GrimoireCssError::RuntimeError(format!("Invalid embedded config schema: {error}"))
        })?;
        let mut issues = validator
            .iter_errors(&instance)
            .map(|error| ValidationIssue {
                stage: "schema".to_string(),
                path: error.instance_path.to_string(),
                message: error.to_string(),
            })
            .collect::<Vec<_>>();
        issues.extend(validate_external_config_files(current_dir, &validator));
        let schema_valid = issues.is_empty();

        let engine_load_valid = match Self::load_config(current_dir) {
            Ok(_) => true,
            Err(error) => {
                issues.push(ValidationIssue {
                    stage: "engine_load".to_string(),
                    path: display_path.clone(),
                    message: error.to_string(),
                });
                false
            }
        };

        Ok(ConfigValidationResult {
            valid: schema_valid && engine_load_valid,
            config_path: display_path,
            schema_valid,
            engine_load_valid,
            issues,
        })
    }

    #[cfg(feature = "mcp")]
    /// Validates each Spell or Scroll invocation by parsing and compiling it.
    pub fn validate_spells(
        current_dir: &Path,
        tokens: &[String],
    ) -> Result<SpellsValidationResult, GrimoireCssError> {
        Ok(Self::validate_spells_using(tokens, |token| {
            Self::explain_class_token(current_dir, token)
        }))
    }

    #[cfg(feature = "mcp")]
    fn validate_spells_using(
        tokens: &[String],
        explain: impl Fn(&str) -> Result<ExplainClassTokenResult, GrimoireCssError>,
    ) -> SpellsValidationResult {
        let items = tokens
            .iter()
            .map(|token| match explain(token) {
                Ok(result) => SpellValidationItem {
                    token: token.clone(),
                    valid: true,
                    expanded_spells: Some(result.expanded_spells),
                    css: Some(result.css),
                    error: None,
                },
                Err(error) => SpellValidationItem {
                    token: token.clone(),
                    valid: false,
                    expanded_spells: None,
                    css: None,
                    error: Some(error.to_string()),
                },
            })
            .collect::<Vec<_>>();
        SpellsValidationResult {
            valid: !items.is_empty() && items.iter().all(|item| item.valid),
            checked: items.len(),
            items,
        }
    }

    #[cfg(feature = "mcp")]
    fn validate_transmuted_spells(
        config: &ConfigFs,
        transmutation: &Transmutation,
    ) -> Result<SpellsValidationResult, GrimoireCssError> {
        let names = config
            .scrolls
            .as_ref()
            .map(|scrolls| scrolls.keys().cloned().collect())
            .unwrap_or_default();
        transmutation.validate_component_scroll_conflicts(&names)?;
        let tokens = transmutation
            .scrolls
            .iter()
            .flat_map(|scroll| scroll.spells.iter().cloned())
            .collect::<Vec<_>>();
        Ok(Self::validate_spells_using(&tokens, |token| {
            Self::explain_class_token_with_config(config, token)
        }))
    }

    #[cfg(feature = "mcp")]
    pub fn transmute_and_validate(
        current_dir: &Path,
        css: &str,
        options: TransmuteOptions,
    ) -> Result<TransmutationValidationResult, GrimoireCssError> {
        let transmutation = transmute_css(css, options)?;
        let config = Self::load_config(current_dir)?;
        let validation = Self::validate_transmuted_spells(&config, &transmutation)?;

        Ok(TransmutationValidationResult {
            valid: validation.valid,
            transmutation,
            validation,
        })
    }

    #[cfg(feature = "mcp")]
    pub fn import_css(
        current_dir: &Path,
        content: Option<&str>,
        paths: Option<&[String]>,
        import_name: &str,
        options: TransmuteOptions,
        replace: bool,
    ) -> Result<CssImportResult, GrimoireCssError> {
        let config_path = current_dir.join("grimoire/config/grimoire.config.json");
        if !config_path.is_file() {
            return Err(GrimoireCssError::InvalidInput(
                "Grimoire CSS is not initialized; call grimoire_init before importing CSS".into(),
            ));
        }
        if !is_valid_import_name(import_name) {
            return Err(GrimoireCssError::InvalidInput(
                "import_name must contain only ASCII letters, digits, '-' or '_'".into(),
            ));
        }
        if content.is_some() == paths.is_some() {
            return Err(GrimoireCssError::InvalidInput(
                "CSS import requires exactly one of content or paths".into(),
            ));
        }

        let config_dir = current_dir.join("grimoire/config");
        let target = config_dir.join(format!("grimoire.{import_name}.scrolls.json"));
        let import_path = Self::to_rel(current_dir, &target);
        let transaction_lock = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(config_dir.join(".grimoire-css-import.lock"))?;
        transaction_lock.lock()?;

        let initial_config = Self::validate_config(current_dir)?;
        if !initial_config.valid {
            return Err(GrimoireCssError::InvalidInput(
                "The existing project configuration is invalid; fix it before importing CSS".into(),
            ));
        }

        let transmutation = if let Some(css) = content {
            transmute_css(css, options)?
        } else {
            let patterns = paths.unwrap_or_default();
            transmute_paths(current_dir, patterns, options)?
        };
        let existing_names = collect_existing_scroll_names(&config_path, &config_dir, &target)?;
        let mut config = Self::load_config(current_dir)?;
        // Exclude definitions removed by this replacement from conflict checks.
        if let Some(scrolls) = &mut config.scrolls {
            scrolls.retain(|name, _| existing_names.contains(name));
        }
        let validation = Self::validate_transmuted_spells(&config, &transmutation)?;
        let warning = "Build outputs follow the existing build semantics and are not part of the import transaction".to_string();

        if !validation.valid {
            return Ok(CssImportResult {
                valid: false,
                import_path,
                transmutation,
                validation,
                project_check: None,
                rolled_back: false,
                rollback_error: None,
                error: Some("Generated spells were rejected by the Grimoire CSS engine".into()),
                build_outputs_transactional: false,
                warning,
            });
        }

        let previous = match fs::read(&target) {
            Ok(bytes) => Some(bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(GrimoireCssError::Io(error)),
        };
        if previous.is_some() && !replace {
            return Err(GrimoireCssError::InvalidInput(format!(
                "Import file '{}' already exists; pass replace=true to replace it",
                import_path
            )));
        }

        let conflicts = transmutation
            .scrolls
            .iter()
            .map(|scroll| scroll.name.as_str())
            .filter(|name| existing_names.contains(*name))
            .collect::<Vec<_>>();
        if !conflicts.is_empty() {
            return Err(GrimoireCssError::InvalidInput(format!(
                "scroll name conflict: {}",
                conflicts.join(", ")
            )));
        }

        let external = json!({
            "scrolls": transmutation.scrolls.iter().map(|scroll| json!({
                "name": scroll.name,
                "spells": scroll.spells,
            })).collect::<Vec<_>>()
        });
        let encoded = serde_json::to_vec_pretty(&external)?;
        atomic_write(&target, &encoded, previous.is_some())?;

        match Self::check_project(current_dir) {
            Ok(project_check) if project_check.valid => Ok(CssImportResult {
                valid: true,
                import_path,
                transmutation,
                validation,
                project_check: Some(project_check),
                rolled_back: false,
                rollback_error: None,
                error: None,
                build_outputs_transactional: false,
                warning,
            }),
            Ok(project_check) => {
                let rollback = restore_import(&target, previous.as_deref(), &encoded);
                Ok(CssImportResult {
                    valid: false,
                    import_path,
                    transmutation,
                    validation,
                    project_check: Some(project_check),
                    rolled_back: rollback.is_ok(),
                    rollback_error: rollback.err().map(|error| error.to_string()),
                    error: Some("Project verification failed after CSS import".into()),
                    build_outputs_transactional: false,
                    warning,
                })
            }
            Err(error) => {
                let message = error.to_string();
                let rollback = restore_import(&target, previous.as_deref(), &encoded);
                Ok(CssImportResult {
                    valid: false,
                    import_path,
                    transmutation,
                    validation,
                    project_check: None,
                    rolled_back: rollback.is_ok(),
                    rollback_error: rollback.err().map(|error| error.to_string()),
                    error: Some(message),
                    build_outputs_transactional: false,
                    warning,
                })
            }
        }
    }

    #[cfg(feature = "mcp")]
    /// Validates configuration, indexes and lints spells, then builds the project.
    pub fn check_project(current_dir: &Path) -> Result<ProjectCheckResult, GrimoireCssError> {
        let config = Self::validate_config(current_dir)?;
        if !config.valid {
            return Ok(ProjectCheckResult {
                valid: false,
                config,
                spells_valid: false,
                spell_errors: Vec::new(),
                lint_clean: false,
                lint: LintResult {
                    errors: Vec::new(),
                    warnings: Vec::new(),
                    notes: Vec::new(),
                },
                build: BuildCheckResult {
                    attempted: false,
                    successful: false,
                    error: Some("Build skipped because the configuration is invalid".to_string()),
                },
            });
        }

        let index = Self::index(current_dir, 0)?;
        let spells_valid = index.errors.is_empty();
        let spell_errors = index.errors;
        let lint = Self::lint(current_dir)?;
        let lint_clean = lint.errors.is_empty() && lint.warnings.is_empty();
        let build = match crate::build_with_options(current_dir, false) {
            Ok(()) => BuildCheckResult {
                attempted: true,
                successful: true,
                error: None,
            },
            Err(error) => BuildCheckResult {
                attempted: true,
                successful: false,
                error: Some(error.to_string()),
            },
        };
        let valid = config.valid && spells_valid && lint_clean && build.successful;

        Ok(ProjectCheckResult {
            valid,
            config,
            spells_valid,
            spell_errors,
            lint_clean,
            lint,
            build,
        })
    }

    /// Finds variable and Scroll references, with a fallback to Spell references.
    pub fn refs(current_dir: &Path, target: &str) -> Result<Value, GrimoireCssError> {
        let config = Self::load_config(current_dir)?;
        let is_dollar = target.starts_with('$');
        let variable = target.trim_start_matches('$');
        let known_variable = config
            .variables
            .as_ref()
            .is_some_and(|variables| variables.iter().any(|(name, _)| name == variable));
        let known_scroll = config
            .scrolls
            .as_ref()
            .is_some_and(|scrolls| scrolls.contains_key(target));
        let mut results = Vec::new();

        if is_dollar || known_variable {
            let references = Self::refs_grimoire_variable(current_dir, variable)?;
            if !references.is_empty() {
                results.push(json!({"kind":"var","name":variable,"refs":references}));
            }
        }
        if known_scroll {
            let references = Self::refs_scroll(current_dir, target)?;
            if !references.is_empty() {
                results.push(json!({"kind":"scroll","name":target,"refs":references}));
            }
        }
        if results.is_empty() && !is_dollar {
            let references = Self::refs_spell(current_dir, target)?;
            if !references.is_empty() {
                results.push(json!({"kind":"spell","name":target,"refs":references}));
            }
        }

        Ok(if results.is_empty() {
            json!({
                "query":target,
                "results":[],
                "note":"No references found. If you meant a variable, try prefixing with '$' (e.g. $spacing-unit)."
            })
        } else {
            json!({"query":target,"results":results})
        })
    }

    /// Returns project statistics, optionally filtered by group or token.
    pub fn stats(
        current_dir: &Path,
        group: Option<&str>,
        token: Option<&str>,
        top: usize,
    ) -> Result<Value, GrimoireCssError> {
        let group = group.unwrap_or("all");
        if !matches!(group, "all" | "spells" | "scrolls" | "vars") {
            return Err(GrimoireCssError::InvalidInput(format!(
                "Unknown stats group: {group}"
            )));
        }

        let config = Self::load_config(current_dir)?;
        let index = Self::index(current_dir, top)?;
        let mut output = serde_json::Map::new();
        output.insert("top".to_string(), json!(top));

        if let Some(token) = token {
            let variable = token.trim_start_matches('$');
            let known_variable = token.starts_with('$')
                || config
                    .variables
                    .as_ref()
                    .is_some_and(|variables| variables.iter().any(|(name, _)| name == variable));
            let known_scroll = config
                .scrolls
                .as_ref()
                .is_some_and(|scrolls| scrolls.contains_key(token));

            if known_variable {
                let references = Self::refs_grimoire_variable(current_dir, variable)?;
                output.insert(
                    "token".to_string(),
                    json!({"kind":"var","name":variable,"count":references.len()}),
                );
            } else if known_scroll {
                let count = index
                    .scroll_references
                    .iter()
                    .filter(|reference| reference.scroll == token)
                    .count();
                output.insert(
                    "token".to_string(),
                    json!({"kind":"scroll","name":token,"count":count}),
                );
            } else {
                let count = Self::spell_count(current_dir, token)?;
                output.insert(
                    "token".to_string(),
                    if count > 0 {
                        json!({"kind":"spell","name":token,"count":count})
                    } else {
                        json!({
                            "error":"Unknown token",
                            "hint":"Provide a scroll name, $var name, or a spell"
                        })
                    },
                );
            }
            return Ok(Value::Object(output));
        }

        if matches!(group, "all" | "spells") {
            output.insert("spells".to_string(), json!(index.top_expanded_spells));
        }
        if matches!(group, "all" | "scrolls") {
            let mut counts = HashMap::<String, u64>::new();
            for reference in &index.scroll_references {
                *counts.entry(reference.scroll.clone()).or_insert(0) += 1;
            }
            let mut items = counts
                .into_iter()
                .map(|(spell, count)| SpellFrequency { spell, count })
                .collect::<Vec<_>>();
            items.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.spell.cmp(&b.spell)));
            items.truncate(top);
            output.insert("scrolls".to_string(), json!(items));
        }
        if matches!(group, "all" | "vars") {
            let mut variables = config
                .variables
                .as_ref()
                .map(|variables| {
                    variables
                        .iter()
                        .map(|(name, _)| name.clone())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            variables.sort();
            variables.dedup();
            let mut items = Vec::new();
            for variable in variables {
                items.push(SpellFrequency {
                    count: Self::refs_grimoire_variable(current_dir, &variable)?.len() as u64,
                    spell: variable,
                });
            }
            items.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.spell.cmp(&b.spell)));
            items.truncate(top);
            output.insert("vars".to_string(), json!(items));
        }

        Ok(Value::Object(output))
    }

    /// Explain a single class token (e.g. `md3-btn`, `box=10px_20px`, `hover:bg-c=red`).
    ///
    /// Returns:
    /// - resolved spells (scroll expansion if applicable)
    /// - compiled CSS for the exact class token
    pub fn explain_class_token(
        current_dir: &Path,
        class_token: &str,
    ) -> Result<ExplainClassTokenResult, GrimoireCssError> {
        let config_fs = Self::load_config(current_dir)?;
        Self::explain_class_token_with_config(&config_fs, class_token)
    }

    fn explain_class_token_with_config(
        config_fs: &ConfigFs,
        class_token: &str,
    ) -> Result<ExplainClassTokenResult, GrimoireCssError> {
        let shared_spells = config_fs.shared_spells.clone();
        let spell = Spell::new(
            class_token,
            &shared_spells,
            &config_fs.scrolls,
            (0, 0),
            None,
        )?
        .ok_or_else(|| {
            GrimoireCssError::InvalidInput(format!(
                "Could not parse '{class_token}' as a spell or scroll invocation"
            ))
        })?;

        let expanded_spells: Vec<String> = if let Some(scroll_spells) = &spell.scroll_spells {
            scroll_spells
                .iter()
                .map(|s| s.raw_spell.clone())
                .collect::<Vec<String>>()
        } else {
            vec![spell.raw_spell.clone()]
        };

        let optimizer = LightningCssOptimizer::new_from_with_printer_minify("", false)?;
        let builder = CssBuilder::new(
            &optimizer,
            &config_fs.variables,
            &config_fs.custom_animations,
        )?;
        let css = builder.combine_spells_to_optimized_css_string(&[spell])?;

        Ok(ExplainClassTokenResult {
            class_token: class_token.to_string(),
            expanded_spells,
            css,
        })
    }

    pub fn config_summary(current_dir: &Path) -> Result<ConfigSummary, GrimoireCssError> {
        let config_fs = Self::load_config(current_dir)?;
        let config_path = Filesystem::get_config_path(current_dir)?;
        let config_dir = config_path.parent().unwrap_or(current_dir);

        let projects = config_fs
            .projects
            .iter()
            .map(|p| ConfigProjectSummary {
                name: p.project_name.clone(),
                input_paths: p.input_paths.clone(),
                output_dir_path: p.output_dir_path.clone(),
                single_output_file_name: p.single_output_file_name.clone(),
            })
            .collect::<Vec<_>>();

        let mut scrolls = config_fs
            .scrolls
            .as_ref()
            .map(|m| m.keys().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        scrolls.sort();

        let variables = config_fs
            .variables
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|(name, value)| GrimoireVariableDefinition { name, value })
            .collect::<Vec<_>>();

        let shared_spells = Self::sorted_set(config_fs.shared_spells.clone());

        let mut custom_animations = config_fs
            .custom_animations
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        custom_animations.sort();

        let css_custom_properties =
            Self::sorted_set(Self::defined_css_custom_properties(&config_fs));

        let external_scroll_files = external_config_files(config_dir, ".scrolls.json")?
            .into_iter()
            .map(|p| Self::to_rel(current_dir, &p))
            .collect::<Vec<_>>();
        let external_variable_files = external_config_files(config_dir, ".variables.json")?
            .into_iter()
            .map(|p| Self::to_rel(current_dir, &p))
            .collect::<Vec<_>>();

        Ok(ConfigSummary {
            config_path: Self::to_rel(current_dir, &config_path),
            projects,
            scrolls,
            variables,
            shared_spells,
            custom_animations,
            css_custom_properties,
            external_scroll_files,
            external_variable_files,
        })
    }

    pub fn index(current_dir: &Path, top: usize) -> Result<IndexResult, GrimoireCssError> {
        let config_fs = Self::load_config(current_dir)?;
        let parser = Parser::new();

        let mut files = HashSet::<PathBuf>::new();
        for project in &config_fs.projects {
            for pattern in &project.input_paths {
                for path in Self::expand_input_pattern(current_dir, pattern)? {
                    if path.is_file() {
                        files.insert(path);
                    }
                }
            }
        }

        let mut scroll_references: Vec<ScrollReference> = Vec::new();
        let mut errors: Vec<IndexError> = Vec::new();
        let mut token_occurrences: usize = 0;

        let mut expanded_spell_counts: HashMap<String, u64> = HashMap::new();
        let mut css_variables_read: HashSet<String> = HashSet::new();
        let mut css_variables_written: HashSet<String> = HashSet::new();

        let mut file_list: Vec<PathBuf> = files.into_iter().collect();
        file_list.sort();

        for file_path in &file_list {
            let content = match fs::read_to_string(file_path) {
                Ok(c) => c,
                Err(e) => {
                    errors.push(IndexError {
                        file: Self::to_rel(current_dir, file_path),
                        byte_offset: 0,
                        byte_len: 0,
                        message: format!("Failed to read file: {e}"),
                    });
                    continue;
                }
            };

            let line_index = LineIndex::new(&content);

            let mut candidates: Vec<(String, (usize, usize))> = Vec::new();
            // This method is available only when built with `--features analyzer`.
            parser.collect_candidates_all(&content, &mut candidates)?;

            for (token, (byte_offset, byte_len)) in candidates {
                token_occurrences += 1;

                let (line, column) = line_index.line_col(byte_offset);
                let occurrence = TokenOccurrence {
                    token: token.clone(),
                    file: Self::to_rel(current_dir, file_path),
                    byte_offset,
                    byte_len,
                    line,
                    column,
                };

                let parsed = Spell::new(
                    &token,
                    &config_fs.shared_spells,
                    &config_fs.scrolls,
                    (byte_offset, byte_len),
                    None,
                );

                let spell = match parsed {
                    Ok(Some(s)) => s,
                    Ok(None) => continue,
                    Err(e) => {
                        errors.push(IndexError {
                            file: occurrence.file.clone(),
                            byte_offset,
                            byte_len,
                            message: e.to_string(),
                        });
                        continue;
                    }
                };

                if let Some(expanded_spells) = &spell.scroll_spells {
                    // `scroll_spells` is used for both scroll invocations AND template tokens.
                    // Only treat it as a scroll reference when the component (scroll name) exists.
                    let scroll_name = spell.component().to_string();
                    if !scroll_name.is_empty() {
                        let arity = if spell.component_target().is_empty() {
                            0
                        } else {
                            spell.component_target().split('_').count()
                        };

                        scroll_references.push(ScrollReference {
                            scroll: scroll_name,
                            arity,
                            occurrence: occurrence.clone(),
                        });
                    }

                    for inner in expanded_spells {
                        Self::collect_css_variable_usage(
                            &inner.raw_spell,
                            &mut css_variables_read,
                            &mut css_variables_written,
                        );
                        *expanded_spell_counts
                            .entry(inner.raw_spell.clone())
                            .or_default() += 1;
                    }
                } else {
                    Self::collect_css_variable_usage(
                        &spell.raw_spell,
                        &mut css_variables_read,
                        &mut css_variables_written,
                    );
                    *expanded_spell_counts
                        .entry(spell.raw_spell.clone())
                        .or_default() += 1;
                }
            }
        }

        let mut top_expanded_spells = Self::top_counts(expanded_spell_counts, top);
        // Stable output for same counts.
        top_expanded_spells
            .sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.spell.cmp(&b.spell)));

        Ok(IndexResult {
            files_scanned: file_list.len(),
            token_occurrences,
            scroll_references,
            top_expanded_spells,
            css_variables_read: Self::sorted_set(css_variables_read),
            css_variables_written: Self::sorted_set(css_variables_written),
            errors,
        })
    }

    /// Find DRY candidates: sets of spells/scrolls that appear together in regular `class`/`className`
    /// attributes multiple times (order-insensitive).
    ///
    /// This is intended as IDE tooling: it helps you extract a shared scroll for repeated clusters.
    pub fn dry_candidates(
        current_dir: &Path,
        min_support: usize,
        min_items: usize,
    ) -> Result<DryCandidatesResult, GrimoireCssError> {
        let config_fs = Self::load_config(current_dir)?;
        let parser = Parser::new();

        let mut files = HashSet::<PathBuf>::new();
        for project in &config_fs.projects {
            for pattern in &project.input_paths {
                for path in Self::expand_input_pattern(current_dir, pattern)? {
                    if path.is_file() {
                        files.insert(path);
                    }
                }
            }
        }

        let mut file_list: Vec<PathBuf> = files.into_iter().collect();
        file_list.sort();

        let mut occurrences: Vec<DryOccurrence> = Vec::new();
        for file_path in &file_list {
            let content = match fs::read_to_string(file_path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let line_index = LineIndex::new(&content);

            let mut groups: Vec<crate::core::parser::RegularClassGroup> = Vec::new();
            // Analyzer-only API.
            parser.collect_regular_class_groups(&content, &mut groups)?;

            for g in groups {
                // Keep only tokens that parse as Grimoire spells/scroll invocations.
                let mut toks: Vec<(String, (usize, usize))> = Vec::new();
                for (t, span) in g.tokens {
                    if t.is_empty() {
                        continue;
                    }

                    let parsed =
                        Spell::new(&t, &config_fs.shared_spells, &config_fs.scrolls, span, None)?;

                    if parsed.is_some() {
                        toks.push((t, span));
                    }
                }

                if toks.len() < min_items {
                    continue;
                }

                // Normalize tokens for set comparisons (order-insensitive, de-dup).
                let mut norm: Vec<String> = toks.iter().map(|(t, _)| t.clone()).collect();
                norm.sort();
                norm.dedup();
                if norm.len() < min_items {
                    continue;
                }

                let (line, column) = line_index.line_col(toks[0].1.0);

                occurrences.push(DryOccurrence {
                    file: Self::to_rel(current_dir, file_path),
                    line,
                    column,
                    tokens: norm,
                });
            }
        }

        // Mine frequent intersections via pairwise set intersections.
        let mut candidate_support: HashMap<String, (Vec<String>, HashSet<usize>)> = HashMap::new();

        for i in 0..occurrences.len() {
            for j in (i + 1)..occurrences.len() {
                let inter = intersect_sorted(&occurrences[i].tokens, &occurrences[j].tokens);
                if inter.len() < min_items {
                    continue;
                }

                let key = inter.join("\u{1f}");
                let entry = candidate_support
                    .entry(key)
                    .or_insert_with(|| (inter.clone(), HashSet::new()));
                entry.1.insert(i);
                entry.1.insert(j);
            }
        }

        // Expand support: if an occurrence contains all tokens, include it.
        for (_, (tokens, support)) in candidate_support.iter_mut() {
            for (idx, occ) in occurrences.iter().enumerate() {
                if is_subset(tokens, &occ.tokens) {
                    support.insert(idx);
                }
            }
        }

        let mut candidates: Vec<(Vec<String>, Vec<usize>)> = candidate_support
            .into_values()
            .filter_map(|(tokens, support)| {
                if support.len() >= min_support {
                    let mut v: Vec<usize> = support.into_iter().collect();
                    v.sort();
                    Some((tokens, v))
                } else {
                    None
                }
            })
            .collect();

        // Prune redundant candidates: drop strict subsets with identical support.
        candidates.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        let mut kept: Vec<(Vec<String>, Vec<usize>)> = Vec::new();
        'outer: for (toks, supp) in candidates {
            for (kt, ks) in &kept {
                if ks == &supp && is_subset(&toks, kt) {
                    continue 'outer;
                }
            }
            kept.push((toks, supp));
        }

        let mut out: Vec<DryCandidate> = Vec::new();
        for (tokens, support) in kept {
            let occs = support
                .iter()
                .map(|&i| occurrences[i].clone())
                .collect::<Vec<_>>();
            out.push(DryCandidate {
                support: occs.len(),
                tokens,
                occurrences: occs,
            });
        }

        // Prefer bigger clusters first.
        out.sort_by(|a, b| {
            b.tokens
                .len()
                .cmp(&a.tokens.len())
                .then_with(|| b.support.cmp(&a.support))
        });

        Ok(DryCandidatesResult {
            files_scanned: file_list.len(),
            class_occurrences: occurrences.len(),
            candidates: out,
        })
    }

    pub fn refs_scroll(
        current_dir: &Path,
        scroll_name: &str,
    ) -> Result<Vec<ScrollReference>, GrimoireCssError> {
        let mut index = Self::index(current_dir, 0)?;
        index.scroll_references.retain(|r| r.scroll == scroll_name);
        Ok(index.scroll_references)
    }

    pub fn refs_spell(
        current_dir: &Path,
        raw_spell: &str,
    ) -> Result<Vec<SpellReference>, GrimoireCssError> {
        let config_fs = Self::load_config(current_dir)?;
        let parser = Parser::new();

        let mut files = HashSet::<PathBuf>::new();
        for project in &config_fs.projects {
            for pattern in &project.input_paths {
                for path in Self::expand_input_pattern(current_dir, pattern)? {
                    if path.is_file() {
                        files.insert(path);
                    }
                }
            }
        }

        let mut file_list: Vec<PathBuf> = files.into_iter().collect();
        file_list.sort();

        let mut refs: Vec<SpellReference> = Vec::new();

        for file_path in &file_list {
            let content = fs::read_to_string(file_path)?;
            let line_index = LineIndex::new(&content);

            let mut candidates: Vec<(String, (usize, usize))> = Vec::new();
            parser.collect_candidates_all(&content, &mut candidates)?;

            for (token, (byte_offset, byte_len)) in candidates {
                let parsed = Spell::new(
                    &token,
                    &config_fs.shared_spells,
                    &config_fs.scrolls,
                    (byte_offset, byte_len),
                    None,
                );

                let spell = match parsed {
                    Ok(Some(s)) => s,
                    _ => continue,
                };

                let (line, column) = line_index.line_col(byte_offset);
                let occurrence = TokenOccurrence {
                    token: token.clone(),
                    file: Self::to_rel(current_dir, file_path),
                    byte_offset,
                    byte_len,
                    line,
                    column,
                };

                if let Some(scroll_spells) = &spell.scroll_spells {
                    for inner in scroll_spells {
                        if inner.raw_spell == raw_spell {
                            refs.push(SpellReference {
                                spell: raw_spell.to_string(),
                                occurrence: occurrence.clone(),
                            });
                        }
                    }
                } else if spell.raw_spell == raw_spell {
                    refs.push(SpellReference {
                        spell: raw_spell.to_string(),
                        occurrence,
                    });
                }
            }
        }

        Ok(refs)
    }

    pub fn spell_count(current_dir: &Path, raw_spell: &str) -> Result<u64, GrimoireCssError> {
        Ok(Self::refs_spell(current_dir, raw_spell)?.len() as u64)
    }

    pub fn stats_spells(
        current_dir: &Path,
        top: usize,
    ) -> Result<Vec<SpellFrequency>, GrimoireCssError> {
        let index = Self::index(current_dir, top)?;
        Ok(index.top_expanded_spells)
    }

    pub fn refs_variable(
        current_dir: &Path,
        variable: &str,
    ) -> Result<Vec<VariableReference>, GrimoireCssError> {
        let config_fs = Self::load_config(current_dir)?;
        let parser = Parser::new();

        let variable = if variable.starts_with("--") {
            variable.to_string()
        } else {
            format!("--{variable}")
        };

        let mut files = HashSet::<PathBuf>::new();
        for project in &config_fs.projects {
            for pattern in &project.input_paths {
                for path in Self::expand_input_pattern(current_dir, pattern)? {
                    if path.is_file() {
                        files.insert(path);
                    }
                }
            }
        }

        let mut file_list: Vec<PathBuf> = files.into_iter().collect();
        file_list.sort();

        let mut refs: Vec<VariableReference> = Vec::new();

        // Project files: scan token candidates and expand scroll/template spells.
        for file_path in &file_list {
            let content = fs::read_to_string(file_path)?;
            let line_index = LineIndex::new(&content);

            let mut candidates: Vec<(String, (usize, usize))> = Vec::new();
            parser.collect_candidates_all(&content, &mut candidates)?;

            for (token, (byte_offset, byte_len)) in candidates {
                let parsed = Spell::new(
                    &token,
                    &config_fs.shared_spells,
                    &config_fs.scrolls,
                    (byte_offset, byte_len),
                    None,
                );

                let spell = match parsed {
                    Ok(Some(s)) => s,
                    _ => continue,
                };

                let (line, column) = line_index.line_col(byte_offset);
                let occurrence = TokenOccurrence {
                    token: token.clone(),
                    file: Self::to_rel(current_dir, file_path),
                    byte_offset,
                    byte_len,
                    line,
                    column,
                };

                let expanded: Vec<&str> = if let Some(scroll_spells) = &spell.scroll_spells {
                    scroll_spells.iter().map(|s| s.raw_spell.as_str()).collect()
                } else {
                    vec![spell.raw_spell.as_str()]
                };

                for raw_spell in expanded {
                    let mut reads = Vec::new();
                    let mut writes = Vec::new();
                    Self::extract_css_variable_usage(raw_spell, &mut reads, &mut writes);

                    if reads.iter().any(|v| v == &variable) {
                        refs.push(VariableReference {
                            variable: variable.clone(),
                            kind: "read".to_string(),
                            spell: raw_spell.to_string(),
                            occurrence: occurrence.clone(),
                        });
                    }
                    if writes.iter().any(|v| v == &variable) {
                        refs.push(VariableReference {
                            variable: variable.clone(),
                            kind: "write".to_string(),
                            spell: raw_spell.to_string(),
                            occurrence: occurrence.clone(),
                        });
                    }
                }
            }
        }

        // Scroll config files: scan raw scroll spell strings in JSON.
        for file_path in Self::scroll_config_files(current_dir) {
            let content = fs::read_to_string(&file_path)?;
            let line_index = LineIndex::new(&content);
            let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
                GrimoireCssError::InvalidInput(format!(
                    "Failed to parse JSON in {}: {e}",
                    Self::to_rel(current_dir, &file_path)
                ))
            })?;

            let mut search_from: usize = 0;

            let Some(scrolls) = json.get("scrolls").and_then(|v| v.as_array()) else {
                continue;
            };

            for scroll in scrolls {
                // base spells
                if let Some(spells) = scroll.get("spells").and_then(|v| v.as_array()) {
                    for s in spells.iter().filter_map(|v| v.as_str()) {
                        Self::push_css_var_ref_if_match(
                            current_dir,
                            &file_path,
                            &content,
                            &line_index,
                            &variable,
                            s,
                            &mut search_from,
                            &mut refs,
                        )?;
                    }
                }

                // overloads: spellsByArgs
                if let Some(obj) = scroll.get("spellsByArgs").and_then(|v| v.as_object()) {
                    for (_k, arr) in obj {
                        let Some(spells) = arr.as_array() else {
                            continue;
                        };
                        for s in spells.iter().filter_map(|v| v.as_str()) {
                            Self::push_css_var_ref_if_match(
                                current_dir,
                                &file_path,
                                &content,
                                &line_index,
                                &variable,
                                s,
                                &mut search_from,
                                &mut refs,
                            )?;
                        }
                    }
                }
            }
        }

        Ok(refs)
    }

    #[allow(clippy::too_many_arguments)]
    fn push_css_var_ref_if_match(
        current_dir: &Path,
        file_path: &Path,
        content: &str,
        line_index: &LineIndex,
        variable: &str,
        raw_spell: &str,
        search_from: &mut usize,
        out: &mut Vec<VariableReference>,
    ) -> Result<(), GrimoireCssError> {
        let mut reads = Vec::new();
        let mut writes = Vec::new();
        Self::extract_css_variable_usage(raw_spell, &mut reads, &mut writes);

        let matched_reads = reads
            .into_iter()
            .filter(|v| v == variable)
            .collect::<Vec<_>>();
        let matched_writes = writes
            .into_iter()
            .filter(|v| v == variable)
            .collect::<Vec<_>>();

        if matched_reads.is_empty() && matched_writes.is_empty() {
            return Ok(());
        }

        // Find a stable byte range by locating the JSON string literal.
        let json_string = serde_json::to_string(raw_spell).map_err(|e| {
            GrimoireCssError::InvalidInput(format!(
                "Failed to encode JSON string for spell in {}: {e}",
                Self::to_rel(current_dir, file_path)
            ))
        })?;

        let mut found = None;
        if *search_from < content.len()
            && let Some(rel) = content[*search_from..].find(&json_string)
        {
            found = Some(*search_from + rel);
        }
        if found.is_none() {
            found = content.find(&json_string);
        }

        let Some(byte_offset) = found else {
            // Fallback: location unknown.
            for v in matched_reads {
                out.push(VariableReference {
                    variable: v,
                    kind: "read".to_string(),
                    spell: raw_spell.to_string(),
                    occurrence: TokenOccurrence {
                        token: raw_spell.to_string(),
                        file: Self::to_rel(current_dir, file_path),
                        byte_offset: 0,
                        byte_len: 0,
                        line: 1,
                        column: 1,
                    },
                });
            }
            for v in matched_writes {
                out.push(VariableReference {
                    variable: v,
                    kind: "write".to_string(),
                    spell: raw_spell.to_string(),
                    occurrence: TokenOccurrence {
                        token: raw_spell.to_string(),
                        file: Self::to_rel(current_dir, file_path),
                        byte_offset: 0,
                        byte_len: 0,
                        line: 1,
                        column: 1,
                    },
                });
            }
            return Ok(());
        };

        *search_from = byte_offset + json_string.len();
        let byte_len = json_string.len();
        let (line, column) = line_index.line_col(byte_offset);
        let occurrence = TokenOccurrence {
            token: raw_spell.to_string(),
            file: Self::to_rel(current_dir, file_path),
            byte_offset,
            byte_len,
            line,
            column,
        };

        for v in matched_reads {
            out.push(VariableReference {
                variable: v,
                kind: "read".to_string(),
                spell: raw_spell.to_string(),
                occurrence: occurrence.clone(),
            });
        }
        for v in matched_writes {
            out.push(VariableReference {
                variable: v,
                kind: "write".to_string(),
                spell: raw_spell.to_string(),
                occurrence: occurrence.clone(),
            });
        }

        Ok(())
    }

    pub fn list_grimoire_variables(
        current_dir: &Path,
    ) -> Result<Vec<GrimoireVariableDefinition>, GrimoireCssError> {
        let config_fs = Self::load_config(current_dir)?;
        let mut out = config_fs
            .variables
            .unwrap_or_default()
            .into_iter()
            .map(|(name, value)| GrimoireVariableDefinition { name, value })
            .collect::<Vec<_>>();
        out.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(out)
    }

    pub fn refs_grimoire_variable(
        current_dir: &Path,
        variable: &str,
    ) -> Result<Vec<GrimoireVariableReference>, GrimoireCssError> {
        let config_fs = Self::load_config(current_dir)?;
        let parser = Parser::new();

        let mut files = HashSet::<PathBuf>::new();
        for project in &config_fs.projects {
            for pattern in &project.input_paths {
                for path in Self::expand_input_pattern(current_dir, pattern)? {
                    if path.is_file() {
                        files.insert(path);
                    }
                }
            }
        }

        let mut file_list: Vec<PathBuf> = files.into_iter().collect();
        file_list.sort();

        let needle = format!("${variable}");
        let mut refs: Vec<GrimoireVariableReference> = Vec::new();

        for file_path in &file_list {
            let content = fs::read_to_string(file_path)?;
            let line_index = LineIndex::new(&content);

            let mut candidates: Vec<(String, (usize, usize))> = Vec::new();
            parser.collect_candidates_all(&content, &mut candidates)?;

            for (token, (byte_offset, byte_len)) in candidates {
                let parsed = Spell::new(
                    &token,
                    &config_fs.shared_spells,
                    &config_fs.scrolls,
                    (byte_offset, byte_len),
                    None,
                );

                let spell = match parsed {
                    Ok(Some(s)) => s,
                    _ => continue,
                };

                let (line, column) = line_index.line_col(byte_offset);
                let occurrence = TokenOccurrence {
                    token: token.clone(),
                    file: Self::to_rel(current_dir, file_path),
                    byte_offset,
                    byte_len,
                    line,
                    column,
                };

                let expanded: Vec<&str> = if let Some(scroll_spells) = &spell.scroll_spells {
                    scroll_spells.iter().map(|s| s.raw_spell.as_str()).collect()
                } else {
                    vec![spell.raw_spell.as_str()]
                };

                for raw_spell in expanded {
                    if raw_spell.contains(&needle) {
                        refs.push(GrimoireVariableReference {
                            variable: variable.to_string(),
                            spell: raw_spell.to_string(),
                            occurrence: occurrence.clone(),
                        });
                    }
                }
            }
        }

        // Also scan raw scroll definitions (grimoire.config.json and grimoire.*.scrolls.json)
        // for occurrences inside scroll spell strings.
        for file_path in Self::scroll_config_files(current_dir) {
            let content = fs::read_to_string(&file_path)?;
            let line_index = LineIndex::new(&content);
            let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
                GrimoireCssError::InvalidInput(format!(
                    "Failed to parse JSON in {}: {e}",
                    Self::to_rel(current_dir, &file_path)
                ))
            })?;

            let mut search_from: usize = 0;

            let Some(scrolls) = json.get("scrolls").and_then(|v| v.as_array()) else {
                continue;
            };

            for scroll in scrolls {
                // base spells
                if let Some(spells) = scroll.get("spells").and_then(|v| v.as_array()) {
                    for s in spells.iter().filter_map(|v| v.as_str()) {
                        Self::push_gvar_ref_if_match(
                            current_dir,
                            &file_path,
                            &content,
                            &line_index,
                            variable,
                            &needle,
                            s,
                            &mut search_from,
                            &mut refs,
                        )?;
                    }
                }

                // overloads: spellsByArgs
                if let Some(obj) = scroll.get("spellsByArgs").and_then(|v| v.as_object()) {
                    for (_k, arr) in obj {
                        let Some(spells) = arr.as_array() else {
                            continue;
                        };
                        for s in spells.iter().filter_map(|v| v.as_str()) {
                            Self::push_gvar_ref_if_match(
                                current_dir,
                                &file_path,
                                &content,
                                &line_index,
                                variable,
                                &needle,
                                s,
                                &mut search_from,
                                &mut refs,
                            )?;
                        }
                    }
                }
            }
        }

        Ok(refs)
    }

    fn scroll_config_files(current_dir: &Path) -> Vec<PathBuf> {
        let config_dir = current_dir.join("grimoire").join("config");
        if !config_dir.exists() {
            return Vec::new();
        }

        let mut out = Vec::new();
        let main = config_dir.join("grimoire.config.json");
        if main.is_file() {
            out.push(main);
        }

        let pattern = config_dir
            .join("grimoire.*.scrolls.json")
            .to_string_lossy()
            .to_string();
        if let Ok(entries) = glob(&pattern) {
            for p in entries.flatten() {
                if p.is_file() {
                    out.push(p);
                }
            }
        }

        out.sort();
        out.dedup();
        out
    }

    #[allow(clippy::too_many_arguments)]
    fn push_gvar_ref_if_match(
        current_dir: &Path,
        file_path: &Path,
        content: &str,
        line_index: &LineIndex,
        variable: &str,
        needle: &str,
        raw_spell: &str,
        search_from: &mut usize,
        out: &mut Vec<GrimoireVariableReference>,
    ) -> Result<(), GrimoireCssError> {
        if !raw_spell.contains(needle) {
            return Ok(());
        }

        // Find a stable byte range by locating the JSON string literal.
        let json_string = serde_json::to_string(raw_spell).map_err(|e| {
            GrimoireCssError::InvalidInput(format!(
                "Failed to encode JSON string for spell in {}: {e}",
                Self::to_rel(current_dir, file_path)
            ))
        })?;

        let mut found = None;
        if *search_from < content.len()
            && let Some(rel) = content[*search_from..].find(&json_string)
        {
            found = Some(*search_from + rel);
        }
        if found.is_none() {
            found = content.find(&json_string);
        }

        let Some(byte_offset) = found else {
            // If we can't locate it precisely, still return a reference without location.
            out.push(GrimoireVariableReference {
                variable: variable.to_string(),
                spell: raw_spell.to_string(),
                occurrence: TokenOccurrence {
                    token: raw_spell.to_string(),
                    file: Self::to_rel(current_dir, file_path),
                    byte_offset: 0,
                    byte_len: 0,
                    line: 1,
                    column: 1,
                },
            });
            return Ok(());
        };

        *search_from = byte_offset + json_string.len();

        let byte_len = json_string.len();
        let (line, column) = line_index.line_col(byte_offset);

        out.push(GrimoireVariableReference {
            variable: variable.to_string(),
            spell: raw_spell.to_string(),
            occurrence: TokenOccurrence {
                token: raw_spell.to_string(),
                file: Self::to_rel(current_dir, file_path),
                byte_offset,
                byte_len,
                line,
                column,
            },
        });

        Ok(())
    }

    pub fn lint(current_dir: &Path) -> Result<LintResult, GrimoireCssError> {
        let config_fs = Self::load_config(current_dir)?;
        let index = Self::index(current_dir, 200)?;
        let _config_path = Filesystem::get_config_path(current_dir)?;

        let mut errors: Vec<LintMessage> = Vec::new();
        let mut warnings: Vec<LintMessage> = Vec::new();
        let notes: Vec<LintMessage> = Vec::new();

        if !index.errors.is_empty() {
            let occurrence = index.errors.first().and_then(|e| {
                // Best-effort: map byte offset to (line, column) for click-to-open.
                let abs = current_dir.join(&e.file);
                let content = fs::read_to_string(&abs).ok()?;
                let (line, column) = line_col_from_byte_offset(&content, e.byte_offset);
                Some(TokenOccurrence {
                    token: "parse_error".to_string(),
                    file: e.file.clone(),
                    byte_offset: e.byte_offset,
                    byte_len: e.byte_len,
                    line,
                    column,
                })
            });

            errors.push(LintMessage {
                level: "error".to_string(),
                code: "parse_error".to_string(),
                message: format!(
                    "Encountered {} parse/compile errors while scanning project files",
                    index.errors.len()
                ),
                occurrence,
            });
        }

        if let Some(scrolls) = &config_fs.scrolls {
            // Overload sanity for actual usages.
            for r in &index.scroll_references {
                if r.arity == 0 {
                    continue;
                }
                if let Some(def) = scrolls.get(&r.scroll)
                    && let Some(map) = &def.spells_by_args
                    && !map.is_empty()
                {
                    let key = r.arity.to_string();
                    if !map.contains_key(&key) {
                        errors.push(LintMessage {
                            level: "error".to_string(),
                            code: "missing_overload".to_string(),
                            message: format!(
                                "Scroll '{}' is used with arity {}, but spellsByArgs['{}'] is not defined",
                                r.scroll, r.arity, key
                            ),
                            occurrence: Some(r.occurrence.clone()),
                        });
                    }
                }
            }
        }

        // Shared styles lint: warn about styles declared in `shared.styles` that are never used
        // in scanned project inputs (these would bloat output CSS).
        if let Some(shared) = &config_fs.shared {
            let parser = Parser::new();
            let mut files = HashSet::<PathBuf>::new();
            for project in &config_fs.projects {
                for pattern in &project.input_paths {
                    for path in Self::expand_input_pattern(current_dir, pattern)? {
                        if path.is_file() {
                            files.insert(path);
                        }
                    }
                }
            }

            let mut file_list: Vec<PathBuf> = files.into_iter().collect();
            file_list.sort();

            let mut used_tokens: HashSet<String> = HashSet::new();
            for file_path in &file_list {
                let Ok(content) = fs::read_to_string(file_path) else {
                    continue;
                };

                let mut candidates: Vec<(String, (usize, usize))> = Vec::new();
                parser.collect_candidates_all(&content, &mut candidates)?;
                for (token, _span) in candidates {
                    if token.is_empty() {
                        continue;
                    }
                    used_tokens.insert(token);
                }
            }

            let mut unused_shared: Vec<String> = Vec::new();
            for s in shared {
                let Some(styles) = &s.styles else {
                    continue;
                };
                for t in styles {
                    if t.is_empty() {
                        continue;
                    }

                    // Only lint Grimoire tokens/spells.
                    let parsed = Spell::new(
                        t,
                        &config_fs.shared_spells,
                        &config_fs.scrolls,
                        (0, 0),
                        None,
                    );
                    let Ok(Some(_)) = parsed else {
                        continue;
                    };

                    if !used_tokens.contains(t) {
                        unused_shared.push(t.clone());
                    }
                }
            }

            unused_shared.sort();
            unused_shared.dedup();

            if !unused_shared.is_empty() {
                warnings.push(LintMessage {
                    level: "warning".to_string(),
                    code: "unused_shared_style".to_string(),
                    message: format!(
                        "{} shared style(s) are configured but never used in scanned project inputs: {}",
                        unused_shared.len(),
                        unused_shared.join(", ")
                    ),
                    occurrence: None,
                });
            }
        }

        // Token lint: variables defined in cssCustomProperties but never referenced.
        let defined_tokens = Self::defined_css_custom_properties(&config_fs);
        if !defined_tokens.is_empty() {
            let used_tokens: HashSet<String> = index.css_variables_read.iter().cloned().collect();

            let mut unused_tokens: Vec<String> = defined_tokens
                .iter()
                .filter(|t| !used_tokens.contains(*t))
                .cloned()
                .collect();
            unused_tokens.sort();

            if !unused_tokens.is_empty() {
                warnings.push(LintMessage {
                    level: "warning".to_string(),
                    code: "unused_token".to_string(),
                    message: format!(
                        "{} token(s) are defined in cssCustomProperties but never read via var(--token): {}",
                        unused_tokens.len(),
                        unused_tokens.join(", ")
                    ),
                    occurrence: None,
                });
            }
        }

        Ok(LintResult {
            errors,
            warnings,
            notes,
        })
    }

    fn expand_input_pattern(
        current_dir: &Path,
        pattern: &str,
    ) -> Result<Vec<PathBuf>, GrimoireCssError> {
        let abs = current_dir.join(pattern);

        if abs.exists() && abs.is_dir() {
            let mut dir_pattern = glob::Pattern::escape(&abs.to_string_lossy());
            if !dir_pattern.ends_with('/') {
                dir_pattern.push('/');
            }
            dir_pattern.push_str("**/*");
            return Self::glob_paths(&dir_pattern);
        }

        // ConfigFs already expanded globs; resolved filenames must remain literal.
        Ok(vec![abs])
    }

    fn glob_paths(pattern: &str) -> Result<Vec<PathBuf>, GrimoireCssError> {
        let mut out = Vec::new();
        let entries = glob(pattern).map_err(|e| {
            GrimoireCssError::InvalidInput(format!("Invalid glob pattern '{pattern}': {e}"))
        })?;
        for entry in entries {
            match entry {
                Ok(path) => out.push(path),
                Err(e) => {
                    return Err(GrimoireCssError::InvalidInput(format!(
                        "Failed to expand glob '{pattern}': {e}"
                    )));
                }
            }
        }
        Ok(out)
    }

    fn to_rel(current_dir: &Path, p: &Path) -> String {
        p.strip_prefix(current_dir)
            .unwrap_or(p)
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, "/")
    }

    fn sorted_set(set: HashSet<String>) -> Vec<String> {
        let mut out: Vec<String> = set.into_iter().collect();
        out.sort();
        out
    }

    fn top_counts(map: HashMap<String, u64>, top: usize) -> Vec<SpellFrequency> {
        if top == 0 {
            return Vec::new();
        }

        let mut items: Vec<(String, u64)> = map.into_iter().collect();
        items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        items
            .into_iter()
            .take(top)
            .map(|(spell, count)| SpellFrequency { spell, count })
            .collect()
    }

    fn defined_css_custom_properties(config_fs: &ConfigFs) -> HashSet<String> {
        let mut set = HashSet::new();

        if let Some(shared) = &config_fs.shared {
            for s in shared {
                if let Some(props) = &s.css_custom_properties {
                    for p in props {
                        for (k, _) in &p.css_variables {
                            if k.starts_with("--") {
                                set.insert(k.clone());
                            } else {
                                set.insert(format!("--{k}"));
                            }
                        }
                    }
                }
            }
        }
        if let Some(critical) = &config_fs.critical {
            for c in critical {
                if let Some(props) = &c.css_custom_properties {
                    for p in props {
                        for (k, _) in &p.css_variables {
                            if k.starts_with("--") {
                                set.insert(k.clone());
                            } else {
                                set.insert(format!("--{k}"));
                            }
                        }
                    }
                }
            }
        }

        set
    }

    fn collect_css_variable_usage(
        raw_spell: &str,
        reads: &mut HashSet<String>,
        writes: &mut HashSet<String>,
    ) {
        let mut r = Vec::new();
        let mut w = Vec::new();
        Self::extract_css_variable_usage(raw_spell, &mut r, &mut w);
        for v in r {
            reads.insert(v);
        }
        for v in w {
            writes.insert(v);
        }
    }

    fn extract_css_variable_usage(
        raw_spell: &str,
        reads: &mut Vec<String>,
        writes: &mut Vec<String>,
    ) {
        // Writes: spell of the form "--token=value".
        if let Some(name) = Self::extract_css_variable_write(raw_spell) {
            writes.push(name);
        }

        // Reads: occurrences of "var(--token".
        let bytes = raw_spell.as_bytes();
        let mut i = 0;
        while i + 6 < bytes.len() {
            // "var(--" is 6 bytes.
            if bytes[i] == b'v'
                && bytes[i + 1] == b'a'
                && bytes[i + 2] == b'r'
                && bytes[i + 3] == b'('
                && bytes[i + 4] == b'-'
                && bytes[i + 5] == b'-'
            {
                let start = i + 4;
                let mut j = start;
                while j < bytes.len() {
                    let c = bytes[j];
                    let ok = c.is_ascii_lowercase()
                        || c.is_ascii_uppercase()
                        || c.is_ascii_digit()
                        || c == b'-'
                        || c == b'_';
                    if !ok {
                        break;
                    }
                    j += 1;
                }
                if j > start {
                    reads.push(String::from_utf8_lossy(&bytes[start..j]).to_string());
                }
                i = j;
                continue;
            }
            i += 1;
        }
    }

    fn extract_css_variable_write(raw_spell: &str) -> Option<String> {
        // Fast path: starts with "--" and contains '='.
        if !raw_spell.starts_with("--") {
            return None;
        }
        let eq = raw_spell.find('=')?;
        if eq <= 2 {
            return None;
        }
        let name = &raw_spell[..eq];
        if name.as_bytes().iter().all(|c| {
            (*c >= b'a' && *c <= b'z')
                || (*c >= b'A' && *c <= b'Z')
                || (*c >= b'0' && *c <= b'9')
                || *c == b'-'
                || *c == b'_'
        }) {
            Some(name.to_string())
        } else {
            None
        }
    }
}

fn line_col_from_byte_offset(content: &str, byte_offset: usize) -> (usize, usize) {
    let mut i = byte_offset.min(content.len());
    while i > 0 && !content.is_char_boundary(i) {
        i -= 1;
    }

    let prefix = &content[..i];
    let line = prefix.bytes().filter(|b| *b == b'\n').count();

    let last_nl = prefix.rfind('\n').map(|p| p + 1).unwrap_or(0);
    let col = prefix[last_nl..].chars().count();

    (line, col)
}

fn intersect_sorted(a: &[String], b: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut i = 0usize;
    let mut j = 0usize;
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                out.push(a[i].clone());
                i += 1;
                j += 1;
            }
        }
    }
    out
}

fn is_subset(needles: &[String], haystack_sorted: &[String]) -> bool {
    // both sorted
    let mut i = 0usize;
    let mut j = 0usize;
    while i < needles.len() && j < haystack_sorted.len() {
        match needles[i].cmp(&haystack_sorted[j]) {
            std::cmp::Ordering::Less => return false,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                i += 1;
                j += 1;
            }
        }
    }
    i == needles.len()
}

struct LineIndex {
    // Byte indices of '\n' characters.
    newlines: Vec<usize>,
}

impl LineIndex {
    fn new(content: &str) -> Self {
        let mut newlines = Vec::new();
        for (i, b) in content.as_bytes().iter().enumerate() {
            if *b == b'\n' {
                newlines.push(i);
            }
        }
        Self { newlines }
    }

    fn line_col(&self, byte_offset: usize) -> (usize, usize) {
        // line: 1-based, column: 1-based
        let line_idx = match self.newlines.binary_search(&byte_offset) {
            Ok(i) => i + 1,
            Err(i) => i,
        };

        let line = line_idx + 1;
        let last_nl = if line_idx == 0 {
            None
        } else {
            self.newlines.get(line_idx - 1).copied()
        };

        let col0 = match last_nl {
            Some(nl) => byte_offset.saturating_sub(nl + 1),
            None => byte_offset,
        };
        (line, col0 + 1)
    }
}

#[cfg(all(test, feature = "mcp"))]
mod mcp_import_transaction_tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn no_replace_publication_cannot_clobber_an_existing_target() {
        let dir = tempdir().unwrap();
        let target = dir.path().join("grimoire.concurrent.scrolls.json");
        atomic_write(&target, b"first writer", false).unwrap();

        assert!(atomic_write(&target, b"second writer", false).is_err());
        assert_eq!(fs::read(&target).unwrap(), b"first writer");
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn explicit_replace_publishes_complete_contents_without_temp_files() {
        let dir = tempdir().unwrap();
        let target = dir.path().join("grimoire.import.scrolls.json");
        atomic_write(&target, b"original contents", false).unwrap();

        atomic_write(&target, b"replacement", true).unwrap();

        assert_eq!(fs::read(&target).unwrap(), b"replacement");
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn concurrent_no_replace_writers_publish_exactly_one_complete_file() {
        let dir = tempdir().unwrap();
        let target = dir.path().join("grimoire.concurrent.scrolls.json");
        let barrier = std::sync::Barrier::new(8);
        let results = std::thread::scope(|scope| {
            let handles = (0..8u8)
                .map(|writer| {
                    let target = &target;
                    let barrier = &barrier;
                    scope.spawn(move || {
                        let bytes = vec![writer; 4096];
                        barrier.wait();
                        (bytes.clone(), atomic_write(target, &bytes, false).is_ok())
                    })
                })
                .collect::<Vec<_>>();
            handles
                .into_iter()
                .map(|handle| handle.join().unwrap())
                .collect::<Vec<_>>()
        });

        let winners = results
            .iter()
            .filter(|(_, success)| *success)
            .collect::<Vec<_>>();
        assert_eq!(winners.len(), 1);
        assert_eq!(fs::read(&target).unwrap(), winners[0].0);
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn failed_publication_preserves_destination_and_cleans_temp_files() {
        for replace in [false, true] {
            let dir = tempdir().unwrap();
            let target = dir.path().join("grimoire.import.scrolls.json");
            fs::create_dir(&target).unwrap();
            fs::write(target.join("sentinel"), b"keep").unwrap();

            assert!(atomic_write(&target, b"replacement", replace).is_err());
            assert_eq!(fs::read(target.join("sentinel")).unwrap(), b"keep");
            assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
        }
    }

    #[cfg(unix)]
    #[test]
    fn import_permissions_match_regular_file_creation() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempdir().unwrap();
        let regular = dir.path().join("regular");
        let target = dir.path().join("grimoire.import.scrolls.json");
        fs::write(&regular, b"regular").unwrap();

        for replace in [false, true] {
            atomic_write(&target, b"import", replace).unwrap();
            assert_eq!(
                fs::metadata(&target).unwrap().permissions().mode(),
                fs::metadata(&regular).unwrap().permissions().mode()
            );
        }
    }

    #[test]
    fn rollback_refuses_to_clobber_a_concurrently_changed_target() {
        let dir = tempdir().unwrap();
        let target = dir.path().join("grimoire.concurrent.scrolls.json");
        fs::write(&target, b"other writer").unwrap();

        assert!(restore_import(&target, Some(b"original"), b"our import").is_err());
        assert_eq!(fs::read(&target).unwrap(), b"other writer");
    }
}
