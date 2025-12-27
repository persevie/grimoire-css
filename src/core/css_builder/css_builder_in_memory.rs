//! Provides in-memory CSS building without filesystem dependencies.
//!
//! Unlike [`CssBuilderFs`], this implementation works entirely in memory
//! and is suitable for environments where file I/O is not desired.

use std::collections::HashSet;
use std::sync::Arc;

use crate::core::{
    CssOptimizer, GrimoireCssError, compiled_css::CompiledCssInMemory,
    config::config_in_memory::ConfigInMemory, parser::Parser, source_file::SourceFile,
    spell::Spell,
};

use super::CssBuilder;

/// Manages CSS compilation purely in memory
pub struct CssBuilderInMemory<'a> {
    css_builder: CssBuilder<'a>,
    config: &'a ConfigInMemory,
    parser: Parser,
}

impl<'a> CssBuilderInMemory<'a> {
    /// Creates a new `CssBuilderInMemory` instance.
    ///
    /// # Arguments
    ///
    /// * `config` - Reference to the in-memory Grimoire CSS configuration
    /// * `optimizer` - A reference to an implementation of the `CSSOptimizer` trait
    ///
    /// # Returns
    ///
    /// * `Ok(CssBuilderInMemory)` - A new instance ready for building CSS
    /// * `Err(GrimoireCSSError)` - If initialization fails
    pub fn new<O: CssOptimizer>(
        config: &'a ConfigInMemory,
        optimizer: &'a O,
    ) -> Result<Self, GrimoireCssError> {
        let css_builder = CssBuilder::new(optimizer, &config.variables, &config.custom_animations)?;
        let parser = Parser::new();

        Ok(Self {
            css_builder,
            config,
            parser,
        })
    }

    /// Builds CSS for all projects in the configuration
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<CompiledCssInMemory>)` - Vector of compiled CSS results
    /// * `Err(GrimoireCSSError)` - If the build process fails
    pub fn build(&mut self) -> Result<Vec<CompiledCssInMemory>, GrimoireCssError> {
        let mut results = Vec::new();

        for project in &self.config.projects {
            let mut class_names = Vec::new();
            let mut seen_class_names = HashSet::new();

            // Join all spells into a single string for parsing
            let content = project.content.join(" ");
            self.parser
                .collect_candidates(&content, &mut class_names, &mut seen_class_names)?;

            let source = Arc::new(SourceFile::new(None, project.name.clone(), content));

            // Generate spells using empty shared_spells set since we're working in memory
            let spells = Spell::generate_spells_from_classes(
                class_names,
                &HashSet::new(),
                &self.config.scrolls,
                Some(source),
            )?;

            // Combine spells into CSS
            // Avoid validate() + optimize() double-parsing for the common success path.
            let css = self
                .css_builder
                .combine_spells_to_optimized_css_string(&spells)?;

            results.push(CompiledCssInMemory {
                name: project.name.clone(),
                content: css,
            });
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::config_in_memory::ConfigInMemoryEntry;
    use std::collections::HashMap;

    struct MockOptimizer;

    impl CssOptimizer for MockOptimizer {
        fn optimize(&self, css: &str) -> Result<String, GrimoireCssError> {
            Ok(css.to_string())
        }

        fn validate(&self, _css: &str) -> Result<(), GrimoireCssError> {
            Ok(())
        }
    }

    #[test]
    fn test_builder_empty_config() {
        let config = ConfigInMemory {
            projects: vec![],
            variables: None,
            scrolls: None,
            custom_animations: HashMap::new(),
            browserslist_content: None,
        };

        let optimizer = MockOptimizer;
        let mut builder = CssBuilderInMemory::new(&config, &optimizer).unwrap();
        let result = builder.build().unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_builder_with_simple_project() {
        let config = ConfigInMemory {
            projects: vec![ConfigInMemoryEntry {
                name: "test".to_string(),
                content: vec!["<p class='display=flex'>".to_string()],
            }],
            variables: None,
            scrolls: None,
            custom_animations: HashMap::new(),
            browserslist_content: None,
        };

        let optimizer = MockOptimizer;
        let mut builder = CssBuilderInMemory::new(&config, &optimizer).unwrap();
        let result = builder.build().unwrap();

        println!("result: {result:?}");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "test");
        assert!(result[0].content.eq(".display\\=flex{display:flex;}"));
    }

    #[test]
    fn test_builder_with_templated_scroll_invocation() {
        let mut scrolls_map: HashMap<String, Vec<String>> = HashMap::new();
        scrolls_map.insert(
            "complex-card".to_string(),
            vec!["h=$".to_string(), "c=$".to_string(), "w=$".to_string()],
        );

        let config = ConfigInMemory {
            projects: vec![ConfigInMemoryEntry {
                name: "test".to_string(),
                content: vec!["<div g!complex-card=120px_red_100px;></div>".to_string()],
            }],
            variables: None,
            scrolls: Some(scrolls_map),
            custom_animations: HashMap::new(),
            browserslist_content: None,
        };

        let optimizer = MockOptimizer;
        let mut builder = CssBuilderInMemory::new(&config, &optimizer).unwrap();
        let result = builder.build().unwrap();

        assert_eq!(result.len(), 1);
        let css = &result[0].content;

        // The output must use the outer template selector, not the inner scroll spell selectors.
        assert!(css.contains(".g\\!complex-card\\=120px\\_red\\_100px\\;{height:120px;}"));
        assert!(css.contains(".g\\!complex-card\\=120px\\_red\\_100px\\;{color:red;}"));
        assert!(css.contains(".g\\!complex-card\\=120px\\_red\\_100px\\;{width:100px;}"));

        assert!(!css.contains(".h\\=120px"));
        assert!(!css.contains(".c\\=red"));
        assert!(!css.contains(".w\\=100px"));
    }

    #[test]
    fn test_builder_with_templated_scroll_invocation_with_prefixes() {
        let mut scrolls_map: HashMap<String, Vec<String>> = HashMap::new();
        scrolls_map.insert(
            "complex-card".to_string(),
            vec!["h=$".to_string(), "c=$".to_string(), "w=$".to_string()],
        );

        let config = ConfigInMemory {
            projects: vec![ConfigInMemoryEntry {
                name: "test".to_string(),
                // Prefixes live on the scroll invocation and must apply to all expanded spells.
                // - md__      => @media (min-width: 768px)
                // - hover:    => :hover pseudo
                // - {_>_p}    => " > p" focus selector
                content: vec![
                    "<div g!md__{_>_p}hover:complex-card=120px_red_100px;></div>".to_string(),
                ],
            }],
            variables: None,
            scrolls: Some(scrolls_map),
            custom_animations: HashMap::new(),
            browserslist_content: None,
        };

        let optimizer = MockOptimizer;
        let mut builder = CssBuilderInMemory::new(&config, &optimizer).unwrap();
        let result = builder.build().unwrap();

        assert_eq!(result.len(), 1);
        let css = &result[0].content;

        // Ensure the area prefix becomes a media query.
        assert!(css.contains("@media (min-width: 768px)"));
        // Ensure effects+focus survive the selector replacement.
        assert!(css.contains(":hover > p"));

        // Ensure the outer template selector is used (not inner h=/c=/w= selectors).
        assert!(css.contains(
            ".g\\!md\\_\\_\\{\\_\\>\\_p\\}hover\\:complex-card\\=120px\\_red\\_100px\\;"
        ));

        assert!(!css.contains(".md\\_\\_\\{\\_\\>\\_p\\}hover\\:h\\=120px"));
        assert!(!css.contains(".md\\_\\_\\{\\_\\>\\_p\\}hover\\:c\\=red"));
        assert!(!css.contains(".md\\_\\_\\{\\_\\>\\_p\\}hover\\:w\\=100px"));
    }
}
