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
    GrimoireCssError, ScrollDefinition, component::get_css_property, source_file::SourceFile,
    spell_value_validator,
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
    fn is_plausible_component_name(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        // Reject JS/TS operator tokens that may appear in `class={...}`.
        name.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    }

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
        scrolls: &Option<HashMap<String, ScrollDefinition>>,
        span: (usize, usize),
        source: Option<Arc<SourceFile>>,
    ) -> Result<Option<Self>, GrimoireCssError> {
        let mut expansion_stack: Vec<String> = Vec::new();
        Self::new_impl(
            raw_spell,
            shared_spells,
            scrolls,
            span,
            source,
            &mut expansion_stack,
        )
    }

    fn new_impl(
        raw_spell: &str,
        shared_spells: &HashSet<String>,
        scrolls: &Option<HashMap<String, ScrollDefinition>>,
        span: (usize, usize),
        source: Option<Arc<SourceFile>>,
        expansion_stack: &mut Vec<String>,
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
        // Note: templates can either be a list of property spells (e.g. g!color=red--display=flex;)
        // or a scroll invocation with args (e.g. g!box=10px_20px;).
        if with_template && !raw_spell_split.is_empty() {
            let mut scroll_spells: Vec<Spell> = Vec::new();

            for rs in &raw_spell_split {
                if let Some(spell) = Spell::new_impl(
                    rs,
                    shared_spells,
                    scrolls,
                    span,
                    source.clone(),
                    expansion_stack,
                )? {
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
                                if let Some(reparsed) = Spell::new_impl(
                                    &combined,
                                    shared_spells,
                                    scrolls,
                                    span,
                                    source.clone(),
                                    expansion_stack,
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

            let component_candidate = &raw[component_range.clone()];
            if !Self::is_plausible_component_name(component_candidate) {
                return Ok(None);
            }

            let component_target_candidate = &raw[component_target_range.clone()];
            if component_target_candidate.starts_with('=') {
                return Ok(None);
            }

            let component_target = component_target_candidate;
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

            if let Some(scroll_def) = Self::check_raw_scroll_spells(component, scrolls) {
                spell.scroll_spells = Self::parse_scroll(
                    component,
                    scroll_def,
                    spell.component_target(),
                    shared_spells,
                    scrolls,
                    span,
                    source,
                    expansion_stack,
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
            && let Some(scroll_def) =
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
                scroll_def,
                "",
                shared_spells,
                scrolls,
                span,
                source,
                expansion_stack,
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
        scrolls: &'a Option<HashMap<String, ScrollDefinition>>,
    ) -> Option<&'a ScrollDefinition> {
        scrolls.as_ref()?.get(scroll_name)
    }

    #[allow(clippy::too_many_arguments)]
    fn parse_scroll(
        scroll_name: &str,
        scroll_def: &ScrollDefinition,
        component_target: &str,
        shared_spells: &HashSet<String>,
        scrolls: &Option<HashMap<String, ScrollDefinition>>,
        span: (usize, usize),
        source: Option<Arc<SourceFile>>,
        expansion_stack: &mut Vec<String>,
    ) -> Result<Option<Vec<Spell>>, GrimoireCssError> {
        let key = if component_target.is_empty() {
            scroll_name.to_string()
        } else {
            format!("{scroll_name}={component_target}")
        };

        if let Some(start) = expansion_stack.iter().position(|k| k == &key) {
            let mut cycle = expansion_stack[start..].to_vec();
            cycle.push(key.clone());
            let message = format!("Cycle detected in scroll expansion: {}", cycle.join(" -> "));

            if let Some(src) = &source {
                return Err(GrimoireCssError::InvalidSpellFormat {
                    message,
                    span,
                    label: "Error in this spell".to_string(),
                    help: Some(
                        "Fix the scroll definitions so they don't reference each other in a cycle."
                            .to_string(),
                    ),
                    source_file: Some(src.clone()),
                });
            }

            return Err(GrimoireCssError::InvalidInput(message));
        }

        expansion_stack.push(key);
        let result: Result<Option<Vec<Spell>>, GrimoireCssError> = (|| {
            let scroll_variables: Vec<&str> = if component_target.is_empty() {
                Vec::new()
            } else {
                component_target.split('_').collect()
            };
            let count_of_variables = scroll_variables.len();

            // Select overload by argument count if present.
            let overload_key = count_of_variables.to_string();
            let overload_spells_opt = scroll_def
                .spells_by_args
                .as_ref()
                .and_then(|m| m.get(&overload_key));

            // If spellsByArgs exists but no matching arity is defined:
            // - for 0 args: treat it as "no overload" and compile base spells only
            // - for N>0: keep strictness (likely a user mistake / unsupported arity)
            if count_of_variables > 0
                && let Some(map) = &scroll_def.spells_by_args
                && !map.is_empty()
                && overload_spells_opt.is_none()
            {
                let mut available: Vec<_> = map.keys().cloned().collect();
                available.sort();
                let message = format!(
                    "No overload for scroll '{scroll_name}' with {count_of_variables} arguments"
                );

                if let Some(src) = &source {
                    return Err(GrimoireCssError::InvalidSpellFormat {
                        message,
                        span,
                        label: "Error in this spell".to_string(),
                        help: Some(format!(
                            "Define spellsByArgs['{count_of_variables}'] for this scroll, or pass one of the supported arities: {}",
                            available.join(", ")
                        )),
                        source_file: Some(src.clone()),
                    });
                } else {
                    return Err(GrimoireCssError::InvalidInput(message));
                }
            }

            // Build selected spells: base + overload.
            let mut selected: Vec<&String> = scroll_def.spells.iter().collect();
            if let Some(overload_spells) = overload_spells_opt {
                selected.extend(overload_spells.iter());
            }

            if selected.is_empty() {
                return Ok(None);
            }

            // Keep strictness: the provided arg count must match what the selected spells require.
            let expected_arity = Self::infer_expected_scroll_arity(&selected);
            if expected_arity != count_of_variables {
                let message = format!(
                    "Variable count mismatch for scroll '{scroll_name}'. Provided {count_of_variables} arguments, but scroll definition expects {expected_arity}",
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

            let mut sequential_index: usize = 0;
            let mut spells = Vec::with_capacity(selected.len());

            for raw_spell in selected {
                if let Some((placeholder_pos, digits_len)) = Self::find_placeholder(raw_spell) {
                    let explicit_index = if digits_len == 0 {
                        None
                    } else {
                        raw_spell[placeholder_pos + 2..placeholder_pos + 2 + digits_len]
                            .parse::<usize>()
                            .ok()
                    };

                    let arg_index_0_based = if let Some(one_based) = explicit_index {
                        if one_based == 0 {
                            let message = format!(
                                "Invalid placeholder '$0' in scroll '{scroll_name}' (arguments are 1-based: $1, $2, ...)"
                            );
                            if let Some(src) = &source {
                                return Err(GrimoireCssError::InvalidSpellFormat {
                                    message,
                                    span,
                                    label: "Error in this spell".to_string(),
                                    help: Some("Use $1 for the first argument.".to_string()),
                                    source_file: Some(src.clone()),
                                });
                            }
                            return Err(GrimoireCssError::InvalidInput(message));
                        }
                        one_based - 1
                    } else {
                        let idx = sequential_index;
                        sequential_index += 1;
                        idx
                    };

                    if arg_index_0_based >= scroll_variables.len() {
                        let message = format!(
                            "Scroll '{scroll_name}' references argument {} but only {count_of_variables} were provided",
                            arg_index_0_based + 1
                        );
                        if let Some(src) = &source {
                            return Err(GrimoireCssError::InvalidSpellFormat {
                                message,
                                span,
                                label: "Error in this spell".to_string(),
                                help: Some(
                                    "Pass enough arguments separated by '_' (underscores), or fix the scroll definition placeholders."
                                        .to_string(),
                                ),
                                source_file: Some(src.clone()),
                            });
                        }
                        return Err(GrimoireCssError::InvalidInput(message));
                    }

                    let replacement = scroll_variables[arg_index_0_based];
                    let mut variabled_raw_spell = String::new();
                    variabled_raw_spell.push_str(&raw_spell[..placeholder_pos]);
                    variabled_raw_spell.push('=');
                    variabled_raw_spell.push_str(replacement);
                    variabled_raw_spell.push_str(&raw_spell[placeholder_pos + 2 + digits_len..]);

                    if let Some(spell) = Spell::new_impl(
                        &variabled_raw_spell,
                        shared_spells,
                        scrolls,
                        span,
                        source.clone(),
                        expansion_stack,
                    )? {
                        Self::push_flattened_spell(
                            spell,
                            &mut spells,
                            shared_spells,
                            scrolls,
                            span,
                            source.clone(),
                            expansion_stack,
                        )?;
                    }
                } else if let Some(spell) = Spell::new_impl(
                    raw_spell,
                    shared_spells,
                    scrolls,
                    span,
                    source.clone(),
                    expansion_stack,
                )? {
                    Self::push_flattened_spell(
                        spell,
                        &mut spells,
                        shared_spells,
                        scrolls,
                        span,
                        source.clone(),
                        expansion_stack,
                    )?;
                }
            }

            if spells.is_empty() {
                Ok(None)
            } else {
                Ok(Some(spells))
            }
        })();

        // Pop our key before returning.
        expansion_stack.pop();
        result
    }

    #[allow(clippy::too_many_arguments)]
    fn push_flattened_spell(
        mut spell: Spell,
        out: &mut Vec<Spell>,
        shared_spells: &HashSet<String>,
        scrolls: &Option<HashMap<String, ScrollDefinition>>,
        span: (usize, usize),
        source: Option<Arc<SourceFile>>,
        expansion_stack: &mut Vec<String>,
    ) -> Result<(), GrimoireCssError> {
        let area = spell.area().to_string();
        let focus = spell.focus().to_string();
        let effects = spell.effects().to_string();

        if let Some(inner_scroll_spells) = spell.scroll_spells.take() {
            let has_prefix = !area.is_empty() || !focus.is_empty() || !effects.is_empty();

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
                    if let Some(reparsed) = Spell::new_impl(
                        &combined,
                        shared_spells,
                        scrolls,
                        span,
                        source.clone(),
                        expansion_stack,
                    )? {
                        Self::push_flattened_spell(
                            reparsed,
                            out,
                            shared_spells,
                            scrolls,
                            span,
                            source.clone(),
                            expansion_stack,
                        )?;
                    }
                }
            } else {
                for inner in inner_scroll_spells {
                    Self::push_flattened_spell(
                        inner,
                        out,
                        shared_spells,
                        scrolls,
                        span,
                        source.clone(),
                        expansion_stack,
                    )?;
                }
            }

            return Ok(());
        }

        out.push(spell);
        Ok(())
    }

    /// Finds the first placeholder occurrence in a spell value.
    ///
    /// Supported patterns are `=$` (sequential) and `=$N` (explicit 1-based index).
    /// Returns `(pos_of_"=$"_start, digits_len_after_$)`.
    fn find_placeholder(raw_spell: &str) -> Option<(usize, usize)> {
        let pos = raw_spell.find("=$")?;
        let mut digits_len = 0usize;
        for ch in raw_spell[pos + 2..].chars() {
            if ch.is_ascii_digit() {
                digits_len += 1;
            } else {
                break;
            }
        }
        Some((pos, digits_len))
    }

    /// Infer expected arity from placeholders in the selected scroll spells.
    ///
    /// - Each `=$` consumes one sequential argument.
    /// - Each `=$N` requires at least `N` arguments.
    fn infer_expected_scroll_arity(spells: &[&String]) -> usize {
        let mut sequential = 0usize;
        let mut max_explicit = 0usize;

        for s in spells {
            if let Some((pos, digits_len)) = Self::find_placeholder(s) {
                if digits_len == 0 {
                    sequential += 1;
                } else if let Ok(n) = s[pos + 2..pos + 2 + digits_len].parse::<usize>() {
                    max_explicit = max_explicit.max(n);
                }
            }
        }

        sequential.max(max_explicit)
    }

    pub fn generate_spells_from_classes(
        css_classes: Vec<(String, (usize, usize))>,
        shared_spells: &HashSet<String>,
        scrolls: &Option<HashMap<String, ScrollDefinition>>,
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
    use crate::core::ScrollDefinition;
    use crate::core::source_file::SourceFile;
    use crate::core::spell::Spell;
    use std::collections::{HashMap, HashSet};
    use std::sync::Arc;

    #[test]
    fn test_operator_tokens_are_not_spells() {
        let shared_spells: HashSet<String> = HashSet::new();
        let scrolls: Option<HashMap<String, ScrollDefinition>> = None;

        assert!(
            Spell::new("===", &shared_spells, &scrolls, (0, 3), None)
                .unwrap()
                .is_none()
        );
        assert!(
            Spell::new("a<=b", &shared_spells, &scrolls, (0, 4), None)
                .unwrap()
                .is_none()
        );
        assert!(
            Spell::new("foo==bar", &shared_spells, &scrolls, (0, 7), None)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn test_multiple_raw_spells_in_template() {
        let shared_spells = HashSet::new();
        let scrolls: Option<HashMap<String, ScrollDefinition>> = None;
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
        let mut scrolls_map: HashMap<String, ScrollDefinition> = HashMap::new();
        scrolls_map.insert(
            "complex-card".to_string(),
            ScrollDefinition {
                spells: vec!["h=$".to_string(), "c=$".to_string(), "w=$".to_string()],
                spells_by_args: None,
            },
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
        let scrolls: Option<HashMap<String, ScrollDefinition>> = None;

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

    #[test]
    fn test_scroll_spells_by_args_overload_and_explicit_indices() {
        let shared_spells = HashSet::new();

        let mut scrolls_map: HashMap<String, ScrollDefinition> = HashMap::new();
        scrolls_map.insert(
            "box".to_string(),
            ScrollDefinition {
                spells: vec![
                    "height=var(--box-height)".to_string(),
                    "width=var(--box-width)".to_string(),
                ],
                spells_by_args: Some(HashMap::from([
                    (
                        "0".to_string(),
                        vec![
                            "padding-top=100%".to_string(),
                            "padding-right=100%".to_string(),
                            "padding-bottom=100%".to_string(),
                            "padding-left=100%".to_string(),
                        ],
                    ),
                    (
                        "2".to_string(),
                        vec![
                            "padding-top=$1".to_string(),
                            "padding-bottom=$1".to_string(),
                            "padding-left=$2".to_string(),
                            "padding-right=$2".to_string(),
                        ],
                    ),
                ])),
            },
        );

        let scrolls = Some(scrolls_map);

        let raw = "g!box=10px_20px;";
        let spell = Spell::new(raw, &shared_spells, &scrolls, (0, 0), None)
            .expect("parse ok")
            .expect("not None");
        let spells = spell.scroll_spells.as_ref().expect("template spells");
        let raw_spells: Vec<String> = spells.iter().map(|s| s.raw_spell.clone()).collect();

        assert!(raw_spells.contains(&"height=var(--box-height)".to_string()));
        assert!(raw_spells.contains(&"width=var(--box-width)".to_string()));
        assert!(raw_spells.contains(&"padding-top=10px".to_string()));
        assert!(raw_spells.contains(&"padding-bottom=10px".to_string()));
        assert!(raw_spells.contains(&"padding-left=20px".to_string()));
        assert!(raw_spells.contains(&"padding-right=20px".to_string()));

        // 0-args overload via scroll invocation without '=' (inside template)
        let raw0 = "g!box;";
        let spell0 = Spell::new(raw0, &shared_spells, &scrolls, (0, 0), None)
            .expect("parse ok")
            .expect("not None");
        let spells0 = spell0.scroll_spells.as_ref().expect("template spells");
        let raw_spells0: Vec<String> = spells0.iter().map(|s| s.raw_spell.clone()).collect();
        assert!(raw_spells0.contains(&"padding-top=100%".to_string()));
    }

    #[test]
    fn test_scroll_spells_by_args_missing_zero_overload_compiles_base_spells() {
        let shared_spells = HashSet::new();

        let mut scrolls_map: HashMap<String, ScrollDefinition> = HashMap::new();
        scrolls_map.insert(
            "box".to_string(),
            ScrollDefinition {
                spells: vec![
                    "height=var(--box-height)".to_string(),
                    "width=var(--box-width)".to_string(),
                ],
                spells_by_args: Some(HashMap::from([(
                    "1".to_string(),
                    vec!["padding-top=$1".to_string()],
                )])),
            },
        );

        let scrolls = Some(scrolls_map);

        // 0-args invocation must not error even without spellsByArgs["0"].
        let raw0 = "g!box;";
        let spell0 = Spell::new(raw0, &shared_spells, &scrolls, (0, 0), None)
            .expect("parse ok")
            .expect("not None");
        let spells0 = spell0.scroll_spells.as_ref().expect("template spells");
        let raw_spells0: Vec<String> = spells0.iter().map(|s| s.raw_spell.clone()).collect();

        assert!(raw_spells0.contains(&"height=var(--box-height)".to_string()));
        assert!(raw_spells0.contains(&"width=var(--box-width)".to_string()));
        assert!(!raw_spells0.iter().any(|s| s.starts_with("padding-")));
    }

    #[test]
    fn test_nested_scroll_invocation_inside_scroll_spells_is_flattened() {
        let shared_spells = HashSet::new();

        let mut scrolls_map: HashMap<String, ScrollDefinition> = HashMap::new();
        scrolls_map.insert(
            "box".to_string(),
            ScrollDefinition {
                spells: vec![],
                spells_by_args: Some(HashMap::from([(
                    "2".to_string(),
                    vec![
                        "padding-top=$1".to_string(),
                        "padding-bottom=$1".to_string(),
                        "padding-left=$2".to_string(),
                        "padding-right=$2".to_string(),
                    ],
                )])),
            },
        );
        scrolls_map.insert(
            "wrap".to_string(),
            ScrollDefinition {
                spells: vec!["box=10px_20px".to_string()],
                spells_by_args: None,
            },
        );
        let scrolls = Some(scrolls_map);

        let spell = Spell::new("wrap", &shared_spells, &scrolls, (0, 0), None)
            .expect("parse ok")
            .expect("not None");
        let spells = spell.scroll_spells.as_ref().expect("scroll spells");
        let raw_spells: Vec<String> = spells.iter().map(|s| s.raw_spell.clone()).collect();

        assert!(raw_spells.contains(&"padding-top=10px".to_string()));
        assert!(raw_spells.contains(&"padding-bottom=10px".to_string()));
        assert!(raw_spells.contains(&"padding-left=20px".to_string()));
        assert!(raw_spells.contains(&"padding-right=20px".to_string()));
    }

    #[test]
    fn test_nested_scroll_invocation_preserves_effects_prefix() {
        let shared_spells = HashSet::new();

        let mut scrolls_map: HashMap<String, ScrollDefinition> = HashMap::new();
        scrolls_map.insert(
            "box".to_string(),
            ScrollDefinition {
                spells: vec![],
                spells_by_args: Some(HashMap::from([(
                    "1".to_string(),
                    vec!["padding-top=$1".to_string()],
                )])),
            },
        );
        scrolls_map.insert(
            "hoverWrap".to_string(),
            ScrollDefinition {
                spells: vec!["hover:box=4px".to_string()],
                spells_by_args: None,
            },
        );
        let scrolls = Some(scrolls_map);

        let spell = Spell::new("hoverWrap", &shared_spells, &scrolls, (0, 0), None)
            .expect("parse ok")
            .expect("not None");
        let spells = spell.scroll_spells.as_ref().expect("scroll spells");
        assert_eq!(spells.len(), 1);
        assert_eq!(spells[0].effects(), "hover");
        assert_eq!(spells[0].component(), "padding-top");
        assert_eq!(spells[0].component_target(), "4px");
    }

    #[test]
    fn test_nested_scroll_invocation_inside_template_token_in_scroll_spells() {
        let shared_spells = HashSet::new();

        let mut scrolls_map: HashMap<String, ScrollDefinition> = HashMap::new();
        scrolls_map.insert(
            "box".to_string(),
            ScrollDefinition {
                spells: vec![],
                spells_by_args: Some(HashMap::from([(
                    "2".to_string(),
                    vec!["padding-top=$1".to_string(), "padding-left=$2".to_string()],
                )])),
            },
        );
        scrolls_map.insert(
            "templateWrap".to_string(),
            ScrollDefinition {
                spells: vec!["g!box=10px_20px;".to_string()],
                spells_by_args: None,
            },
        );
        let scrolls = Some(scrolls_map);

        let spell = Spell::new("templateWrap", &shared_spells, &scrolls, (0, 0), None)
            .expect("parse ok")
            .expect("not None");
        let spells = spell.scroll_spells.as_ref().expect("scroll spells");
        let raw_spells: Vec<String> = spells.iter().map(|s| s.raw_spell.clone()).collect();
        assert!(raw_spells.contains(&"padding-top=10px".to_string()));
        assert!(raw_spells.contains(&"padding-left=20px".to_string()));
    }

    #[test]
    fn test_scroll_cycle_detection_errors() {
        let shared_spells = HashSet::new();

        let mut scrolls_map: HashMap<String, ScrollDefinition> = HashMap::new();
        scrolls_map.insert(
            "a".to_string(),
            ScrollDefinition {
                spells: vec!["b".to_string()],
                spells_by_args: None,
            },
        );
        scrolls_map.insert(
            "b".to_string(),
            ScrollDefinition {
                spells: vec!["a".to_string()],
                spells_by_args: None,
            },
        );
        let scrolls = Some(scrolls_map);

        let err = Spell::new("a", &shared_spells, &scrolls, (0, 0), None).unwrap_err();
        let msg = err.to_string();
        assert!(msg.to_lowercase().contains("cycle"));
    }
}
