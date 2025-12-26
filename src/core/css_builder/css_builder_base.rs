//! Provides the `CSSBuilder` struct and its associated methods for compiling and building CSS files based on a configuration.
//!
//! Both filesystem and in-memory builders extend this functionality.

use crate::core::{CssOptimizer, GrimoireCssError, css_generator::CssGenerator, spell::Spell};
use std::collections::HashMap;

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
    pub fn combine_spells_to_css(&self, spells: &[Spell]) -> Result<Vec<String>, GrimoireCssError> {
        let mut base_rules: Vec<(String, usize)> = Vec::new();
        let mut media_queries: Vec<(String, usize)> = Vec::new();

        for (spell_index, spell) in spells.iter().enumerate() {
            match &spell.scroll_spells {
                Some(ss) if !ss.is_empty() => {
                    let mut local_scroll_css_vec = Vec::new();
                    let mut local_scroll_additional_css_vec = Vec::new();

                    for s in ss {
                        if let Some(css) = self.css_generator.generate_css(s)? {
                            let class_name = self.css_generator.generate_css_class_name(
                                &spell.raw_spell,
                                &spell.effects,
                                &spell.focus,
                                spell.with_template,
                            )?;

                            let updated_css = self.css_generator.replace_class_name(
                                &css.1.1,
                                &class_name.0,
                                &css.0,
                            );

                            local_scroll_css_vec.push(updated_css);

                            if let Some(additional_css) = css.2 {
                                local_scroll_additional_css_vec.push(additional_css);
                            }
                        }
                    }

                    let combined_css = local_scroll_css_vec.join("");
                    let wrapped_css = if spell.area.is_empty() {
                        combined_css
                    } else {
                        self.css_generator
                            .wrap_base_css_with_media_query(&spell.area, &combined_css)
                    };
                    if wrapped_css.trim_start().starts_with("@media") {
                        media_queries.push((wrapped_css, spell_index));
                    } else {
                        base_rules.push((wrapped_css, spell_index));
                    }

                    for add_css in local_scroll_additional_css_vec {
                        base_rules.push((add_css, spell_index));
                    }
                }
                _ => {
                    if let Some(css) = self.css_generator.generate_css(spell)? {
                        if css.0.trim_start().starts_with("@media") {
                            media_queries.push((css.0, spell_index));
                        } else {
                            base_rules.push((css.0, spell_index));
                        }

                        if let Some(additional_css) = css.2 {
                            base_rules.push((additional_css, spell_index));
                        }
                    }
                }
            }
        }

        media_queries.sort_by(|a, b| {
            fn extract_min_width(s: &str) -> Option<u32> {
                let re = regex::Regex::new(r"min-width:\s*(\\d+)").unwrap();
                re.captures(s)
                    .and_then(|cap| cap.get(1))
                    .and_then(|m| m.as_str().parse::<u32>().ok())
            }
            match (extract_min_width(&a.0), extract_min_width(&b.0)) {
                (Some(aw), Some(bw)) => aw.cmp(&bw),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a.0.cmp(&b.0),
            }
        });
        base_rules.extend(media_queries);

        // Validate the combined output once (fast path).
        if let Err(e) = self.validate_joined_css(&base_rules) {
            if let Some((spell_index, rule_error)) = self.find_first_invalid_rule(&base_rules) {
                return Err(self.create_compile_error(&spells[spell_index], rule_error));
            }

            // Fallback: no rule isolated (should be rare), attach to first spell if available.
            if let Some(first) = spells.first() {
                return Err(self.create_compile_error(first, e));
            }
            return Err(e);
        }

        Ok(base_rules.into_iter().map(|(css, _)| css).collect())
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

    fn validate_joined_css(&self, rules: &[(String, usize)]) -> Result<(), GrimoireCssError> {
        if rules.is_empty() {
            return Ok(());
        }
        let mut joined = String::new();
        for (css, _) in rules {
            joined.push_str(css);
        }
        self.optimizer.validate(&joined)
    }

    fn validate_rules_slice(&self, rules: &[(String, usize)]) -> Result<(), GrimoireCssError> {
        if rules.is_empty() {
            return Ok(());
        }
        let mut joined = String::new();
        for (css, _) in rules {
            joined.push_str(css);
        }
        self.optimizer.validate(&joined)
    }

    /// Returns the first invalid rule in source order (by spell index), if any.
    fn find_first_invalid_rule(
        &self,
        rules: &[(String, usize)],
    ) -> Option<(usize, GrimoireCssError)> {
        if rules.is_empty() {
            return None;
        }

        // If the entire set validates, nothing to isolate.
        if self.validate_rules_slice(rules).is_ok() {
            return None;
        }

        if rules.len() == 1 {
            let rule_error = self.optimizer.validate(&rules[0].0).err()?;
            return Some((rules[0].1, rule_error));
        }

        let mid = rules.len() / 2;
        let (left, right) = rules.split_at(mid);

        if self.validate_rules_slice(left).is_err() {
            return self.find_first_invalid_rule(left);
        }

        self.find_first_invalid_rule(right)
    }
}
