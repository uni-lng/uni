use crate::errors::*;
use std::convert::TryInto;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
  /// "12_u8", "0o100", "0b120i99"
  Int { base: NumericBase, empty_int: bool },
  /// "12.34f32", "0b100.100"
  Float {
    base: NumericBase,
    empty_exponent: bool,
  },
  /// "'a'", "'\\'", "'''", "';"
  Char { terminated: bool },
  /// "b'a'", "b'\\'", "b'''", "b';"
  Byte { terminated: bool },
  /// ""abc"", ""abc"
  Str { terminated: bool },
  /// "b"abc"", "b"abc"
  ByteStr { terminated: bool },
  /// "r"abc"", "r#"abc"#", "r####"ab"###"c"####", "r#"a"
  RawStr(UnvalidatedRawStr),
  /// "br"abc"", "br#"abc"#", "br####"ab"###"c"####", "br#"a"
  RawByteStr(UnvalidatedRawStr),
}

/// Raw String that contains a valid prefix (`#+"`) and postfix (`"#+`) where
/// there are a matching number of `#` characters in both. Note that this will
/// not consume extra trailing `#` characters: `r###"abcde"####` is lexed as a
/// `ValidatedRawString { n_hashes: 3 }` followed by a `#` token.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ValidatedRawStr {
  pub(crate) n_hashes: u16,
}

/// Represents something that looks like a raw string, but may have some
/// problems. Use `.validate()` to convert it into something
/// usable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnvalidatedRawStr {
  /// The prefix (`r###"`) is valid
  pub(crate) valid_start: bool,

  /// The postfix (`"###`) is valid
  pub(crate) valid_end: bool,

  /// The number of leading `#`
  pub(crate) n_start_hashes: usize,
  /// The number of trailing `#`. `n_end_hashes` <= `n_start_hashes`
  pub(crate) n_end_hashes: usize,
  /// The offset starting at `r` or `br` where the user may have intended to end the string.
  /// Currently, it is the longest sequence of pattern `"#+"`.
  pub(crate) possible_terminator_offset: Option<usize>,
}

impl UnvalidatedRawStr {
  pub fn validate(self) -> Result<ValidatedRawStr, LexRawStrError> {
    if !self.valid_start {
      return Err(LexRawStrError::InvalidStarter);
    }

    // Only up to 65535 `#`s are allowed in raw strings
    let n_start_safe: u16 = self
      .n_start_hashes
      .try_into()
      .map_err(|_| LexRawStrError::TooManyDelimiters)?;

    if self.n_start_hashes > self.n_end_hashes || !self.valid_end {
      Err(LexRawStrError::NoTerminator {
        expected: self.n_start_hashes,
        found: self.n_end_hashes,
        possible_terminator_offset: self.possible_terminator_offset,
      })
    } else {
      // Since the lexer should never produce a literal with n_end > n_start, if n_start <= n_end,
      // they must be equal.
      debug_assert_eq!(self.n_start_hashes, self.n_end_hashes);
      Ok(ValidatedRawStr {
        n_hashes: n_start_safe,
      })
    }
  }
}

/// Base of numeric literal encoding according to its prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumericBase {
  /// Literal starts with "0b".
  Binary,
  /// Literal starts with "0o".
  Octal,
  /// Literal starts with "0x".
  Hexadecimal,
  /// Literal doesn't contain a prefix.
  Decimal,
}
