use clap::command;
use clap::Parser;

use crate::error::SyntaxError;
use crate::syntax;
use crate::syntax::Token;

#[derive(Parser)]
#[command(bin_name = "haml", author = "Harrison Turton", version)]
struct Args {
    path: String,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

pub fn main() {
    let args = Args::parse();

    let file = match std::fs::read(&args.path) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to read file \"{}\": {}", args.path, err);
            return;
        }
    };

    let input = match std::str::from_utf8(&file) {
        Ok(input) => input,
        Err(err) => {
            println!(
                "Failed to read file \"{}\" because it is not a valid UTF-8 sequence: {}",
                args.path, err
            );
            return;
        }
    };

    let mut parser = syntax::Parser::new(input);

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            printerr(input, err);
            return;
        }
    };

    println!("{ast:?}");

    println!("Parsed successfully!");
}

fn printerr(input: &str, err: SyntaxError) {
    match &err {
        SyntaxError::UnexpectedToken(token, msg) => {
            println!(
                "Syntax error on line {}: expected {msg} but found \"{}\"",
                get_token_loc(input, token),
                get_span(input, token)
            )
        }
        SyntaxError::UnknownToken(ch) => {
            println!("Unknown token: {ch}");
        }
        SyntaxError::UnterminatedComment => {
            println!("Unterminated comment");
        }
        SyntaxError::UnterminatedString => {
            println!("Unterminated string");
        }
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
