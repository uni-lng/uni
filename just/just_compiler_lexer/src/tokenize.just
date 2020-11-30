pub let tokenize = (mut input: &str) => {
  if isEmpty(input) return []

  let token = {
    kind: "whitespace",
    len: 1
  }

  return [token, ..tokenize(&input[token.len..])]
}
