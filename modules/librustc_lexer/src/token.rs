use crate::token_kind::TokenKind;

/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
pub struct Token {
  pub kind: TokenKind,
  pub len: usize,
}

impl Token {
  pub(crate) fn new(kind: TokenKind, len: usize) -> Token {
    Token { kind, len }
  }
}
