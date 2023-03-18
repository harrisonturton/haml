#![warn(clippy::all)]

mod ast;
mod cli;
mod error;
mod syntax;

fn main() {
    cli::main()
}
