//!
//!
//!

use std::str::Chars;

pub(crate) const EOF_CHAR: char = '\0';

/**
 * Peekable iterator over a char sequence.
 */
pub struct Cursor<'a> {
  initial_len: usize,
  chars: Chars<'a>,
  #[cfg(debug_assertions)]
  prev: char,
}

impl<'a> Cursor<'a> {
  pub fn new(input: &'a str) -> Cursor<'a> {
    Cursor {
      initial_len: input.len(),
      chars: input.chars(),
      #[cfg(debug_assertions)]
      prev: EOF_CHAR,
    }
  }

  /// Moves to the next character.
  pub fn bump(&mut self) -> Option<char> {
    let c = self.chars.next()?;
    #[cfg(debug_assertions)]
    {
      self.prev = c;
    }

    Some(c)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty_bump_to_none() {
    let mut c = Cursor::new("");
    assert_eq!(None, c.bump());
  }
}
