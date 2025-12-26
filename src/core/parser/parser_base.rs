//! This module defines the base `Parser` struct, which is responsible for collecting CSS class names
//! and templated spells from content strings. It uses regular expressions to find and extract
//! class names and spell patterns.

use regex::Regex;
use std::collections::HashSet;

use crate::core::GrimoireCssError;

/// Represents the type of class collection being performed
#[derive(Debug, Clone, Copy)]
enum CollectionType {
    TemplatedSpell,
    CurlyClass,
    RegularClass,
}

/// Base `Parser` is responsible for extracting CSS class names and templated spells from content.
/// It uses regular expressions to find class names and spell-like patterns.
pub struct Parser {
    tepmplated_spell_regex: Regex,
    class_name_regex: Regex,
    class_regex: Regex,
    curly_class_name_regex: Regex,
    curly_class_regex: Regex,
}

impl Parser {
    /// Creates a new `Parser` instance with predefined regular expressions for extracting class names
    /// and templated spells.
    pub fn new() -> Self {
        let class_name_regex = Regex::new(r#"className=("([^"]*)"|'([^']*)'|`([^`]*)`)"#).unwrap();
        let class_regex = Regex::new(r#"class=("([^"]*)"|'([^']*)'|`([^`]*)`)"#).unwrap();
        let tepmplated_spell_regex = Regex::new(r#"(g![^;]*;)"#).unwrap();
        let curly_class_name_regex = Regex::new(r#"className=\{((?:[^{}]|\{[^}]*\})*)\}"#).unwrap();
        let curly_class_regex = Regex::new(r#"class=\{((?:[^{}]|\{[^}]*\})*)\}"#).unwrap();

        Self {
            tepmplated_spell_regex,
            class_name_regex,
            class_regex,
            curly_class_name_regex,
            curly_class_regex,
        }
    }

    /// Removes unpaired brackets and quotes from a string
    fn clean_unpaired_brackets(s: &str) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut result = Vec::with_capacity(chars.len());
        let mut stack = Vec::new();
        let mut keep = vec![false; chars.len()];

        // First pass: mark paired brackets
        for (i, &ch) in chars.iter().enumerate() {
            match ch {
                '(' | '[' | '{' => stack.push((ch, i)),
                ')' => {
                    if let Some((open, open_idx)) = stack.pop()
                        && open == '('
                    {
                        keep[open_idx] = true;
                        keep[i] = true;
                    }
                }
                ']' => {
                    if let Some((open, open_idx)) = stack.pop()
                        && open == '['
                    {
                        keep[open_idx] = true;
                        keep[i] = true;
                    }
                }
                '}' => {
                    if let Some((open, open_idx)) = stack.pop()
                        && open == '{'
                    {
                        keep[open_idx] = true;
                        keep[i] = true;
                    }
                }
                _ => {}
            }
        }

        // Second pass: build result, keeping only paired brackets and other chars
        for (i, &ch) in chars.iter().enumerate() {
            match ch {
                '(' | ')' | '[' | ']' | '{' | '}' => {
                    if keep[i] {
                        result.push(ch);
                    }
                }
                '\'' | '"' | '`' => {} // Remove quotes
                _ => result.push(ch),
            }
        }

        result.into_iter().collect()
    }

    /// Collects class names from content based on the given regular expression.
    ///
    /// # Arguments
    ///
    /// * `content` - The content to be parsed.
    /// * `regex` - A regular expression used to search for class names.
    /// * `split_by_whitespace` - Whether to split the matched value by whitespace.
    /// * `class_names` - A mutable reference to a vector to store the collected class names and their spans.
    /// * `seen_class_names` - A mutable reference to a `HashSet` used to track seen class names.
    /// * `collection_type` - The type of collection being performed.
    ///
    /// # Errors
    ///
    /// Returns a `GrimoireCssError` if there is an issue during processing.
    fn collect_classes(
        content: &str,
        regex: &Regex,
        split_by_whitespace: bool,
        class_names: &mut Vec<(String, (usize, usize))>,
        seen_class_names: &mut HashSet<String>,
        collection_type: CollectionType,
    ) -> Result<(), GrimoireCssError> {
        for cap in regex.captures_iter(content) {
            let match_obj = match collection_type {
                CollectionType::TemplatedSpell => cap.get(1),
                CollectionType::CurlyClass => cap.get(1),
                CollectionType::RegularClass => {
                    cap.get(2).or_else(|| cap.get(3)).or_else(|| cap.get(4))
                }
            };

            if let Some(m) = match_obj {
                let full_value = m.as_str();
                let base_offset = m.start();

                if split_by_whitespace {
                    for part in full_value.split_whitespace() {
                        // Calculate the offset of the part within the full content
                        let part_start = part.as_ptr() as usize - full_value.as_ptr() as usize;
                        let start = base_offset + part_start;
                        let length = part.len();

                        let mut class_string = part.to_string();

                        if matches!(collection_type, CollectionType::CurlyClass) {
                            class_string = Self::clean_unpaired_brackets(&class_string);
                        }

                        if !class_string.is_empty() && !seen_class_names.contains(&class_string) {
                            seen_class_names.insert(class_string.clone());
                            class_names.push((class_string, (start, length)));
                        }
                    }
                } else {
                    let start = base_offset;
                    let length = full_value.len();
                    let class_string = full_value.to_string();

                    if class_string.contains(' ') {
                        // IMPORTANT:
                        // - In HTML attributes, spaces are class separators, not part of a single class token.
                        // - If a user tries to encode a value that contains spaces (e.g. calc(100vh - 50px)),
                        //   the correct Grimoire convention is to use underscores instead: calc(100vh_-_50px).
                        //
                        // We return a Diagnostic-style error so the CLI can render it like rustc.
                        return Err(GrimoireCssError::CompileError {
                            message: "Spaces are not allowed inside a single spell token."
                                .to_string(),
                            span: (start, length),
                            label: "Error in this spell".to_string(),
                            help: Some(format!(
                                "You likely wrote a value with spaces inside a class attribute (HTML treats spaces as class separators).\n\
Fix: replace spaces with '_' inside the value, e.g.:\n\
    h=calc(100vh - 50px)  ->  h=calc(100vh_-_50px)\n\n\
Offending spell: '{class_string}'"
                            )),
                            source_file: None,
                        });
                    }

                    if !class_string.is_empty() && !seen_class_names.contains(&class_string) {
                        seen_class_names.insert(class_string.clone());
                        class_names.push((class_string, (start, length)));
                    }
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
    /// * `class_names` - A mutable reference to a vector that stores the collected class names and their spans
    /// * `seen_class_names` - A mutable reference to a HashSet for tracking seen class names
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub fn collect_candidates(
        &self,
        content: &str,
        class_names: &mut Vec<(String, (usize, usize))>,
        seen_class_names: &mut HashSet<String>,
    ) -> Result<(), GrimoireCssError> {
        // Collect all 'className' matches
        Self::collect_classes(
            content,
            &self.class_name_regex,
            true,
            class_names,
            seen_class_names,
            CollectionType::RegularClass,
        )?;

        // Collect all 'class' matches
        Self::collect_classes(
            content,
            &self.class_regex,
            true,
            class_names,
            seen_class_names,
            CollectionType::RegularClass,
        )?;

        // Collect all 'templated class' (starts with 'g!', ends with ';') matches
        Self::collect_classes(
            content,
            &self.tepmplated_spell_regex,
            false,
            class_names,
            seen_class_names,
            CollectionType::TemplatedSpell,
        )?;

        // Collect all curly 'className' matches
        Self::collect_classes(
            content,
            &self.curly_class_name_regex,
            true,
            class_names,
            seen_class_names,
            CollectionType::CurlyClass,
        )?;

        // Collect all curly 'class' matches
        Self::collect_classes(
            content,
            &self.curly_class_regex,
            true,
            class_names,
            seen_class_names,
            CollectionType::CurlyClass,
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

        let names: Vec<String> = class_names.iter().map(|(n, _)| n.clone()).collect();
        assert!(names.contains(&"test1".to_string()));
        assert!(names.contains(&"test2".to_string()));
        assert!(names.contains(&"test3".to_string()));
        assert!(names.contains(&"test4".to_string()));
        assert!(names.contains(&"g!display=block;".to_string()));
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
        let names: Vec<String> = class_names.iter().map(|(n, _)| n.clone()).collect();
        assert!(names.contains(&"g!display=flex;".to_string()));
        assert!(names.contains(&"g!color=red;".to_string()));
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
        let names: Vec<String> = class_names.iter().map(|(n, _)| n.clone()).collect();
        for i in 1..=6 {
            assert!(names.contains(&format!("test{i}")));
        }
    }

    #[test]
    fn test_collect_curly_class_and_classname_attributes() {
        let parser = Parser::new();
        let mut class_names = Vec::new();
        let mut seen_class_names = HashSet::new();

        let content = r#"
            <div className={isError ? 'color=red regular-class-error' : 'color=green regular-class-success'}></div>
            <div class={`display=grid state-${state}`}></div>
        "#;

        parser
            .collect_candidates(content, &mut class_names, &mut seen_class_names)
            .unwrap();

        assert_eq!(class_names.len(), 9);

        let names: Vec<String> = class_names.iter().map(|(n, _)| n.clone()).collect();
        assert!(names.contains(&"isError".to_string()));
        assert!(names.contains(&"?".to_string()));
        assert!(names.contains(&"color=red".to_string()));
        assert!(names.contains(&"regular-class-error".to_string()));
        assert!(names.contains(&":".to_string()));
        assert!(names.contains(&"color=green".to_string()));
        assert!(names.contains(&"regular-class-success".to_string()));
        assert!(names.contains(&"display=grid".to_string()));
        assert!(names.contains(&"state-${state}".to_string()));
    }

    #[test]
    fn test_clean_unpaired_brackets() {
        let parser = Parser::new();
        let mut class_names = Vec::new();
        let mut seen_class_names = HashSet::new();

        let content = r#"
            <div className={`class-with-{unpaired} (brackets] and [quotes"`}></div>
            <div class={`normal-class {paired} [brackets] (work)`}></div>
        "#;

        parser
            .collect_candidates(content, &mut class_names, &mut seen_class_names)
            .unwrap();

        // Should clean unpaired brackets and quotes
        let names: Vec<String> = class_names.iter().map(|(n, _)| n.clone()).collect();
        assert!(names.contains(&"class-with-{unpaired}".to_string()));
        assert!(names.contains(&"brackets".to_string()));
        assert!(names.contains(&"and".to_string()));
        assert!(names.contains(&"quotes".to_string()));
        assert!(names.contains(&"normal-class".to_string()));
        assert!(names.contains(&"{paired}".to_string()));
        assert!(names.contains(&"[brackets]".to_string()));
        assert!(names.contains(&"(work)".to_string()));
    }

    #[test]
    fn test_spans() {
        let parser = Parser::new();
        let mut class_names = Vec::new();
        let mut seen_class_names = HashSet::new();

        let content = r#"<div class="foo bar"></div>"#;
        //           012345678901234567890123456
        // foo is at 12..15
        // bar is at 16..19

        parser
            .collect_candidates(content, &mut class_names, &mut seen_class_names)
            .unwrap();

        assert_eq!(class_names.len(), 2);

        let foo = class_names.iter().find(|(n, _)| n == "foo").unwrap();
        assert_eq!(foo.1, (12, 3));

        let bar = class_names.iter().find(|(n, _)| n == "bar").unwrap();
        assert_eq!(bar.1, (16, 3));
    }
}
