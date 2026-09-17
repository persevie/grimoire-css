use cssparser::{Parser, ParserInput, Token};
use std::ops::Range;

/// Skips CSS comments while preserving original byte offsets.
pub(super) fn chars_without_comments(raw: &str) -> impl Iterator<Item = (usize, char)> + '_ {
    let mut comment_ranges = Vec::new();
    if raw.contains("/*") {
        let mut source = ParserInput::new(raw);
        collect_comment_ranges(&mut Parser::new(&mut source), &mut comment_ranges);
    }
    let mut comments = comment_ranges.into_iter().peekable();
    raw.char_indices().filter(move |(offset, _)| {
        while comments.peek().is_some_and(|range| range.end <= *offset) {
            comments.next();
        }
        !comments.peek().is_some_and(|range| range.contains(offset))
    })
}

// Tokenization keeps comment-like text in strings and URLs intact.
fn collect_comment_ranges(input: &mut Parser<'_, '_>, ranges: &mut Vec<Range<usize>>) {
    loop {
        let start = input.position().byte_index();
        let Ok(token) = input.next_including_whitespace_and_comments() else {
            return;
        };
        match token {
            Token::Comment(_) => ranges.push(start..input.position().byte_index()),
            Token::Function(_)
            | Token::ParenthesisBlock
            | Token::SquareBracketBlock
            | Token::CurlyBracketBlock => {
                let _ = input.parse_nested_block(|nested| {
                    collect_comment_ranges(nested, ranges);
                    Ok::<_, cssparser::ParseError<'_, ()>>(())
                });
            }
            _ => {}
        }
    }
}
