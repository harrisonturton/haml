use super::interner::{InternIndex, Interner};
use crate::syntax::Token;

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Symbol {
    ConstructorDecl(Token),
    StructDecl(Token),
    AnnotationDecl(Token),
}

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolTable<'i> {
    interner: &'i mut Interner<'i>,
    symbols: HashMap<InternIndex, Symbol>,
}

impl<'i> SymbolTable<'i> {
    pub fn new(interner: &'i mut Interner<'i>) -> SymbolTable<'i> {
        let symbols = HashMap::new();
        SymbolTable { symbols, interner }
    }

    pub fn insert(&mut self, name: &Token, symbol: Symbol) {
        let idx = self.interner.intern(name);
        self.symbols.insert(idx, symbol);
    }

    pub fn contains(&mut self, name: &Token) -> bool {
        let idx = self.interner.intern(name);
        self.symbols.contains_key(&idx)
    }
}
