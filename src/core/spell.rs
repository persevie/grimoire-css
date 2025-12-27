//! This module defines the `Spell` struct, which represents a parsed CSS spell string.
//!
//! A spell is a complex string format used to encode CSS properties and their associated targets,
//! including screen size (area), pseudo-classes (effects), and specific focus areas.
//! The `Spell` struct provides methods to parse such a string into its components and store
//! them in a structured way.
//!
//! # Example
//!
//! A spell string might look like this:
//!
//! ```text
//! "md__{_>_p}hover:display=none"
//! ```
//!
//! This string is parsed into the following components:
//!
//! * `area`: "md" (medium screen size)
//! * `focus`: "_>_p" (a specific selector path)
//! * `effects`: "hover" (pseudo-class)
//! * `component`: "display" (CSS property)
//! * `component_target`: "none" (CSS value)
//!
//! The `Spell` struct is created by passing a spell string to the `Spell::new` function, which
//! parses the string and returns a `Result` containing either the parsed `Spell` or a `GrimoireCSSError`
//! if the string format is invalid.

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use super::{
    GrimoireCssError, component::get_css_property, source_file::SourceFile, spell_value_validator,
};

#[derive(Debug, Clone)]
struct SpellParts {
    area: std::ops::Range<usize>,
    focus: std::ops::Range<usize>,
    effects: std::ops::Range<usize>,
    component: std::ops::Range<usize>,
    component_target: std::ops::Range<usize>,
}

#[derive(Debug, Clone)]
pub struct Spell {
    pub raw_spell: String,
    pub with_template: bool,
    pub scroll_spells: Option<Vec<Spell>>,
    pub span: (usize, usize),
    pub source: Option<Arc<SourceFile>>,
    parts: Option<SpellParts>,
}

impl PartialEq for Spell {
    fn eq(&self, other: &Self) -> bool {
        self.raw_spell == other.raw_spell
            && self.with_template == other.with_template
            && self.scroll_spells == other.scroll_spells
    }
}

impl Eq for Spell {}

impl Hash for Spell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw_spell.hash(state);
        self.with_template.hash(state);
        self.scroll_spells.hash(state);
    }
}

impl Spell {
    pub fn area(&self) -> &str {
        self.parts
            .as_ref()
            .map(|p| &self.raw_spell[p.area.clone()])
            .unwrap_or("")
    }

    pub fn focus(&self) -> &str {
        self.parts
            .as_ref()
            .map(|p| &self.raw_spell[p.focus.clone()])
            .unwrap_or("")
    }

    pub fn effects(&self) -> &str {
        self.parts
            .as_ref()
            .map(|p| &self.raw_spell[p.effects.clone()])
            .unwrap_or("")
    }

    pub fn component(&self) -> &str {
        self.parts
            .as_ref()
            .map(|p| &self.raw_spell[p.component.clone()])
            .unwrap_or("")
    }

    pub fn component_target(&self) -> &str {
        self.parts
            .as_ref()
            .map(|p| &self.raw_spell[p.component_target.clone()])
            .unwrap_or("")
    }

    /// Example input: "md__{_>_p}hover:display=none"
    pub fn new(
        raw_spell: &str,
        shared_spells: &HashSet<String>,
        scrolls: &Option<HashMap<String, Vec<String>>>,
        span: (usize, usize),
        source: Option<Arc<SourceFile>>,
    ) -> Result<Option<Self>, GrimoireCssError> {
        let with_template = Self::check_for_template(raw_spell);
        let raw_spell_cleaned = if with_template {
            raw_spell
                .strip_prefix("g!")
                .and_then(|s| s.strip_suffix(";"))
                .unwrap_or(raw_spell)
        } else {
            raw_spell
        };

        let raw_spell_split: Vec<&str> = raw_spell_cleaned
            .split("--")
            .filter(|s| !s.is_empty())
            .collect();

        // Template spell: keep outer spell and parse inner spells.
        if with_template && !raw_spell_split.is_empty() {
            let mut scroll_spells: Vec<Spell> = Vec::new();

            for rs in raw_spell_split {
                if let Some(spell) = Spell::new(rs, shared_spells, scrolls, span, source.clone())? {
                    let mut spell = spell;

                    // If a template part is a scroll invocation (e.g. complex-card=120px_red_100px),
                    // `Spell::new` will produce a *container spell* whose `scroll_spells` are the
                    // real property spells.
                    //
                    // For templates we want to flatten those property spells into the template list
                    // so the builder can generate CSS and unify the class name to the outer template.
                    let area = spell.area().to_string();
                    let focus = spell.focus().to_string();
                    let effects = spell.effects().to_string();

                    if let Some(inner_scroll_spells) = spell.scroll_spells.take() {
                        let has_prefix =
                            !area.is_empty() || !focus.is_empty() || !effects.is_empty();

                        if has_prefix {
                            let mut prefix = String::new();

                            if !area.is_empty() {
                                prefix.push_str(&area);
                                prefix.push_str("__");
                            }

                            if !focus.is_empty() {
                                prefix.push('{');
                                prefix.push_str(&focus);
                                prefix.push('}');
                            }

                            if !effects.is_empty() {
                                prefix.push_str(&effects);
                                prefix.push(':');
                            }

                            for inner in inner_scroll_spells {
                                let combined = format!("{prefix}{}", inner.raw_spell);
                                if let Some(reparsed) = Spell::new(
                                    &combined,
                                    shared_spells,
                                    scrolls,
                                    span,
                                    source.clone(),
                                )? {
                                    scroll_spells.push(reparsed);
                                }
                            }
                        } else {
                            scroll_spells.extend(inner_scroll_spells);
                        }
                    } else {
                        scroll_spells.push(spell);
                    }
                }
            }

            return Ok(Some(Spell {
                raw_spell: raw_spell_cleaned.to_string(),
                with_template,
                scroll_spells: Some(scroll_spells),
                span,
                source,
                parts: None,
            }));
        }

        let raw = raw_spell_cleaned.to_string();

        // Parse into byte ranges within `raw`.
        let mut area_range = 0..0;
        let mut focus_range = 0..0;
        let mut effects_range = 0..0;

        let mut rest_start = 0usize;
        if let Some(pos) = raw.find("__") {
            area_range = 0..pos;
            rest_start = pos + 2;
        }

        let mut after_focus_start = rest_start;
        if rest_start < raw.len()
            && let Some(close_rel) = raw[rest_start..].find('}')
        {
            let focus_part_start = if raw.as_bytes().get(rest_start) == Some(&b'{') {
                rest_start + 1
            } else {
                rest_start
            };
            focus_range = focus_part_start..(rest_start + close_rel);
            after_focus_start = rest_start + close_rel + 1;
        }

        let mut after_effects_start = after_focus_start;
        if after_focus_start < raw.len()
            && let Some(colon_rel) = raw[after_focus_start..].find(':')
        {
            effects_range = after_focus_start..(after_focus_start + colon_rel);
            after_effects_start = after_focus_start + colon_rel + 1;
        }

        // component=target
        if after_effects_start <= raw.len()
            && let Some(eq_rel) = raw[after_effects_start..].find('=')
        {
            let component_range = after_effects_start..(after_effects_start + eq_rel);
            let component_target_range = (after_effects_start + eq_rel + 1)..raw.len();

            let component_target = &raw[component_target_range.clone()];
            if let Some(err) = spell_value_validator::validate_component_target(component_target) {
                let message = match err {
                    spell_value_validator::SpellValueValidationError::UnexpectedClosingParen => {
                        format!(
                            "Invalid value '{component_target}': unexpected ')'.\n\n\
If you intended a CSS function (e.g. calc(...)), ensure parentheses are balanced."
                        )
                    }
                    spell_value_validator::SpellValueValidationError::UnclosedParen => {
                        format!(
                            "Invalid value '{component_target}': unclosed '('.\n\n\
Common cause: spaces inside a class attribute split the spell into multiple tokens.\n\
Fix: replace spaces with '_' inside the value, e.g.:\n\
  h=calc(100vh - 50px)  ->  h=calc(100vh_-_50px)"
                        )
                    }
                };

                if let Some(src) = &source {
                    return Err(GrimoireCssError::CompileError {
                        message,
                        span,
                        label: "invalid spell value".to_string(),
                        help: Some(
                            "In HTML class attributes, spaces split classes.\n\
Use '_' inside spell values to represent spaces."
                                .to_string(),
                        ),
                        source_file: Some(src.clone()),
                    });
                }

                return Err(GrimoireCssError::InvalidInput(message));
            }

            let parts = SpellParts {
                area: area_range,
                focus: focus_range,
                effects: effects_range,
                component: component_range.clone(),
                component_target: component_target_range.clone(),
            };

            let mut spell = Spell {
                raw_spell: raw,
                with_template,
                scroll_spells: None,
                span,
                source: source.clone(),
                parts: Some(parts),
            };

            let component = spell.component();

            if let Some(raw_scroll_spells) = Self::check_raw_scroll_spells(component, scrolls) {
                spell.scroll_spells = Self::parse_scroll(
                    component,
                    raw_scroll_spells,
                    spell.component_target(),
                    shared_spells,
                    scrolls,
                    span,
                    source,
                )?;
            } else if !component.starts_with("--") && get_css_property(component).is_none() {
                let message = format!("Unknown component or scroll: '{component}'");
                if let Some(src) = &source {
                    return Err(GrimoireCssError::InvalidSpellFormat {
                            message,
                            span,
                            label: "Error in this spell".to_string(),
                            help: Some(
                                "Check that the component name exists (built-in CSS property alias) or that the scroll is defined in config.scrolls."
                                    .to_string(),
                            ),
                            source_file: Some(src.clone()),
                        });
                } else {
                    return Err(GrimoireCssError::InvalidInput(message));
                }
            }

            return Ok(Some(spell));
        }

        // scroll (no '=')
        if after_effects_start <= raw.len()
            && let Some(raw_scroll_spells) =
                Self::check_raw_scroll_spells(&raw[after_effects_start..], scrolls)
        {
            let component_range = after_effects_start..raw.len();
            let parts = SpellParts {
                area: area_range,
                focus: focus_range,
                effects: effects_range,
                component: component_range.clone(),
                component_target: 0..0,
            };

            let mut spell = Spell {
                raw_spell: raw,
                with_template,
                scroll_spells: None,
                span,
                source: source.clone(),
                parts: Some(parts),
            };

            let component = spell.component();
            spell.scroll_spells = Self::parse_scroll(
                component,
                raw_scroll_spells,
                "",
                shared_spells,
                scrolls,
                span,
                source,
            )?;

            return Ok(Some(spell));
        }

        Ok(None) // Return None if format is invalid
    }

    fn check_for_template(raw_spell: &str) -> bool {
        raw_spell.starts_with("g!") && raw_spell.ends_with(';')
    }

    fn check_raw_scroll_spells<'a>(
        scroll_name: &str,
        scrolls: &'a Option<HashMap<String, Vec<String>>>,
    ) -> Option<&'a Vec<String>> {
        scrolls.as_ref()?.get(scroll_name)
    }

    #[allow(clippy::too_many_arguments)]
    fn parse_scroll(
        scroll_name: &str,
        raw_scroll_spells: &[String],
        component_target: &str,
        shared_spells: &HashSet<String>,
        scrolls: &Option<HashMap<String, Vec<String>>>,
        span: (usize, usize),
        source: Option<Arc<SourceFile>>,
    ) -> Result<Option<Vec<Spell>>, GrimoireCssError> {
        if raw_scroll_spells.is_empty() {
            return Ok(None);
        }

        let scroll_variables: Vec<&str> = component_target.split('_').collect();
        let count_of_variables = if component_target.is_empty() {
            0
        } else {
            scroll_variables.len()
        };
        let mut count_of_used_variables = 0;

        let mut spells = Vec::with_capacity(raw_scroll_spells.len());

        for raw_spell in raw_scroll_spells.iter() {
            if raw_spell.contains("=$") {
                if count_of_used_variables > scroll_variables.len().saturating_sub(1) {
                    break;
                }

                let variabled_raw_spell = raw_spell.replace(
                    "=$",
                    format!("={}", scroll_variables[count_of_used_variables]).as_str(),
                );

                if let Ok(Some(spell)) = Spell::new(
                    &variabled_raw_spell,
                    shared_spells,
                    scrolls,
                    span,
                    source.clone(),
                ) {
                    spells.push(spell);
                }

                count_of_used_variables += 1;
            } else if let Ok(Some(spell)) =
                Spell::new(raw_spell, shared_spells, scrolls, span, source.clone())
            {
                spells.push(spell);
            }
        }

        if count_of_used_variables != count_of_variables {
            let message = format!(
                "Variable count mismatch for scroll '{scroll_name}'. Provided {count_of_variables} arguments, but scroll definition uses {count_of_used_variables}",
            );

            if let Some(src) = &source {
                return Err(GrimoireCssError::InvalidSpellFormat {
                    message,
                    span,
                    label: "Error in this spell".to_string(),
                    help: Some(
                        "Pass exactly N arguments separated by '_' (underscores).\n\
Example: complex-card=arg1_arg2_arg3"
                            .to_string(),
                    ),
                    source_file: Some(src.clone()),
                });
            } else {
                return Err(GrimoireCssError::InvalidInput(message));
            }
        }

        if spells.is_empty() {
            Ok(None)
        } else {
            Ok(Some(spells))
        }
    }

    pub fn generate_spells_from_classes(
        css_classes: Vec<(String, (usize, usize))>,
        shared_spells: &HashSet<String>,
        scrolls: &Option<HashMap<String, Vec<String>>>,
        source: Option<Arc<SourceFile>>,
    ) -> Result<Vec<Spell>, GrimoireCssError> {
        let mut spells = Vec::with_capacity(css_classes.len());

        for (cs, span) in css_classes {
            if !shared_spells.contains(&cs)
                && let Some(spell) = Spell::new(&cs, shared_spells, scrolls, span, source.clone())?
            {
                spells.push(spell);
            }
        }

        Ok(spells)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::source_file::SourceFile;
    use crate::core::spell::Spell;
    use std::collections::{HashMap, HashSet};
    use std::sync::Arc;

    #[test]
    fn test_multiple_raw_spells_in_template() {
        let shared_spells = HashSet::new();
        let scrolls: Option<HashMap<String, Vec<String>>> = None;
        let raw = "g!color=red--display=flex;";
        let spell = Spell::new(raw, &shared_spells, &scrolls, (0, 0), None)
            .expect("parse ok")
            .expect("not None");
        assert!(spell.with_template);
        assert!(spell.scroll_spells.is_some());
        let spells = spell.scroll_spells.as_ref().unwrap();
        assert_eq!(spells.len(), 2);
        assert_eq!(spells[0].component(), "color");
        assert_eq!(spells[0].component_target(), "red");
        assert_eq!(spells[1].component(), "display");
        assert_eq!(spells[1].component_target(), "flex");
    }

    #[test]
    fn test_scroll_can_be_used_inside_template_attribute() {
        let shared_spells = HashSet::new();
        let mut scrolls_map: HashMap<String, Vec<String>> = HashMap::new();
        scrolls_map.insert(
            "complex-card".to_string(),
            vec!["h=$".to_string(), "c=$".to_string(), "w=$".to_string()],
        );
        let scrolls = Some(scrolls_map);

        // This is the desired HTML usage pattern: use scroll invocation via g! ... ;
        // (i.e. not inside class="...").
        let raw = "g!complex-card=120px_red_100px;";
        let spell = Spell::new(raw, &shared_spells, &scrolls, (0, 0), None)
            .expect("parse ok")
            .expect("not None");

        assert!(spell.with_template);
        let spells = spell.scroll_spells.as_ref().expect("template spells");
        assert_eq!(spells.len(), 3);
        assert_eq!(spells[0].component(), "h");
        assert_eq!(spells[0].component_target(), "120px");
        assert_eq!(spells[1].component(), "c");
        assert_eq!(spells[1].component_target(), "red");
        assert_eq!(spells[2].component(), "w");
        assert_eq!(spells[2].component_target(), "100px");
    }

    #[test]
    fn test_non_grimoire_plain_class_is_ignored() {
        let shared_spells = HashSet::new();
        let scrolls: Option<HashMap<String, Vec<String>>> = None;

        // Plain CSS class (no '=') must not be treated as a spell.
        let spell = Spell::new(
            "red",
            &shared_spells,
            &scrolls,
            (12, 3),
            Some(Arc::new(SourceFile::new(
                None,
                "test".to_string(),
                "<div class=\"red primary-button\"></div>".to_string(),
            ))),
        )
        .expect("parsing must not fail");

        assert!(spell.is_none());
    }
}
