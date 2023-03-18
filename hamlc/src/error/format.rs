use super::SyntaxError;
use crate::syntax::Token;

pub fn format(input: &str, err: SyntaxError) -> String {
    match &err {
        SyntaxError::UnexpectedToken(token, msg) => {
            let loc = get_token_loc(input, token);
            let span = get_span(input, token);
            format!("Syntax error on line {loc}: expected {msg} but found \"{span}\"")
        }
        SyntaxError::UnknownToken(ch) => format!("Unknown token: {ch}"),
        SyntaxError::UnterminatedComment => "Unterminated comment".to_string(),
        SyntaxError::UnterminatedString => "Unterminated string".to_string(),
    }
}

fn get_token_loc(input: &str, token: &Token) -> u32 {
    let mut pos = 0;
    for (i, line) in input.lines().enumerate() {
        pos = pos + line.len() + 1;
        if pos >= token.start {
            return i as u32;
        }
    }
    panic!("could not find line number");
}

fn get_span<'a>(input: &'a str, token: &Token) -> &'a str {
    &input[token.start..token.start + token.len]
}
