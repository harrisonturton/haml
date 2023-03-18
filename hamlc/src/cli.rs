use clap::command;
use clap::Parser;

use crate::error;
use crate::syntax;

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
            println!("{}", error::format(input, err));
            return;
        }
    };

    println!("{ast:?}");

    println!("Parsed successfully!");
}
