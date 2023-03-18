use crate::{
    ast::{
        self,
        node::{ConstructorDecl, ImportStmt, Node},
        Ast, Visitor,
    },
    error::SemanticError,
};

use super::{
    interner::Interner,
    symbols::{Symbol, SymbolTable},
};

pub fn resolve_module(input: &str, import: &ImportStmt, module_root: &std::path::Path) {
    let path_span = crate::symbol::span(input, &import.path);

    let mut import_file_path = module_root.clone().to_path_buf();
    let parts = path_span.split(".").collect::<Vec<_>>();
    let len = parts.len();
    for (i, part) in parts.into_iter().enumerate() {
        let part = &part[1..part.len() - 1];
        if i == len - 1 {
            import_file_path.push(format!("{}.haml", part))
        } else {
            import_file_path.push(part);
        }
    }

    println!("{:?}", import_file_path.as_path());
}

pub fn check_can_resolve_symbols(
    module_root: &std::path::Path,
    input: &str,
    ast: &Ast,
) -> (bool, Vec<SemanticError>) {
    let mut interner = Interner::new(input);
    let mut symbols = SymbolTable::new(&mut interner);

    let mut pass = SymbolTablePass {
        symbols: &mut symbols,
        errors: Vec::new(),
    };
    ast::walk(ast, &mut pass);

    for node in ast.nodes.iter() {
        if let Node::ImportStmt(import) = node {
            resolve_module(input, import, module_root);
        }
    }

    (pass.errors.is_empty(), pass.errors)
}

pub struct SymbolTablePass<'i> {
    symbols: &'i mut SymbolTable<'i>,
    errors: Vec<SemanticError>,
}

impl Visitor for SymbolTablePass<'_> {
    fn constructor_decl(&mut self, decl: &ConstructorDecl) {
        if self.symbols.contains(&decl.name) {
            let duplicate = decl.name;
            let err = SemanticError::DuplicateIdentifier { duplicate };
            self.errors.push(err);
        } else {
            let symbol = Symbol::ConstructorDecl(decl.name);
            self.symbols.insert(&decl.name, symbol);
        }
    }
}
