use std.str
use std.iter
use std.slice.isEmpty

pub let tokenize = (mut input: &str) => {
  return iter(() => {
    if (isEmpty(input)) {
      return null
    }
    let token: Token = {
      kind: Whitespace,
      len: 1
    }
    input = &input[token.len..]
    return token
  })
}: iter<Token>
