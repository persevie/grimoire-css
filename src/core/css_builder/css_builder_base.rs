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
        let mut base_rules = Vec::new();
        let mut media_queries = Vec::new();

        for spell in spells {
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
                        media_queries.push(wrapped_css);
                    } else {
                        base_rules.push(wrapped_css);
                    }

                    for add_css in local_scroll_additional_css_vec {
                        base_rules.push(add_css);
                    }
                }
                _ => {
                    if let Some(css) = self.css_generator.generate_css(spell)? {
                        if css.0.trim_start().starts_with("@media") {
                            media_queries.push(css.0);
                        } else {
                            base_rules.push(css.0);
                        }

                        if let Some(additional_css) = css.2 {
                            base_rules.push(additional_css);
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
            match (extract_min_width(a), extract_min_width(b)) {
                (Some(aw), Some(bw)) => aw.cmp(&bw),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a.cmp(b),
            }
        });
        base_rules.extend(media_queries);
        Ok(base_rules)
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
}
