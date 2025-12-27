#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellValueValidationError {
    UnexpectedClosingParen,
    UnclosedParen,
}

pub fn validate_component_target(component_target: &str) -> Option<SpellValueValidationError> {
    let mut depth: i32 = 0;
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escape_next = false;

    for ch in component_target.chars() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match ch {
            '\\' if in_single_quote || in_double_quote => {
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

    if depth != 0 {
        return Some(SpellValueValidationError::UnclosedParen);
    }

    None
}
