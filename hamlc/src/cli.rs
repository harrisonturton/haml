use crate::ast::{
    AliasDecl, AnnotationDecl, ConstructorDecl, FieldDecl, FieldSetDecl, FieldType, ImportStmt,
    PackageStmt, StructDecl,
};
use crate::ast::{Ast, Comment};
use clap::command;
use clap::Parser;

use crate::ast::Visitor;
use crate::parser::{self, ParseError};
use crate::token::Token;

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

    let mut parser = parser::Parser::new(input);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            printerr(input, err);
            return;
        }
    };

    let mut walker = SimpleWalker::new(&ast);
    ast.walk(&mut walker);

    println!("Parsed successfully!");
}

fn printerr(input: &str, err: ParseError) {
    match &err {
        ParseError::UnexpectedToken(token, msg) => {
            println!(
                "Syntax error on line {}: expected {} but found \"{}\"",
                get_token_loc(input, &token),
                msg,
                get_span(input, &token)
            )
        }
        ParseError::TokenError(err) => {
            println!("Token error: {:?}", err)
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

struct SimpleWalker<'a, 'b> {
    ast: &'a Ast<'b>,
}

impl<'a, 'b> SimpleWalker<'a, 'b> {
    pub fn new(ast: &'a Ast<'b>) -> SimpleWalker<'a, 'b> {
        SimpleWalker { ast }
    }
}

impl Visitor for SimpleWalker<'_, '_> {
    fn package(&mut self, stmt: &PackageStmt) {
        println!("Walking package: {:?}", stmt);
    }

    fn import(&mut self, stmt: &ImportStmt) {
        println!("Walking import: {:?}", stmt);
    }

    fn constructor_decl(&mut self, stmt: &ConstructorDecl) {
        println!("Walking constructor: {:?}", stmt);
    }

    fn struct_decl(&mut self, stmt: &StructDecl) {
        println!("Walking struct: {:?}", stmt);
    }

    fn union_decl(&mut self, decl: &FieldSetDecl) {
        println!("Walking union fieldset: {:?}", decl);
    }

    fn repetable_decl(&mut self, decl: &FieldSetDecl) {
        println!("Walking repeatable fieldset: {:?}", decl);
    }

    fn alias_decl(&mut self, decl: &AliasDecl) {
        println!("Walking alias block: {:?}", decl);
    }

    fn field_decl(&mut self, decl: &FieldDecl) {
        println!("Walking field: {:?}", decl);
    }

    fn field_type_decl(&mut self, decl: &FieldType) {
        println!("Walking field type: {:?}", decl);
    }

    fn annotation_decl(&mut self, stmt: &AnnotationDecl) {
        println!("Walking annotation decl: {:?}", stmt);
    }

    fn annotation_def(&mut self, stmt: &Token) {
        println!("Walking annotation def: {:?}", stmt);
    }

    fn comment(&mut self, stmt: &Comment) {
        let comment = self.ast.span(&stmt.value);
        println!("Walking comment: {}", comment);
    }
}
