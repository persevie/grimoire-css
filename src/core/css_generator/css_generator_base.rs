//! This module provides functionality for generating CSS based on spells and configuration settings.
//!
//! The module includes functions to generate CSS class names, handle media queries, and adapt targets
//! based on a given configuration. It is designed to work with `Spell` objects and `GrimoireCSSConfig`
//! to produce the final CSS output, which can include complex rules such as responsive sizing (`mrs` function).
//!
//! Key functionalities:
//!
//! * **CSS Class Name Generation**: Handles the creation of CSS class names from spell components, including
//!   escaping special characters and incorporating spell effects.
//!
//! * **Media Query Wrapping**: Provides functionality to wrap CSS rules within media queries based on screen sizes.
//!
//! * **Grimoire Funtions Handling**: like `mrs`, allowing for flexible and adaptive designs.
//!
//! * **Target Adaptation**: Modifies and adapts CSS targets based on predefined variables in the configuration.
//!
//! The module also includes internal helper functions to manage specific CSS-related tasks such as
//! unit stripping, handling of regex patterns, and combining base CSS with media queries.

use crate::buffer::add_message;
use crate::core::GrimoireCssError;
use crate::core::animations::ANIMATIONS;
use crate::core::component::get_css_property;
use crate::core::spell::Spell;

use super::color_functions;

use color_functions::try_handle_color_function;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

type GeneratedCSS = (String, (String, String), Option<String>);

type MRSRes = (String, [(String, String); 2]);

/// Core CSS generator that transforms spells into valid CSS.
pub struct CssGenerator<'a> {
    variables: &'a Option<Vec<(String, String)>>,
    custom_animations: &'a HashMap<String, String>,
    base_css_regex: Regex,
    mrs_regex: Regex,
    unit_regex: Regex,
    animation_block_regex: Regex,
}

#[derive(Debug)]
struct CalculationInfo {
    calculated: String,
    media_queries: Option<[(String, String); 2]>,
}

#[derive(Debug)]
struct Media {
    size: String,
    value: Vec<String>,
}

impl<'a> CssGenerator<'a> {
    pub fn new(
        variables: &'a Option<Vec<(String, String)>>,
        custom_animations: &'a HashMap<String, String>,
    ) -> Result<Self, GrimoireCssError> {
        let base_css_regex = Regex::new(r"(\w+)\(([^)]*)\)").map_err(|_| {
            GrimoireCssError::Regex(regex::Error::Syntax("Invalid regex pattern".to_string()))
        })?;
        let mrs_regex = Regex::new(r"[a-zA-Z]+")?;
        let unit_regex = Regex::new(r"(\d+(\.\d+)?)")?;
        let animation_block_regex =
            Regex::new(r"(?m)(\.GRIMOIRE_CSS_ANIMATION\s*\{[^}]*\})").unwrap();

        Ok(Self {
            variables,
            custom_animations,
            base_css_regex,
            mrs_regex,
            unit_regex,
            animation_block_regex,
        })
    }

    /// Generates CSS based on the given `Spell` and `GrimoireCSSConfig`.
    ///
    /// # Arguments
    ///
    /// * `spell` - A reference to the `Spell` object containing the spell's details.
    /// * `config` - A reference to the `GrimoireCSSConfig` object containing CSS configuration.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(String, String))` 0: containing the generated CSS string if the spell's component is recognized; 1: css class name
    /// * `Ok(None)` if the spell's component is not recognized.
    /// * `Err(GrimoireCSSError)` if there is an error during CSS generation.
    pub fn generate_css(&self, spell: &Spell) -> Result<Option<GeneratedCSS>, GrimoireCssError> {
        // generate css class name
        let css_class_name = self.generate_css_class_name(
            &spell.raw_spell,
            &spell.effects,
            &spell.focus,
            spell.with_template,
        )?;

        let component_str = spell.component.as_str();

        // match component and get css property
        let css_property: Option<&str> = if component_str.starts_with("--") {
            // css custom properties support
            Some(component_str)
        } else {
            get_css_property(component_str)
        };

        match css_property {
            Some(css_property) => {
                // adapt target
                let adapted_target = self.adapt_targets(&spell.component_target, self.variables)?;
                // generate base css without any media queries (except for the mrs function)
                let (base_css, additional_css) = self.generate_base_and_additional_css(
                    &adapted_target,
                    &css_class_name.0,
                    css_property,
                )?;

                if !spell.area.is_empty() {
                    return Ok(Some((
                        self.wrap_base_css_with_media_query(&spell.area, &base_css),
                        css_class_name,
                        additional_css,
                    )));
                }

                Ok(Some((base_css, css_class_name, additional_css)))
            }
            None => Ok(None),
        }
    }

    /// Wraps base CSS with a media query.
    ///
    /// # Arguments
    ///
    /// * `size_var` - A reference to a string specifying the size variant (e.g., "sm", "md").
    /// * `base_css` - A reference to the base CSS string.
    ///
    /// # Returns
    ///
    /// * A `String` containing the base CSS wrapped in the appropriate media query.
    pub fn wrap_base_css_with_media_query(&self, area: &str, base_css: &str) -> String {
        match area {
            "sm" => self.wrap_size_area("640px", base_css),
            "md" => self.wrap_size_area("768px", base_css),
            "lg" => self.wrap_size_area("1024px", base_css),
            "xl" => self.wrap_size_area("1280px", base_css),
            "2xl" => self.wrap_size_area("1536px", base_css),
            _ => format!(
                "@media {}{{{}}}",
                area.split('_').collect::<Vec<&str>>().join(" "),
                base_css
            ),
        }
    }

    fn wrap_size_area(&self, area: &str, base_css: &str) -> String {
        format!("@media (min-width: {area}){{{base_css}}}")
    }

    pub fn generate_css_class_name(
        &self,
        raw_spell: &str,
        effects: &str,
        raw_spell_focus: &str,
        with_template: bool,
    ) -> Result<(String, String), GrimoireCssError> {
        let spell_focus = raw_spell_focus.split('_').collect::<Vec<&str>>().join(" ");
        let mut escaped_class_name = self.escape_css_class_name(raw_spell)?;

        if with_template {
            escaped_class_name = format!(".g\\!{escaped_class_name}\\;");
        } else {
            escaped_class_name = format!(".{escaped_class_name}");
        }

        let effects_string = Self::generate_effect(effects)?;

        let base_class_name = escaped_class_name.clone();

        if !effects_string.is_empty() {
            escaped_class_name.push_str(&format!(":{effects_string}"));
        }

        if !spell_focus.is_empty() {
            escaped_class_name.push_str(&spell_focus);
        }

        Ok((escaped_class_name, base_class_name))
    }

    /// Escapes special characters in the CSS class name.
    ///
    /// # Arguments
    ///
    /// * `class_name` - A reference to the class name string to be escaped.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` containing the escaped class name.
    /// * `Err(GrimoireCSSError)` if the input is invalid.
    fn escape_css_class_name(&self, class_name: &str) -> Result<String, GrimoireCssError> {
        let escaped = class_name
            .chars()
            .map(|c| match c {
                '!' | '"' | '#' | '$' | '%' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | '.'
                | '/' | ':' | ';' | '<' | '=' | '>' | '?' | '@' | '[' | '\\' | ']' | '^' | '_'
                | '`' | '{' | '|' | '}' | '~' => format!("\\{c}"),
                ' ' => {
                    add_message("HTML does not support spaces. To separate values use underscore ('_') instead".to_string());
                    c.to_string()
                }
                _ => c.to_string(),
            })
            .collect::<String>();

        if escaped.is_empty() {
            return Err(GrimoireCssError::InvalidSpellFormat(class_name.to_string()));
        }

        Ok(escaped)
    }

    /// Generates a string representing the effects of the spell.
    ///
    /// # Arguments
    ///
    /// * `effect` - A reference to the effects string.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` containing the formatted effects string.
    /// * `Err(GrimoireCSSError)` if there is an error during effect generation.
    fn generate_effect(effect: &str) -> Result<String, GrimoireCssError> {
        Ok(effect.split(",").collect::<Vec<&str>>().join(":"))
    }

    /// Adapts the target string based on the configuration.
    ///
    /// # Arguments
    ///
    /// * `target` - A reference to the target string.
    /// * `config` - A reference to the `GrimoireCSSConfig` object containing CSS configuration.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` containing the adapted target string.
    /// * `Err(GrimoireCSSError)` if there is an error during target adaptation.
    fn adapt_targets(
        &self,
        target: &str,
        variables: &Option<Vec<(String, String)>>,
    ) -> Result<String, GrimoireCssError> {
        let mut result = String::new();

        let formatted_target = target.split('_').collect::<Vec<&str>>().join(" ");

        let variables = variables.as_ref();
        let mut replaced_target = formatted_target.clone();

        if let Some(v) = variables {
            for (key, value) in v {
                let placeholder = format!("${key}");
                replaced_target = replaced_target.replace(&placeholder, value);
            }
        }

        result.push_str(&replaced_target);

        Ok(result)
    }

    /// Generates the base CSS and optional additional CSS (keyframes) based on the given property.
    ///
    /// This function delegates the generation logic to specific handlers depending on the property
    /// (e.g., `g-anim`, `animation`, `animation-name`). For other properties, it generates a generic CSS string.
    ///
    /// # Arguments
    ///
    /// * `adapted_target` - A reference to the adapted target string, representing the value for the CSS property.
    /// * `css_class_name` - A reference to the CSS class name.
    /// * `property` - A reference to the CSS property name (e.g., `g-anim`, `animation`, `animation-name`).
    ///
    /// # Returns
    ///
    /// * `Ok((String, Option<String>))` - The base CSS string and an optional string containing additional keyframes CSS.
    /// * `Err(GrimoireCSSError)` - If an error occurs during the CSS generation process.
    fn generate_base_and_additional_css(
        &self,
        adapted_target: &str,
        css_class_name: &str,
        property: &str,
    ) -> Result<(String, Option<String>), GrimoireCssError> {
        match property {
            "g-anim" => self.handle_g_anim(adapted_target, css_class_name),
            "animation" | "anim" => self.handle_animation(adapted_target, css_class_name),
            "animation-name" | "anim-n" => {
                self.handle_animation_name(adapted_target, css_class_name)
            }
            _ => {
                if let Some(css_str) = try_handle_color_function(adapted_target) {
                    self.handle_generic_css(&css_str, css_class_name, property)
                } else {
                    self.handle_generic_css(adapted_target, css_class_name, property)
                }
            }
        }
    }

    /// Handles CSS generation for `g-anim` property.
    ///
    /// This function retrieves the corresponding animation from `ANIMATIONS`, replaces the
    /// placeholder with the actual class name, and returns both the base CSS and keyframes.
    ///
    /// # Arguments
    ///
    /// * `adapted_target` - A reference to the target animation name (e.g., `heart-beat`).
    /// * `css_class_name` - A reference to the CSS class name that will replace the placeholder in the animation.
    ///
    /// # Returns
    ///
    /// * `Ok((String, Option<String>))` - The base CSS and additional keyframes CSS.
    /// * `Err(GrimoireCSSError)` - If the target animation is not found or any error occurs during processing.
    fn handle_g_anim(
        &self,
        adapted_target: &str,
        css_class_name: &str,
    ) -> Result<(String, Option<String>), GrimoireCssError> {
        if let Some(animation) = ANIMATIONS.get(adapted_target) {
            let (keyframes, class) =
                self.get_keyframe_class_from_animation(animation, adapted_target)?;
            let base_css = class.replace(".GRIMOIRE_CSS_ANIMATION", css_class_name);
            return Ok((base_css, Some(keyframes)));
        }

        if let Some(animation) = self.custom_animations.get(adapted_target) {
            let (keyframes, class) =
                self.get_keyframe_class_from_animation(animation, adapted_target)?;
            let base_css = class.replace(".GRIMOIRE_CSS_ANIMATION", css_class_name);
            return Ok((base_css, Some(keyframes)));
        }

        Err(GrimoireCssError::InvalidSpellFormat(
            adapted_target.to_string(),
        ))
    }

    /// Handles CSS generation for the `animation` property.
    ///
    /// This function checks for the presence of keyframes in the animation name and generates the
    /// appropriate base CSS and optional additional keyframes.
    ///
    /// # Arguments
    ///
    /// * `adapted_target` - A reference to the animation value (e.g., `3s linear wobble`).
    /// * `css_class_name` - A reference to the CSS class name used for generating the base CSS.
    ///
    /// # Returns
    ///
    /// * `Ok((String, Option<String>))` - The base CSS and optional keyframes.
    /// * `Err(GrimoireCSSError)` - If any error occurs during processing.
    fn handle_animation(
        &self,
        adapted_target: &str,
        css_class_name: &str,
    ) -> Result<(String, Option<String>), GrimoireCssError> {
        let additional_css = self.get_additional_css(adapted_target)?;
        let base_css = format!("{css_class_name}{{animation:{adapted_target};}}");
        Ok((base_css, additional_css))
    }

    /// Handles CSS generation for the `animation-name` property.
    ///
    /// This function retrieves keyframes from the animation name and generates the base CSS for the `animation-name` property.
    ///
    /// # Arguments
    ///
    /// * `adapted_target` - A reference to the animation name (e.g., `tada`).
    /// * `css_class_name` - A reference to the CSS class name for generating the base CSS.
    ///
    /// # Returns
    ///
    /// * `Ok((String, Option<String>))` - The base CSS and optional additional keyframes.
    /// * `Err(GrimoireCSSError)` - If any error occurs during processing.
    fn handle_animation_name(
        &self,
        adapted_target: &str,
        css_class_name: &str,
    ) -> Result<(String, Option<String>), GrimoireCssError> {
        let additional_css = self.get_additional_css(adapted_target)?;
        let base_css = format!("{css_class_name}{{animation-name:{adapted_target};}}");
        Ok((base_css, additional_css))
    }

    /// Generates generic CSS for properties not specifically handled (`g-anim`, `animation`, or `animation-name`).
    ///
    /// This function uses regular expressions to capture patterns in the target string and apply any necessary transformations.
    ///
    /// # Arguments
    ///
    /// * `adapted_target` - A reference to the target value for the CSS property.
    /// * `css_class_name` - A reference to the CSS class name.
    /// * `property` - A reference to the CSS property name.
    ///
    /// # Returns
    ///
    /// * `Ok((String, Option<String>))` - The base CSS string and an optional string containing additional keyframes CSS.
    /// * `Err(GrimoireCSSError)` - If an error occurs during processing.
    fn handle_generic_css(
        &self,
        adapted_target: &str,
        css_class_name: &str,
        property: &str,
    ) -> Result<(String, Option<String>), GrimoireCssError> {
        let base_css = format!("{css_class_name}{{{property}:{adapted_target};}}");
        let captures = self
            .base_css_regex
            .captures_iter(adapted_target)
            .collect::<Vec<_>>();

        if !captures.is_empty() {
            if let Some((base, media)) =
                self.handle_grimoire_functions(adapted_target, captures, property, css_class_name)?
            {
                Ok((
                    format!("{css_class_name}{{{property}:{base};}}{media}"),
                    None,
                ))
            } else {
                Ok((base_css, None))
            }
        } else {
            Ok((base_css, None))
        }
    }

    /// Retrieves additional CSS (keyframes) based on the animation name.
    ///
    /// This function checks if the given animation name corresponds to any predefined animation in the `ANIMATIONS` list
    /// or in the user's `custom_animations`.
    /// If a matching animation is found, the corresponding keyframes are returned.
    ///
    /// # Arguments
    ///
    /// * `adapted_target` - A reference to the target animation value or name.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(String))` - The keyframes CSS if a matching animation is found.
    /// * `Ok(None)` - If no matching keyframes are found.
    /// * `Err(GrimoireCSSError)` - If an error occurs during processing.
    fn get_additional_css(&self, adapted_target: &str) -> Result<Option<String>, GrimoireCssError> {
        if let Some(grimoire_animation_name) = Self::find_grimoire_animation_name(adapted_target)
            && let Some(animation) = ANIMATIONS.get(grimoire_animation_name)
        {
            let (keyframes, _) =
                self.get_keyframe_class_from_animation(animation, grimoire_animation_name)?;
            return Ok(Some(keyframes));
        };

        for adapted_target_item in adapted_target.split_whitespace() {
            if let Some(custom_animation) = self.custom_animations.get(adapted_target_item) {
                let (keyframes, _) =
                    self.get_keyframe_class_from_animation(custom_animation, adapted_target_item)?;
                return Ok(Some(keyframes));
            }
        }

        Ok(None)
    }

    /// Handles specific grimoire functions in the target string.
    ///
    /// # Arguments
    ///
    /// * `target` - A reference to the target string.
    /// * `captures` - A vector of regex captures from the target string.
    /// * `property` - A reference to the CSS property string.
    /// * `css_class_name` - A reference to the CSS class name string.
    ///
    /// # Returns
    ///
    /// * `Ok(Some((String, String)))` containing the base and media query CSS strings if functions are handled.
    /// * `Ok(None)` if no functions are found.
    /// * `Err(GrimoireCSSError)` if there is an error during function handling.
    fn handle_grimoire_functions(
        &self,
        target: &str,
        captures: Vec<regex::Captures>,
        property: &str,
        css_class_name: &str,
    ) -> Result<Option<(String, String)>, GrimoireCssError> {
        let mut base = target.to_owned();
        let mut screen_sizes_state: HashSet<String> = HashSet::with_capacity(2);
        let mut calculations_base_count = 0;
        let mut calculation_map: HashMap<String, CalculationInfo> = HashMap::new();
        let mut media: Vec<Media> = Vec::new();

        for capture in captures {
            let function_name = &capture[1];
            let args = &capture[2];

            match function_name {
                "mrs" => {
                    if let Some((base_value, media_queries)) =
                        self.handle_mrs(args, &mut screen_sizes_state)?
                    {
                        let key = format!("mrs_{calculations_base_count}");
                        calculations_base_count += 1;

                        // Add media sizes in the order returned from handle_mrs
                        for (media_size, _) in &media_queries {
                            if !media.iter().any(|m| m.size == *media_size) {
                                media.push(Media {
                                    size: media_size.to_owned(),
                                    value: Vec::new(),
                                });
                            }
                        }

                        calculation_map.insert(
                            key.to_owned(),
                            CalculationInfo {
                                calculated: base_value.to_owned(),
                                media_queries: Some(media_queries),
                            },
                        );

                        base = base.replace(&capture[0], &key);
                    }
                }
                "mfs" => {
                    let clamp_value = self.handle_mfs(args)?;
                    let key = format!("mfs_{calculations_base_count}");
                    calculations_base_count += 1;

                    calculation_map.insert(
                        key.to_owned(),
                        CalculationInfo {
                            calculated: clamp_value,
                            media_queries: None,
                        },
                    );

                    base = base.replace(&capture[0], &key);
                }
                _ => {}
            }
        }

        let parts = base.split_whitespace().collect::<Vec<&str>>();

        // Generate the base CSS, replacing keys with computed values
        let new_base = parts
            .iter()
            .map(|p| {
                calculation_map
                    .get(*p)
                    .map_or(p.to_string(), |info| info.calculated.clone())
            })
            .collect::<Vec<String>>()
            .join(" ");

        // If nothing has changed, return None
        if new_base == target {
            return Ok(None);
        }

        // Generate media queries, if any exist
        if media.is_empty() {
            //  No media queries — return only the base
            Ok(Some((new_base, String::new())))
        } else {
            // Iterate over media in the order they were added
            let mut media_queries_str = String::new();
            for media_item in &mut media {
                let media_value: Vec<String> = parts
                    .iter()
                    .map(|p| {
                        calculation_map.get(*p).map_or(p.to_string(), |info| {
                            if let Some(mq) = &info.media_queries {
                                mq.iter()
                                    .find(|(s, _)| s == &media_item.size)
                                    .map_or(p.to_string(), |(_, value)| value.clone())
                            } else {
                                p.to_string()
                            }
                        })
                    })
                    .collect();

                media_item.value = media_value;

                let values_str = media_item.value.join(" ");
                media_queries_str.push_str(&format!(
                    "@media screen and (min-width: {}) {{{}{{{}: {};}}}}",
                    media_item.size, css_class_name, property, values_str
                ));
            }

            Ok(Some((new_base, media_queries_str)))
        }
    }

    /// Handles the `mrs` function for responsive sizing.
    ///
    /// # Arguments
    ///
    /// * `args` - A reference to the arguments string.
    /// * `screen_sizes_state` - A mutable reference to a set of screen sizes.
    ///
    /// # Returns
    ///
    /// * `Ok(Some((String, [(String, String); 2])))` containing the base size and media queries.
    /// * `Ok(None)` if no valid responsive size is found.
    /// * `Err(GrimoireCSSError)` if there is an error during responsive size handling.
    fn handle_mrs(
        &self,
        args: &str,
        screen_sizes_state: &mut HashSet<String>,
    ) -> Result<Option<MRSRes>, GrimoireCssError> {
        let mut parts = args.split(' ');

        let min_size = parts.next().unwrap_or("0px");
        let max_size = parts.next().unwrap_or("0px");
        let min_vw = parts.next();
        let max_vw = parts.next();

        self.make_responsive_size(min_size, max_size, min_vw, max_vw, screen_sizes_state)
    }

    /// Handles the `mfs` function for fluid sizing using CSS clamp().
    ///
    /// # Arguments
    ///
    /// * `args` - A reference to the arguments string.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` containing the CSS clamp() function with calculated values.
    /// * `Err(GrimoireCSSError)` if there is an error during fluid size handling.
    fn handle_mfs(&self, args: &str) -> Result<String, GrimoireCssError> {
        let mut parts = args.split(' ');

        let min_size = parts.next().unwrap_or("0px");
        let max_size = parts.next().unwrap_or("0px");
        let min_vw = parts.next();
        let max_vw = parts.next();

        self.make_fluid_size(min_size, max_size, min_vw, max_vw)
    }

    /// Generates a responsive size and corresponding media queries based on the given parameters.
    ///
    /// # Arguments
    ///
    /// * `min_size` - A reference to the minimum size string (e.g., "100px").
    /// * `max_size` - A reference to the maximum size string (e.g., "200px").
    /// * `min_vw` - An optional reference to the minimum viewport width (e.g., "480px").
    /// * `max_vw` - An optional reference to the maximum viewport width (e.g., "1280px").
    /// * `screen_sizes_state` - A mutable reference to a `HashSet` tracking the current screen sizes used.
    ///
    /// # Returns
    ///
    /// * `Ok(Some((String, [(String, String); 2])))` containing the base size and an array of media queries.
    /// * `Ok(None)` if the input sizes and viewport widths are incompatible.
    /// * `Err(GrimoireCSSError)` if there is an error in processing the sizes or if the screen sizes state is invalid.
    fn make_responsive_size(
        &self,
        min_size: &str,
        max_size: &str,
        min_vw: Option<&str>,
        max_vw: Option<&str>,
        screen_sizes_state: &mut HashSet<String>,
    ) -> Result<Option<MRSRes>, GrimoireCssError> {
        let min_size_value: f64 = self.strip_unit(min_size)?;
        let max_size_value: f64 = self.strip_unit(max_size)?;
        let min_vw_value: f64 = match min_vw {
            Some(i) => self.strip_unit(i)?,
            None => 480.0,
        };
        let max_vw_value: f64 = match max_vw {
            Some(i) => self.strip_unit(i)?,
            None => 1280.0,
        };

        let min_size_unit = self.mrs_regex.find(min_size).map_or("", |m| m.as_str());
        let max_size_unit = self.mrs_regex.find(max_size).map_or("", |m| m.as_str());
        let min_vw_unit = match min_vw {
            Some(i) => self.mrs_regex.find(i).map_or("", |m| m.as_str()),
            None => "px",
        };
        let max_vw_unit = match max_vw {
            Some(i) => self.mrs_regex.find(i).map_or("", |m| m.as_str()),
            None => "px",
        };

        let full_min_vw = format!("{min_vw_value}{min_vw_unit}");
        let full_max_vw = format!("{max_vw_value}{max_vw_unit}");

        // update state and handle different screen sizes
        if screen_sizes_state.is_empty() {
            screen_sizes_state.insert(full_min_vw.clone());
            screen_sizes_state.insert(full_max_vw.clone());
        } else if screen_sizes_state.len() == 2
            && (screen_sizes_state.get(&full_min_vw).is_none()
                || screen_sizes_state.get(&full_max_vw).is_none())
        {
            return Err(GrimoireCssError::InvalidInput(
                "Different screen sizes are not allowed in one rule".to_string(),
            ));
        } else if screen_sizes_state.len() != 2 {
            return Err(GrimoireCssError::InvalidInput(format!(
                "Unexpected screen size state: {screen_sizes_state:?}"
            )));
        }

        if min_vw_unit == max_vw_unit
            && min_vw_unit == min_size_unit
            && min_vw_unit == max_size_unit
        {
            let vw_diff = max_vw_value - min_vw_value;
            let size_diff = max_size_value - min_size_value;

            let base = min_size.to_owned();
            let media: [(String, String); 2] = [
                (
                    format!("{min_vw_value}{min_vw_unit}"),
                    format!(
                        "calc({min_size} + {size_diff} * ((100vw - {min_vw_value}{min_vw_unit}) / {vw_diff}))"
                    ),
                ),
                (format!("{max_vw_value}{max_vw_unit}"), max_size.to_string()),
            ];

            Ok(Some((base, media)))
        } else {
            Ok(None)
        }
    }

    /// Generates a fluid size using CSS clamp() function based on the given parameters.
    ///
    /// # Arguments
    ///
    /// * `min_size` - A reference to the minimum size string (e.g., "100px").
    /// * `max_size` - A reference to the maximum size string (e.g., "200px").
    /// * `min_vw` - An optional reference to the minimum viewport width (e.g., "480px").
    /// * `max_vw` - An optional reference to the maximum viewport width (e.g., "1280px").
    ///
    /// # Returns
    ///
    /// * `Ok(String)` containing the CSS clamp() function with calculated values.
    /// * `Err(GrimoireCSSError)` if there is an error in processing the sizes.
    fn make_fluid_size(
        &self,
        min_size: &str,
        max_size: &str,
        min_vw: Option<&str>,
        max_vw: Option<&str>,
    ) -> Result<String, GrimoireCssError> {
        let min_size_value: f64 = self.strip_unit(min_size)?;
        let max_size_value: f64 = self.strip_unit(max_size)?;

        let min_vw_value: f64 = match min_vw {
            Some(i) => self.strip_unit(i)?,
            None => 480.0,
        };
        let max_vw_value: f64 = match max_vw {
            Some(i) => self.strip_unit(i)?,
            None => 1280.0,
        };
        let min_size_unit = self.mrs_regex.find(min_size).map_or("", |m| m.as_str());
        let max_size_unit = self.mrs_regex.find(max_size).map_or("", |m| m.as_str());

        if min_size_unit != max_size_unit {
            return Err(GrimoireCssError::InvalidInput(
                "Units must be consistent".to_string(),
            ));
        }

        if min_vw_value == max_vw_value {
            return Err(GrimoireCssError::InvalidInput(
                "Viewport widths must differ".to_string(),
            ));
        }

        let vw_diff = max_vw_value - min_vw_value;
        let size_diff = max_size_value - min_size_value;

        let slope = size_diff / vw_diff;
        let intercept = min_size_value - (slope * min_vw_value);

        let preferred = format!("{}vw + {}{}", slope * 100.0, intercept, min_size_unit);

        Ok(format!("clamp({min_size}, {preferred}, {max_size})"))
    }

    /// Strips the unit from a CSS size value and returns the numeric part.
    ///
    /// # Arguments
    ///
    /// * `value` - A reference to the value string containing the unit.
    ///
    /// # Returns
    ///
    /// * `Ok(f64)` containing the numeric part of the value.
    /// * `Err(GrimoireCSSError)` if there is an error during unit stripping.
    fn strip_unit(&self, value: &str) -> Result<f64, GrimoireCssError> {
        if let Some(captures) = self.unit_regex.captures(value) {
            captures[1].parse::<f64>().map_err(|_| {
                GrimoireCssError::InvalidInput(format!("Failed to parse unit from value: {value}"))
            })
        } else {
            Err(GrimoireCssError::InvalidInput(format!(
                "No numeric value found in: {value}"
            )))
        }
    }

    pub fn replace_class_name(
        &self,
        old_class_name: &str,
        new_class_name: &str,
        generated_css: &str,
    ) -> String {
        generated_css.replace(old_class_name, new_class_name)
    }

    pub fn find_grimoire_animation_name(adapted_target: &str) -> Option<&str> {
        adapted_target
            .split_whitespace()
            .find(|&target| ANIMATIONS.contains_key(target))
    }

    pub fn get_keyframe_class_from_animation(
        &self,
        animation: &str,
        animation_name: &str,
    ) -> Result<(String, String), GrimoireCssError> {
        let mut keyframes = animation.to_string();

        if let Some(class_block_match) = self.animation_block_regex.find(&keyframes) {
            let class_block = class_block_match.as_str().to_string();
            keyframes.replace_range(class_block_match.range(), "");

            Ok((keyframes.trim().to_string(), class_block))
        } else {
            Err(GrimoireCssError::InvalidInput(format!(
                "No keyframes found in animation: {animation_name}"
            )))
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::core::{ConfigFs, GrimoireCssError, css_generator::CssGenerator, spell::Spell};

    #[test]
    fn test_escape_css_class_name() {
        let config = ConfigFs::default();
        let generator = CssGenerator::new(&config.variables, &config.custom_animations).unwrap();

        let class_name = "g!font-size=mrs(14px_16px_380px_800px);";
        let result = generator.escape_css_class_name(class_name);

        assert!(result.is_ok());
        let escaped_name = result.unwrap();

        assert_eq!(
            escaped_name,
            r"g\!font-size\=mrs\(14px\_16px\_380px\_800px\)\;"
        );
    }

    #[test]
    fn test_generate_css_class_name() {
        let config = ConfigFs::default();
        let generator = CssGenerator::new(&config.variables, &config.custom_animations).unwrap();

        let raw_spell = "md__{_>_p}hover:h=100px";
        let effects = "hover".to_string();
        let raw_spell_focus = "_>_p";
        let with_template = false;

        let result =
            generator.generate_css_class_name(raw_spell, &effects, raw_spell_focus, with_template);

        assert!(result.is_ok());
        let (class_name, base_name) = result.unwrap();

        assert_eq!(class_name, r".md\_\_\{\_\>\_p\}hover\:h\=100px:hover > p");
        assert_eq!(base_name, r".md\_\_\{\_\>\_p\}hover\:h\=100px");
    }

    #[test]
    fn test_generate_base_and_additional_css_g_anim() -> Result<(), GrimoireCssError> {
        let config = ConfigFs::default();
        let generator = CssGenerator::new(&config.variables, &config.custom_animations).unwrap();

        let raw_spell = "md__{_>_p}hover:g-anim=bounce-in";
        let effects = "hover".to_string();
        let raw_spell_focus = "_>_p";
        let with_template = false;
        let adapted_target = "bounce-in";

        let (class_name, _) = generator.generate_css_class_name(
            raw_spell,
            &effects,
            raw_spell_focus,
            with_template,
        )?;

        let result =
            generator.generate_base_and_additional_css(adapted_target, &class_name, "g-anim");

        assert!(result.is_ok());
        let (base_css, additional_css) = result.unwrap();

        let expect = r#".md\_\_\{\_\>\_p\}hover\:g-anim\=bounce-in:hover > p {
  animation-duration: 0.75s;
  animation-name: bounce-in;
}"#;

        assert_eq!(base_css, expect);
        assert!(additional_css.is_some());
        assert!(additional_css.unwrap().starts_with("@keyframes bounce-in"));

        Ok(())
    }

    #[test]
    fn test_generate_base_and_additional_css_anim_n() -> Result<(), GrimoireCssError> {
        let config = ConfigFs::default();
        let generator = CssGenerator::new(&config.variables, &config.custom_animations).unwrap();

        let raw_spell = "md__{_>_p}hover:anim-n=swing";
        let effects = "hover".to_string();
        let raw_spell_focus = "_>_p";
        let with_template = false;
        let adapted_target = "swing";

        let (class_name, _) = generator.generate_css_class_name(
            raw_spell,
            &effects,
            raw_spell_focus,
            with_template,
        )?;

        let result = generator.generate_base_and_additional_css(
            adapted_target,
            &class_name,
            "animation-name",
        );

        assert!(result.is_ok());
        let (base_css, additional_css) = result.unwrap();

        assert_eq!(
            base_css,
            r".md\_\_\{\_\>\_p\}hover\:anim-n\=swing:hover > p{animation-name:swing;}"
        );
        assert!(additional_css.is_some());
        assert!(additional_css.unwrap().starts_with("@keyframes swing"));

        Ok(())
    }

    #[test]
    fn test_generate_base_and_additional_css_animation() -> Result<(), GrimoireCssError> {
        let config = ConfigFs::default();
        let generator = CssGenerator::new(&config.variables, &config.custom_animations).unwrap();

        let raw_spell = "anim=3s_linear_wobble";
        let effects = String::new();
        let raw_spell_focus = String::new();
        let with_template = false;
        let adapted_target = "3s linear wobble";

        let (class_name, _) = generator.generate_css_class_name(
            raw_spell,
            &effects,
            &raw_spell_focus,
            with_template,
        )?;

        let result =
            generator.generate_base_and_additional_css(adapted_target, &class_name, "animation");

        assert!(result.is_ok());
        let (base_css, additional_css) = result.unwrap();

        assert_eq!(
            base_css,
            r".anim\=3s\_linear\_wobble{animation:3s linear wobble;}"
        );
        assert!(additional_css.is_some());
        assert!(additional_css.unwrap().starts_with("@keyframes wobble"));

        Ok(())
    }

    #[test]
    fn test_generate_base_and_additional_css_regular_spell() -> Result<(), GrimoireCssError> {
        let config = ConfigFs::default();
        let generator = CssGenerator::new(&config.variables, &config.custom_animations).unwrap();

        let raw_spell = "d=grid";
        let effects = String::new();
        let raw_spell_focus = String::new();
        let with_template = false;
        let adapted_target = "grid";

        let (class_name, _) = generator.generate_css_class_name(
            raw_spell,
            &effects,
            &raw_spell_focus,
            with_template,
        )?;

        let result =
            generator.generate_base_and_additional_css(adapted_target, &class_name, "display");

        assert!(result.is_ok());
        let (base_css, additional_css) = result.unwrap();

        assert_eq!(base_css, r".d\=grid{display:grid;}");
        assert!(additional_css.is_none());

        Ok(())
    }

    #[test]
    fn test_generate_base_and_additional_css_templated_spell() -> Result<(), GrimoireCssError> {
        let config = ConfigFs::default();
        let generator = CssGenerator::new(&config.variables, &config.custom_animations).unwrap();

        let raw_spell = "d=grid";
        let effects = String::new();
        let raw_spell_focus = String::new();
        let with_template = true;
        let adapted_target = "grid";

        let (class_name, _) = generator.generate_css_class_name(
            raw_spell,
            &effects,
            &raw_spell_focus,
            with_template,
        )?;

        let result =
            generator.generate_base_and_additional_css(adapted_target, &class_name, "display");

        assert!(result.is_ok());
        let (base_css, additional_css) = result.unwrap();

        assert_eq!(base_css, r".g\!d\=grid\;{display:grid;}");
        assert!(additional_css.is_none());

        Ok(())
    }

    #[test]
    fn test_handle_generic_css() {
        let config = ConfigFs::default();
        let generator = CssGenerator::new(&config.variables, &config.custom_animations).unwrap();

        let adapted_target = "100px";
        let css_class_name = ".test-class";
        let property = "width";

        let result = generator.handle_generic_css(adapted_target, css_class_name, property);

        assert!(result.is_ok());
        let (base_css, additional_css) = result.unwrap();

        assert_eq!(base_css, ".test-class{width:100px;}");
        assert!(additional_css.is_none());
    }

    #[test]
    fn test_wrap_base_css_with_media_query() {
        let config = ConfigFs::default();
        let generator = CssGenerator::new(&config.variables, &config.custom_animations).unwrap();

        let base_css = ".test-class{width:100px;}";

        let result = generator.wrap_base_css_with_media_query("sm", base_css);

        assert_eq!(
            result,
            "@media (min-width: 640px){.test-class{width:100px;}}"
        );
    }

    #[test]
    fn test_generate_css() {
        let config = ConfigFs::default();
        let generator = CssGenerator::new(&config.variables, &config.custom_animations).unwrap();

        let spell = Spell {
            raw_spell: "bg-c=pink".to_string(),
            component: "bg-c".to_string(),
            component_target: "pink".to_string(),
            effects: "".to_string(),
            area: "".to_string(),
            focus: "".to_string(),
            with_template: false,
            scroll_spells: None,
        };

        let result = generator.generate_css(&spell);

        assert!(result.is_ok());

        let option_value = result.unwrap();

        assert!(option_value.is_some());

        let (css, _, _) = option_value.unwrap();

        assert_eq!(css, ".bg-c\\=pink{background-color:pink;}");

        // --- COMPLEX ---

        let spell_complex = Spell {
            raw_spell: "{[data-theme='light']_p}font-sz=mrs(14px_16px_380px_800px)".to_string(),
            component: "font-sz".to_string(),
            component_target: "mrs(14px_16px_380px_800px)".to_string(),
            effects: "".to_string(),
            area: "".to_string(),
            focus: "[data-theme='light']_p".to_string(),
            with_template: true,
            scroll_spells: None,
        };

        let result = generator.generate_css(&spell_complex);

        assert!(result.is_ok());

        let option_value = result.unwrap();

        assert!(option_value.is_some());

        let (css, _, _) = option_value.unwrap();

        assert_eq!(
            css,
            r".g\!\{\[data-theme\=\'light\'\]\_p\}font-sz\=mrs\(14px\_16px\_380px\_800px\)\;[data-theme='light'] p{font-size:14px;}@media screen and (min-width: 380px) {.g\!\{\[data-theme\=\'light\'\]\_p\}font-sz\=mrs\(14px\_16px\_380px\_800px\)\;[data-theme='light'] p{font-size: calc(14px + 2 * ((100vw - 380px) / 420));}}@media screen and (min-width: 800px) {.g\!\{\[data-theme\=\'light\'\]\_p\}font-sz\=mrs\(14px\_16px\_380px\_800px\)\;[data-theme='light'] p{font-size: 16px;}}"
        );
    }

    #[test]
    fn test_make_fluent_size() {
        let config = ConfigFs::default();
        let generator = CssGenerator::new(&config.variables, &config.custom_animations).unwrap();

        // regular case
        let result = generator.make_fluid_size("14px", "16px", Some("380px"), Some("800px"));
        assert!(result.is_ok());

        let clamp_value = result.unwrap();
        assert_eq!(
            clamp_value,
            "clamp(14px, 0.4761904761904762vw + 12.19047619047619px, 16px)"
        );

        // case without min_vw и max_vw
        let result = generator.make_fluid_size("10rem", "20rem", None, None);
        assert!(result.is_ok());

        let clamp_value = result.unwrap();
        assert_eq!(clamp_value, "clamp(10rem, 1.25vw + 4rem, 20rem)");

        // units are not consistent (error)
        let result = generator.make_fluid_size("10px", "20rem", Some("400px"), Some("1200px"));
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "Invalid input: Units must be consistent");
        }

        // screen width are not consistent (error)
        let result = generator.make_fluid_size("14px", "16px", Some("400px"), Some("400px"));
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(
                err.to_string(),
                "Invalid input: Viewport widths must differ"
            );
        }
    }
}
