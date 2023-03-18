use std::collections::HashMap;

use crate::syntax::Token;

/// A symbol is an index into a vector of spans. This allows us to store strings
/// without any copies; it's just an pointer into the original bytes.
pub type InternIndex = usize;

/// A mechanism for interning string symbols.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Interner<'i> {
    input: &'i str,
    table: HashMap<&'i str, InternIndex>,
    spans: Vec<&'i str>,
}

impl<'i> Interner<'i> {
    pub fn new(input: &'i str) -> Interner<'i> {
        let table = HashMap::new();
        let spans = Vec::new();
        Interner {
            input,
            table,
            spans,
        }
    }

    pub fn intern(&mut self, token: &Token) -> InternIndex {
        let span = crate::symbol::span(self.input, token);
        if let Some(symbol) = self.table.get(span) {
            return *symbol;
        }
        self.spans.push(span);
        let symbol = self.spans.len() - 1;
        self.table.insert(span, symbol);
        symbol
    }

    pub fn lookup(&self, symbol: InternIndex) -> Option<&'i str> {
        // Copies a pointer to the span, not the string
        self.spans.get(symbol).copied()
    }
}
