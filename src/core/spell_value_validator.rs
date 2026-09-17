use super::css_comments::chars_without_comments;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellValueValidationError {
    UnexpectedClosingParen,
    UnclosedParen,
    UnclosedString,
}

pub fn validate_component_target(component_target: &str) -> Option<SpellValueValidationError> {
    let mut depth: i32 = 0;
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escape_next = false;

    for (_, ch) in chars_without_comments(component_target) {
        if escape_next {
            escape_next = false;
            continue;
        }

        match ch {
            '\\' => {
                escape_next = true;
            }
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }
            '(' if !in_single_quote && !in_double_quote => depth += 1,
            ')' if !in_single_quote && !in_double_quote => {
                depth -= 1;
                if depth < 0 {
                    return Some(SpellValueValidationError::UnexpectedClosingParen);
                }
            }
            _ => {}
        }
    }

    if in_single_quote || in_double_quote {
        return Some(SpellValueValidationError::UnclosedString);
    }

    if depth != 0 {
        return Some(SpellValueValidationError::UnclosedParen);
    }

    None
}
