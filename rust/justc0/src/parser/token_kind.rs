use super::literal_kind::LiteralKind;

/// Enum representing common lexeme types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
  // Multi-char tokens:
  /// "// comment"
  LineComment,
  /// "/* block comment */"
  /// Block comments can be recursive, so the sequence like "/* /* */"
  /// will not be considered terminated and will result in a parsing error.
  BlockComment { terminated: bool },
  /// Any whitespace characters sequence.
  Whitespace,
  /// "ident" or "continue"
  /// At this step keywords are also considered identifiers.
  Ident,
  /// "r#ident"
  RawIdent,
  /// "12_u8", "1.0e-40", "b"123"". See `LiteralKind` for more details.
  Literal {
    kind: LiteralKind,
    suffix_start: usize,
  },
  /// "'a" UNI: likely will be introduced, but in different syntax.
  Lifetime { starts_with_number: bool },

  // One-char tokens:
  /// ";"
  Semi,
  /// ","
  Comma,
  /// "."
  Dot,
  /// "("
  OpenParen,
  /// ")"
  CloseParen,
  /// "{"
  OpenBrace,
  /// "}"
  CloseBrace,
  /// "["
  OpenBracket,
  /// "]"
  CloseBracket,
  /// "@"
  At,
  /// "#"
  Pound,
  /// "~"
  Tilde,
  /// "?"
  Question,
  /// ":"
  Colon,
  /// "$"
  Dollar,
  /// "="
  Eq,
  /// "!"
  Not,
  /// "<"
  Lt,
  /// ">"
  Gt,
  /// "-"
  Minus,
  /// "&"
  And,
  /// "|"
  Or,
  /// "+"
  Plus,
  /// "*"
  Star,
  /// "/"
  Slash,
  /// "\"
  BackSlash,
  /// "^"
  Caret,
  /// "%"
  Percent,

  /// Unknown token, not expected by the lexer, e.g. "№"
  Unknown,
}
