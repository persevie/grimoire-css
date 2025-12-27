//! Provides the `CSSBuilder` struct and its associated methods for compiling and building CSS files based on a configuration.
//!
//! Both filesystem and in-memory builders extend this functionality.

use crate::core::{CssOptimizer, GrimoireCssError, css_generator::CssGenerator, spell::Spell};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct PieceRange {
    start: usize,
    end: usize,
    spell_index: usize,
}

#[derive(Debug, Clone)]
struct MediaEntry {
    min_width: Option<u32>,
    start: usize,
    end: usize,
    spell_index: usize,
}

/// Core CSS builder that handles spell compilation and optimization
pub struct CssBuilder<'a> {
    css_generator: CssGenerator<'a>,
    optimizer: &'a dyn CssOptimizer,
}

impl<'a> CssBuilder<'a> {
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
        optimizer: &'a O,
        variables: &'a Option<Vec<(String, String)>>,
        custom_animations: &'a HashMap<String, String>,
    ) -> Result<Self, GrimoireCssError> {
        let css_generator = CssGenerator::new(variables, custom_animations)?;

        Ok(Self {
            css_generator,
            optimizer,
        })
    }

    /// Combines spells into CSS strings.
    ///
    /// # Arguments
    ///
    /// * `spells` - Vector of `Spell` instances to convert into CSS.
    ///
    /// # Returns
    ///
    /// Vector of CSS strings corresponding to the provided spells.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if CSS generation fails.
    #[allow(dead_code)]
    pub fn combine_spells_to_css(&self, spells: &[Spell]) -> Result<Vec<String>, GrimoireCssError> {
        let (raw_css, pieces) = self.build_joined_css_and_pieces(spells)?;
        self.validate_or_isolate(spells, &raw_css, &pieces)?;
        Ok(pieces
            .iter()
            .map(|p| raw_css[p.start..p.end].to_string())
            .collect())
    }

    /// Memory-efficient variant that returns a single joined CSS string.
    pub fn combine_spells_to_css_string(
        &self,
        spells: &[Spell],
    ) -> Result<String, GrimoireCssError> {
        let (raw_css, pieces) = self.build_joined_css_and_pieces(spells)?;
        self.validate_or_isolate(spells, &raw_css, &pieces)?;
        Ok(raw_css)
    }

    /// Builds and returns optimized CSS in one step.
    ///
    /// This avoids the common `validate()` then `optimize()` double-parse on the success path.
    /// On failure, it still performs rule isolation to produce a precise, spell-linked error.
    pub fn combine_spells_to_optimized_css_string(
        &self,
        spells: &[Spell],
    ) -> Result<String, GrimoireCssError> {
        let (raw_css, pieces) = self.build_joined_css_and_pieces(spells)?;

        match self.optimizer.optimize(&raw_css) {
            Ok(css) => Ok(css),
            Err(optimize_err) => match self.validate_or_isolate(spells, &raw_css, &pieces) {
                // Optimization may fail even if parsing succeeds (e.g. minify stage).
                Ok(()) => Err(optimize_err),
                Err(isolated_err) => Err(isolated_err),
            },
        }
    }

    /// Optimizes and minifies CSS.
    ///
    /// # Arguments
    ///
    /// * `raw_css` - Raw CSS string to optimize.
    ///
    /// # Returns
    ///
    /// Optimized and minified CSS string.
    pub fn optimize_css(&self, raw_css: &str) -> Result<String, GrimoireCssError> {
        self.optimizer.optimize(raw_css)
    }

    fn create_compile_error(&self, spell: &Spell, error: GrimoireCssError) -> GrimoireCssError {
        GrimoireCssError::CompileError {
            message: format!("Invalid CSS generated: {}", error),
            span: spell.span,
            label: "This spell generated invalid CSS".to_string(),
            help: Some(
                "This usually means the spell value is not valid CSS after Grimoire transformations.\n\
If you intended spaces inside a value, encode them as '_' (underscores)."
                    .to_string(),
            ),
            source_file: spell.source.clone(),
        }
    }

    fn build_joined_css_and_pieces(
        &self,
        spells: &[Spell],
    ) -> Result<(String, Vec<PieceRange>), GrimoireCssError> {
        use once_cell::sync::Lazy;

        static MIN_WIDTH_RE: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"min-width:\s*(\\d+)").unwrap());

        fn extract_min_width(re: &regex::Regex, s: &str) -> Option<u32> {
            re.captures(s)
                .and_then(|cap| cap.get(1))
                .and_then(|m| m.as_str().parse::<u32>().ok())
        }

        let mut base_css = String::new();
        let mut base_pieces: Vec<PieceRange> = Vec::new();

        let mut media_css = String::new();
        let mut media_entries: Vec<MediaEntry> = Vec::new();

        for (spell_index, spell) in spells.iter().enumerate() {
            match &spell.scroll_spells {
                Some(ss) if !ss.is_empty() => {
                    let mut combined_scroll_css = String::new();

                    for s in ss {
                        if let Some(css) = self.css_generator.generate_css(s)? {
                            let class_name = self.css_generator.generate_css_class_name(
                                &spell.raw_spell,
                                spell.effects(),
                                spell.focus(),
                                spell.with_template,
                            )?;

                            let updated_css = self.css_generator.replace_class_name(
                                &css.1.1,
                                &class_name.0,
                                &css.0,
                            );

                            combined_scroll_css.push_str(&updated_css);

                            if let Some(additional_css) = css.2 {
                                let start = base_css.len();
                                base_css.push_str(&additional_css);
                                let end = base_css.len();
                                base_pieces.push(PieceRange {
                                    start,
                                    end,
                                    spell_index,
                                });
                            }
                        }
                    }

                    let wrapped_css = if spell.area().is_empty() {
                        combined_scroll_css
                    } else {
                        self.css_generator
                            .wrap_base_css_with_media_query(spell.area(), &combined_scroll_css)
                    };

                    if wrapped_css.trim_start().starts_with("@media") {
                        let start = media_css.len();
                        media_css.push_str(&wrapped_css);
                        let end = media_css.len();
                        media_entries.push(MediaEntry {
                            min_width: extract_min_width(&MIN_WIDTH_RE, &wrapped_css),
                            start,
                            end,
                            spell_index,
                        });
                    } else {
                        let start = base_css.len();
                        base_css.push_str(&wrapped_css);
                        let end = base_css.len();
                        base_pieces.push(PieceRange {
                            start,
                            end,
                            spell_index,
                        });
                    }
                }
                _ => {
                    if let Some(css) = self.css_generator.generate_css(spell)? {
                        if css.0.trim_start().starts_with("@media") {
                            let start = media_css.len();
                            media_css.push_str(&css.0);
                            let end = media_css.len();
                            media_entries.push(MediaEntry {
                                min_width: extract_min_width(&MIN_WIDTH_RE, &css.0),
                                start,
                                end,
                                spell_index,
                            });
                        } else {
                            let start = base_css.len();
                            base_css.push_str(&css.0);
                            let end = base_css.len();
                            base_pieces.push(PieceRange {
                                start,
                                end,
                                spell_index,
                            });
                        }

                        if let Some(additional_css) = css.2 {
                            let start = base_css.len();
                            base_css.push_str(&additional_css);
                            let end = base_css.len();
                            base_pieces.push(PieceRange {
                                start,
                                end,
                                spell_index,
                            });
                        }
                    }
                }
            }
        }

        // Sort media queries by min-width, then by the text itself (stable deterministic output).
        media_entries.sort_by(|a, b| match (a.min_width, b.min_width) {
            (Some(aw), Some(bw)) => aw.cmp(&bw),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => {
                let aslice = &media_css[a.start..a.end];
                let bslice = &media_css[b.start..b.end];
                aslice.cmp(bslice)
            }
        });

        // Final output: base rules, then sorted media queries.
        let mut raw_css = base_css;
        let mut pieces = base_pieces;

        raw_css.reserve(media_css.len());

        for m in media_entries {
            let start = raw_css.len();
            raw_css.push_str(&media_css[m.start..m.end]);
            let end = raw_css.len();
            pieces.push(PieceRange {
                start,
                end,
                spell_index: m.spell_index,
            });
        }

        Ok((raw_css, pieces))
    }

    fn validate_or_isolate(
        &self,
        spells: &[Spell],
        raw_css: &str,
        pieces: &[PieceRange],
    ) -> Result<(), GrimoireCssError> {
        if pieces.is_empty() {
            return Ok(());
        }

        if let Err(e) = self.optimizer.validate(raw_css) {
            if let Some((spell_index, rule_error)) = self.find_first_invalid_piece(raw_css, pieces)
            {
                return Err(self.create_compile_error(&spells[spell_index], rule_error));
            }

            if let Some(first) = spells.first() {
                return Err(self.create_compile_error(first, e));
            }
            return Err(e);
        }

        Ok(())
    }

    /// Returns the first invalid piece in source order (by spell index), if any.
    fn find_first_invalid_piece(
        &self,
        raw_css: &str,
        pieces: &[PieceRange],
    ) -> Option<(usize, GrimoireCssError)> {
        if pieces.is_empty() {
            return None;
        }

        // If the entire slice validates, nothing to isolate.
        let full_start = pieces.first()?.start;
        let full_end = pieces.last()?.end;
        if self
            .optimizer
            .validate(&raw_css[full_start..full_end])
            .is_ok()
        {
            return None;
        }

        if pieces.len() == 1 {
            let p = pieces[0];
            let rule_error = self.optimizer.validate(&raw_css[p.start..p.end]).err()?;
            return Some((p.spell_index, rule_error));
        }

        let mid = pieces.len() / 2;
        let (left, right) = pieces.split_at(mid);

        let left_start = left.first()?.start;
        let left_end = left.last()?.end;
        if self
            .optimizer
            .validate(&raw_css[left_start..left_end])
            .is_err()
        {
            return self.find_first_invalid_piece(raw_css, left);
        }

        self.find_first_invalid_piece(raw_css, right)
    }
}
