#![warn(clippy::all)]

mod ast;
mod cli;
mod error;
mod semantic;
mod symbol;
mod syntax;

fn main() {
    cli::main()
}
