use std::{
    collections::{HashMap, HashSet},
    fs::{self},
    ops::Range,
    path::{Path, PathBuf},
};

use crate::{
    GrimoireCssError, Spell,
    core::{CssGenerator, config::ScrollDefinition},
};
use cssparser::{
    AtRuleParser, BasicParseErrorKind, CowRcStr, DeclarationParser, ParseError, Parser,
    ParserInput, QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, StyleSheetParser, Token,
};
use glob::glob;
use indexmap::{IndexMap, IndexSet};
use lightningcss::{
    media_query::MediaList,
    properties::{Property, PropertyId},
    rules::{CssRule, CssRuleList},
    selector::{Component, Selector, SelectorList},
    stylesheet::{ParserOptions as LightningParserOptions, StyleSheet as LightningStyleSheet},
    traits::{ParseWithOptions, ToCss},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransmuteOptions {
    #[serde(default)]
    pub with_oneliner: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transmutation {
    pub scrolls: Vec<TransmutedScroll>,
}

impl Transmutation {
    pub(crate) fn validate_component_scroll_conflicts(
        &self,
        existing_names: &HashSet<String>,
    ) -> Result<(), GrimoireCssError> {
        let definitions = Some(
            existing_names
                .iter()
                .map(String::as_str)
                .chain(self.scrolls.iter().map(|scroll| scroll.name.as_str()))
                .map(|name| (name.to_string(), ScrollDefinition::default()))
                .collect::<HashMap<_, _>>(),
        );
        let shared = HashSet::new();
        for scroll in &self.scrolls {
            for token in &scroll.spells {
                // Empty definitions detect invocations without expanding Scroll bodies.
                if let Ok(Some(spell)) =
                    Spell::new(token, &shared, &definitions, (0, token.len()), None)
                    && definitions
                        .as_ref()
                        .unwrap()
                        .contains_key(spell.component())
                {
                    return Err(conversion_error(format!(
                        "CSS component/Scroll conflict: '{}' in migrated Scroll '{}' would invoke a Scroll instead of a CSS property; rename the conflicting Scroll before migration",
                        spell.component(),
                        scroll.name
                    )));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransmutedScroll {
    pub name: String,
    pub spells: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oneliner: Option<String>,
}

type TransmutedMap = IndexMap<String, IndexSet<String>>;

fn read_and_clean_files(paths: &[PathBuf]) -> Result<String, GrimoireCssError> {
    let total_size: usize = paths
        .iter()
        .filter_map(|path| fs::metadata(path).ok())
        .map(|metadata| metadata.len() as usize)
        .sum();

    let mut all_contents = String::with_capacity(total_size);

    for path in paths {
        let content = fs::read_to_string(path).map_err(|e| {
            GrimoireCssError::Io(std::io::Error::new(
                e.kind(),
                format!("Failed to read '{}': {}", path.display(), e),
            ))
        })?;

        validate_file_urls(&content, path)?;
        all_contents.push_str(&content);
        all_contents.push('\n');
    }

    if all_contents.capacity() > all_contents.len() * 2 {
        all_contents.shrink_to_fit();
    }

    Ok(all_contents)
}

// Scrolls carry no source URL, so relative URLs would lose their base.
fn validate_file_urls(css: &str, path: &Path) -> Result<(), GrimoireCssError> {
    let mut input = ParserInput::new(css);
    check_file_url_tokens(&mut Parser::new(&mut input), false).map_err(|error| {
        let message = match error.kind {
            cssparser::ParseErrorKind::Custom(error) => error.to_string(),
            _ => format!("Cannot inspect CSS URLs: {error:?}"),
        };
        conversion_error(format!("{}: {message}", path.display()))
    })
}

fn check_file_url_tokens<'i, 't>(
    input: &mut Parser<'i, 't>,
    strings_are_urls: bool,
) -> Result<(), ParseError<'i, GrimoireCssError>> {
    loop {
        let token = match input.next_including_whitespace_and_comments() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => return Ok(()),
            Err(error) => return Err(error.into()),
        };
        match token {
            Token::UnquotedUrl(url) => check_file_url(input, &url)?,
            Token::QuotedString(url) if strings_are_urls => check_file_url(input, &url)?,
            Token::Function(name) => {
                if strings_are_urls && name.eq_ignore_ascii_case("var") {
                    return Err(input.new_custom_error(conversion_error(
                        "Dynamic CSS URL cannot retain its source base; use an explicit absolute or root-relative URL before file migration",
                    )));
                }
                let is_url = name.eq_ignore_ascii_case("url") || name.eq_ignore_ascii_case("src");
                let is_image = ["image", "image-set", "-webkit-image-set"]
                    .iter()
                    .any(|candidate| name.eq_ignore_ascii_case(candidate));
                input.parse_nested_block(|nested| {
                    if is_url {
                        let url = nested.expect_string_cloned().map_err(|_| {
                            nested.new_custom_error(conversion_error(
                                "Dynamic CSS URL cannot retain its source base; use an explicit absolute or root-relative URL before file migration",
                            ))
                        })?;
                        check_file_url(nested, &url)?;
                    }
                    check_file_url_tokens(nested, is_image)
                })?;
            }
            Token::ParenthesisBlock | Token::SquareBracketBlock | Token::CurlyBracketBlock => {
                input.parse_nested_block(|nested| check_file_url_tokens(nested, false))?;
            }
            _ => {}
        }
    }
}

fn check_file_url<'i>(
    input: &Parser<'i, '_>,
    url: &str,
) -> Result<(), ParseError<'i, GrimoireCssError>> {
    let value = url.trim_matches(|ch: char| ch.is_ascii_whitespace());
    let has_scheme = value.split_once(':').is_some_and(|(scheme, _)| {
        scheme
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_alphabetic)
            && scheme
                .bytes()
                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, b'+' | b'-' | b'.'))
    });
    // Fragment-only URLs have CSS's local-url behavior, independent of this base.
    if value.starts_with('/') || value.starts_with('#') || has_scheme {
        return Ok(());
    }
    Err(input.new_custom_error(conversion_error(format!(
        "Relative CSS URL '{url}' depends on the source stylesheet directory, which Scrolls do not retain; use an absolute or root-relative URL before file migration"
    ))))
}

fn is_css_whitespace(ch: char) -> bool {
    matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{000c}')
}

/// Trailing escaped spaces belong to an identifier, not a whitespace token.
fn trim_css_fragment_end(fragment: &str) -> Result<&str, GrimoireCssError> {
    let mut source = ParserInput::new(fragment);
    let mut input = Parser::new(&mut source);
    let mut end = 0;
    loop {
        let token = match input.next_including_whitespace_and_comments() {
            Ok(token) => token,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(conversion_error(format!("Invalid CSS fragment: {error:?}"))),
        };
        if matches!(token, Token::WhiteSpace(_)) {
            continue;
        }
        if matches!(
            token,
            Token::Function(_)
                | Token::ParenthesisBlock
                | Token::SquareBracketBlock
                | Token::CurlyBracketBlock
        ) {
            input
                .parse_nested_block(|nested| {
                    consume_rule_tokens(nested);
                    Ok::<_, ParseError<'_, ()>>(())
                })
                .map_err(|error| conversion_error(format!("Invalid CSS fragment: {error:?}")))?;
        }
        end = input.position().byte_index();
    }
    Ok(&fragment[..end])
}

fn merge_maps(map1: &mut TransmutedMap, map2: TransmutedMap) {
    for (key, values) in map2 {
        let spells = map1.entry(key).or_default();
        for value in values {
            insert_last(spells, value);
        }
    }
}

/// An identical later declaration wins at its later cascade position.
fn insert_last(values: &mut IndexSet<String>, value: String) {
    values.shift_remove(&value);
    values.insert(value);
}

#[derive(Clone, PartialEq, Eq)]
enum CascadeEffect {
    Custom(String),
    Longhand(String),
    // Shorthands, logical properties and aliases may affect the same longhand.
    Uncertain,
}

impl CascadeEffect {
    fn from_property(property: &Property<'_>) -> Self {
        let id = property.property_id();
        let name = id.name();
        if name.starts_with("--") {
            return Self::Custom(name.to_string());
        }
        if matches!(id, PropertyId::Custom(_) | PropertyId::All)
            || id.is_shorthand()
            || (!id.prefix().is_empty()
                && id.prefix() != lightningcss::vendor_prefix::VendorPrefix::None)
            || name
                .split('-')
                .any(|part| matches!(part, "block" | "inline" | "start" | "end"))
        {
            return Self::Uncertain;
        }
        Self::Longhand(
            if name == "word-wrap" {
                "overflow-wrap"
            } else {
                name
            }
            .to_string(),
        )
    }

    fn may_overlap(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Custom(a), Self::Custom(b)) => a == b,
            (Self::Custom(_), _) | (_, Self::Custom(_)) => false,
            (Self::Longhand(a), Self::Longhand(b)) => a == b,
            _ => true,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct CascadeDeclaration {
    effect: CascadeEffect,
    css: String,
    important: bool,
}

struct CascadeRule {
    class: String,
    selector: String,
    specificity: u32,
    line: u32,
    declarations: Vec<CascadeDeclaration>,
}

// Scroll discovery can reorder rules; reject order-dependent cascade results.
fn validate_scroll_cascade(
    css: &str,
    transmutation: &Transmutation,
) -> Result<(), GrimoireCssError> {
    if transmutation.scrolls.len() < 2 {
        return Ok(());
    }
    let sheet = LightningStyleSheet::parse(css, LightningParserOptions::default())
        .map_err(|error| conversion_error(format!("Cannot inspect CSS cascade: {error}")))?;
    let names = transmutation
        .scrolls
        .iter()
        .map(|scroll| scroll.name.as_str())
        .collect();
    let mut rules = Vec::new();
    collect_cascade_rules(&sheet.rules, &names, &mut rules)?;
    for (index, a) in rules.iter().enumerate() {
        for b in &rules[index + 1..] {
            if a.class == b.class
                || a.specificity != b.specificity
                || a.declarations == b.declarations
            {
                continue;
            }
            for left in &a.declarations {
                for right in &b.declarations {
                    if left.important == right.important
                        && left.css != right.css
                        && left.effect.may_overlap(&right.effect)
                    {
                        return Err(conversion_error(format!(
                            "CSS cascade conflict: '{}' (line {}, '{}') and '{}' (line {}, '{}') may depend on source order, which ordinary Scrolls cannot guarantee. No partial migration was produced. Keep the original CSS file connected through shared.styles, or refactor these rules before converting them to Scrolls",
                            a.selector, a.line, left.css, b.selector, b.line, right.css
                        )));
                    }
                }
            }
        }
    }
    Ok(())
}

fn collect_cascade_rules(
    rules: &CssRuleList<'_>,
    names: &HashSet<&str>,
    output: &mut Vec<CascadeRule>,
) -> Result<(), GrimoireCssError> {
    for rule in &rules.0 {
        match rule {
            CssRule::Media(media) => collect_cascade_rules(&media.rules, names, output)?,
            CssRule::Style(style) => {
                let mut declarations = Vec::new();
                for (properties, important) in [
                    (&style.declarations.declarations, false),
                    (&style.declarations.important_declarations, true),
                ] {
                    for property in properties {
                        declarations.push(CascadeDeclaration {
                            effect: CascadeEffect::from_property(property),
                            css: property
                                .to_css_string(important, Default::default())
                                .map_err(|error| {
                                    conversion_error(format!(
                                        "Cannot inspect CSS declaration: {error}"
                                    ))
                                })?,
                            important,
                        });
                    }
                }
                for selector in &style.selectors.0 {
                    let text = selector
                        .to_css_string(Default::default())
                        .map_err(|error| {
                            conversion_error(format!("Cannot inspect CSS selector: {error}"))
                        })?;
                    let name = {
                        let mut source = ParserInput::new(&text);
                        let mut parser = Parser::new(&mut source);
                        if parser.expect_delim('.').is_err() {
                            continue;
                        }
                        let name = parser.expect_ident_cloned().map_err(|error| {
                            conversion_error(format!("Cannot inspect Scroll name: {error:?}"))
                        })?;
                        name.to_string()
                    };
                    if names.contains(name.as_str()) {
                        output.push(CascadeRule {
                            class: name.to_string(),
                            selector: text,
                            specificity: selector.specificity(),
                            line: style.loc.line + 1,
                            declarations: declarations.clone(),
                        });
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn push_css_escape(result: &mut String, value: char, followed_by_space: bool) {
    result.push_str(&format!("\\{:06x}", value as u32));
    if followed_by_space {
        // Spell decoding turns '_' into an escape terminator, preserving the next space.
        result.push('_');
    }
}

/// Protect CSS literals from Grimoire substitutions and implicit animation imports.
fn encode_css_fragment(
    fragment: &str,
    protect_dollars: bool,
    protect_animation_names: bool,
) -> Result<String, GrimoireCssError> {
    let mut dollars = HashSet::new();
    let mut strings = HashMap::new();
    let mut names = HashMap::new();
    let mut parser_input = ParserInput::new(fragment);
    collect_fragment_literals(
        &mut Parser::new(&mut parser_input),
        &mut dollars,
        &mut strings,
        &mut names,
        protect_animation_names,
    )
    .map_err(|error| conversion_error(format!("Invalid CSS fragment: {error:?}")))?;
    let mut result = String::with_capacity(fragment.len());
    let mut chars = fragment.char_indices().peekable();
    while let Some((offset, ch)) = chars.next() {
        if let Some((end, name, suffix)) = names.get(&offset) {
            for value in name.chars() {
                result.push_str(&format!("\\{:06x}", value as u32));
            }
            result.push_str(suffix);
            if suffix.is_empty() {
                // Even a six-digit escape would consume the following whitespace.
                result.push_str("/**/");
            }
            while chars.peek().is_some_and(|(position, _)| position < end) {
                chars.next();
            }
            continue;
        }
        if let Some((end, value)) = strings.get(&offset) {
            result.push(ch);
            let mut values = value.chars().peekable();
            while let Some(value) = values.next() {
                // Escape every word of a quoted name to prevent keyframe injection.
                if protect_animation_names {
                    result.push_str(&format!("\\{:06x}", value as u32));
                    continue;
                }
                match value {
                    ' ' => result.push('_'),
                    // Escape '(' so string data cannot trigger Grimoire functions.
                    '_' | '$' | '(' => {
                        push_css_escape(&mut result, value, values.peek() == Some(&' '))
                    }
                    value if value.is_control() || value.is_whitespace() => {
                        push_css_escape(&mut result, value, values.peek() == Some(&' '));
                    }
                    value if value == ch || value == '\\' => {
                        result.push('\\');
                        result.push(value);
                    }
                    value => result.push(value),
                }
            }
            result.push(ch);
            while chars.peek().is_some_and(|(position, _)| position < end) {
                chars.next();
            }
            continue;
        }
        match ch {
            '_' => push_css_escape(
                &mut result,
                ch,
                chars
                    .peek()
                    .is_some_and(|(_, next)| is_css_whitespace(*next)),
            ),
            '$' if protect_dollars && dollars.contains(&offset) => {
                // Preserve the delimiter while blocking '=$' arguments and '$name' variables.
                result.push_str("/**/$/**/");
            }
            '$' if protect_dollars => push_css_escape(
                &mut result,
                ch,
                chars
                    .peek()
                    .is_some_and(|(_, next)| is_css_whitespace(*next)),
            ),
            '\\' => match chars.next() {
                Some((_, '_')) => push_css_escape(
                    &mut result,
                    '_',
                    chars
                        .peek()
                        .is_some_and(|(_, next)| is_css_whitespace(*next)),
                ),
                Some((_, '$')) if protect_dollars => push_css_escape(
                    &mut result,
                    '$',
                    chars
                        .peek()
                        .is_some_and(|(_, next)| is_css_whitespace(*next)),
                ),
                Some((_, next))
                    if next.is_whitespace() && !matches!(next, '\n' | '\r' | '\u{000c}') =>
                {
                    push_css_escape(
                        &mut result,
                        next,
                        chars
                            .peek()
                            .is_some_and(|(_, next)| is_css_whitespace(*next)),
                    );
                }
                Some((_, next)) => {
                    result.push('\\');
                    result.push(if is_css_whitespace(next) { '_' } else { next });
                }
                None => result.push('\\'),
            },
            ch if is_css_whitespace(ch) => result.push('_'),
            ch if ch.is_whitespace() => push_css_escape(
                &mut result,
                ch,
                chars
                    .peek()
                    .is_some_and(|(_, next)| is_css_whitespace(*next)),
            ),
            ch => result.push(ch),
        }
    }
    Ok(result)
}

fn collect_fragment_literals<'i, 't>(
    input: &mut Parser<'i, 't>,
    offsets: &mut HashSet<usize>,
    strings: &mut HashMap<usize, (usize, String)>,
    names: &mut HashMap<usize, (usize, String, &'static str)>,
    protect_animation_names: bool,
) -> Result<(), ParseError<'i, GrimoireCssError>> {
    loop {
        let start = input.position().byte_index();
        let token = match input.next_including_whitespace_and_comments() {
            Ok(token) => token,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => return Ok(()),
            Err(error) => return Err(error.into()),
        };
        let string_value = match token {
            Token::QuotedString(value) => Some(value.to_string()),
            _ => None,
        };
        let protected_name = match token {
            // The size-function regex also matches names such as vendor-mfs(...).
            Token::Function(name)
                if name.ends_with("mfs") || name.ends_with("mrs") || name.starts_with("g-") =>
            {
                Some((name.to_string(), "("))
            }
            // Project animation names are unknown here, so protect all identifiers.
            Token::Ident(name) if protect_animation_names => Some((name.to_string(), "")),
            _ => None,
        };
        if matches!(token, Token::Delim('$')) {
            offsets.insert(start);
        }
        let nested = matches!(
            token,
            Token::Function(_)
                | Token::ParenthesisBlock
                | Token::SquareBracketBlock
                | Token::CurlyBracketBlock
        );
        if let Some((name, suffix)) = protected_name {
            names.insert(start, (input.position().byte_index(), name, suffix));
        }
        if nested {
            input.parse_nested_block(|nested| {
                collect_fragment_literals(nested, offsets, strings, names, protect_animation_names)
            })?;
        }
        if let Some(value) = string_value {
            strings.insert(start, (input.position().byte_index(), value));
        }
    }
}

fn validate_scroll_name(name: &str, generator: &CssGenerator<'_>) -> Result<(), GrimoireCssError> {
    // Match source discovery, which splits class attributes on Unicode whitespace.
    if !name.split_whitespace().eq([name]) {
        return Err(conversion_error(format!(
            "Unsupported Scroll name {name:?}: the Grimoire source scanner splits this name at whitespace; rename the source class before migration"
        )));
    }
    let invalid = || {
        conversion_error(format!(
            "Unsupported Scroll name '{name}': the engine cannot invoke it as a plain class; rename the source class before migration"
        ))
    };
    let scrolls = Some(HashMap::from([(
        name.to_string(),
        ScrollDefinition::default(),
    )]));
    let spell = Spell::new(name, &HashSet::new(), &scrolls, (0, name.len()), None)
        .map_err(|_| invalid())?
        .ok_or_else(invalid)?;
    if spell.component() != name
        || !spell.area().is_empty()
        || !spell.focus().is_empty()
        || !spell.effects().is_empty()
        || spell.with_template
    {
        return Err(invalid());
    }
    let (selector, _) = generator
        .generate_css_class_name(name, "", "", false)
        .map_err(|_| invalid())?;
    let mut input = ParserInput::new(&selector);
    let mut parser = Parser::new(&mut input);
    let selectors =
        SelectorList::parse_with_options(&mut parser, &LightningParserOptions::default())
            .map_err(|_| invalid())?;
    parser.expect_exhausted().map_err(|_| invalid())?;
    if selectors.0.len() != 1 {
        return Err(invalid());
    }
    let mut components = selectors.0[0].iter_raw_match_order();
    if !matches!(components.next(), Some(Component::Class(class)) if class.0.as_ref() == name)
        || components.next().is_some()
    {
        return Err(invalid());
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BlockKind {
    Parenthesis,
    Square,
    Curly,
}

fn validate_css_syntax(css: &str) -> Result<(), GrimoireCssError> {
    let mut blocks = Vec::new();
    let mut chars = css.chars().peekable();
    let mut quote = None;
    let mut comment = false;

    while let Some(ch) = chars.next() {
        if comment {
            if ch == '*' && chars.peek() == Some(&'/') {
                chars.next();
                comment = false;
            }
            continue;
        }
        if let Some(delimiter) = quote {
            match ch {
                '\\' => {
                    if chars.next() == Some('\r') && chars.peek() == Some(&'\n') {
                        chars.next();
                    }
                }
                value if value == delimiter => quote = None,
                '\n' | '\r' => {
                    return Err(GrimoireCssError::InvalidInput(
                        "Malformed CSS contains an unterminated string".into(),
                    ));
                }
                _ => {}
            }
            continue;
        }

        match ch {
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                comment = true;
            }
            '\'' | '"' => quote = Some(ch),
            '\\' => {
                chars.next();
            }
            '(' => blocks.push(BlockKind::Parenthesis),
            '[' => blocks.push(BlockKind::Square),
            '{' => blocks.push(BlockKind::Curly),
            ')' => close_block(&mut blocks, BlockKind::Parenthesis)?,
            ']' => close_block(&mut blocks, BlockKind::Square)?,
            '}' => close_block(&mut blocks, BlockKind::Curly)?,
            _ => {}
        }
    }

    if comment {
        return Err(GrimoireCssError::InvalidInput(
            "Malformed CSS contains an unterminated comment".into(),
        ));
    } else if quote.is_some() {
        return Err(GrimoireCssError::InvalidInput(
            "Malformed CSS contains an unterminated string".into(),
        ));
    } else if !blocks.is_empty() {
        return Err(GrimoireCssError::InvalidInput(
            "Malformed CSS contains an unclosed block".into(),
        ));
    }

    // Validate at-rule grammar that the token-level checks below cannot enforce.
    LightningStyleSheet::parse(css, LightningParserOptions::default())
        .map_err(|error| GrimoireCssError::InvalidInput(format!("Malformed CSS: {error}")))?;

    let mut input = ParserInput::new(css);
    let mut parser = Parser::new(&mut input);
    validate_stylesheet_rules(&mut parser).map_err(|error| {
        GrimoireCssError::InvalidInput(format!(
            "Malformed CSS at line {}, column {}: {:?}",
            error.location.line, error.location.column, error.kind
        ))
    })
}

struct SyntaxRuleParser;

impl<'i> QualifiedRuleParser<'i> for SyntaxRuleParser {
    type Prelude = ();
    type QualifiedRule = ();
    type Error = ();

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        validate_selector_list(input)
    }

    fn parse_block<'t>(
        &mut self,
        _prelude: Self::Prelude,
        _start: &cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        validate_declaration_list(input)
    }
}

struct SyntaxDeclarationParser;

impl<'i> DeclarationParser<'i> for SyntaxDeclarationParser {
    type Declaration = ();
    type Error = ();

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        if validate_css_tokens(input)? || (name.starts_with("--") && name.len() > 2) {
            Ok(())
        } else {
            Err(input.new_custom_error(()))
        }
    }
}

impl<'i> AtRuleParser<'i> for SyntaxDeclarationParser {
    type Prelude = ();
    type AtRule = ();
    type Error = ();
}

impl<'i> QualifiedRuleParser<'i> for SyntaxDeclarationParser {
    type Prelude = ();
    type QualifiedRule = ();
    type Error = ();
}

impl<'i> RuleBodyItemParser<'i, (), ()> for SyntaxDeclarationParser {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        false
    }
}

struct SyntaxKeyframeParser;

impl<'i> QualifiedRuleParser<'i> for SyntaxKeyframeParser {
    type Prelude = ();
    type QualifiedRule = ();
    type Error = ();

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        if validate_css_tokens(input)? {
            Ok(())
        } else {
            Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid))
        }
    }

    fn parse_block<'t>(
        &mut self,
        _prelude: Self::Prelude,
        _start: &cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        validate_declaration_list(input)
    }
}

impl<'i> AtRuleParser<'i> for SyntaxKeyframeParser {
    type Prelude = ();
    type AtRule = ();
    type Error = ();
}

fn validate_declaration_list<'i, 't>(input: &mut Parser<'i, 't>) -> Result<(), ParseError<'i, ()>> {
    let mut syntax = SyntaxDeclarationParser;
    for declaration in RuleBodyParser::new(input, &mut syntax) {
        declaration.map_err(|(error, _)| error)?;
    }
    Ok(())
}

impl<'i> AtRuleParser<'i> for SyntaxRuleParser {
    type Prelude = CowRcStr<'i>;
    type AtRule = ();
    type Error = ();

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        if name.eq_ignore_ascii_case("media") {
            validate_media_query_list(input)?;
        } else {
            validate_css_tokens(input)?;
        }
        Ok(name)
    }

    fn rule_without_block(
        &mut self,
        name: Self::Prelude,
        _start: &cssparser::ParserState,
    ) -> Result<Self::AtRule, ()> {
        if name.eq_ignore_ascii_case("media") {
            Err(())
        } else {
            Ok(())
        }
    }

    fn parse_block<'t>(
        &mut self,
        name: Self::Prelude,
        _start: &cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
        if name.eq_ignore_ascii_case("media") {
            validate_stylesheet_rules(input)
        } else if name.eq_ignore_ascii_case("keyframes")
            || name.eq_ignore_ascii_case("-webkit-keyframes")
        {
            validate_keyframe_rules(input)
        } else {
            validate_css_tokens(input).map(|_| ())
        }
    }
}

fn validate_stylesheet_rules<'i, 't>(
    parser: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, ()>> {
    let mut syntax = SyntaxRuleParser;
    for rule in StyleSheetParser::new(parser, &mut syntax) {
        rule.map_err(|(error, _)| error)?;
    }
    Ok(())
}

fn validate_keyframe_rules<'i, 't>(parser: &mut Parser<'i, 't>) -> Result<(), ParseError<'i, ()>> {
    let mut syntax = SyntaxKeyframeParser;
    for rule in StyleSheetParser::new(parser, &mut syntax) {
        rule.map_err(|(error, _)| error)?;
    }
    Ok(())
}

fn validate_selector_list<'i, 't>(input: &mut Parser<'i, 't>) -> Result<(), ParseError<'i, ()>> {
    let options = LightningParserOptions::default();
    let selectors = match SelectorList::parse_with_options(input, &options) {
        Ok(selectors) => selectors,
        Err(_) => return Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid)),
    };
    if selectors.0.iter().any(selector_contains_nesting) {
        return Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid));
    }
    input.expect_exhausted().map_err(Into::into)
}

fn selector_contains_nesting(selector: &Selector<'_>) -> bool {
    selector
        .iter_raw_match_order()
        .any(component_contains_nesting)
}

fn component_contains_nesting(component: &Component<'_>) -> bool {
    match component {
        Component::Nesting => true,
        Component::Negation(selectors)
        | Component::Where(selectors)
        | Component::Is(selectors)
        | Component::Has(selectors)
        | Component::Any(_, selectors) => selectors.iter().any(selector_contains_nesting),
        Component::Slotted(selector) => selector_contains_nesting(selector),
        Component::Host(Some(selector)) => selector_contains_nesting(selector),
        Component::NthOf(data) => data.selectors().iter().any(selector_contains_nesting),
        _ => false,
    }
}

fn validate_media_query_list<'i, 't>(input: &mut Parser<'i, 't>) -> Result<(), ParseError<'i, ()>> {
    let options = LightningParserOptions::default();
    let media = MediaList::parse(input, &options)
        .map_err(|_| input.new_error(BasicParseErrorKind::AtRuleBodyInvalid))?;
    if media.media_queries.is_empty() {
        return Err(input.new_error(BasicParseErrorKind::AtRuleBodyInvalid));
    }
    input.expect_exhausted().map_err(Into::into)
}

fn validate_css_tokens<'i, 't>(parser: &mut Parser<'i, 't>) -> Result<bool, ParseError<'i, ()>> {
    let mut meaningful = false;
    loop {
        let nested = match parser.next_including_whitespace_and_comments() {
            Ok(token) if token.is_parse_error() => return Err(parser.new_custom_error(())),
            Ok(token) => {
                meaningful |= !matches!(token, Token::WhiteSpace(_) | Token::Comment(_));
                matches!(
                    token,
                    Token::Function(_)
                        | Token::ParenthesisBlock
                        | Token::SquareBracketBlock
                        | Token::CurlyBracketBlock
                )
            }
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => {
                return Ok(meaningful);
            }
            Err(error) => return Err(error.into()),
        };
        if nested {
            parser.parse_nested_block(|input| validate_css_tokens(input).map(|_| ()))?;
        }
    }
}

fn strip_source_comments(css: &str) -> Result<String, GrimoireCssError> {
    let mut input = ParserInput::new(css);
    let mut parser = Parser::new(&mut input);
    let mut ranges = Vec::new();
    collect_comment_ranges(&mut parser, &mut ranges).map_err(|error| {
        GrimoireCssError::InvalidInput(format!(
            "Malformed CSS at line {}, column {}: {:?}",
            error.location.line, error.location.column, error.kind
        ))
    })?;

    if ranges.is_empty() {
        return Ok(css.to_string());
    }

    let removed_bytes = ranges.iter().map(|(range, _)| range.len()).sum::<usize>();
    let mut cleaned = String::with_capacity(css.len().saturating_sub(removed_bytes));
    let mut cursor = 0;
    for (range, separator) in ranges {
        cleaned.push_str(&css[cursor..range.start]);
        if separator {
            cleaned.push_str("/**/");
        }
        cursor = range.end;
    }
    cleaned.push_str(&css[cursor..]);
    Ok(cleaned)
}

fn collect_comment_ranges<'i, 't>(
    parser: &mut Parser<'i, 't>,
    ranges: &mut Vec<(Range<usize>, bool)>,
) -> Result<(), ParseError<'i, ()>> {
    let mut previous = cssparser::TokenSerializationType::nothing();
    let mut pending_comment = None;
    loop {
        let start = parser.position().byte_index();
        let token = match parser.next_including_whitespace_and_comments() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => return Ok(()),
            Err(error) => return Err(error.into()),
        };
        if matches!(token, Token::Comment(_)) {
            pending_comment = Some(ranges.len());
            ranges.push((start..parser.position().byte_index(), false));
            continue;
        }
        if let Some(index) = pending_comment.take() {
            // Whitespace would change compound selectors into descendant selectors.
            ranges[index].1 = previous.needs_separator_when_before(token.serialization_type());
        }
        let nested = matches!(
            token,
            Token::Function(_)
                | Token::ParenthesisBlock
                | Token::SquareBracketBlock
                | Token::CurlyBracketBlock
        );
        previous = token.serialization_type();
        if nested {
            parser.parse_nested_block(|input| collect_comment_ranges(input, ranges))?;
            previous = Token::CloseParenthesis.serialization_type();
        }
    }
}

fn close_block(blocks: &mut Vec<BlockKind>, expected: BlockKind) -> Result<(), GrimoireCssError> {
    if blocks.pop() == Some(expected) {
        Ok(())
    } else {
        Err(GrimoireCssError::InvalidInput(
            "Malformed CSS contains an unmatched closing delimiter".into(),
        ))
    }
}

struct TransmuteRuleParser {
    area: Option<String>,
}

fn conversion_error(message: impl Into<String>) -> GrimoireCssError {
    GrimoireCssError::InvalidInput(message.into())
}

fn consume_rule_tokens(input: &mut Parser<'_, '_>) {
    while input.next_including_whitespace_and_comments().is_ok() {}
}

impl<'i> QualifiedRuleParser<'i> for TransmuteRuleParser {
    type Prelude = Vec<(String, String)>;
    type QualifiedRule = TransmutedMap;
    type Error = GrimoireCssError;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        input.parse_comma_separated(|selector| {
            selector.expect_delim('.').map_err(|_| {
                selector.new_custom_error(conversion_error(
                    "Unsupported selector: migration requires a leading class selector",
                ))
            })?;
            let name = selector.expect_ident_cloned()?.to_string();
            let start = selector.position();
            consume_rule_tokens(selector);
            let suffix = trim_css_fragment_end(selector.slice_from(start))
                .map_err(|error| selector.new_custom_error(error))?
                .to_string();
            Ok((name, suffix))
        })
    }

    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        _start: &cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let mut declarations = IndexSet::new();
        for item in RuleBodyParser::new(input, &mut TransmuteDeclarationParser) {
            insert_last(&mut declarations, item.map_err(|(error, _)| error)?);
        }
        let mut result = TransmutedMap::new();
        for (name, suffix) in selectors {
            if Spell::new(&name, &HashSet::new(), &None, (0, name.len()), None)
                .map_err(|error| input.new_custom_error(error))?
                .is_some()
            {
                continue;
            }
            let mut prefix = String::new();
            if let Some(area) = &self.area {
                // Prevent literal media types from matching named Grimoire breakpoints.
                if matches!(area.as_str(), "sm" | "md" | "lg" | "xl" | "2xl") {
                    prefix.push_str("/**/");
                }
                prefix.push_str(
                    &encode_css_fragment(area, false, false)
                        .map_err(|error| input.new_custom_error(error))?,
                );
                prefix.push_str("__");
            }
            if !suffix.is_empty() {
                prefix.push('{');
                prefix.push_str(
                    &encode_css_fragment(&suffix, false, false)
                        .map_err(|error| input.new_custom_error(error))?,
                );
                prefix.push('}');
            }
            let spells = result.entry(name).or_default();
            for declaration in &declarations {
                insert_last(spells, format!("{prefix}{declaration}"));
            }
        }
        Ok(result)
    }
}

struct TransmuteDeclarationParser;

impl<'i> DeclarationParser<'i> for TransmuteDeclarationParser {
    type Declaration = String;
    type Error = GrimoireCssError;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let start = input.position();
        consume_rule_tokens(input);
        let value = trim_css_fragment_end(input.slice_from(start))
            .map_err(|error| input.new_custom_error(error))?
            .trim_start_matches(is_css_whitespace);
        let name = if name.starts_with("--") {
            name.to_string()
        } else {
            name.to_ascii_lowercase()
        };
        if name == "g-anim"
            || crate::core::component::get_css_property(&name)
                .is_some_and(|property| property != name)
        {
            return Err(input.new_custom_error(conversion_error(format!(
                "CSS property '{name}' would invoke a Grimoire component with different semantics; use a full CSS property name before migration"
            ))));
        }
        if name.starts_with("--") {
            let token = format!("{name}=initial");
            let preserves_name = Spell::new(&token, &HashSet::new(), &None, (0, token.len()), None)
                .ok()
                .flatten()
                .is_some_and(|spell| {
                    spell.component() == name
                        && spell.area().is_empty()
                        && spell.focus().is_empty()
                        && spell.effects().is_empty()
                        && !spell.with_template
                });
            if !preserves_name {
                return Err(input.new_custom_error(conversion_error(format!(
                    "Unsupported custom property name {name:?}: the Grimoire Spell parser cannot preserve this name; keep the original CSS in shared.styles"
                ))));
            }
        }
        let protect_animation_names = matches!(name.as_str(), "animation" | "animation-name");
        Ok(format!(
            "{name}={}",
            encode_css_fragment(value, true, protect_animation_names)
                .map_err(|error| input.new_custom_error(error))?
        ))
    }
}

impl<'i> AtRuleParser<'i> for TransmuteDeclarationParser {
    type Prelude = ();
    type AtRule = String;
    type Error = GrimoireCssError;
}
impl<'i> QualifiedRuleParser<'i> for TransmuteDeclarationParser {
    type Prelude = ();
    type QualifiedRule = String;
    type Error = GrimoireCssError;
}
impl<'i> RuleBodyItemParser<'i, String, GrimoireCssError> for TransmuteDeclarationParser {
    fn parse_declarations(&self) -> bool {
        true
    }
    fn parse_qualified(&self) -> bool {
        false
    }
}

impl<'i> AtRuleParser<'i> for TransmuteRuleParser {
    type Prelude = String;
    type AtRule = TransmutedMap;
    type Error = GrimoireCssError;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        if !name.eq_ignore_ascii_case("media") {
            return Err(input.new_custom_error(conversion_error(format!(
                "Unsupported CSS at-rule '@{name}'; no partial migration was produced"
            ))));
        }
        let start = input.position();
        consume_rule_tokens(input);
        let area = trim_css_fragment_end(input.slice_from(start))
            .map_err(|error| input.new_custom_error(error))?
            .trim_start_matches(is_css_whitespace);
        match &self.area {
            None => Ok(area.to_string()),
            Some(parent) => {
                intersect_media_queries(parent, area).map_err(|error| input.new_custom_error(error))
            }
        }
    }

    fn parse_block<'t>(
        &mut self,
        area: Self::Prelude,
        _start: &cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
        collect_transmuted_rules(input, Some(area))
    }
}

fn intersect_media_queries(parent: &str, child: &str) -> Result<String, GrimoireCssError> {
    let options = LightningParserOptions::default();
    let mut parent_input = ParserInput::new(parent);
    let mut child_input = ParserInput::new(child);
    let parent = MediaList::parse(&mut Parser::new(&mut parent_input), &options)
        .map_err(|error| conversion_error(format!("Invalid parent media query: {error:?}")))?;
    let child = MediaList::parse(&mut Parser::new(&mut child_input), &options)
        .map_err(|error| conversion_error(format!("Invalid nested media query: {error:?}")))?;
    // Media lists are disjunctions: intersect every pair of queries.
    let mut queries = Vec::new();
    for outer in &parent.media_queries {
        for inner in &child.media_queries {
            let mut combined = outer.clone();
            combined.and(inner).map_err(|_| conversion_error(
                "Unsupported intersection of nested media queries; no partial migration was produced"
            ))?;
            queries.push(combined);
        }
    }
    MediaList {
        media_queries: queries,
    }
    .to_css_string(Default::default())
    .map_err(|error| conversion_error(format!("Cannot serialize media query: {error}")))
}

fn collect_transmuted_rules<'i, 't>(
    input: &mut Parser<'i, 't>,
    area: Option<String>,
) -> Result<TransmutedMap, ParseError<'i, GrimoireCssError>> {
    let mut result = TransmutedMap::new();
    for rule in StyleSheetParser::new(input, &mut TransmuteRuleParser { area }) {
        merge_maps(&mut result, rule.map_err(|(error, _)| error)?);
    }
    Ok(result)
}

fn process_css_into_raw_spells(css_input: &str) -> Result<TransmutedMap, GrimoireCssError> {
    let mut input = ParserInput::new(css_input);
    collect_transmuted_rules(&mut Parser::new(&mut input), None).map_err(|error| match error.kind {
        cssparser::ParseErrorKind::Custom(error) => error,
        _ => conversion_error(format!("Cannot migrate CSS: {error:?}")),
    })
}

/// Transmutes CSS files matched below an explicit project root.
pub fn transmute_paths(
    root: &Path,
    patterns: &[String],
    options: TransmuteOptions,
) -> Result<Transmutation, GrimoireCssError> {
    if patterns.is_empty() {
        return Err(GrimoireCssError::InvalidInput(
            "No CSS file patterns provided.".into(),
        ));
    }

    let expanded_paths = expand_file_paths(root, patterns)?;
    if expanded_paths.is_empty() {
        return Err(GrimoireCssError::InvalidPath(
            "No files found matching the provided patterns.".into(),
        ));
    }

    let all_css_string = read_and_clean_files(&expanded_paths)?;
    transmute_css(&all_css_string, options)
}

/// Transmutes inline CSS without filesystem access.
pub fn transmute_css(
    css_content: &str,
    options: TransmuteOptions,
) -> Result<Transmutation, GrimoireCssError> {
    validate_css_syntax(css_content)?;
    let css_without_comments = strip_source_comments(css_content)?;
    let processed_css = process_css_into_raw_spells(&css_without_comments)?;

    if processed_css.is_empty() {
        return Err(GrimoireCssError::InvalidInput(
            "There is nothing to transmute.".into(),
        ));
    }

    let mut transmuted = Transmutation {
        scrolls: Vec::with_capacity(processed_css.len()),
    };
    let animations = HashMap::new();
    let generator = CssGenerator::new(&None, &animations)?;

    for (name, spells) in processed_css {
        if !name.is_empty() && !spells.is_empty() {
            validate_scroll_name(&name, &generator)?;
            let spells = spells.into_iter().collect::<Vec<_>>();
            let oneliner = if options.with_oneliner {
                Some(spells.join(" "))
            } else {
                None
            };

            transmuted.scrolls.push(TransmutedScroll {
                name,
                spells,
                oneliner,
            });
        }
    }

    if transmuted.scrolls.is_empty() {
        return Err(GrimoireCssError::InvalidInput(
            "There is nothing to transmute.".into(),
        ));
    }

    transmuted.validate_component_scroll_conflicts(&HashSet::new())?;
    validate_scroll_cascade(css_content, &transmuted)?;
    Ok(transmuted)
}

fn expand_file_paths(cwd: &Path, patterns: &[String]) -> Result<Vec<PathBuf>, GrimoireCssError> {
    let mut paths = Vec::with_capacity(patterns.len() * 4);
    let canonical_root = fs::canonicalize(cwd).map_err(GrimoireCssError::Io)?;

    for pattern in patterns {
        let input = Path::new(pattern);
        if pattern.is_empty()
            || input.is_absolute()
            || input.components().any(|component| {
                matches!(
                    component,
                    std::path::Component::ParentDir
                        | std::path::Component::RootDir
                        | std::path::Component::Prefix(_)
                )
            })
        {
            return Err(GrimoireCssError::InvalidPath(format!(
                "CSS paths must stay below the explicit root: {pattern}"
            )));
        }
        let root_text = canonical_root.to_string_lossy();
        let prefix_len = match canonical_root.components().next() {
            Some(std::path::Component::Prefix(prefix)) => {
                prefix.as_os_str().to_string_lossy().len()
            }
            _ => 0,
        };
        // Windows device prefixes are path syntax, not glob metacharacters.
        let mut absolute_pattern = root_text[..prefix_len].to_string();
        absolute_pattern.push_str(&glob::Pattern::escape(&root_text[prefix_len..]));
        if !absolute_pattern.ends_with(std::path::MAIN_SEPARATOR) {
            absolute_pattern.push(std::path::MAIN_SEPARATOR);
        }
        absolute_pattern.push_str(pattern);

        for entry_result in glob(&absolute_pattern)
            .map_err(|e| GrimoireCssError::GlobPatternError(e.msg.to_string()))?
        {
            match entry_result {
                Ok(path) if path.is_file() => {
                    let canonical = fs::canonicalize(&path).map_err(GrimoireCssError::Io)?;
                    if !canonical.starts_with(&canonical_root) {
                        return Err(GrimoireCssError::InvalidPath(format!(
                            "CSS path escapes the explicit root: {}",
                            path.display()
                        )));
                    }
                    paths.push(canonical);
                }
                Ok(_) => {}
                Err(e) => return Err(GrimoireCssError::InvalidPath(e.to_string())),
            }
        }
    }

    paths.sort();
    paths.dedup();

    if paths.len() < paths.capacity() / 2 {
        paths.shrink_to_fit();
    }

    Ok(paths)
}
