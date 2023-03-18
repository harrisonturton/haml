use clap::command;
use clap::Parser;
use std::path::PathBuf;

use core::error;
use core::error::format::{bold, green, red};
use core::semantic::check_can_resolve_symbols;
use core::syntax;

#[derive(Parser)]
#[command(bin_name = "haml", author = "Harrison Turton", version)]
struct Args {
    path: String,

    #[arg(short, long)]
    module_root: Option<String>,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

pub fn main() {
    let args = Args::parse();

    println!("{} {}", bold(&green("Checking")), args.path);

    let module_root = {
        if let Some(module_root) = args.module_root {
            if module_root == "." {
                std::env::current_dir().unwrap()
            } else {
                PathBuf::from(&module_root)
            }
        } else {
            PathBuf::from(&args.path).parent().unwrap().to_path_buf()
        }
    };

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

    let mut lexer = syntax::Lexer::new(input);
    let mut parser = syntax::Parser::new(&mut lexer);

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            let err = error::GenericError::Syntax(err);
            println!("{}", error::format(&args.path, input, err));
            return;
        }
    };

    let (_, errs) = check_can_resolve_symbols(&module_root, input, &ast);
    if errs.len() > 0 {
        println!("");
    }

    for err in errs.iter() {
        let err = error::GenericError::Semantic(err.clone());
        let msg = error::format(&args.path, input, err);
        println!("{msg}\n");
    }

    if errs.is_empty() {
        println!("{} successfully!", bold(&green("Finished")));
    } else {
        println!("{} with {} errors", bold(&red("Failed")), errs.len());
    }
}
