use std.str.Chars

pub let tokenize = (mut input: &str) => {
  if isEmpty(input) return []

  let token = getFirstToken(input)

  let remaining = tokenize(&input[token.len..])
  return remaining.shift(token)
}

let getFirstToken = (input: &str) => {
  return advanceToken(Cursor.new(input))
}

type Cursor = {
  initial_len: uint,
  chars: Chars
}

Cursor.new = (input: &str) => {
  return {
    initial_len: input.len(),
    chars: input.chars(),
  }: Cursor
}

let advanceToken = (cur: &mut Cursor) => {
  let firstChar = cur.peek().unwrap()
  return match firstChar {
    Whitespace.match => Whitespace.token(cur),
    Identifier.match => Identifier.token(cur),
    _ => "unknown"
  }

  return { kind, len: 1 }
}

let isWhitespace = (c: char) => c == ' '

let isIdStart = (c: char) => ('a' <= c && c <= 'z')
    || ('A' <= c && c <= 'Z')
    || c == '_'
