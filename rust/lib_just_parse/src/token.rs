/**
 * Parsed Token.
 * It doesn't contain information about data that has been parsed,
 * only the type of the token and its size.
 * This is because the source code is owned elsewhere.
 */
pub struct Token {
  pub kind: TokenKind,
  pub len: usize
}
