use std::str::Chars;

pub struct Cursor<'a> {
  initial_len: usize,
  chars: Chars<'a>,
  #[cfg(debug_assertions)]
  prev: char,
}

pub const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
  pub fn new(input: &'a str) -> Cursor<'a> {
    Cursor {
      initial_len: input.len(),
      chars: input.chars(),
      #[cfg(debug_assertions)]
      prev: EOF_CHAR,
    }
  }

  pub fn bump(&mut self) -> Option<char> {
    let c = self.chars.next()?;

    #[cfg(debug_assertions)]
    {
      self.prev = c;
    }

    Some(c)
  }

  /// Peeks the next symbol from the input stream without consuming it.
  pub fn first(&self) -> char {
    self.nth_char(0)
  }

  /// Returns nth character relative to the current cursor position.
  /// If requested position doesn't exist, `EOF_CHAR` is returned.
  /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
  /// it should be checked with `is_eof` method.
  fn nth_char(&self, n: usize) -> char {
    self.chars().nth(n).unwrap_or(EOF_CHAR)
  }

  /// Returns a `Chars` iterator over the remaining characters.
  fn chars(&self) -> Chars<'a> {
    self.chars.clone()
  }
  /// Checks if there is nothing more to consume.
  pub fn is_eof(&self) -> bool {
    self.chars.as_str().is_empty()
  }

  /// Returns amount of already consumed symbols.
  pub fn len_consumed(&self) -> usize {
    self.initial_len - self.chars.as_str().len()
  }
}
