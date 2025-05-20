//! This module defines the base `Parser` struct, which is responsible for collecting CSS class names
//! and templated spells from content strings. It uses regular expressions to find and extract
//! class names and spell patterns.

use regex::Regex;
use std::collections::HashSet;

use crate::core::GrimoireCssError;

/// Base `Parser` is responsible for extracting CSS class names and templated spells from content.
/// It uses regular expressions to find class names and spell-like patterns.
pub struct Parser {
    tepmplated_spell_regex: Regex,
    class_name_regex: Regex,
    class_regex: Regex,
}

impl Parser {
    /// Creates a new `Parser` instance with predefined regular expressions for extracting class names
    /// and templated spells.
    pub fn new() -> Self {
        let class_name_regex = Regex::new(r#"className=("([^"]*)"|'([^']*)'|`([^`]*)`)"#).unwrap();
        let class_regex = Regex::new(r#"class=("([^"]*)"|'([^']*)'|`([^`]*)`)"#).unwrap();
        let tepmplated_spell_regex = Regex::new(r#"(g![^;]*;)"#).unwrap();

        Self {
            tepmplated_spell_regex,
            class_name_regex,
            class_regex,
        }
    }

    /// Collects class names from content based on the given regular expression and optional predicate/splitter functions.
    ///
    /// # Arguments
    ///
    /// * `content` - The content to be parsed.
    /// * `regex` - A regular expression used to search for class names.
    /// * `predicate` - An optional function used to filter the results.
    /// * `splitter` - An optional function used to split the result into multiple class names.
    /// * `class_names` - A mutable reference to a vector to store the collected class names.
    /// * `seen_class_names` - A mutable reference to a `HashSet` used to track seen class names.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCSSError` if there is an issue during processing.
    fn collect_classes<P, S>(
        content: &str,
        regex: &Regex,
        mut predicate: Option<P>,
        mut splitter: Option<S>,
        class_names: &mut Vec<String>,
        seen_class_names: &mut HashSet<String>,
        is_templated_spell: bool,
    ) -> Result<(), GrimoireCssError>
    where
        P: FnMut(&str) -> bool,
        S: FnMut(&str) -> Vec<String>,
    {
        for cap in regex.captures_iter(content) {
            let class_value = if is_templated_spell {
                cap.get(1).map(|m| m.as_str()).unwrap_or("")
            } else {
                cap.get(2)
                    .or_else(|| cap.get(3))
                    .or_else(|| cap.get(4))
                    .map(|m| m.as_str())
                    .unwrap_or("")
            };

            let classes = if let Some(splitter_fn) = &mut splitter {
                splitter_fn(class_value)
            } else {
                vec![class_value.to_string()]
            };

            for class in classes {
                let should_include = predicate.as_mut().is_none_or(|p| p(&class));

                if should_include && !seen_class_names.contains(&class) {
                    seen_class_names.insert(class.clone());
                    class_names.push(class);
                }
            }
        }

        Ok(())
    }

    /// Collects all class names and templated spells from content.
    ///
    /// # Arguments
    ///
    /// * `content` - The content to parse
    /// * `class_names` - A mutable reference to a vector that stores the collected class names
    /// * `seen_class_names` - A mutable reference to a HashSet for tracking seen class names
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub fn collect_candidates(
        &self,
        content: &str,
        class_names: &mut Vec<String>,
        seen_class_names: &mut HashSet<String>,
    ) -> Result<(), GrimoireCssError> {
        let whitespace_splitter = |input: &str| {
            input
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
        };

        // Collect all 'className' matches
        Self::collect_classes::<fn(&str) -> bool, fn(&str) -> Vec<String>>(
            content,
            &self.class_name_regex,
            None,
            Some(whitespace_splitter),
            class_names,
            seen_class_names,
            false,
        )?;

        // Collect all 'class' matches
        Self::collect_classes::<fn(&str) -> bool, fn(&str) -> Vec<String>>(
            content,
            &self.class_regex,
            None,
            Some(whitespace_splitter),
            class_names,
            seen_class_names,
            false,
        )?;

        // Collect all 'templated class' (starts with 'g!', ends with ';') matches
        Self::collect_classes::<fn(&str) -> bool, fn(&str) -> Vec<String>>(
            content,
            &self.tepmplated_spell_regex,
            None,
            None,
            class_names,
            seen_class_names,
            true,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_class_names() {
        let parser = Parser::new();
        let mut class_names = Vec::new();
        let mut seen_class_names = HashSet::new();

        let content = r#"
            <div class="test1 test2"></div>
            <div className="test3 test4"></div>
            <div class="test1"></div>
            <span g!display=block;></span>
        "#;

        parser
            .collect_candidates(content, &mut class_names, &mut seen_class_names)
            .unwrap();

        assert_eq!(class_names.len(), 5);
        assert!(class_names.contains(&"test1".to_string()));
        assert!(class_names.contains(&"test2".to_string()));
        assert!(class_names.contains(&"test3".to_string()));
        assert!(class_names.contains(&"test4".to_string()));
        assert!(class_names.contains(&"g!display=block;".to_string()));
    }

    #[test]
    fn test_collect_templated_spells() {
        let parser = Parser::new();
        let mut class_names = Vec::new();
        let mut seen_class_names = HashSet::new();

        let content = r#"
            <div g!display=flex;></div>
            <div g!color=red;></div>
            <div g!display=flex;></div>
        "#;

        parser
            .collect_candidates(content, &mut class_names, &mut seen_class_names)
            .unwrap();

        assert_eq!(class_names.len(), 2);
        assert!(class_names.contains(&"g!display=flex;".to_string()));
        assert!(class_names.contains(&"g!color=red;".to_string()));
    }

    #[test]
    fn test_collect_class_and_classname_attributes() {
        let parser = Parser::new();
        let mut class_names = Vec::new();
        let mut seen_class_names = HashSet::new();

        let content = r#"
            <div class="test1"></div>
            <div className="test2"></div>
            <div class='test3'></div>
            <div className='test4'></div>
            <div class=`test5`></div>
            <div className=`test6`></div>
        "#;

        parser
            .collect_candidates(content, &mut class_names, &mut seen_class_names)
            .unwrap();

        assert_eq!(class_names.len(), 6);
        for i in 1..=6 {
            assert!(class_names.contains(&format!("test{}", i)));
        }
    }
}
