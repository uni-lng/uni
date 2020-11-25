const closure = () {
  let state = 1
  const dec = () state--
  return {
    inc = () {
      state++
    }
  }
}

pub const main = fn () "Hello, World"
