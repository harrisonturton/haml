use crate::{
    queries::{read_span, Path, SourceFile, TrackedSpan},
    span::Span,
    syntax::Token,
    Db,
};
use derive_new::new;

#[salsa::accumulator]
pub struct Diagnostics(Diagnostic);

#[derive(Clone, Debug)]
pub enum Level {
    Error,
    Warning,
    Info,
}

#[derive(new, Clone, Debug)]
pub struct Diagnostic {
    pub level: Level,
    pub message: String,
    pub span: Option<Span>,
    pub span_message: Option<String>,
}

impl Diagnostic {
    pub fn to_user_message(&self, db: &dyn Db, file: &SourceFile) -> String {
        let Diagnostic {
            message,
            span,
            span_message,
            ..
        } = self;

        let common_indent = " ".repeat(2);
        let input = file.text(db);
        let path = file.path(db).to_string_lossy();

        if span.is_none() || span_message.is_none() {
            let error = bold(&red("error"));
            let msg = bold(&format!(": {message}"));
            let err_line = format!("{common_indent}{}{}", error, msg);
            let file_line_prefix = dim("  ");
            let file_line = format!("{common_indent}{file_line_prefix}{path}");
            return vec![err_line, file_line].join("\n").to_owned();
        }

        let span = span.unwrap();
        let span_message = &span_message.as_ref().unwrap();

        let path = file.path(db);
        let path = Path::new(db, path.clone());

        let line_number = get_token_line_number(input, span);
        let char_col = get_token_char_col(input, span);
        let line = get_line_span(input, line_number as usize);

        let ident = read_span(db, path, TrackedSpan::new(db, span)).unwrap();

        let line_number = line_number + 1;
        let line_num_len = format!("{}", line_number).len();
        let min_indent = 3;
        let loc_indent = " ".repeat(1);
        let rest_indent = " ".repeat((line_num_len + 2).clamp(min_indent, usize::MAX));
        let char_col_indent = " ".repeat(char_col);

        let error = bold(&red("error"));
        let msg = bold(&format!(": {message}"));
        let err_line = format!("{common_indent}{}{}", error, msg);

        let path = path.path(db);
        let path = &path.to_string_lossy();

        let file_line_prefix = dim("  ");
        let file_line = format!("{common_indent}{file_line_prefix}{path}:{line_number}:{char_col}");

        let blank_loc_prefix = dim(&format!("{common_indent}{rest_indent}|"));
        let code_line = format!(
            "{common_indent} {line_number}{loc_indent}{} {line}",
            dim("|")
        );
        let pointer_line = format!(
            "{common_indent}{rest_indent}{}{char_col_indent}{} {}",
            dim("|"),
            bold(&purple(&"-".repeat(ident.len()))),
            bold(&purple(span_message)),
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
}

pub fn emit_message(db: &dyn Db, message: &str) {
    let diagnostic = Diagnostic::new(Level::Error, message.to_string(), None, None);
    Diagnostics::push(db, diagnostic);
}

pub fn emit_unexpected_eof(db: &dyn Db, token: Token) {
    let diagnostic = Diagnostic::new(
        Level::Error,
        format!("file ended unexpectedly when reading `{:?}`", token.kind),
        Some(token.span),
        Some("expected more code, but the file ended".to_string()),
    );
    Diagnostics::push(db, diagnostic);
}

pub fn emit_duplicate_identifier(db: &dyn Db, token: Token) {
    let diagnostic = Diagnostic::new(
        Level::Error,
        format!("type `{:?}` defined multiple times", token.kind),
        Some(token.span),
        Some("there can only be one type with this name".to_string()),
    );
    Diagnostics::push(db, diagnostic);
}

pub fn emit_unexpected_token(db: &dyn Db, token: Token, expected: &str) {
    let diagnostic = Diagnostic::new(
        Level::Error,
        format!("expected {expected} but found a {}", token.kind),
        Some(token.span),
        Some(format!("expected {expected}")),
    );
    Diagnostics::push(db, diagnostic);
}

pub fn emit_unterminated_comment(db: &dyn Db, token: Token) {
    let diagnostic = Diagnostic::new(
        Level::Error,
        "found unterminated comment".to_string(),
        Some(token.span),
        Some(format!("this comment must be ended with a `*/` characters")),
    );
    Diagnostics::push(db, diagnostic);
}

pub fn emit_unterminated_string(db: &dyn Db, token: Token) {
    let diagnostic = Diagnostic::new(
        Level::Error,
        "found unterminated string".to_string(),
        Some(token.span),
        Some("add a `\"` character".to_string()),
    );
    Diagnostics::push(db, diagnostic);
}

fn bold(value: &str) -> String {
    format!("\x1B[1m{value}\x1B[0m")
}

fn red(value: &str) -> String {
    format!("\x1B[31m{value}\x1B[0m")
}

fn dim(value: &str) -> String {
    format!("\x1B[2m{value}\x1B[0m")
}

fn purple(value: &str) -> String {
    format!("\x1B[34m{value}\x1B[0m")
}

fn get_token_line_number(input: &str, token: Span) -> usize {
    let mut pos = 0;
    for (line_num, line) in input.lines().enumerate() {
        pos = pos + line.len() + 1;
        if pos >= token.start {
            return line_num;
        }
    }
    panic!("could not find line number");
}

fn get_token_char_col(input: &str, token: Span) -> usize {
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
