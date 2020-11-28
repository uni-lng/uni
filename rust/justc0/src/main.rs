use std::env;
use std::fs;
use std::path;

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let dir = &args[1];
  compile(&dir);
}

fn compile(dir: &str) {
  let p = path::Path::new(dir);

  let x = fs::read_dir(p);
  println!("{}", x.unwrap().count());
}

#[cfg(test)]
mod tests {
  // use super::*;

  #[test]
  fn empty_source() {
    // let source = "";
    // assert_eq!(vec![], tokenize(source));
  }
}
