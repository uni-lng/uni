use super::token::Token;

pub fn parse(source: &str) {
  tokenize(source);
}

fn tokenize(source: &str) -> Vec<Token> {
  let tokens = Vec::new();

  tokens
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty_source() {
    let source = "";
    let actual = tokenize(source);
    assert_eq!(0, actual.len());
  }
}
