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

use super::{Config, GrimoireCSSError};

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
    pub fn new(raw_spell: &str, config: &Config) -> Result<Option<Self>, GrimoireCSSError> {
        let with_template = Self::check_for_template(raw_spell);

        let raw_spell = if with_template {
            raw_spell
                .strip_prefix("g!")
                .and_then(|s| s.strip_suffix(";"))
                .unwrap_or(raw_spell)
        } else {
            raw_spell
        };

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

            if let Some(raw_scroll_spells) = Self::check_raw_scroll_spells(&spell.component, config)
            {
                spell.scroll_spells = Self::parse_scroll(
                    component,
                    raw_scroll_spells,
                    &spell.component_target,
                    config,
                )?;
            }

            return Ok(Some(spell));
        } else if let Some(raw_scroll_spells) = Self::check_raw_scroll_spells(rest, config) {
            return Ok(Some(Spell {
                raw_spell: raw_spell.to_string(),
                component: rest.to_string(),
                component_target: String::new(),
                effects: effects.to_string(),
                area: area.to_string(),
                focus: focus.to_string(),
                with_template,
                scroll_spells: Self::parse_scroll(rest, raw_scroll_spells, "", config)?,
            }));
        }

        Ok(None) // Return None if format is invalid
    }

    fn check_for_template(class_name: &str) -> bool {
        class_name.starts_with("g!") && class_name.ends_with(";")
    }

    fn check_raw_scroll_spells<'a>(
        spell_component: &'a str,
        config: &'a Config,
    ) -> Option<&'a Vec<String>> {
        if let Some(scrolls) = &config.scrolls {
            return scrolls.get(spell_component);
        };

        None
    }

    fn parse_scroll(
        scroll_name: &str,
        raw_scroll_spells: &[String],
        component_target: &str,
        config: &Config,
    ) -> Result<Option<Vec<Spell>>, GrimoireCSSError> {
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

                if let Ok(Some(spell)) = Spell::new(&variabled_raw_spell, config) {
                    spells.push(spell);
                }

                count_of_used_variables += 1;
            } else if let Ok(Some(spell)) = Spell::new(raw_spell, config) {
                spells.push(spell);
            }
        }

        if count_of_used_variables != count_of_variables {
            return Err(GrimoireCSSError::InvalidInput(format!(
                "Not all variables used in scroll '{}'. Expected {}, but used {}",
                scroll_name, count_of_variables, count_of_used_variables,
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
        config: &Config,
    ) -> Result<Vec<Spell>, GrimoireCSSError> {
        let mut spells = Vec::with_capacity(css_classes.len());

        for cs in css_classes {
            if !config.shared_spells.contains(&cs) {
                if let Some(spell) = Spell::new(&cs, config)? {
                    spells.push(spell);
                }
            }
        }

        Ok(spells)
    }
}
