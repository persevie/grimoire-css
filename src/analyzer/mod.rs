use crate::{
    GrimoireCssError, Spell,
    config::{ConfigFs, ConfigInMemory, ConfigInMemoryEntry},
    core::{Filesystem, parser::Parser},
};
use serde::Serialize;
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

impl Analyzer {
    pub fn load_config(current_dir: &Path) -> Result<ConfigFs, GrimoireCssError> {
        ConfigFs::load(current_dir)
    }

    /// Explain a single class token (e.g. `md3-btn`, `box=10px_20px`, `hover:bg-c=red`).
    ///
    /// Returns:
    /// - resolved spells (scroll expansion if applicable)
    /// - compiled CSS for a minimal HTML snippet containing that class token
    pub fn explain_class_token(
        current_dir: &Path,
        class_token: &str,
    ) -> Result<ExplainClassTokenResult, GrimoireCssError> {
        let config_fs = Self::load_config(current_dir)?;

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

        // Compile a tiny in-memory project so we can show the *actual* generated CSS.
        // This is heavier than a pure expansion, but it's feature-gated and ideal for IDE tooling.
        let html = format!("<div class=\"{class_token}\"></div>");

        let config_in_memory = ConfigInMemory {
            projects: vec![ConfigInMemoryEntry {
                name: "explain".to_string(),
                content: vec![html],
            }],
            variables: config_fs.variables.clone(),
            scrolls: config_fs.scrolls.clone(),
            custom_animations: config_fs.custom_animations.clone(),
            browserslist_content: None,
        };

        let compiled = crate::start_in_memory_pretty(&config_in_memory)?;
        let css = compiled
            .into_iter()
            .next()
            .map(|c| c.content)
            .unwrap_or_default();

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

        let scroll_pattern = config_dir
            .join("grimoire.*.scrolls.json")
            .to_string_lossy()
            .to_string();
        let variable_pattern = config_dir
            .join("grimoire.*.variables.json")
            .to_string_lossy()
            .to_string();

        let mut external_scroll_files = Self::glob_paths(&scroll_pattern)?
            .into_iter()
            .map(|p| Self::to_rel(current_dir, &p))
            .collect::<Vec<_>>();
        external_scroll_files.sort();

        let mut external_variable_files = Self::glob_paths(&variable_pattern)?
            .into_iter()
            .map(|p| Self::to_rel(current_dir, &p))
            .collect::<Vec<_>>();
        external_variable_files.sort();

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
            let mut dir_pattern = abs.to_string_lossy().to_string();
            if !dir_pattern.ends_with('/') {
                dir_pattern.push('/');
            }
            dir_pattern.push_str("**/*");
            return Self::glob_paths(&dir_pattern);
        }

        // Always try glob expansion (supports both globs and plain paths).
        Self::glob_paths(&abs.to_string_lossy())
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
            .to_string()
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
