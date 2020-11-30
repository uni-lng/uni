/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
#[derive(Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub len: usize,
}

/// Enum representing common lexeme types.
// perf note: Changing all `usize` to `u32` doesn't change performance. See #77629
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
  /// Any whitespace characters sequence.
  Whitespace,
}

use TokenKind::*;

pub fn tokenize(mut input: &str) -> impl Iterator<Item = Token> + '_ {
  std::iter::from_fn(move || {
    if input.is_empty() {
      return None;
    }
    let token = Token {
      kind: Whitespace,
      len: 1
    };
    input = &input[token.len..];
    return Some(token);
  })
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
}
