use super::cursor::Cursor;

/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
#[derive(Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub len: usize,
}

impl Token {
  fn new(kind: TokenKind, len: usize) -> Token {
    Token { kind, len }
  }
}

/// Enum representing common lexeme types.
// perf note: Changing all `usize` to `u32` doesn't change performance. See #77629
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
  /// Any whitespace characters sequence.
  Whitespace,
  /// Identifier, including keywords
  Identifier,
  /// Unknown token, not expected by the lexer, e.g. "№"
  Unknown,
}

use TokenKind::*;

pub fn tokenize(mut input: &str) -> impl Iterator<Item = Token> + '_ {
  std::iter::from_fn(move || {
    if input.is_empty() {
      return None;
    }

    let token = first_token(input);
    input = &input[token.len..];
    return Some(token);
  })
}

fn first_token(input: &str) -> Token {
  Cursor::new(input).advance_token()
}

impl Cursor<'_> {
  fn advance_token(&mut self) -> Token {
    let first_char = self.bump().unwrap();
    let token_kind = match first_char {
      c if is_whitespace(c) => Whitespace,
      c if is_id_start(c) => self.identifier(),
      _ => Unknown,
    };

    Token::new(token_kind, self.len_consumed())
  }
  fn identifier(&mut self) -> TokenKind {
    self.eat_while(is_id_continue);
    Identifier
  }

  fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
    while predicate(self.first()) && !self.is_eof() {
      self.bump();
    }
  }
}

/// True if `c` is considered a whitespace according to Rust language definition.
/// See [Rust language reference](https://doc.rust-lang.org/reference/whitespace.html)
/// for definitions of these classes.
pub fn is_whitespace(c: char) -> bool {
  // This is Pattern_White_Space.
  //
  // Note that this set is stable (ie, it doesn't change with different
  // Unicode versions), so it's ok to just hard-code the values.

  matches!(
    c,
    // Usual ASCII suspects
    '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
  )
}

/// True if `c` is valid as a first character of an identifier.
/// See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for
/// a formal definition of valid identifier name.
pub fn is_id_start(c: char) -> bool {
  // This is XID_Start OR '_' (which formally is not a XID_Start).
  // We also add fast-path for ascii idents
  ('a' <= c && c <= 'z')
    || ('A' <= c && c <= 'Z')
    || c == '_'
    || (c > '\x7f' && unicode_xid::UnicodeXID::is_xid_start(c))
}

/// True if `c` is valid as a non-first character of an identifier.
/// See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for
/// a formal definition of valid identifier name.
pub fn is_id_continue(c: char) -> bool {
  // This is exactly XID_Continue.
  // We also add fast-path for ascii idents
  ('a' <= c && c <= 'z')
    || ('A' <= c && c <= 'Z')
    || ('0' <= c && c <= '9')
    || c == '_'
    || (c > '\x7f' && unicode_xid::UnicodeXID::is_xid_continue(c))
}

#[cfg(test)]
mod tests {
  use super::*;
  use expect_test::{expect, Expect};

  fn check_lexing(src: &str, expect: Expect) {
    let actual: String = tokenize(src)
      .map(|token| format!("{:?}\n", token))
      .collect();
    expect.assert_eq(&actual)
  }

  #[test]
  fn empty_input() {
    check_lexing("", expect![[r#""#]]);
  }

  #[test]
  fn whitespace() {
    check_lexing(
      " ",
      expect![[r#"
    Token { kind: Whitespace, len: 1 }
    "#]],
    );
  }
  #[test]
  fn identifier() {
    check_lexing(
      "pub",
      expect![[r#"
    Token { kind: Identifier, len: 3 }
    "#]],
    );
  }
}
