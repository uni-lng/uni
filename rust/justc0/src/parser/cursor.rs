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

  /// For debug assertions only
  /// Returns the last eaten symbol (or '\0' in release builds).
  pub(crate) fn prev(&self) -> char {
    #[cfg(debug_assertions)]
    {
      self.prev
    }

    #[cfg(not(debug_assertions))]
    {
      '\0'
    }
  }

  pub(crate) fn peek(&self) -> char {
    self.peek_nth(0)
  }

  /// Returns nth character relative to the current cursor position.
  /// If requested position doesn't exist, `EOF_CHAR` is returned.
  /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
  /// it should be checked with `is_eof` method.
  pub(crate) fn peek_nth(&self, n: usize) -> char {
    self.chars().nth(n).unwrap_or(EOF_CHAR)
  }

  pub(crate) fn seek<F>(&self, predicate: F) -> usize
  where
    F: FnMut(char) -> bool,
  {
    0
  }

  /// Checks if there is nothing more to consume.
  pub(crate) fn is_eof(&self) -> bool {
    self.chars.as_str().is_empty()
  }

  /// Returns amount of already consumed symbols.
  pub(crate) fn len_consumed(&self) -> usize {
    self.initial_len - self.chars.as_str().len()
  }

  /// Create a new `Cursor` at the current position.
  pub(crate) fn fork(&self) -> Self {
    Cursor {
      initial_len: self.chars.as_str().len(),
      chars: self.chars(),
      #[cfg(debug_assertions)]
      prev: EOF_CHAR,
    }
  }

  /// Moves to the next character.
  pub(crate) fn bump(&mut self) -> Option<char> {
    let c = self.chars.next()?;
    #[cfg(debug_assertions)]
    {
      self.prev = c;
    }

    Some(c)
  }

  /// Moves to the next n character.
  pub(crate) fn bump_n(&mut self, n: usize) -> Option<char> {
    if n == 0 {
      return None;
    }

    let c = self.chars.next()?;
    #[cfg(debug_assertions)]
    {
      self.prev = c;
    }

    if n == 1 {
      return Some(c);
    }

    self.bump_n(n - 1)
  }

  /// Returns a `Chars` iterator over the remaining characters.
  fn chars(&self) -> Chars<'a> {
    self.chars.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn peek_empty_is_eof_char() {
    assert_eq!(EOF_CHAR, Cursor::new("").peek());
  }

  #[test]
  fn peek_current_char() {
    assert_eq!('A', Cursor::new("A").peek());
  }

  #[test]
  fn peek_nth_not_exist_gets_eof_char() {
    assert_eq!(EOF_CHAR, Cursor::new("A").peek_nth(1))
  }

  #[test]
  fn peek_nth_char() {
    assert_eq!('B', Cursor::new("AB").peek_nth(1));
  }

  #[test]
  fn bump_empty_get_none() {
    assert_eq!(None, Cursor::new("").bump());
  }

  #[test]
  fn bump_get_current_char() {
    assert_eq!(Some('A'), Cursor::new("A").bump());
  }

  #[test]
  fn fork_do_not_affect_orig_cursor() {
    let c = Cursor::new("ABC");
    let mut d = c.fork();
    assert_eq!(Some('A'), d.bump());
    assert_eq!('B', d.peek());
    assert_eq!('A', c.peek());
  }
}
