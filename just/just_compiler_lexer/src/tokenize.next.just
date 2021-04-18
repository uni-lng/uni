use std.str.Chars

pub let tokenize = (mut input: &str) => {
  if isEmpty(input) return []

  let token = getFirstToken(input)

  return [token, ..tokenize(&input[token.len..])]
}

let getFirstToken = (input: &str) => Cursor.new(input).advanceToken()

type Cursor = {
  initial_len: uint,
  chars: Chars
}

Cursor.new = (input: &str) => ({
  initial_len: input.len(),
  chars: input.chars(),
}: Cursor)

let advanceToken = (this: &mut Cursor) => {
  return {}: Token
}
