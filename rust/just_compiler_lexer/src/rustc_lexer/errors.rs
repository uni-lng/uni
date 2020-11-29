/// Error produced validating a raw string. Represents cases like:
/// - `r##~"abcde"##`: `LexRawStrError::InvalidStarter`
/// - `r###"abcde"##`: `LexRawStrError::NoTerminator { expected: 3, found: 2, possible_terminator_offset: Some(11)`
/// - Too many `#`s (>65536): `TooManyDelimiters`
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LexRawStrError {
  /// Non `#` characters exist between `r` and `"` eg. `r#~"..`
  InvalidStarter,
  /// The string was never terminated. `possible_terminator_offset` is the number of characters after `r` or `br` where they
  /// may have intended to terminate it.
  NoTerminator {
    expected: usize,
    found: usize,
    possible_terminator_offset: Option<usize>,
  },
  /// More than 65536 `#`s exist.
  TooManyDelimiters,
}
