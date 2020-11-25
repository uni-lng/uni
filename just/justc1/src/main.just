
use std::path;
use std::env;
use std::fs;

pub fn main() {
  let args = env::args().collect();
  let dir = &args[1];
  compile(&dir);
}

fn compile(dir: &str) {
  let p = path::Path::new(dir);
  let x = fs::read_dir(p);
}
