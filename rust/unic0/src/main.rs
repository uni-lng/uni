mod compiler_options;

use clap::clap_app;
use compiler_options::CompilerOptions;

pub fn main() {
  let matches = clap_app!(myapp =>
    (version: "0.1")
    (author: "Homa wong <homawong@gmail.com>")
    (about: "Uni compiler")
    (@arg files: ... "file(s) to compile")
  )
  .get_matches();

  let options = CompilerOptions::from_arg_matches(&matches);
  println!("matches: {:?}", options);
}
