use crate::syntax::Token;

pub fn span<'i>(input: &'i str, token: &Token) -> &'i str {
    &input[token.start..token.start + token.len]
}
