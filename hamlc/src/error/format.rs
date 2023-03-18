use super::{GenericError, SemanticError, SyntaxError};
use crate::symbol;
use crate::syntax::Token;

pub fn format_err(
    message: &str,
    file_path: &str,
    line_number: usize,
    line: &str,
    char_col: usize,
    span_len: usize,
    recommendation: &str,
) -> String {
    let line_number = line_number + 1;
    let line_num_len = format!("{}", line_number).len();
    let min_indent = 3;
    let loc_indent = " ".repeat(1);
    let rest_indent = " ".repeat((line_num_len + 2).clamp(min_indent, usize::MAX));
    let char_col_indent = " ".repeat(char_col);
    let common_indent = " ".repeat(2);

    let error = bold(&red("error"));
    let msg = bold(&format!(": {message}"));
    let err_line = format!("{common_indent}{}{}", error, msg);

    let file_line_prefix = dim("  ");
    let file_line =
        format!("{common_indent}{file_line_prefix}{file_path}:{line_number}:{char_col}");

    let blank_loc_prefix = dim(&format!("{common_indent}{rest_indent}|"));
    let code_line = format!(
        "{common_indent} {line_number}{loc_indent}{} {line}",
        dim("|")
    );
    let pointer_line = format!(
        "{common_indent}{rest_indent}{}{char_col_indent}{} {}",
        dim("|"),
        bold(&purple(&"-".repeat(span_len))),
        bold(&purple(recommendation)),
    );

    vec![
        err_line,
        file_line,
        blank_loc_prefix,
        code_line,
        pointer_line,
    ]
    .join("\n")
    .to_owned()
}

pub fn bold(value: &str) -> String {
    format!("\x1B[1m{value}\x1B[0m")
}

pub fn red(value: &str) -> String {
    format!("\x1B[31m{value}\x1B[0m")
}

pub fn dim(value: &str) -> String {
    format!("\x1B[2m{value}\x1B[0m")
}

pub fn purple(value: &str) -> String {
    format!("\x1B[34m{value}\x1B[0m")
}

pub fn green(value: &str) -> String {
    format!("\x1B[32m{value}\x1B[0m")
}

pub fn format(path: &str, input: &str, err: GenericError) -> String {
    match err {
        GenericError::Syntax(err) => format_syntax_err(input, err),
        GenericError::Semantic(err) => format_semantic_err(path, input, err),
    }
}

fn format_syntax_err(input: &str, err: SyntaxError) -> String {
    match &err {
        SyntaxError::UnexpectedToken(token, msg) => {
            let loc = get_token_line_number(input, token);
            let span = symbol::span(input, token);
            format!("Syntax error on line {loc}: expected {msg} but found \"{span}\"")
        }
        SyntaxError::UnexpectedEof => "Unexpected EOF".to_string(),
        SyntaxError::UnknownToken(ch) => format!("Unknown token: {ch}"),
        SyntaxError::UnterminatedComment => "Unterminated comment".to_string(),
        SyntaxError::UnterminatedString => "Unterminated string".to_string(),
    }
}

fn format_semantic_err(path: &str, input: &str, err: SemanticError) -> String {
    match &err {
        SemanticError::DuplicateIdentifier { duplicate } => {
            let line_num = get_token_line_number(input, duplicate);
            let char_col = get_token_char_col(input, duplicate);
            let line = get_line_span(input, line_num as usize);
            let ident = symbol::span(input, duplicate);
            let msg = format!("Found duplicate identifier \"{ident}\"");
            format_err(
                &msg,
                path,
                line_num,
                line,
                char_col,
                duplicate.len,
                "there can only be one type with this name",
            )
        }
    }
}

fn get_token_line_number(input: &str, token: &Token) -> usize {
    let mut pos = 0;
    for (line_num, line) in input.lines().enumerate() {
        pos = pos + line.len() + 1;
        if pos >= token.start {
            return line_num;
        }
    }
    panic!("could not find line number");
}

// pos 5 on line 0

fn get_token_char_col(input: &str, token: &Token) -> usize {
    let mut pos = 0;
    for (_, line) in input.lines().enumerate() {
        if token.start < pos + line.len() {
            return token.start - pos + 1;
        }
        pos = pos + line.len() + 1;
    }
    panic!("could not find char index");
}

fn get_line_span(input: &str, line: usize) -> &str {
    input.lines().nth(line).expect("Could not find line")
}
