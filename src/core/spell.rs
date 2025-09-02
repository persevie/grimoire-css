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

use super::{GrimoireCssError, component::get_css_property};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Spell {
    pub raw_spell: String,
    pub component: String,
    pub component_target: String,
    pub effects: String,
    pub area: String,
    pub focus: String,
    pub with_template: bool,
    pub scroll_spells: Option<Vec<Spell>>,
}

impl Spell {
    /// Example input: "md__{_>_p}hover:display=none"
    pub fn new(
        raw_spell: &str,
        shared_spells: &HashSet<String>,
        scrolls: &Option<HashMap<String, Vec<String>>>,
    ) -> Result<Option<Self>, GrimoireCssError> {
        let with_template = Self::check_for_template(raw_spell);
        let raw_spell = if with_template {
            raw_spell
                .strip_prefix("g!")
                .and_then(|s| s.strip_suffix(";"))
                .unwrap_or(raw_spell)
        } else {
            raw_spell
        };

        let raw_spell_split: Vec<&str> = raw_spell.split("--").filter(|s| !s.is_empty()).collect();

        if with_template && !raw_spell_split.is_empty() {
            let mut scroll_spells: Vec<Spell> = Vec::new();
            for rs in raw_spell_split {
                if let Some(spell) = Spell::new(rs, shared_spells, scrolls)? {
                    scroll_spells.push(spell);
                }
            }

            return Ok(Some(Spell {
                raw_spell: raw_spell.to_string(),
                component: String::new(),
                component_target: String::new(),
                effects: String::new(),
                area: String::new(),
                focus: String::new(),
                with_template,
                scroll_spells: Some(scroll_spells),
            }));
        }

        // Split the input string by "__" to separate the area (screen size) and the rest
        let (area, rest) = raw_spell.split_once("__").unwrap_or(("", raw_spell));

        // Split the raw spell by "}" to get the focus and the rest
        let (focus, rest) = rest
            .split_once('}')
            .map_or(("", rest), |(f, r)| (f.strip_prefix('{').unwrap_or(f), r));

        // Split the rest by ":" to get the effects (pseudo-class) and the rest
        let (effects, rest) = rest.split_once(':').unwrap_or(("", rest));

        // Split the rest by "=" to separate the component (property) and component_target (value)
        if let Some((component, component_target)) = rest.split_once("=") {
            let mut spell = Spell {
                raw_spell: raw_spell.to_string(),
                component: component.to_string(),
                component_target: component_target.to_string(),
                effects: effects.to_string(),
                area: area.to_string(),
                focus: focus.to_string(),
                with_template,
                scroll_spells: None,
            };

            if let Some(raw_scroll_spells) =
                Self::check_raw_scroll_spells(&spell.component, scrolls)
            {
                spell.scroll_spells = Self::parse_scroll(
                    component,
                    raw_scroll_spells,
                    &spell.component_target,
                    shared_spells,
                    scrolls,
                )?;
            }

            return Ok(Some(spell));
        } else if let Some(raw_scroll_spells) = Self::check_raw_scroll_spells(rest, scrolls) {
            return Ok(Some(Spell {
                raw_spell: raw_spell.to_string(),
                component: rest.to_string(),
                component_target: String::new(),
                effects: effects.to_string(),
                area: area.to_string(),
                focus: focus.to_string(),
                with_template,
                scroll_spells: Self::parse_scroll(
                    rest,
                    raw_scroll_spells,
                    "",
                    shared_spells,
                    scrolls,
                )?,
            }));
        }

        Ok(None) // Return None if format is invalid
    }

    fn check_for_template(class_name: &str) -> bool {
        class_name.starts_with("g!") && class_name.ends_with(";")
    }

    fn check_raw_scroll_spells<'a>(
        spell_component: &'a str,
        scrolls: &'a Option<HashMap<String, Vec<String>>>,
    ) -> Option<&'a Vec<String>> {
        if get_css_property(spell_component).is_some() {
            return None;
        }

        if let Some(scrolls) = scrolls {
            return scrolls.get(spell_component);
        };

        None
    }

    fn parse_scroll(
        scroll_name: &str,
        raw_scroll_spells: &[String],
        component_target: &str,
        shared_spells: &HashSet<String>,
        scrolls: &Option<HashMap<String, Vec<String>>>,
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
                if count_of_used_variables > scroll_variables.len() - 1 {
                    break;
                }

                let variabled_raw_spell = raw_spell.replace(
                    "=$",
                    format!("={}", scroll_variables[count_of_used_variables]).as_str(),
                );

                if let Ok(Some(spell)) = Spell::new(&variabled_raw_spell, shared_spells, scrolls) {
                    spells.push(spell);
                }

                count_of_used_variables += 1;
            } else if let Ok(Some(spell)) = Spell::new(raw_spell, shared_spells, scrolls) {
                spells.push(spell);
            }
        }

        if count_of_used_variables != count_of_variables {
            return Err(GrimoireCssError::InvalidInput(format!(
                "Not all variables used in scroll '{scroll_name}'. Expected {count_of_variables}, but used {count_of_used_variables}",
            )));
        }

        if spells.is_empty() {
            Ok(None)
        } else {
            Ok(Some(spells))
        }
    }

    pub fn generate_spells_from_classes(
        css_classes: Vec<String>,
        shared_spells: &HashSet<String>,
        scrolls: &Option<HashMap<String, Vec<String>>>,
    ) -> Result<Vec<Spell>, GrimoireCssError> {
        let mut spells = Vec::with_capacity(css_classes.len());

        for cs in css_classes {
            if !shared_spells.contains(&cs)
                && let Some(spell) = Spell::new(&cs, shared_spells, scrolls)?
            {
                spells.push(spell);
            }
        }

        Ok(spells)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::spell::Spell;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_multiple_raw_spells_in_template() {
        let shared_spells = HashSet::new();
        let scrolls: Option<HashMap<String, Vec<String>>> = None;
        let raw = "g!color=red--display=flex;";
        let spell = Spell::new(raw, &shared_spells, &scrolls)
            .expect("parse ok")
            .expect("not None");
        assert!(spell.with_template);
        assert!(spell.scroll_spells.is_some());
        let spells = spell.scroll_spells.as_ref().unwrap();
        assert_eq!(spells.len(), 2);
        assert_eq!(spells[0].component, "color");
        assert_eq!(spells[0].component_target, "red");
        assert_eq!(spells[1].component, "display");
        assert_eq!(spells[1].component_target, "flex");
    }
}
