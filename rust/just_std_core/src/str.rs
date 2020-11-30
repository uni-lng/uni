#![allow(warnings)]

pub fn isEmpty(value: &str) -> bool {
  value.len() == 0
}

#[cfg(test)]
mod isEmpty_tests {
  use super::*;

  #[test]
  fn empty_true() {
    assert_eq!(true, isEmpty(""));
  }

  #[test]
  fn non_empty_false() {
      assert_eq!(false, isEmpty("a"));
  }
}
